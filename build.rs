use std::{collections::HashMap, path::Path};

const LIB: &str = "libtangent_ffi_sys";
const SUPPORTED_PLATFORMS: &[(&str, &str, &str)] = &[
    ("apple", "x86_64-apple-darwin", "dylib"),
    ("linux", "x86_64-unknown-linux-gnu", "so"),
];
