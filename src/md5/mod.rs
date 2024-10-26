include!("md5_bindings.rs");

pub fn calculate(s: &String) -> Box<String> {
    const MD5_SIZE: usize = 16;
    let mut digest = Box::new(vec![0; MD5_SIZE]);
    //let digest_ptr = digest.as_mut_ptr() as *mut u8;
    // let mut digest = Box::new([0u8; 16]);
    unsafe {
        md5(s.as_ptr(), s.len(), digest.as_mut_ptr());
    }
    let mut md5str = Box::new("".to_string());
    for c in digest.iter() {
        let digits = format!("{c:02x}");
        md5str.push_str(digits.as_str());
    }
    return md5str;
}
