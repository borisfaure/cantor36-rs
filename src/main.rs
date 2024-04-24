#![no_std]
#![no_main]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Firmware for the [Cantor31 keyboard](https://github.com/borisfaure/cantor36)

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::usart;
use embassy_usb::class::hid::{HidReaderWriter, HidWriter, State};
use embassy_usb::Builder;
use usbd_hid::descriptor::{KeyboardReport, MouseReport, SerializedDescriptor};

use crate::hid::{hid_kb_writer_handler, hid_mouse_writer_handler};
use crate::side::{SERIAL_BUF_SIZE, USART_BAUDRATE};
use futures::future;
use panic_probe as _;

/// Configuration
mod config;
/// USB HID configuration
mod hid;
/// Key handling
mod keys;
/// Layout events processing
mod layout;
/// Act as a mouse
mod mouse;
/// Handling the other half of the keyboard
mod side;

/// Basic layout for the keyboard
#[cfg(feature = "keymap_basic")]
mod keymap_basic;

/// Keymap by Boris Faure
#[cfg(feature = "keymap_borisfaure")]
mod keymap_borisfaure;

/// Test layout for the keyboard
#[cfg(feature = "keymap_test")]
mod keymap_test;

#[cfg(not(any(feature = "right", feature = "left",)))]
compile_error!("Either feature \"right\" or \"left\" must be enabled.");

#[cfg(not(any(
    feature = "keymap_borisfaure",
    feature = "keymap_basic",
    feature = "keymap_test"
)))]
compile_error!(
    "Either feature \"keymap_basic\" or \"keymap_borisfaure\" or \"keymap_test\" must be enabled."
);

bind_interrupts!(struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
    USART1 => usart::BufferedInterruptHandler<embassy_stm32::peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = config::init_device();

    // Create the driver, from the HAL.
    let mut ep_out_buffer = [0u8; 256];
    let driver = embassy_stm32::usb::Driver::new_fs(
        p.USB_OTG_FS,
        Irqs,
        p.PA12,
        p.PA11,
        &mut ep_out_buffer,
        config::stm32_usb_config(),
    );

    // Create embassy-usb Config
    let usb_config = config::usb_config();

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    // You can also add a Microsoft OS descriptor.
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut device_handler = side::DeviceHandler::new();

    let mut state_kb = State::new();
    let mut state_mouse = State::new();

    let mut builder = Builder::new(
        driver,
        usb_config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    builder.handler(&mut device_handler);

    // Create classes on the builder.
    let hidkb_config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 8,
    };
    let hidkb = HidReaderWriter::<_, 64, 64>::new(&mut builder, &mut state_kb, hidkb_config);

    let hidm_config = embassy_usb::class::hid::Config {
        report_descriptor: MouseReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 4,
    };
    let hidm = HidWriter::<_, 64>::new(&mut builder, &mut state_mouse, hidm_config);

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();

    let mut request_handler = hid::HidRequestHandler::new(&spawner);
    let (hid_kb_reader, hid_kb_writer) = hidkb.split();
    let hid_kb_reader_fut = async {
        hid_kb_reader.run(false, &mut request_handler).await;
    };
    let hid_kb_writer_fut = hid_kb_writer_handler(hid_kb_writer);
    let hid_mouse_writer_fut = hid_mouse_writer_handler(hidm);

    let matrix = [
        [
            Some(Input::new(p.PB10, Pull::Up)),
            Some(Input::new(p.PA8, Pull::Up)),
            Some(Input::new(p.PB15, Pull::Up)),
            Some(Input::new(p.PB14, Pull::Up)),
            Some(Input::new(p.PB13, Pull::Up)),
        ],
        [
            Some(Input::new(p.PB8, Pull::Up)),
            Some(Input::new(p.PB5, Pull::Up)),
            Some(Input::new(p.PB4, Pull::Up)),
            Some(Input::new(p.PB3, Pull::Up)),
            Some(Input::new(p.PA15, Pull::Up)),
        ],
        [
            Some(Input::new(p.PA4, Pull::Up)),
            Some(Input::new(p.PA5, Pull::Up)),
            Some(Input::new(p.PA6, Pull::Up)),
            Some(Input::new(p.PA7, Pull::Up)),
            Some(Input::new(p.PB0, Pull::Up)),
        ],
        [
            None,
            None,
            Some(Input::new(p.PA2, Pull::Up)),
            Some(Input::new(p.PA1, Pull::Up)),
            Some(Input::new(p.PA0, Pull::Up)),
        ],
    ];
    let matrix_fut = keys::matrix_scanner(matrix);

    let layout_fut = layout::layout_handler();

    let mut tx_buf = [0u8; SERIAL_BUF_SIZE];
    let mut rx_buf = [0u8; SERIAL_BUF_SIZE];
    let mut usart_config = usart::Config::default();
    usart_config.baudrate = USART_BAUDRATE;
    let buf_usart = usart::BufferedUart::new(
        p.USART1,
        Irqs,
        p.PB7,
        p.PB6,
        &mut tx_buf,
        &mut rx_buf,
        usart_config,
    )
    .unwrap();
    let (usart_writer, usart_reader) = buf_usart.split();
    let usart_rx_fut = side::usart_rx(usart_reader);
    let usart_tx_fut = side::usart_tx(usart_writer);

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.

    future::join4(
        future::join3(usb_fut, usart_rx_fut, usart_tx_fut),
        future::join3(hid_kb_reader_fut, hid_kb_writer_fut, hid_mouse_writer_fut),
        matrix_fut,
        layout_fut,
    )
    .await;
}
