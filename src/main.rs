#![no_std]
#![no_main]

use core::panic::PanicInfo;

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use esp_println::println;

use esp_wifi::{initialize, EspWifiInitFor};

use esp_hal::{systimer::SystemTimer, Rng};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // FIXME: the loop will overwork the microcontroller
    loop { }
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
