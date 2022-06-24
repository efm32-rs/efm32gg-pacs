# EFM32GG (Giant Gecko / Giant Gecko S1) support for Rust

[![PACs](https://github.com/vpetrigo/efm32-pacs/actions/workflows/pacs.yml/badge.svg)](https://github.com/vpetrigo/efm32-pacs/actions/workflows/pacs.yml)

This repository contains Peripheral Access Crates (PACs) for Silabs' EFM32 series of Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Refer to the [CHANGELOG](CHANGELOG.md) to see what changed in the last releases.

## Crates

Every EFM32GG chip has its own PAC, listed below:

| Crate               | Docs                                                                                         | crates.io                                                                                                         | target                  |
|---------------------|----------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------|-------------------------|
| `efm32gg230-pac`    | [![docs.rs](https://docs.rs/efm32gg230-pac/badge.svg)](https://docs.rs/efm32gg230-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg230-pac.svg)](https://crates.io/crates/efm32gg230-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg232-pac`    | [![docs.rs](https://docs.rs/efm32gg232-pac/badge.svg)](https://docs.rs/efm32gg232-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg232-pac.svg)](https://crates.io/crates/efm32gg232-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg280-pac`    | [![docs.rs](https://docs.rs/efm32gg280-pac/badge.svg)](https://docs.rs/efm32gg280-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg280-pac.svg)](https://crates.io/crates/efm32gg280-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg290-pac`    | [![docs.rs](https://docs.rs/efm32gg290-pac/badge.svg)](https://docs.rs/efm32gg290-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg290-pac.svg)](https://crates.io/crates/efm32gg290-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg295-pac`    | [![docs.rs](https://docs.rs/efm32gg295-pac/badge.svg)](https://docs.rs/efm32gg295-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg295-pac.svg)](https://crates.io/crates/efm32gg295-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg330-pac`    | [![docs.rs](https://docs.rs/efm32gg330-pac/badge.svg)](https://docs.rs/efm32gg330-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg330-pac.svg)](https://crates.io/crates/efm32gg330-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg332-pac`    | [![docs.rs](https://docs.rs/efm32gg332-pac/badge.svg)](https://docs.rs/efm32gg332-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg332-pac.svg)](https://crates.io/crates/efm32gg332-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg380-pac`    | [![docs.rs](https://docs.rs/efm32gg380-pac/badge.svg)](https://docs.rs/efm32gg380-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg380-pac.svg)](https://crates.io/crates/efm32gg380-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg390-pac`    | [![docs.rs](https://docs.rs/efm32gg390-pac/badge.svg)](https://docs.rs/efm32gg390-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg390-pac.svg)](https://crates.io/crates/efm32gg390-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg395-pac`    | [![docs.rs](https://docs.rs/efm32gg395-pac/badge.svg)](https://docs.rs/efm32gg395-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg395-pac.svg)](https://crates.io/crates/efm32gg395-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg840-pac`    | [![docs.rs](https://docs.rs/efm32gg840-pac/badge.svg)](https://docs.rs/efm32gg840-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg840-pac.svg)](https://crates.io/crates/efm32gg840-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg842-pac`    | [![docs.rs](https://docs.rs/efm32gg842-pac/badge.svg)](https://docs.rs/efm32gg842-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg842-pac.svg)](https://crates.io/crates/efm32gg842-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg880-pac`    | [![docs.rs](https://docs.rs/efm32gg880-pac/badge.svg)](https://docs.rs/efm32gg880-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg880-pac.svg)](https://crates.io/crates/efm32gg880-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg890-pac`    | [![docs.rs](https://docs.rs/efm32gg890-pac/badge.svg)](https://docs.rs/efm32gg890-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg890-pac.svg)](https://crates.io/crates/efm32gg890-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg895-pac`    | [![docs.rs](https://docs.rs/efm32gg895-pac/badge.svg)](https://docs.rs/efm32gg895-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg895-pac.svg)](https://crates.io/crates/efm32gg895-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg900-pac`    | [![docs.rs](https://docs.rs/efm32gg900-pac/badge.svg)](https://docs.rs/efm32gg900-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg900-pac.svg)](https://crates.io/crates/efm32gg900-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg940-pac`    | [![docs.rs](https://docs.rs/efm32gg940-pac/badge.svg)](https://docs.rs/efm32gg940-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg940-pac.svg)](https://crates.io/crates/efm32gg940-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg942-pac`    | [![docs.rs](https://docs.rs/efm32gg942-pac/badge.svg)](https://docs.rs/efm32gg942-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg942-pac.svg)](https://crates.io/crates/efm32gg942-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg980-pac`    | [![docs.rs](https://docs.rs/efm32gg980-pac/badge.svg)](https://docs.rs/efm32gg980-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg980-pac.svg)](https://crates.io/crates/efm32gg980-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg990-pac`    | [![docs.rs](https://docs.rs/efm32gg990-pac/badge.svg)](https://docs.rs/efm32gg990-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg990-pac.svg)](https://crates.io/crates/efm32gg990-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg995-pac`    | [![docs.rs](https://docs.rs/efm32gg995-pac/badge.svg)](https://docs.rs/efm32gg995-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32gg995-pac.svg)](https://crates.io/crates/efm32gg995-pac)       | `thumbv7em-none-eabi`   |
 | `efm32gg11b110-pac` | [![docs.rs](https://docs.rs/efm32gg11b110-pac/badge.svg)](https://docs.rs/efm32gg11b110-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg11b110-pac.svg)](https://crates.io/crates/efm32gg11b110-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg11b120-pac` | [![docs.rs](https://docs.rs/efm32gg11b120-pac/badge.svg)](https://docs.rs/efm32gg11b120-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg11b120-pac.svg)](https://crates.io/crates/efm32gg11b120-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg11b310-pac` | [![docs.rs](https://docs.rs/efm32gg11b310-pac/badge.svg)](https://docs.rs/efm32gg11b310-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg11b310-pac.svg)](https://crates.io/crates/efm32gg11b310-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg11b320-pac` | [![docs.rs](https://docs.rs/efm32gg11b320-pac/badge.svg)](https://docs.rs/efm32gg11b320-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg11b320-pac.svg)](https://crates.io/crates/efm32gg11b320-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg11b420-pac` | [![docs.rs](https://docs.rs/efm32gg11b420-pac/badge.svg)](https://docs.rs/efm32gg11b420-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg11b420-pac.svg)](https://crates.io/crates/efm32gg11b420-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg11b510-pac` | [![docs.rs](https://docs.rs/efm32gg11b510-pac/badge.svg)](https://docs.rs/efm32gg11b510-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg11b510-pac.svg)](https://crates.io/crates/efm32gg11b510-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg11b520-pac` | [![docs.rs](https://docs.rs/efm32gg11b520-pac/badge.svg)](https://docs.rs/efm32gg11b520-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg11b520-pac.svg)](https://crates.io/crates/efm32gg11b520-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg11b820-pac` | [![docs.rs](https://docs.rs/efm32gg11b820-pac/badge.svg)](https://docs.rs/efm32gg11b820-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg11b820-pac.svg)](https://crates.io/crates/efm32gg11b820-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg11b840-pac` | [![docs.rs](https://docs.rs/efm32gg11b840-pac/badge.svg)](https://docs.rs/efm32gg11b840-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg11b840-pac.svg)](https://crates.io/crates/efm32gg11b840-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b110-pac` | [![docs.rs](https://docs.rs/efm32gg12b110-pac/badge.svg)](https://docs.rs/efm32gg12b110-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b110-pac.svg)](https://crates.io/crates/efm32gg12b110-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b130-pac` | [![docs.rs](https://docs.rs/efm32gg12b130-pac/badge.svg)](https://docs.rs/efm32gg12b130-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b130-pac.svg)](https://crates.io/crates/efm32gg12b130-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b310-pac` | [![docs.rs](https://docs.rs/efm32gg12b310-pac/badge.svg)](https://docs.rs/efm32gg12b310-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b310-pac.svg)](https://crates.io/crates/efm32gg12b310-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b330-pac` | [![docs.rs](https://docs.rs/efm32gg12b330-pac/badge.svg)](https://docs.rs/efm32gg12b330-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b330-pac.svg)](https://crates.io/crates/efm32gg12b330-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b390-pac` | [![docs.rs](https://docs.rs/efm32gg12b390-pac/badge.svg)](https://docs.rs/efm32gg12b390-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b390-pac.svg)](https://crates.io/crates/efm32gg12b390-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b410-pac` | [![docs.rs](https://docs.rs/efm32gg12b410-pac/badge.svg)](https://docs.rs/efm32gg12b410-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b410-pac.svg)](https://crates.io/crates/efm32gg12b410-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b430-pac` | [![docs.rs](https://docs.rs/efm32gg12b430-pac/badge.svg)](https://docs.rs/efm32gg12b430-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b430-pac.svg)](https://crates.io/crates/efm32gg12b430-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b510-pac` | [![docs.rs](https://docs.rs/efm32gg12b510-pac/badge.svg)](https://docs.rs/efm32gg12b510-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b510-pac.svg)](https://crates.io/crates/efm32gg12b510-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b530-pac` | [![docs.rs](https://docs.rs/efm32gg12b530-pac/badge.svg)](https://docs.rs/efm32gg12b530-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b530-pac.svg)](https://crates.io/crates/efm32gg12b530-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b810-pac` | [![docs.rs](https://docs.rs/efm32gg12b810-pac/badge.svg)](https://docs.rs/efm32gg12b810-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b810-pac.svg)](https://crates.io/crates/efm32gg12b810-pac) | `thumbv7em-none-eabihf` |
 | `efm32gg12b830-pac` | [![docs.rs](https://docs.rs/efm32gg12b830-pac/badge.svg)](https://docs.rs/efm32gg12b830-pac) | [![crates.io](https://img.shields.io/crates/d/efm32gg12b830-pac.svg)](https://crates.io/crates/efm32gg12b830-pac) | `thumbv7em-none-eabihf` |

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