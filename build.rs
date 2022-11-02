use std::{env::var, path::PathBuf};

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();

    let lib_dir = PathBuf::from(manifest_dir).join(PathBuf::from("opencv\\build\\3rdparty\\lib"));

    println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());

    for lib_file in std::fs::read_dir(lib_dir).unwrap() {
        let entry = lib_file.unwrap();
        let lib_path = entry.path();
        let lib = lib_path.file_stem().unwrap().to_str().unwrap();

        println!(
            "cargo:rustc-link-lib={}",
            lib.strip_prefix("lib").unwrap()
        );
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenCL");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
}
