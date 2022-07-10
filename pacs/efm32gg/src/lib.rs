//! Peripheral access API for EFM32GG microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.24.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32gg-pacs)
//!
//! This crate supports all EFM32GG devices; for the complete list please see:
//! [efm32gg](https://github.com/efm32-rs/efm32gg-pacs/pacs/efm32gg)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32gg230")]
pub mod efm32gg230;

#[cfg(feature = "efm32gg232")]
pub mod efm32gg232;

#[cfg(feature = "efm32gg280")]
pub mod efm32gg280;

#[cfg(feature = "efm32gg290")]
pub mod efm32gg290;

#[cfg(feature = "efm32gg295")]
pub mod efm32gg295;

#[cfg(feature = "efm32gg330")]
pub mod efm32gg330;

#[cfg(feature = "efm32gg332")]
pub mod efm32gg332;

#[cfg(feature = "efm32gg380")]
pub mod efm32gg380;

#[cfg(feature = "efm32gg390")]
pub mod efm32gg390;

#[cfg(feature = "efm32gg395")]
pub mod efm32gg395;

#[cfg(feature = "efm32gg840")]
pub mod efm32gg840;

#[cfg(feature = "efm32gg842")]
pub mod efm32gg842;

#[cfg(feature = "efm32gg880")]
pub mod efm32gg880;

#[cfg(feature = "efm32gg890")]
pub mod efm32gg890;

#[cfg(feature = "efm32gg895")]
pub mod efm32gg895;

#[cfg(feature = "efm32gg900")]
pub mod efm32gg900;

#[cfg(feature = "efm32gg940")]
pub mod efm32gg940;

#[cfg(feature = "efm32gg942")]
pub mod efm32gg942;

#[cfg(feature = "efm32gg980")]
pub mod efm32gg980;

#[cfg(feature = "efm32gg990")]
pub mod efm32gg990;

#[cfg(feature = "efm32gg995")]
pub mod efm32gg995;
