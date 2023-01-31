use autocxx_build::Builder;
use glob::glob;
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let is_debug = std::env::var("PROFILE").into_diagnostic()?.as_str() == "debug";

    let clang_args = if is_debug {
        ["-std=c++17"].as_slice()
    } else {
        ["-std=c++17", "-flto"].as_slice()
    };

    Builder::new("src/lib.rs", ["RocketSim/src/", "extra_cpp/"])
        .extra_clang_args(clang_args)
        .build()?
        .static_flag(true)
        .use_plt(false)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++17")
        .file("RocketSim/libsrc/bullet3-3.24/btBulletCollisionAll.cpp")
        .file("RocketSim/libsrc/bullet3-3.24/btBulletDynamicsAll.cpp")
        .file("RocketSim/libsrc/bullet3-3.24/btLinearMathAll.cpp")
        .files(glob("RocketSim/src/**/*.cpp").into_diagnostic()?.flatten())
        .file("extra_cpp/extra.cpp")
        .warnings(false)
        .compile("rocketsim");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=PROFILE");

    Ok(())
}
