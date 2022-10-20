#![no_std]
#![no_main]

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
    let pir = io.pins.gpio1.into_pull_down_input();
    let mut led = io.pins.gpio10.into_push_pull_output();

    let mut delay = Delay::new(&clocks);
    // 0 low 1 high
    let mut pir_state: u8 = 0;
    println!("hello pir!");
    loop {
        if pir.is_high().unwrap() {
            led.set_high().unwrap();
            if pir_state == 0 {
                println!("motion catch");
                pir_state = 1;
            }
        } else {
            led.set_low().unwrap();
            if pir_state == 1 {
                println!("motion end");
                pir_state = 0;
            }
        }
        delay.delay_ms(1000_u32)
    }
}
