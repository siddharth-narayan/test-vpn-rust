use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/network/network.c")
        .compile("network");

    let bindings = bindgen::Builder::default()
        .header("src/network/network.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("c_bindings.rs"))
        .expect("Couldn't write bindings!");
}
