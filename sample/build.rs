use std::{env, path::Path};

fn main() {
    if env::var("TARGET").expect("target").ends_with("windows-msvc") {
        let manifest = Path::new("Microsoft.WindowsAppSdk.Foundation.manifest").canonicalize().unwrap();
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}", manifest.display());
        println!("cargo:rerun-if-changed=hello.exe.manifest");
    }
    println!("cargo:rerun-if-changed=build.rs");
}

