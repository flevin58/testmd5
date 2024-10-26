mod md5;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("missing string parameter");
        exit(1);
    };
    let md5str = md5::calculate(&args[1]);
    println!("{}", md5str);
}

#[test]
fn test_md5() {
    let input = "Hello, Rust!".to_string();
    let calculated = md5::calculate(&input);
    let expected = "9369f0626346def098fb689eb26cc34f".to_string();
    assert_eq!(*calculated, expected);
}
