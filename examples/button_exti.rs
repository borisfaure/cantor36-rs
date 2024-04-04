#![deny(warnings)]
#![no_main]
#![no_std]

/// Set LED on when button is pressed, off when button is released.
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::Config;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);
    info!("Hello World!");

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);
    let mut button = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up);

    info!("Press the USER button...");

    loop {
        button.wait_for_rising_edge().await;
        led.set_high();
        info!("Pressed!");
        button.wait_for_falling_edge().await;
        info!("Released!");
        led.set_low();
    }
}
