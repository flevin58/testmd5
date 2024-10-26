// build.rs

fn main() {
    cc::Build::new().file("src/md5.c").compile("md5");
    println!("cargo::rerun-if-changed=src/md5.c");
}
