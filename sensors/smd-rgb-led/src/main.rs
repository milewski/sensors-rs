#![no_std]
#![no_main]

use core::cmp;
use core::cmp::{max, min};
use core::f32::consts::{FRAC_PI_2, PI};

use cortex_m::{delay::Delay, Peripherals as CortexPeripherals};
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use defmt::export::f32;
use defmt::{info, println};
use defmt_rtt as _;
use micromath::F32Ext;
use panic_probe as _;
use stm32f1xx_hal::{pac::Peripherals as DevicePeripherals, prelude::*};
use stm32f1xx_hal::time::Hertz;
use stm32f1xx_hal::timer::{Channel, Tim2NoRemap};

// global logger

fn max_float(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn compute_duty(phase: f32, shift: f32, max: f32) -> u16 {
    cmp::max(1, (square_root(max_float(0.0, (phase + shift).sin())) * max) as u16)
}

fn square_root(value: f32) -> f32 {
    value * value
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let device_peripheral = DevicePeripherals::take().unwrap();

    let rcc = device_peripheral.RCC.constrain();
    let mut flash = device_peripheral.FLASH.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);

    let mut gpioa = device_peripheral.GPIOA.split();

    let red = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let blue = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let green = gpioa.pa3.into_alternate_push_pull(&mut gpioa.crl);

    let mut afio = device_peripheral.AFIO.constrain();
    let pins = (red, blue, green);

    let mut pwm = device_peripheral.TIM2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 200.Hz(), &clocks);

    pwm.enable(Channel::C1);
    pwm.enable(Channel::C3);
    pwm.enable(Channel::C4);

    pwm.set_period(200.Hz());

    let mut phase = 0;
    let max = pwm.get_max_duty() as i32;

    let show = [
        (0, 0, max),
        (0, max, 0),
        (max, 0, 0),
    ];

    let mut current = (0, 0, 0);

    loop {
        for (index, target) in show.into_iter().enumerate() {
            println!("index: {}", index);

            let from = match index {
                0 => (0, 0, 0),
                _ => show[index - 1]
            };

            tween2(1000, |up, down| {
                let red = tween(up, percent(&from.0, &down), percent(&target.0, &up), 1000);
                let green = tween(up, percent(&from.1, &down), percent(&target.1, &up), 1000);
                let blue = tween(up, percent(&from.2, &down), percent(&target.2, &up), 1000);

                pwm.set_duty(Channel::C1, red as u16);
                pwm.set_duty(Channel::C3, green as u16);
                pwm.set_duty(Channel::C4, blue as u16);
            });
        }

        // let red = (max * f32::sin(phase)).abs() as u16;
        // let green = (max * f32::sin(phase * 2.0)).abs() as u16;
        // let blue = (max * f32::sin(phase * 4.0)).abs() as u16;
        //
        // info!("{}", red);
    }
}

fn tween2(duration: i32, mut callback: impl FnMut(i32, i32)) {
    let mut time = 0;
    loop {
        let value = tween(time, 0, 100, duration);

        if value == 100 {
            break;
        } else {
            callback(value, 100 - value)
        }

        time += 1;
    }
}

fn percent(max: &i32, percent: &i32) -> i32 {
    let value = max * percent;

    value / 100
}

fn tween(current_time: i32, beginning: i32, target: i32, duration: i32) -> i32 {
    let c = target - beginning;

    (c * current_time / duration + beginning)
}
