#![no_std]
#![no_main]

extern crate panic_halt;

use gd32vf103_hal as hal;
use hal::pac;
use hal::prelude::*;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);

    let mut pa2 = gpioa.pa2.into_push_pull_output(&mut gpioa.ctl0);

    // Dumb busy-waiting for quick & dirty blinking without timers
    let mut iteration: u32 = 0;
    let mut on = false;
    loop {
        if iteration % 200_000 == 0 {
            if on {
                pa2.set_high().unwrap();
            } else {
                pa2.set_low().unwrap();
            }
            on = !on;
        }
        iteration += 1;
    }
}
