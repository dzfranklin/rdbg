use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper/");

    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // cmake::Config::new("wrapper").build();

    cxx_build::bridge("src/rr/bridge.rs")
        .file("wrapper/wrapper.cpp")
        .shared_flag(true)
        .include("wrapper/vendor/rr/src/")
        .include("wrapper/vendor/rr/obj")
        // .include(&out.join("build/rr/src/rr-build"))
        .compile("rr-bridge.so");

    // println!("cargo:rustc-link-search=native={}", out.display());
    // println!("cargo:rustc-link-search=native={}/lib", out.display());
    // println!("cargo:rustc-link-lib=static=wrapper");
    // println!("cargo:rustc-link-lib=static=rr-bridge");
    println!("cargo:rustc-link-lib=dylib=rr");
}
