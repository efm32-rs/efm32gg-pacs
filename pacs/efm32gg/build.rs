use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32GG230").is_some() {
            "src/efm32gg230/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG232").is_some() {
            "src/efm32gg232/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG280").is_some() {
            "src/efm32gg280/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG290").is_some() {
            "src/efm32gg290/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG295").is_some() {
            "src/efm32gg295/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG330").is_some() {
            "src/efm32gg330/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG332").is_some() {
            "src/efm32gg332/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG380").is_some() {
            "src/efm32gg380/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG390").is_some() {
            "src/efm32gg390/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG395").is_some() {
            "src/efm32gg395/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG840").is_some() {
            "src/efm32gg840/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG842").is_some() {
            "src/efm32gg842/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG880").is_some() {
            "src/efm32gg880/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG890").is_some() {
            "src/efm32gg890/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG895").is_some() {
            "src/efm32gg895/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG900").is_some() {
            "src/efm32gg900/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG940").is_some() {
            "src/efm32gg940/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG942").is_some() {
            "src/efm32gg942/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG980").is_some() {
            "src/efm32gg980/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG990").is_some() {
            "src/efm32gg990/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32GG995").is_some() {
            "src/efm32gg995/device.x"
        } else { panic!("No device features selected"); };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

