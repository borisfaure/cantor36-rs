#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {

    use stm32f4xx_hal::{
        gpio::{self, Output, PushPull},
        pac::TIM2,
        prelude::*,
        timer::{self, Event},
    };

    // Resources shared between tasks
    #[shared]
    struct Shared {
        timer: timer::CounterMs<TIM2>,
    }

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {
        led: gpio::PC13<Output<PushPull>>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let dp = ctx.device;

        let gpioc = dp.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();

        // Configure and obtain handle for delay abstraction
        // 1) Promote RCC structure to HAL to be able to configure clocks
        let rcc = dp.RCC.constrain();
        // 2) Configure the system clocks
        let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();
        // 3) Create delay handle
        let mut timer = dp.TIM2.counter_ms(&clocks);

        // Kick off the timer with 2 seconds timeout first
        timer.start(2000.millis()).unwrap();

        // Set up to generate interrupt when timer expires
        timer.listen(Event::Update);

        (
            // Initialization of shared resources
            Shared { timer },
            // Initialization of task local resources
            Local { led },
            // Move the monotonic timer to the RTIC run-time, this enables
            // scheduling
            init::Monotonics(),
        )
    }

    // Background task, runs whenever no other tasks are running
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Go to sleep
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = TIM2, local=[led], shared=[timer])]
    fn timer_expired(mut ctx: timer_expired::Context) {
        // When Timer Interrupt Happens Two Things Need to be Done
        // 1) Toggle the LED
        // 2) Clear Timer Pending Interrupt

        ctx.local.led.toggle();
        ctx.shared
            .timer
            .lock(|tim| tim.clear_interrupt(Event::Update));
    }
}
