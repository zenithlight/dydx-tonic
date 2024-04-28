use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::Command;
use glob::glob;
use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dydx_source_dir = Path::new(&current_dir).join("v4-chain");
    let proto_export_dir = Path::new(&dydx_source_dir).join(".proto-export-deps");

    // Prepare .proto dependencies in v4-chain repo
    Command::new("make")
        .arg("proto-export-deps")
        .current_dir(dydx_source_dir)
        .status()
        .expect("Failed to run Make target");

    let _ = fs::remove_dir_all("gen"); // ignore the error in case the directory doesn't already exist
    fs::create_dir_all("gen/src")?;

    // prost (used by tonic-build) can't expand globs, so we have to glob and list protos ourselves
    // See https://github.com/tokio-rs/prost/issues/469
    // This could be upstreamed.
    let proto_glob = Path::new(&proto_export_dir).join("dydxprotocol/**/*.proto");
    let protos: Vec<PathBuf> = glob(proto_glob.to_str().unwrap()).unwrap().map(|p| p.unwrap()).collect();

    // Generate Rust code
    tonic_build::configure()
        .include_file("lib.rs")
        .out_dir("gen/src")
        .build_server(false)
        .compile(&protos, &[proto_export_dir])?;

    // Copy Cargo manifest into /gen directory
    fs::copy("src/Cargo.toml.tpl", "gen/Cargo.toml")?;

    // Ready to publish.
    Ok(())
}