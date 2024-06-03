use cxx_build::bridges;
use glob::glob;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");

    let cpp_files = glob("RocketSim/libsrc/bullet3-3.24/**/*.cpp")
        .unwrap()
        .chain(glob("RocketSim/src/**/*.cpp").unwrap())
        .flatten()
        .chain([PathBuf::from("arenar/arenar.cpp")])
        .collect::<Vec<_>>();

    for file in &cpp_files {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    let rust_files: Vec<PathBuf> = glob("src/sim/*.rs")
        .unwrap()
        .chain(glob("RocketSim/src/**/*.rs").unwrap())
        .flatten()
        .filter(|path| !path.ends_with("mod.rs"))
        .chain([PathBuf::from("src/math.rs"), PathBuf::from("src/lib.rs")])
        .collect::<Vec<_>>();

    let mut builder = bridges(rust_files);

    if !cfg!(debug_assertions) || !cfg!(feature = "debug_logging") {
        builder.define("RS_DONT_LOG", "1");
    }

    if !cfg!(debug_assertions) {
        builder.define("RS_MAX_SPEED", "1");
        builder.flag_if_supported("-flto=thin");
        builder.opt_level_str("3");
    }

    builder
        .includes(["RocketSim/src/", "arenar/"])
        .std("c++20")
        .use_plt(false)
        .flag_if_supported("-march=native")
        .flag_if_supported("-w")
        .files(cpp_files)
        .warnings(false)
        .compile("rocketsim");
}
