use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::Command;
use glob::glob;
use tonic_build;
use syn::{Item};
use prettyplease::unparse;

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

    std::fs::remove_dir_all("gen")?;
    std::fs::create_dir_all("gen/src")?;

    // prost (used by tonic-build) can't expand globs, so we have to glob and list protos ourselves
    // See https://github.com/tokio-rs/prost/issues/469
    // This could be upstreamed.
    let proto_glob = Path::new(&proto_export_dir).join("dydxprotocol/**/*.proto");
    let protos: Vec<PathBuf> = glob(proto_glob.to_str().unwrap()).unwrap().map(|p| p.unwrap()).collect();

    // Generate Rust code
    tonic_build::configure()
        .include_file("lib.rs")
        .out_dir("gen/src")
        .extern_path(".cosmos", "::cosmos_sdk_proto::cosmos")
        .compile(&protos, &[proto_export_dir])?;

    // Patch the syntax tree in lib.rs to make dydxprotocol the top level export
    let file_path = "gen/src/lib.rs";
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let syntax_tree = syn::parse_file(&content).expect("Failed to parse Rust file");

    // Find the AST node representing the dydxprotocol module
    let dydxprotocol_module = syntax_tree.items.into_iter().find(|item|
        if let Item::Mod(item_mod) = item {
            item_mod.ident.to_string() == "dydxprotocol"
        } else {
            false
        }
    );

    if let Some(Item::Mod(dydxprotocol_module)) = dydxprotocol_module {
        if let Some((_, items)) = dydxprotocol_module.content {
            let modified_syntax_tree = syn::File {
                shebang: None,
                attrs: Vec::new(),
                items: items,
            };

            fs::write(file_path, unparse(&modified_syntax_tree)).expect("Failed to write modified file");
        }
    }

    // Copy Cargo manifest into /gen directory
    fs::copy("src/Cargo.toml.tpl", "gen/Cargo.toml")?;

    // Ready to publish.
    Ok(())
}