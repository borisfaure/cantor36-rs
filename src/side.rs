use crate::layout::LAYOUT_CHANNEL;
use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_stm32::peripherals::USART1;
use embassy_stm32::usart::{BufferedUartRx, BufferedUartTx};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_usb::Handler;
use embedded_io_async::{Read, Write};
use keyberon::layout::Event;

/// Number of events in the channel to the other half of the keyboard
const NB_EVENTS: usize = 8;
/// Channel to send `keyberon::layout::event` events to the layout handler
pub static SIDE_CHANNEL: Channel<CriticalSectionRawMutex, Event, NB_EVENTS> = Channel::new();

/// Serialized size of a key event
pub const SERIALIZED_SIZE: usize = 4;
/// Buffer size for serialized key events
pub const SERIAL_BUF_SIZE: usize = 8 * SERIALIZED_SIZE;
/// USART baudrate
pub const USART_BAUDRATE: u32 = 38_400;

/// Deserialize a key event from the serial line
fn deserialize(bytes: &[u8; SERIALIZED_SIZE]) -> Result<Event, ()> {
    match *bytes {
        [b'P', i, j, b'\n'] => Ok(Event::Press(i, j)),
        [b'R', i, j, b'\n'] => Ok(Event::Release(i, j)),
        _ => Err(()),
    }
}

/// Serialize a key event
fn serialize(e: Event) -> [u8; SERIALIZED_SIZE] {
    match e {
        Event::Press(i, j) => [b'P', i, j, b'\n'],
        Event::Release(i, j) => [b'R', i, j, b'\n'],
    }
}

/// Receive key events from the other half of the keyboard
pub async fn usart_rx(mut buf_usart: BufferedUartRx<'_, USART1>) {
    loop {
        let mut buf: [u8; SERIALIZED_SIZE] = [0; SERIALIZED_SIZE];
        buf_usart.read_exact(&mut buf).await.unwrap();
        match deserialize(&buf) {
            Ok(event) => {
                LAYOUT_CHANNEL.send(event).await;
            }
            Err(()) => {
                warn!("Invalid event received: {:?}", buf);
            }
        }
    }
}

/// Send key events to the other half of the keyboard
pub async fn usart_tx(mut buf_usart: BufferedUartTx<'_, USART1>) {
    loop {
        let event = SIDE_CHANNEL.receive().await;
        let buf = serialize(event);
        buf_usart.write_all(&buf).await.unwrap();
        buf_usart.flush().await.unwrap();
    }
}

/// Device configured flag
static CONFIGURED: AtomicBool = AtomicBool::new(false);

/// Whether the device is the host or not
pub fn is_host() -> bool {
    CONFIGURED.load(Ordering::Relaxed)
}

/// Device Handler, used to know when it's configured
pub struct DeviceHandler {}

impl DeviceHandler {
    /// Create a new Device Handler
    pub fn new() -> Self {
        DeviceHandler {}
    }
}

impl Handler for DeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        CONFIGURED.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        CONFIGURED.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        CONFIGURED.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        CONFIGURED.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
