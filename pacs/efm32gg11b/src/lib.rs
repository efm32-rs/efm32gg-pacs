//! Peripheral access API for EFM32GG11B microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.28.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32gg-pacs)
//!
//! This crate supports all EFM32GG11B devices; for the complete list please see:
//! [efm32gg11b](https://github.com/efm32-rs/efm32gg-pacs/pacs/efm32gg11b)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32gg11b110")]
pub mod efm32gg11b110;

#[cfg(feature = "efm32gg11b120")]
pub mod efm32gg11b120;

#[cfg(feature = "efm32gg11b310")]
pub mod efm32gg11b310;

#[cfg(feature = "efm32gg11b320")]
pub mod efm32gg11b320;

#[cfg(feature = "efm32gg11b420")]
pub mod efm32gg11b420;

#[cfg(feature = "efm32gg11b510")]
pub mod efm32gg11b510;

#[cfg(feature = "efm32gg11b520")]
pub mod efm32gg11b520;

#[cfg(feature = "efm32gg11b820")]
pub mod efm32gg11b820;

#[cfg(feature = "efm32gg11b840")]
pub mod efm32gg11b840;
