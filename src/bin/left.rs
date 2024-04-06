#![no_std]
#![no_main]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Firmware for the [Cantor31 keyboard](https://github.com/borisfaure/cantor36)

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_usb::class::hid::{HidReaderWriter, State};
use embassy_usb::Builder;
use futures::future::join;
use panic_probe as _;
use usbd_hid::descriptor::KeyboardReport;

use cantor36_rs::{
    hid_config, init_device, stm32_usb_config, usb_config, DeviceHandler, HidRequestHandler,
};

#[cfg(not(any(
    feature = "keymap_borisfaure",
    feature = "keymap_basic",
    feature = "keymap_test"
)))]
compile_error!(
    "Either feature \"keymap_basic\" or \"keymap_borisfaure\" or \"keymap_test\" must be enabled."
);

/// Basic layout for the keyboard
#[cfg(feature = "keymap_basic")]
mod keymap_basic;
#[cfg(feature = "keymap_basic")]
use keymap_basic::{KBLayout, LAYERS};

/// Keymap by Boris Faure
//#[cfg(feature = "keymap_borisfaure")]
//mod keymap_borisfaure;
//#[cfg(feature = "keymap_borisfaure")]
//use keymap_borisfaure::{KBLayout, LAYERS};

/// Test layout for the keyboard
#[cfg(feature = "keymap_test")]
mod keymap_test;
#[cfg(feature = "keymap_test")]
use keymap_test::{KBLayout, LAYERS};

bind_interrupts!(struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = init_device();
    info!("Hello World!");

    // Create the driver, from the HAL.
    let mut ep_out_buffer = [0u8; 256];
    let driver = embassy_stm32::usb::Driver::new_fs(
        p.USB_OTG_FS,
        Irqs,
        p.PA12,
        p.PA11,
        &mut ep_out_buffer,
        stm32_usb_config(),
    );

    // Create embassy-usb Config
    let usb_config = usb_config();

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    // You can also add a Microsoft OS descriptor.
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let request_handler = HidRequestHandler::new();
    let mut device_handler = DeviceHandler::new();

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
    let hid_config = hid_config(&request_handler);
    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, hid_config);

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();

    let (reader, mut writer) = hid.split();

    let mut button = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up);
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    // Do stuff with the class!
    let in_fut = async {
        loop {
            button.wait_for_falling_edge().await;
            info!("Button pressed!");
            // Create a report with the A key pressed. (no shift modifier)
            let report = KeyboardReport {
                keycodes: [4, 0, 0, 0, 0, 0],
                leds: 0,
                modifier: 0,
                reserved: 0,
            };
            // Send the report.
            match writer.write_serialize(&report).await {
                Ok(()) => {}
                Err(e) => warn!("Failed to send report: {:?}", e),
            };
            led.set_low();

            button.wait_for_rising_edge().await;
            info!("Button released!");
            let report = KeyboardReport {
                keycodes: [0, 0, 0, 0, 0, 0],
                leds: 0,
                modifier: 0,
                reserved: 0,
            };
            match writer.write_serialize(&report).await {
                Ok(()) => {}
                Err(e) => warn!("Failed to send report: {:?}", e),
            };
            led.set_high();
        }
    };

    let out_fut = async {
        reader.run(false, &request_handler).await;
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, join(in_fut, out_fut)).await;
}
