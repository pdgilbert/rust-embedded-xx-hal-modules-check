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
    gpio::{Output, PushPull,
           gpiob::PB6},
    prelude::*,
    rcc, 
    delay::Delay,
//    timer::Delay,
    timer::SysDelay,
};

pub type RccType:  = rcc::Rcc;
pub type SysDelayType = SysDelay;

pub type DelayType = Delay;
//pub type DelayType = Delay<TIM2, 1000000_u32>;


#[entry]
fn main() -> ! {
    let _cp = CorePeripherals::take().unwrap();
    let _dp = Peripherals::take().unwrap();

    loop { }
}

