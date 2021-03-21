use bindgen;
use cmake;
use std::{env, path::PathBuf};

fn main() {
    let external = cmake::Config::new("external/libossia")
        .define("OSSIA_C", "ON")
        .define("OSSIA_STATIC", "ON")
        .no_build_target(true)
        .very_verbose(true)
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/src",
        external.display()
    );
    println!("cargo:rustc-link-lib=static=ossia");
    println!("cargo:rerun-if-changed=wrapper.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}/build/src", env::var("OUT_DIR").unwrap()))
        .header("wrapper.h")
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
