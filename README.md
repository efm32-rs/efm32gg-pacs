# EFM32GG (Giant Gecko / Giant Gecko S1) support for Rust

[![PACs](https://github.com/efm32-rs/efm32gg-pacs/actions/workflows/pacs.yml/badge.svg)](https://github.com/efm32-rs/efm32gg-pacs/actions/workflows/pacs.yml)

This repository contains Peripheral Access Crates (PACs) for Silabs' EFM32GG series of Cortex-M microcontrollers.
All these crates are automatically generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Refer to the [CHANGELOG](CHANGELOG.md) to see what changed in the last releases.

## Crates

Every EFM32G chip has its own PAC, listed below:

| Crate           | Docs                                                                                 | crates.io                                                                                                 | Target               |
|-----------------|--------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------|----------------------|
| `efm32gg-pac`| [![docs.rs](https://docs.rs/efm32gg-pac/badge.svg)](https://docs.rs/efm32gg-pac)| [![crates.io](https://img.shields.io/crates/d/efm32gg-pac)](https://crates.io/crates/efm32gg-pac)| `thumbv7m-none-eabi` |
| `efm32gg11b-pac`| [![docs.rs](https://docs.rs/efm32gg11b-pac/badge.svg)](https://docs.rs/efm32gg11b-pac)| [![crates.io](https://img.shields.io/crates/d/efm32gg11b-pac)](https://crates.io/crates/efm32gg11b-pac)| `thumbv7em-none-eabihf` |
| `efm32gg12b-pac`| [![docs.rs](https://docs.rs/efm32gg12b-pac/badge.svg)](https://docs.rs/efm32gg12b-pac)| [![crates.io](https://img.shields.io/crates/d/efm32gg12b-pac)](https://crates.io/crates/efm32gg12b-pac)| `thumbv7em-none-eabihf` |

## Device Reference Manuals from Silabs

**WIP**

## License

The included SVD files are sourced from https://www.silabs.com/documents/public/cmsis-packs and
are licensed under the Zlib (see [LICENSE-3RD-PARTY](LICENSE-3RD-PARTY-Zlib)).

The remainder of the code is under:

- 3-Clause BSD license ([LICENSE-3BSD](LICENSE-3BSD) or https://opensource.org/licenses/BSD-3-Clause)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the BSD-3-Clause license without any additional terms or conditions.
