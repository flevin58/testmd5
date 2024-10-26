use std::os::raw::c_void;

include!("sha256_bindings.rs");

pub fn calculate(s: &String) -> Box<String> {
    const SHA256_SIZE: usize = 32;
    let mut digest = Box::new(vec![0; SHA256_SIZE]);
    //let digest_ptr = digest.as_mut_ptr() as *mut u8;
    // let mut digest = Box::new([0u8; 16]);
    unsafe {
        sha256(s.as_ptr() as *const c_void, s.len(), digest.as_mut_ptr());
    }
    let mut sha256str = Box::new("".to_string());
    for c in digest.iter() {
        let digits = format!("{c:02x}");
        sha256str.push_str(digits.as_str());
    }
    return sha256str;
}
