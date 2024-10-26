extern "C" {
    // sha256 C lib on github: https://github.com/ilvn/SHA256.git
    // void sha256(const void *data, size_t len, uint8_t *hash);
    fn sha256(initial_msg: *const u8, initial_len: usize, digest: *mut u8);
}

pub fn calculate(s: &String) -> Box<String> {
    const SHA256_SIZE: usize = 32;
    let mut digest = Box::new(vec![0; SHA256_SIZE]);
    //let digest_ptr = digest.as_mut_ptr() as *mut u8;
    // let mut digest = Box::new([0u8; 16]);
    unsafe {
        sha256(s.as_ptr(), s.len(), digest.as_mut_ptr());
    }
    let mut sha256str = Box::new("".to_string());
    for c in digest.iter() {
        let digits = format!("{c:02x}");
        sha256str.push_str(digits.as_str());
    }
    return sha256str;
}
