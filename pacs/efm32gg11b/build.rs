use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32GG11B110").is_some() {
            "src/efm32gg11b110/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG11B120").is_some() {
            "src/efm32gg11b120/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG11B310").is_some() {
            "src/efm32gg11b310/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG11B320").is_some() {
            "src/efm32gg11b320/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG11B420").is_some() {
            "src/efm32gg11b420/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG11B510").is_some() {
            "src/efm32gg11b510/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG11B520").is_some() {
            "src/efm32gg11b520/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG11B820").is_some() {
            "src/efm32gg11b820/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG11B840").is_some() {
            "src/efm32gg11b840/device.x"
        } else { panic!("No device features selected"); };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

