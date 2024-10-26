use std::{env, process::exit};

mod md5;
mod sha256;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("missing string parameter");
        exit(1);
    };
    let input = args[1..args.len()].join(" ");
    let md5str = md5::calculate(&input);
    let sha256str = sha256::calculate(&input);
    println!("Converting {input}");
    println!("md5...: {}", md5str);
    println!("sha256: {}", sha256str);
}

#[test]
fn test_md5() {
    let input = "Hello, Rust!".to_string();
    let calculated = md5::calculate(&input);
    let expected = "9369f0626346def098fb689eb26cc34f".to_string();
    assert_eq!(*calculated, expected);
}

#[test]
fn test_sha256() {
    let input = "Hello, Rust!".to_string();
    let calculated = sha256::calculate(&input);
    let expected = "12a967da1e8654e129d41e3c016f14e81e751e073feb383125bf82080256ca19".to_string();
    assert_eq!(*calculated, expected);
}
