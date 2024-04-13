// FEATURES
#![feature(riscv_ext_intrinsics)]
// END FEATURES
#![no_std]
#![no_main]

use core::arch::riscv32::nop;
use core::panic::PanicInfo;

use esp_backtrace as _;
use esp_hal::clock::ClockControl;
use esp_hal::peripherals::Peripherals;
use esp_hal::prelude::*;
use esp_hal::systimer::SystemTimer;
use esp_hal::{Delay, Rng};
use esp_println::println;
use esp_wifi::{initialize, EspWifiInitFor};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        nop()
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    println!("Saluton Mondo!");
    let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
    let _init = initialize(
        EspWifiInitFor::Wifi,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();
    loop {
        println!("atendante en sistema buklo...");
        delay.delay_ms(500u32);
    }
}
