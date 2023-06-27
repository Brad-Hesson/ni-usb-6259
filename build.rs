use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    let include_folder = PathBuf::from("src/lib").canonicalize().unwrap();
    println!("cargo:rustc-link-search={}", include_folder.display());

    println!("cargo:rustc-link-lib=NIDAQmx");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!(
        "cargo:rerun-if-changed={}",
        include_folder.join("NIDAQmx.h").display()
    );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .header(include_folder.join("NIDAQmx.h").to_str().unwrap())
        .detect_include_paths(true);

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
