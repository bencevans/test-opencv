use std::env::var;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();

    let lib_dir = format!("{}/opencv/build/3rdparty/lib", manifest_dir);

    println!("cargo:rustc-link-search={}", lib_dir);

    for lib_file in std::fs::read_dir(lib_dir).unwrap() {
        let entry = lib_file.unwrap();
        let lib_path = entry.path();
        let lib = lib_path.file_stem().unwrap().to_str().unwrap();

        println!(
            "cargo:rustc-link-lib={}",
            lib.strip_prefix("lib").unwrap().replace(".a", "")
        );
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenCL");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
}
