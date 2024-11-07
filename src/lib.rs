const SHA256_OUTPUT_SIZE: usize = 32;

extern "C" {
    fn sha256_hash(input: *const u8, input_len: u32, output: *mut u8);
    fn check_pow(hex_header: *const i8) -> bool;
}

pub fn sha256(input: &[u8]) -> [u8; SHA256_OUTPUT_SIZE] {
    let mut output = [0u8; SHA256_OUTPUT_SIZE];
    unsafe {
        sha256_hash(input.as_ptr(), input.len() as u32, output.as_mut_ptr());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use std::ffi::CString;

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
    fn test_check_pow_valid() {
        // This is a real Bitcoin block header from block 125552
        let valid_header = "0100000081cd02ab7e569e8bcd9317e2fe99f2de44d49ab2b8851ba4a308000000000000e320b6c2fffc8d750423db8b1eb942ae710e951ed797f7affc8892b0f1fc122bc7f5d74df2b9441a42a14695";
        let c_header = CString::new(valid_header).unwrap();
        let result = unsafe { check_pow(c_header.as_ptr()) };
        assert!(result);
    }

    #[test]
    fn test_check_pow_invalid() {
        // Invalid header (modified nonce)
        let invalid_header = "0100000081cd02ab7e569e8bcd9317e2fe99f2de44d49ab2b8851ba4a308000000000000e320b6c2fffc8d750423db8b1eb942ae710e951ed797f7affc8892b0f1fc122bc7f5d74df2b9441a42a14696";
        let c_header = CString::new(invalid_header).unwrap();
        let result = unsafe { check_pow(c_header.as_ptr()) };
        assert!(!result);
    }
}
