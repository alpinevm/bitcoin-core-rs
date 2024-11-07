mod ffi {
    extern "C" {
        pub(crate) fn sha256_hash(input: *const u8, input_len: u32, output: *mut u8);
        pub(crate) fn check_pow(hash: *const u8, n_bits: u32, pow_limit: *const u8) -> bool;
    }
}

pub fn sha256(input: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    unsafe {
        ffi::sha256_hash(input.as_ptr(), input.len() as u32, output.as_mut_ptr());
    }
    output
}

pub fn check_pow(hash: [u8; 32], n_bits: u32, pow_limit: [u8; 32]) -> bool {
    unsafe {
        ffi::check_pow(hash.as_ptr(), n_bits, pow_limit.as_ptr())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_sha256() {
        let test_bytes = hex!("deadbeef");
        let hash = sha256(&test_bytes);
        assert_eq!(
            hash,
            hex!("5f78c33274e43fa9de5659265c1d917e25c03722dcb0b8d27db8d5feaa813953")
        );
    }

    #[test]
    fn test_sha256_empty() {
        let hash = sha256(&[]);
        assert_eq!(
            hash,
            hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
    }

    #[test]
    fn test_check_pow() {
        // Test case 1: Hash that meets PoW requirement
        let hash = hex!("00000000000000000001f6e715e763d970565883c0d0c424f6236c0f9fed4559");
        let pow_limit = hex!("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        println!("hash: {:?}", hash);
        println!("pow_limit: {:?}", pow_limit);
        println!("check_pow: {:?}", check_pow(hash, 0, pow_limit));
        assert!(check_pow(hash, 0, pow_limit));
    }
}
