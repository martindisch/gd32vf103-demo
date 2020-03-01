//! This example will alternate PA2 between low and high. If you connect a
//! standard LED (protected with a 220 ohm resistor), you can see it blink.

#![no_std]
#![no_main]

use panic_halt as _;

use embedded_hal::blocking::delay::DelayMs;
use gd32vf103_hal::{
    ctimer::CoreTimer, delay::Delay, pac, prelude::*, rcu::Strict,
};
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa2 = gpioa
        .pa2
        .into_push_pull_output(&mut gpioa.ctl0)
        .lock(&mut gpioa.lock);
    gpioa.lock.freeze();

    let clocks = Strict::new().freeze(&mut rcu.cfg);
    let ctimer = CoreTimer::new(dp.CTIMER);
    let mut delay = Delay::new(clocks, ctimer);
    loop {
        pa2.toggle().unwrap();
        delay.delay_ms(1000u32);
    }
}
