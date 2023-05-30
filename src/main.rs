#![no_std]
#![no_main]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Firmware for the [Cantor31 keyboard](https://github.com/borisfaure/cantor36)

// Some panic handler needs to be included. This one halts the processor on panic.
//use panic_halt as _;
use defmt_rtt as _;
use panic_probe as _;

use hal::gpio::{EPin, Input};
use hal::otg_fs::{UsbBusType, USB};
use hal::prelude::*;
use hal::serial;
use hal::timer;
use keyberon::debounce::Debouncer;
use keyberon::key_code::KbHidReport;
use keyberon::layout::{Event, Layout};
use keyberon::matrix::DirectPinMatrix;
use nb::block;
use rtic::app;
use stm32f4xx_hal as hal;
use usb_device::bus::UsbBusAllocator;
use usb_device::class::UsbClass as _;
use usb_device::device::{UsbDeviceBuilder, UsbDeviceState, UsbVidPid};

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

/// Basic layout for the keyboard
#[cfg(feature = "keymap_basic")]
mod keymap_basic;
#[cfg(feature = "keymap_basic")]
use keymap_basic::{KBLayout, LAYERS};

/// Keymap by Boris Faure
#[cfg(feature = "keymap_borisfaure")]
mod keymap_borisfaure;
#[cfg(feature = "keymap_borisfaure")]
use keymap_borisfaure::{KBLayout, LAYERS};

/// Test layout for the keyboard
#[cfg(feature = "keymap_test")]
mod keymap_test;
#[cfg(feature = "keymap_test")]
use keymap_test::{KBLayout, LAYERS};

/// USB VID based on
/// https://github.com/obdev/v-usb/blob/master/usbdrv/USB-IDs-for-free.txt
const VID: u16 = 0x16c0;

/// USB PID
const PID: u16 = 0x27db;

/// USB Product
const PRODUCT: &str = "Cantor36 keyboard";
/// USB Manufacturer
const MANUFACTURER: &str = "Boris Faure";

/// USB Hid
type UsbClass = keyberon::Class<'static, UsbBusType, ()>;
/// USB Device
type UsbDevice = usb_device::device::UsbDevice<'static, UsbBusType>;
/// The Matrix
type Matrix = DirectPinMatrix<EPin<Input>, 5, 4>;

