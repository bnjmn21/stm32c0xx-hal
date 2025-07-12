#![no_std]
#![allow(non_camel_case_types)]

#[cfg(not(feature = "device-selected"))]
compile_error!("This crate requires one of the following features enabled: stm32c011, stm32c031");

pub use embedded_hal_02 as hal_02;

pub use nb::block;

#[cfg(feature = "device-selected")]
pub use stm32 as pac;

#[cfg(feature = "stm32c011")]
pub use stm32c0::stm32c011 as stm32;

#[cfg(feature = "stm32c031")]
pub use stm32c0::stm32c031 as stm32;

#[cfg(feature = "stm32c071")]
pub use stm32c0::stm32c071 as stm32;

#[cfg(feature = "rt")]
pub use crate::stm32::interrupt;

pub mod analog;
pub mod crc;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod power;
pub mod prelude;
pub mod rcc;
pub mod rtc;
pub mod serial;
pub mod spi;
pub mod time;
pub mod timer;
pub mod watchdog;

#[cfg(feature = "device-selected")]
mod sealed {
    pub trait Sealed {}
}

#[cfg(feature = "device-selected")]
pub(crate) use sealed::Sealed;
