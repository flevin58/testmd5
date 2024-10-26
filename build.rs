// build.rs
use bindgen;

fn main() {
    // Generate bindings for MD5
    let bindings = bindgen::Builder::default()
        .header("vendor/md5/md5.c")
        .allowlist_function("md5")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/md5/md5_bindings.rs")
        .expect("Couldn't write bindings!");

    // Generate bindings for SHA256
    let bindings = bindgen::Builder::default()
        .header("vendor/sha256/sha256.h")
        .allowlist_function("sha256")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/sha256/sha256_bindings.rs")
        .expect("Couldn't write bindings!");

    // Library MD5
    cc::Build::new().file("vendor/md5/md5.c").compile("md5");
    println!("cargo::rerun-if-changed=src/md5.c");

    // Library SHA256
    cc::Build::new()
        .file("vendor/sha256/sha256.c")
        .compile("sha256");
    println!("cargo::rerun-if-changed=src/sha256.c");
}
