#[cfg(feature = "build-native")]
use cmake;

#[cfg(feature = "build-native")]
fn build_native() {
    let dst = cmake::Config::new("opus").define("OPUS_BUILD_SHARED_LIBRARY", "ON").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=opus");
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=stdc++");
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    }
}

fn main() {
    #[cfg(feature = "build-native")]
    build_native();
}
