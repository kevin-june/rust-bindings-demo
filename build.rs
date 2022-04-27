use std::{env, path::PathBuf};

fn main() {
    let mut builder = cc::Build::new();
    println!("cargo:rerun-if-changed=cmodule/cmodule.h");
    println!("cargo:rerun-if-changed=cmodule/cmodule.c");
    builder.file("cmodule/cmodule.h");
    builder.file("cmodule/cmodule.c");
    builder.compile("cmodule");

    let out_dir = env::var("OUT_DIR").unwrap();
    let bindings_path = PathBuf::from(out_dir).join("bindings.rs");

    let mut builder = bindgen::Builder::default();
    builder = builder.header("cmodule/cmodule.h");
    // TODO is this appropriate?
    // It might not be if Rust needs to use a type from this file.
    builder = builder.blocklist_file("stdint.h");
    builder
        .generate()
        .expect("Failed to generate Rust bindings")
        .write_to_file(bindings_path)
        .expect("Failed to write Rust bindings file");
}
