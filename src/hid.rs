use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_stm32::peripherals::USB_OTG_FS;
use embassy_stm32::usb::Driver;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_usb::class::hid::{ReportId, RequestHandler};
use embassy_usb::control::OutResponse;
use embassy_usb::Handler;
use keyberon::key_code::KbHidReport;
use usbd_hid::descriptor::KeyboardReport;

const NB_REPORTS: usize = 1;
pub static HID_CHANNEL: Channel<ThreadModeRawMutex, KbHidReport, NB_REPORTS> = Channel::new();

pub type HidWriter<'a, 'b> = embassy_usb::class::hid::HidWriter<'a, Driver<'b, USB_OTG_FS>, 64>;

/// HID handler
pub struct HidRequestHandler {}
impl HidRequestHandler {
    /// Create a new HID request handler
    pub fn new() -> Self {
        HidRequestHandler {}
    }
}

impl RequestHandler for HidRequestHandler {
    fn get_report(&self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

fn convert_report(_kbhr: KbHidReport) -> KeyboardReport {
    KeyboardReport {
        keycodes: [4, 0, 0, 0, 0, 0],
        leds: 0,
        modifier: 0,
        reserved: 0,
    }
}

/// Loop to read HID reports from the HID channel and send them to the HID writer
pub async fn hid_writer_handler<'a>(mut writer: HidWriter<'a, 'a>) {
    loop {
        let report = HID_CHANNEL.receive().await;
        let hid_report: KeyboardReport = convert_report(report);
        match writer.write_serialize(&hid_report).await {
            Ok(()) => {}
            Err(e) => warn!("Failed to send report: {:?}", e),
        };
    }
}

/// Device Handler, used to know when it's configured
pub struct DeviceHandler {
    configured: AtomicBool,
}

impl DeviceHandler {
    /// Create a new Device Handler
    pub fn new() -> Self {
        DeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for DeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
