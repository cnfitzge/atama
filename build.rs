use std::{collections::HashMap, path::Path};

const LIB: &str = "libtangent_ffi_sys";
const SUPPORTED_PLATFORMS: &[(&str, &str, &str)] = &[
    ("apple", "x86_64-apple-darwin", "dylib"),
    ("linux", "x86_64-unknown-linux-gnu", "so"),
];
const FFI_TOOLCHAIN_VERSION: &str = "1.76.0";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?.canonicalize()?;

    // Generate IDL types from 'res/drift.json'
    let idl_source_path = current_dir.join("res/drift.json");
    let idl_mod_path = current_dir.join("crates/src/tangent_idl.rs");
    generate_idl_types(&idl_source_path, idl_mod_path.as_path())?;
