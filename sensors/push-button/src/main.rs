#![no_std]
#![no_main]
#![allow(unused_imports)]

use cortex_m_rt::entry;
use panic_halt;
use stm32f1xx_hal::{pac::Peripherals as DevicePeripherals, prelude::*};
use stm32f1xx_hal::i2c::I2c;

#[entry]
fn main() -> ! {
    let device_peripheral = DevicePeripherals::take().unwrap();

    let mut gpioa = device_peripheral.GPIOA.split();
    let mut gpiob = device_peripheral.GPIOB.split();

    let button = gpioa.pa10.into_pull_up_input(&mut gpioa.crh);
    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    loop {
        if button.is_high() {
            led.set_low();
        } else {
            led.set_high();
        }
    }
}
