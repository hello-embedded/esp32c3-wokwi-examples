#![no_std]
#![no_main]

use dht_sensor::*;
use esp32c3_hal::{
    clock::ClockControl, pac::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc, IO,
};
use esp_backtrace as _;
use esp_println::println;

#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut dht22_io1 = io.pins.gpio1.into_open_drain_output();
    // Pulling the pin high to avoid confusing the sensor when initializing
    dht22_io1.set_high().unwrap();

    let mut delay = Delay::new(&clocks);
    println!("Waiting on the sensor...");
    delay.delay_ms(1000_u32);

    println!("hello dht22!");
    loop {
        let reading_res = dht22::Reading::read(&mut delay, &mut dht22_io1);
        match reading_res {
            Ok(reading) => println!(
                "{}°, {}% RH",
                reading.temperature, reading.relative_humidity,
            ),
            Err(e) => println!("err: {:?}", e),
        }

        // Delay of at least 500ms before polling the sensor again, 1 second or more advised
        delay.delay_ms(1000_u32);
    }
}
