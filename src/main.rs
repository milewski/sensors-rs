#![no_std]
#![no_main]
#![allow(unused_imports)]

use cortex_m::{delay::Delay, Peripherals as CortexPeripherals};
use cortex_m_rt::entry;
use panic_halt;
use stm32f1xx_hal::{pac::Peripherals as DevicePeripherals, prelude::*, time::Hertz};

#[entry]
fn main() -> ! {
    let device_peripheral = DevicePeripherals::take().unwrap();
    let cortex_peripheral = CortexPeripherals::take().unwrap();

    let mut gpiob = device_peripheral.GPIOB.split();
    let mut external_led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    let mut gpioc = device_peripheral.GPIOC.split();
    let mut built_in_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    /**
     * This code basically sets the device clock to 8MHz
     * and then configure the delay to use the same frequency.
     */
    let mut rcc = device_peripheral.RCC.constrain();
    let mut flash = device_peripheral.FLASH.constrain();
    let mut clocks = rcc.cfgr.sysclk(Hertz::MHz(8)).freeze(&mut flash.acr);
    let mut delay = Delay::new(cortex_peripheral.SYST, clocks.sysclk().to_Hz());

    loop {
        built_in_led.set_high();
        external_led.set_high();

        delay.delay_ms(1000);

        built_in_led.set_low();
        external_led.set_low();

        delay.delay_ms(1000);
    }
}