#[app(device = crate::hal::pac, dispatchers = [TIM1_CC])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        /// The USB device
        usb_dev: UsbDevice,
        /// The HID class
        usb_class: UsbClass,
        /// Layout of the keyboard
        #[lock_free]
        layout: KBLayout,
    }

    #[local]
    struct Local {
        /// Matrix
        matrix: Matrix,
        /// Debouncer: only on its own side
        debouncer: Debouncer<[[bool; 5]; 4]>,
        /// Timer when to scan the matrices
        timer: timer::counter::CounterHz<hal::pac::TIM2>,
        /// Transfert to the other side
        serial_tx: serial::Tx<hal::pac::USART1>,
        /// Receive from the other side
        serial_rx: serial::Rx<hal::pac::USART1>,
        /// Buffer to treat data from the other side
        serial_buf: [u8; 4],
    }

    #[init(local = [bus: Option<UsbBusAllocator<UsbBusType>> = None])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        /// Static memory for USB
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];

        defmt::info!("init");

        // setup the monotonic timer
        let clocks = cx
            .device
            .RCC
            .constrain()
            .cfgr
            .use_hse(25.MHz())
            .sysclk(84.MHz())
            .require_pll48clk()
            .freeze();

        // get GPIO pins
        let gpioa = cx.device.GPIOA.split();
        let gpiob = cx.device.GPIOB.split();

        let usb = USB {
            usb_global: cx.device.OTG_FS_GLOBAL,
            usb_device: cx.device.OTG_FS_DEVICE,
            usb_pwrclk: cx.device.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate().into(),
            pin_dp: gpioa.pa12.into_alternate().into(),
            hclk: clocks.hclk(),
        };

        *cx.local.bus = Some(UsbBusType::new(usb, unsafe { &mut EP_MEMORY }));
        let usb_bus = cx.local.bus.as_ref().unwrap();

        let usb_class = keyberon::new_class(usb_bus, ());
        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(VID, PID))
            .manufacturer(MANUFACTURER)
            .product(PRODUCT)
            .serial_number(env!("CARGO_PKG_VERSION"))
            .build();

        let mut timer = hal::timer::Timer::new(cx.device.TIM2, &clocks).counter_hz();
        timer.start(1.kHz()).unwrap();
        timer.listen(hal::timer::Event::Update);

        // Setup USART communication with other half
        let (pb6, pb7) = (gpiob.pb6, gpiob.pb7);
        let serial_pins = cortex_m::interrupt::free(move |_cs| {
            (pb6.into_alternate::<7>(), pb7.into_alternate::<7>())
        });
        let mut serial =
            serial::Serial::new(cx.device.USART1, serial_pins, 38_400.bps(), &clocks).unwrap();
        serial.listen(serial::Event::Rxne);
        let (serial_tx, serial_rx) = serial.split();

        let matrix_pins = [
            [
                Some(gpiob.pb10.into_pull_up_input().erase()),
                Some(gpioa.pa8.into_pull_up_input().erase()),
                Some(gpiob.pb15.into_pull_up_input().erase()),
                Some(gpiob.pb14.into_pull_up_input().erase()),
                Some(gpiob.pb13.into_pull_up_input().erase()),
            ],
            [
                Some(gpiob.pb8.into_pull_up_input().erase()),
                Some(gpiob.pb5.into_pull_up_input().erase()),
                Some(gpiob.pb4.into_pull_up_input().erase()),
                Some(gpiob.pb3.into_pull_up_input().erase()),
                Some(gpioa.pa15.into_pull_up_input().erase()),
            ],
            [
                Some(gpioa.pa4.into_pull_up_input().erase()),
                Some(gpioa.pa5.into_pull_up_input().erase()),
                Some(gpioa.pa6.into_pull_up_input().erase()),
                Some(gpioa.pa7.into_pull_up_input().erase()),
                Some(gpiob.pb0.into_pull_up_input().erase()),
            ],
            [
                None,
                None,
                Some(gpioa.pa2.into_pull_up_input().erase()),
                Some(gpioa.pa1.into_pull_up_input().erase()),
                Some(gpioa.pa0.into_pull_up_input().erase()),
            ],
        ];
        let matrix =
            cortex_m::interrupt::free(move |_cs| DirectPinMatrix::new(matrix_pins)).unwrap();

        (
            Shared {
                usb_dev,
                usb_class,
                layout: Layout::new(&LAYERS),
            },
            Local {
                matrix,
                debouncer: Debouncer::new([[false; 5]; 4], [[false; 5]; 4], 5),
                timer,
                serial_tx,
                serial_rx,
                serial_buf: [0; 4],
            },
            init::Monotonics(),
        )
    }

    /// Register a key press/release event with the layout (it will not be processed, yet)
    #[task(priority=1, capacity=8, shared=[layout])]
    fn register_keyboard_event(cx: register_keyboard_event::Context, event: Event) {
        cx.shared.layout.event(event)
    }
    #[task(
        binds = TIM2,
        priority = 1,
        local = [matrix, debouncer, timer, serial_tx],
        shared = [usb_dev, usb_class, layout]
    )]
    fn tick(mut cx: tick::Context) {
        cx.local.timer.wait().ok();

        let is_host = cx.shared.usb_dev.lock(|d| d.state()) == UsbDeviceState::Configured;

        for event in cx
            .local
            .debouncer
            .events(cx.local.matrix.get().unwrap())
            .map(transform_keypress_coordinates)
        {
            // either register events or send to other half
            if is_host {
                cx.shared.layout.event(event)
            } else {
                for &b in &serialize(event) {
                    block!(cx.local.serial_tx.write(b)).unwrap();
                }
            }
        }

        // if this is the USB-side, send a USB keyboard report
        if is_host {
            let report: KbHidReport = cx.shared.layout.keycodes().collect();
            if cx
                .shared
                .usb_class
                .lock(|k| k.device_mut().set_keyboard_report(report.clone()))
            {
                while let Ok(0) = cx.shared.usb_class.lock(|k| k.write(report.as_bytes())) {}
            }
        }
    }

    /// Transform key events from other keyboard half by mirroring coordinates
    #[cfg(feature = "right")]
    fn transform_keypress_coordinates(e: Event) -> Event {
        // mirror coordinates for events for right half
        e.transform(|i, j| (i, 9 - j))
    }

    /// Do not transform key events from other keyboard half
    #[cfg(feature = "left")]
    fn transform_keypress_coordinates(e: Event) -> Event {
        e
    }

    /// Receive USART events from other keyboard half and register them
    #[task(binds = USART1, priority = 2, local = [serial_rx, serial_buf])]
    fn rx(cx: rx::Context) {
        // receive USART bytes and place into local buffer
        // if buffer is full (ends with '\n'), spawn event registration
        // received events (from other half) are mirrored (transformed)
        if let Ok(b) = cx.local.serial_rx.read() {
            cx.local.serial_buf.rotate_left(1);
            cx.local.serial_buf[3] = b;

            if cx.local.serial_buf[3] == b'\n' {
                if let Ok(event) = deserialize(&cx.local.serial_buf[..]) {
                    register_keyboard_event::spawn(event).unwrap()
                }
            }
        }
    }

    /// Deserialize a key event from the serial line
    fn deserialize(bytes: &[u8]) -> Result<Event, ()> {
        match *bytes {
            [b'P', i, j, b'\n'] => Ok(Event::Press(i, j)),
            [b'R', i, j, b'\n'] => Ok(Event::Release(i, j)),
            _ => Err(()),
        }
    }

    /// Serialize a key event
    fn serialize(e: Event) -> [u8; 4] {
        match e {
            Event::Press(i, j) => [b'P', i, j, b'\n'],
            Event::Release(i, j) => [b'R', i, j, b'\n'],
        }
    }

    #[task(binds = OTG_FS_WKUP, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_rx(cx: usb_rx::Context) {
        (cx.shared.usb_dev, cx.shared.usb_class).lock(|usb_dev, usb_class| {
            if usb_dev.poll(&mut [usb_class]) {
                usb_class.poll();
            }
        });
    }

    // USB events
    #[task(binds = OTG_FS, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_tx(cx: usb_tx::Context) {
        (cx.shared.usb_dev, cx.shared.usb_class).lock(|usb_dev, usb_class| {
            if usb_dev.poll(&mut [usb_class]) {
                usb_class.poll();
            }
        });
    }
}
