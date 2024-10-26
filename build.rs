// build.rs

fn main() {
    cc::Build::new().file("src/md5.c").compile("md5");
    cc::Build::new().file("src/sha256.c").compile("sha256");
    println!("cargo::rerun-if-changed=src/md5.c");
    println!("cargo::rerun-if-changed=src/sha256.c");
}
