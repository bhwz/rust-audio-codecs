#[cfg(feature = "build-native")]
use cmake;

#[cfg(feature = "build-native")]
fn build_native() {
    let dst = cmake::Config::new("fdk-aac").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=fdk-aac");
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
