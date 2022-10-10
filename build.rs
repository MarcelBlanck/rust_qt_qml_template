extern crate cpp_build;

fn main() {
    let qt_lib_path = "C:\\Qt\\6.4.0\\msvc2019_64\\lib";
    let qt_include_path = "C:\\Qt\\6.4.0\\msvc2019_64\\include";

    println!("cargo:rustc-link-search={}", qt_lib_path);

    cpp_build::Config::new()
        .include(qt_include_path)
        .flag("/std:c++17")
        .flag("/Zc:__cplusplus")
        .flag("/permissive-")
        .build("src/main.rs")
}
