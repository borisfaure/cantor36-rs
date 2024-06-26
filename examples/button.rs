#![deny(warnings)]
#![no_main]
#![no_std]

/// Set LED on when button is pressed, off when button is released.
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::Config;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);
    let button = Input::new(p.PA0, Pull::Up);

    loop {
        if button.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
