use autocxx_build::Builder;
use glob::glob;
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let mut builder = Builder::new("src/lib.rs", ["RocketSim/src/", "arenar/"]).extra_clang_args(&["-std=c++20"]).build()?;

    builder
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
