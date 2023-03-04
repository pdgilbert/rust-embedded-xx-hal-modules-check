// example to check basic hal modules

#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use cortex_m_rt::entry;



#[cfg(feature = "stm32f0xx")]
use stm32f0xx_hal as hal;

#[cfg(feature = "stm32f1xx")]
use stm32f1xx_hal as hal;

#[cfg(feature = "stm32f3xx")]
use stm32f3xx_hal as hal;

#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal as hal;

#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal as hal;

#[cfg(feature = "stm32g0xx")]
use stm32g0xx_hal as hal;

#[cfg(feature = "stm32g4xx")]
use stm32g4xx_hal as hal;

#[cfg(feature = "stm32h7xx")]
use stm32h7xx_hal as hal;

#[cfg(feature = "stm32l0xx")]
use stm32l0xx_hal as hal;

#[cfg(feature = "stm32l1xx")]
use stm32l1xx_hal as hal;

#[cfg(feature = "stm32l4xx")]
use stm32l4xx_hal as hal;


use hal::{
    pac::{CorePeripherals, Peripherals},
    pac::{TIM2},
    gpio::{Output, PushPull,
           gpiob::PB6},
    prelude::*,
    rcc, 
    delay::Delay,
    timer::Delay as timerDelay,
    timer::SysDelay,
};

pub type RccType:  = rcc::Rcc;
pub type SysDelayType = SysDelay;

pub type DelayType = Delay;
pub type DelayType = Delay<TIM2, 1000000_u32>;

pub type SomePinType = PB6<Output<PushPull>>;

pub trait LED {
    fn on(&mut self) -> ();
    fn off(&mut self) -> ();

    //fn blink(&mut self, time: u16, delay: &mut Delay) -> () {
    //    self.on();
    //    delay.delay_ms(time);
    //    self.off()
    //}
}


#[entry]
fn main() -> ! {
    let _cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let rcc: rcc::Rcc = dp.RCC.constrain();

    let _gpioc = dp.GPIOC.split();

    let delay = DelayType{};
    let sysdelay = SysDelayType{};

    loop {
//        led.blink(on, &mut delay);
        delay.delay_ms(3000);
    }
}

