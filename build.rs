use autocxx_build::Builder;
use glob::glob;
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let is_debug = std::env::var("PROFILE").into_diagnostic()?.as_str() == "debug";

    let clang_args = if is_debug { ["-std=c++20"].as_slice() } else { ["-std=c++20", "-flto"].as_slice() };

    let mut builder = Builder::new("src/lib.rs", ["RocketSim/src/", "arenar/"]).extra_clang_args(clang_args).build()?;

    // A bug in AutoCXX prevents us from being able to use LTO
    // if !is_debug {
    //     builder.flag_if_supported("-flto").flag_if_supported("/GL");
    // }

    builder
        .static_flag(true)
        .use_plt(false)
        .flag_if_supported("-std=c++20")
        .flag_if_supported("/std:c++20")
        .flag_if_supported("-w")
        .files(glob("RocketSim/libsrc/bullet3-3.24/**/*.cpp").into_diagnostic()?.flatten())
        .files(glob("RocketSim/src/**/*.cpp").into_diagnostic()?.flatten())
        .file("arenar/arenar.cpp")
        .warnings(false)
        .compile("rocketsim");

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
