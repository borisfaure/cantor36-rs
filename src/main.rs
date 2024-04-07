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
use embassy_usb::class::hid::{HidReaderWriter, State};
use embassy_usb::Builder;

use crate::hid::hid_writer_handler;
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
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
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

    let request_handler = hid::HidRequestHandler::new();
    let mut device_handler = hid::DeviceHandler::new();

    let mut state = State::new();

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
    let hid_config = config::hid_config(&request_handler);
    let hid = HidReaderWriter::<_, 64, 64>::new(&mut builder, &mut state, hid_config);

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();

    let (hid_reader, hid_writer) = hid.split();
    let hid_reader_fut = async {
        hid_reader.run(false, &request_handler).await;
    };
    let hid_writer_fut = hid_writer_handler(hid_writer);

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

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.

    future::join5(
        usb_fut,
        hid_reader_fut,
        hid_writer_fut,
        matrix_fut,
        layout_fut,
    )
    .await;
}
