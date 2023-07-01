#![no_std]
#![no_main]

use core::cmp;
use core::cmp::max;
use core::f32::consts::FRAC_PI_2;

use cortex_m::{delay::Delay, Peripherals as CortexPeripherals};
use cortex_m_rt::entry;
use micromath::F32Ext;
use panic_halt;
use stm32f1xx_hal::{pac::Peripherals as DevicePeripherals, prelude::*};
use stm32f1xx_hal::time::Hertz;
use stm32f1xx_hal::timer::{Channel, Tim2NoRemap};

fn max_float(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn compute_duty(phase: f32, shift: f32, max: f32) -> u16 {
    cmp::max(1, (square_root(max_float(0.0, (phase + shift).sin())) * max) as u16)
}

fn square_root(value: f32) -> f32 {
    value * value
}

#[entry]
fn main() -> ! {
    let device_peripheral = DevicePeripherals::take().unwrap();

    let rcc = device_peripheral.RCC.constrain();
    let mut flash = device_peripheral.FLASH.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);

    let mut gpioa = device_peripheral.GPIOA.split();

    let red = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let blue = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let green = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);

    let mut afio = device_peripheral.AFIO.constrain();
    let pins = (red, blue, green);

    let mut pwm = device_peripheral.TIM2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 500.Hz(), &clocks);

    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);
    pwm.enable(Channel::C3);

    pwm.set_period(80.Hz());

    let mut phase: f32 = 0.0;

    let max_duty = pwm.get_max_duty() as f32;

    loop {
        pwm.set_duty(Channel::C1, compute_duty(phase, FRAC_PI_2 * 0.0, max_duty));
        pwm.set_duty(Channel::C2, compute_duty(phase, FRAC_PI_2 * 1.0, max_duty));
        pwm.set_duty(Channel::C3, compute_duty(phase, FRAC_PI_2 * 2.0, max_duty));

        phase += 0.0008;
    }
}
