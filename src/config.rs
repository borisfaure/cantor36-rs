#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use embassy_stm32::Peripherals;

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
    config.serial_number = Some(env!("CARGO_PKG_VERSION"));
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
