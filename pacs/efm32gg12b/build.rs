use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32GG12B110").is_some() {
            "src/efm32gg12b110/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B130").is_some() {
            "src/efm32gg12b130/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B310").is_some() {
            "src/efm32gg12b310/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B330").is_some() {
            "src/efm32gg12b330/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B390").is_some() {
            "src/efm32gg12b390/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B410").is_some() {
            "src/efm32gg12b410/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B430").is_some() {
            "src/efm32gg12b430/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B510").is_some() {
            "src/efm32gg12b510/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B530").is_some() {
            "src/efm32gg12b530/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B810").is_some() {
            "src/efm32gg12b810/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG12B830").is_some() {
            "src/efm32gg12b830/device.x"
        } else { panic!("No device features selected"); };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

