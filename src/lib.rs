use hex_literal::hex;

pub const MAINNET_POW_LIMIT: [u8; 32] =
    hex!("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

mod ffi {
    extern "C" {
        pub(crate) fn sha256_hash(input: *const u8, input_len: u32, output: *mut u8);
        pub(crate) fn check_header_pow(serialized_header: *const u8, pow_limit: *const u8) -> bool;
    }
}

pub fn sha256(input: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    unsafe {
        ffi::sha256_hash(input.as_ptr(), input.len() as u32, output.as_mut_ptr());
    }
    output
}

pub fn check_pow(serialized_header: [u8; 80], pow_limit: [u8; 32]) -> bool {
    unsafe { ffi::check_header_pow(serialized_header.as_ptr(), pow_limit.as_ptr()) }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_check_pow_real_block() {
        // Test case 1: Valid bitcoin block header that meets PoW requirement
        let serialized_header =
            hex!("00606a2a6da096d2b8dbbbed775ac73ebffb4f8005625ff082d902000000000000000000636f25b00a6dba593285caae62bc20cb5c022050efdae664ff52255c1c2e1b754de10867cd0e031739d4a0ef");
        assert!(check_pow(serialized_header, MAINNET_POW_LIMIT));
    }

    #[test]
    fn test_check_pow_null_header() {
        let serialized_header =
            hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        assert!(!check_pow(serialized_header, MAINNET_POW_LIMIT));
    }

    #[test]
    fn test_check_pow_fake_block() {
        let serialized_header =
            hex!("010000006024f927c294aafe77f7eff56d0d35e9309dc6a5595b54ffa79200000000000002d8003f9c8c10750d7cb64d3e9cd36cdfc3f0b20db3afd1f25b3657002515a5fa71b04dacb5001ba2e71604");
        assert!(!check_pow(serialized_header, MAINNET_POW_LIMIT));
    }
}
