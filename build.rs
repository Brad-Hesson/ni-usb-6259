use std::env;
use std::path::PathBuf;

fn main() {
    let lib_folder = PathBuf::from("lib").canonicalize().unwrap();
    let header_file = lib_folder.join("NIDAQmx.h");

    println!("cargo:rustc-link-search={}", lib_folder.display());
    println!("cargo:rustc-link-lib=NIDAQmx");
    println!("cargo:rerun-if-changed={}", header_file.display());

    let bindings = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .header(header_file.to_str().unwrap())
        .detect_include_paths(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
