#[cfg(feature = "build-native")]
use cmake;

#[cfg(feature = "build-native")]
fn build_native() {
    let dst = cmake::Config::new("flac")
    .define("BUILD_CXXLIBS", "OFF")
    .define("BUILD_PROGRAMS", "OFF")
    .define("BUILD_EXAMPLES", "OFF")
    .define("BUILD_DOCS", "OFF")
    .define("INSTALL_MANPAGES", "OFF")
    .define("INSTALL_PKGCONFIG_MODULES", "OFF")
    .define("INSTALL_CMAKE_CONFIG_MODULE", "OFF")
    .define("WITH_OGG", "OFF")
    .define("BUILD_TESTING", "OFF") // flac attempts to test the above disabled/unbuilt modules.
    .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=flac");
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
