extern crate cpp_build;


const WIN_QT_LIB_PATH: &str = "C:\\Qt\\6.4.0\\msvc2019_64\\lib";
const WIN_QT_INCLUDE_PATH: &str = "C:\\Qt\\6.4.0\\msvc2019_64\\include";
const MACOS_QT_LIB_PATH: &str = "/Users/root/Qt6/6.3.1/macos//lib";

fn create_build_config_win() {
    let qt_module_include_path = |module_name| format!("{}/{}", WIN_QT_LIB_PATH, module_name);

    cpp_build::Config::new()
        .include(qt_module_include_path("QtQml"))
        .flag("/std:c++17")
        .flag("/Zc:__cplusplus")
        .flag("/permissive-")
        .build("src/main.rs");

    println!("cargo:rustc-link-search={}", WIN_QT_INCLUDE_PATH);
}

fn create_build_config_macos() {
    let qt_module_include_path = |module_name|
        format!(
            "{}/{}.framework/Versions/A/Headers/",
            MACOS_QT_LIB_PATH, module_name
        );

    cpp_build::Config::new()
        .include(qt_module_include_path("QtQml"))
        .flag("-std=c++17")
        .build("src/main.rs");

    println!("cargo:rustc-link-search={}", MACOS_QT_LIB_PATH);
}

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "macos" => create_build_config_macos(),
        "win" => create_build_config_win(),
        os => unimplemented!("Build for {} not supported.", os),
    }
}
