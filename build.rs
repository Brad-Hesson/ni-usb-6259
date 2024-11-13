use std::env;
use std::path::PathBuf;

fn main() {
    let pwd_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_folder = PathBuf::from(pwd_dir).join("lib");
    let header_file = lib_folder.join("NIDAQmx.h");

    println!("cargo:rustc-link-search={}", lib_folder.to_string_lossy());
    #[cfg(target_family = "unix")]
    println!("cargo:rustc-link-lib=dylib=nidaqmx");
    #[cfg(target_family = "windows")]
    println!("cargo:rustc-link-lib=NIDAQmx");

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
