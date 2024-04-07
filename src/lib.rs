#![no_std]
#![no_main]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Firmware for the [Cantor31 keyboard](https://github.com/borisfaure/cantor36)

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::info;
use embassy_stm32::Peripherals;
use embassy_usb::class::hid::{ReportId, RequestHandler};
use embassy_usb::control::OutResponse;
use embassy_usb::Handler;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

/// Keyboard matrix handling
pub mod keys;

/// USB VID based on
/// <https://github.com/obdev/v-usb/blob/master/usbdrv/USB-IDs-for-free.txt>
const VID: u16 = 0x16c0;

/// USB PID
const PID: u16 = 0x27db;

/// USB Product
const PRODUCT: &str = "Cantor36 keyboard";
/// USB Manufacturer
const MANUFACTURER: &str = "Boris Faure";

/// Initialize the STM32 device
pub fn init_device() -> Peripherals {
    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::{
            mux, AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv,
            PllQDiv, PllSource, Sysclk,
        };
        use embassy_stm32::time::Hertz;
        config.rcc.hse = Some(Hse {
            freq: Hertz(25_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV25,
            mul: PllMul::MUL336,
            divp: Some(PllPDiv::DIV4), // 25mhz / 25 * 336 / 4 = 84Mhz.
            divq: Some(PllQDiv::DIV7), // 25mhz / 25 * 336 / 7 = 48Mhz.
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    }
    embassy_stm32::init(config)
}

/// Generate the STM32 USB configuration
pub fn stm32_usb_config() -> embassy_stm32::usb::Config {
    let mut config = embassy_stm32::usb::Config::default();
    config.vbus_detection = false;
    config
}

/// Generate the Embassy-USB configuration
pub fn usb_config() -> embassy_usb::Config<'static> {
    let mut config = embassy_usb::Config::new(VID, PID);
    config.manufacturer = Some(MANUFACTURER);
    config.product = Some(PRODUCT);
    config.serial_number = Some("CARGO_PKG_VERSION");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;
    config
}

/// Generate HID config
pub fn hid_config<'a>(
    request_handler: &'a dyn RequestHandler,
) -> embassy_usb::class::hid::Config<'a> {
    embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: Some(request_handler),
        poll_ms: 60,
        max_packet_size: 8,
    }
}

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
