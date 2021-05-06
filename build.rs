use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper/");

    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    cxx_build::bridge("src/rr/bridge.rs")
        .file("wrapper/wrapper.cpp")
        .shared_flag(true)
        .include("vendor/rr/src/")
        .include("vendor/rr/obj")
        // .include(&out.join("build/rr/src/rr-build"))
        .compile("librr-bridge.so");

    // println!("cargo:rustc-link-search=native={}", out.display());
    // println!("cargo:rustc-link-search=native={}/lib", out.display());
    // println!("cargo:rustc-link-lib=static=wrapper");
    println!("cargo:rustc-link-lib=dylib=rr");
}
