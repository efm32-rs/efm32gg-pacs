//! Peripheral access API for EFM32GG12B microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.28.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32gg-pacs)
//!
//! This crate supports all EFM32GG12B devices; for the complete list please see:
//! [efm32gg12b](https://github.com/efm32-rs/efm32gg-pacs/pacs/efm32gg12b)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32gg12b110")]
pub mod efm32gg12b110;

#[cfg(feature = "efm32gg12b130")]
pub mod efm32gg12b130;

#[cfg(feature = "efm32gg12b310")]
pub mod efm32gg12b310;

#[cfg(feature = "efm32gg12b330")]
pub mod efm32gg12b330;

#[cfg(feature = "efm32gg12b390")]
pub mod efm32gg12b390;

#[cfg(feature = "efm32gg12b410")]
pub mod efm32gg12b410;

#[cfg(feature = "efm32gg12b430")]
pub mod efm32gg12b430;

#[cfg(feature = "efm32gg12b510")]
pub mod efm32gg12b510;

#[cfg(feature = "efm32gg12b530")]
pub mod efm32gg12b530;

#[cfg(feature = "efm32gg12b810")]
pub mod efm32gg12b810;

#[cfg(feature = "efm32gg12b830")]
pub mod efm32gg12b830;
