use cxx_build::bridges;
use glob::glob;
use std::path::PathBuf;

#[cfg(feature = "bin")]
const SCHEMA_DIR: &str = "./spec";
#[cfg(feature = "bin")]
const OUT_FILE: &str = "./src/flat.rs";

#[cfg(feature = "bin")]
fn format_string(s: &str) -> Result<String, String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("rustfmt");

    child
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = child
        .spawn()
        .map_err(|err| format!("Unable to spawn rustfmt. Perhaps it is not installed? {err}"))?;

    {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin
            .write_all(s.as_bytes())
            .map_err(|err| format!("Unable to write the file to rustfmt: {err}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("Unable to get the formatted file back from rustfmt: {err}"))?;

    if output.status.success() && output.stderr.is_empty() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else if output.stderr.is_empty() {
        Err(format!("rustfmt failed with exit code {}", output.status))
    } else {
        Err(format!(
            "rustfmt failed with exit code {} and message:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}

#[cfg(feature = "bin")]
fn generate_flatbuffers() {
    use std::fs;

    println!("cargo:rerun-if-changed={SCHEMA_DIR}");

    let fbs_path = PathBuf::from(SCHEMA_DIR).join("core.fbs");
    let declarations = planus_translation::translate_files(&[fbs_path.as_path()]).unwrap();
    let raw_out = planus_codegen::generate_rust(&declarations)
        .unwrap()
        .replace("#[no_implicit_prelude]\n", "");
    let formatted_out = format_string(&raw_out).unwrap();

    fs::write(OUT_FILE, formatted_out.as_bytes()).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "bin")]
    generate_flatbuffers();

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
        .flatten()
        .filter(|path| !path.ends_with("mod.rs"))
        .chain([PathBuf::from("src/math.rs"), PathBuf::from("src/lib.rs")])
        .collect::<Vec<_>>();

    for file in &rust_files {
        println!("cargo:rerun-if-changed={}", file.display());
    }

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
