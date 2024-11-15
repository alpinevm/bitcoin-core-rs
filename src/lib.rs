pub mod error;

use crate::error::BitcoinError;

use eyre::Result;

mod ffi {
    extern "C" {
        pub(crate) fn sha256_hash(input: *const u8, input_len: u32, output: *mut u8);
        pub(crate) fn get_header_hash(header_bytes: *const u8, block_hash: *mut u8) -> bool;
        pub(crate) fn check_proof_of_work(header_bytes: *const u8) -> bool;
        pub(crate) fn get_retarget_height(height: u32) -> u32;
        pub(crate) fn get_next_work_required(
            last_retarget_header_bytes: *const u8,
            previous_height: u32,
            previous_header_bytes: *const u8,
            header_bytes: *const u8,
            next_nbits: *mut u32,
        ) -> bool;
        pub(crate) fn get_block_proof(header_bytes: *const u8, proof: *mut u8) -> bool;
    }
}

pub fn sha256(input: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    unsafe {
        ffi::sha256_hash(input.as_ptr(), input.len() as u32, output.as_mut_ptr());
    }
    output
}

pub fn get_block_hash(header: &[u8; 80]) -> Result<[u8; 32]> {
    let mut hash = [0u8; 32];
    let success = unsafe { ffi::get_header_hash(header.as_ptr(), hash.as_mut_ptr()) };
    if success {
        Ok(hash)
    } else {
        Err(BitcoinError::BlockHashError.into())
    }
}

pub fn check_proof_of_work(header: &[u8; 80]) -> bool {
    unsafe { ffi::check_proof_of_work(header.as_ptr()) }
}

pub fn get_retarget_height(height: u32) -> u32 {
    unsafe { ffi::get_retarget_height(height) }
}

pub fn get_next_work_required(
    last_retarget_header: &[u8; 80],
    previous_height: u32,
    previous_header: &[u8; 80],
    current_header: &[u8; 80],
) -> Result<u32> {
    let mut next_nbits = 0u32;
    let success = unsafe {
        ffi::get_next_work_required(
            last_retarget_header.as_ptr(),
            previous_height,
            previous_header.as_ptr(),
            current_header.as_ptr(),
            &mut next_nbits,
        )
    };
    if success {
        Ok(next_nbits)
    } else {
        Err(BitcoinError::WorkRequirementError.into())
    }
}

pub fn get_block_proof(header: &[u8; 80]) -> Result<[u8; 32]> {
    let mut proof = [0u8; 32];
    let success = unsafe { ffi::get_block_proof(header.as_ptr(), proof.as_mut_ptr()) };
    if success {
        Ok(proof)
    } else {
        Err(BitcoinError::DeserializeError.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;
    use alloy_primitives::{utils::format_units, U256};
    use std::collections::HashMap;

    // Define a HashMap of height to hex values
    fn get_headers() -> HashMap<u32, &'static [u8; 80]> {
        let mut headers = HashMap::new();
        headers.insert(38304, &hex!("01000000858a5c6d458833aa83f7b7e56d71c604cb71165ebb8104b82f64de8d00000000e408c11029b5fdbb92ea0eeb8dfa138ffa3acce0f69d7deebeb1400c85042e01723f6b4bc38c001d09bd8bd5"));
        headers.insert(40318, &hex!("0100000007aa5faf1c4273054f1d415318ffc140afacd41708e47442c496e63a00000000a7176bc7da91e5eedfb39c0ad51e58f06f80e45b2edcd3ca900eee746fe1b2f19b86784bc38c001d8ede8c07"));
        headers.insert(40319, &hex!("01000000a4eaafee7ae520b206e56ae73af34cdeedf022fd000081ef74a4830600000000396f6849a2da8ddd7c3452647f30c2fe9feb3e5c089bd1eb36808374ca36d7b4f986784bc38c001d6047eb01"));
        headers.insert(40320, &hex!("010000001a231097b6ab6279c80f24674a2c8ee5b9a848e1d45715ad89b6358100000000a822bafe6ed8600e3ffce6d61d10df1927eafe9bbf677cb44c4d209f143c6ba8db8c784b5746651cce222118"));
        headers.insert(40321, &hex!("0100000045720d24eae33ade0d10397a2e02989edef834701b965a9b161e864500000000993239a44a83d5c427fd3d7902789ea1a4d66a37d5848c7477a7cf47c2b071cd7690784b5746651c3af7ca03"));
        headers
    }

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
        assert!(check_proof_of_work(&serialized_header));
    }

    #[test]
    fn test_check_pow_null_header() {
        let serialized_header =
            hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        assert!(!check_proof_of_work(&serialized_header));
    }

    #[test]
    fn test_check_pow_fake_block() {
        let serialized_header =
            hex!("010000006024f927c294aafe77f7eff56d0d35e9309dc6a5595b54ffa79200000000000002d8003f9c8c10750d7cb64d3e9cd36cdfc3f0b20db3afd1f25b3657002515a5fa71b04dacb5001ba2e71604");
        assert!(!check_proof_of_work(&serialized_header));
    }

    #[test]
    fn test_header_hash() {
        let mut expected_hash =
            hex!("0000000015bb50096055846954f7120e30d6aa2bd5ab8d4a4055ceacc853328a");
        expected_hash.reverse();
        let header = hex!("01000000858a5c6d458833aa83f7b7e56d71c604cb71165ebb8104b82f64de8d00000000e408c11029b5fdbb92ea0eeb8dfa138ffa3acce0f69d7deebeb1400c85042e01723f6b4bc38c001d09bd8bd5");
        let hash = get_block_hash(&header).unwrap();
        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_get_retarget_height() {
        assert_eq!(get_retarget_height(0), 0);
        assert_eq!(get_retarget_height(2015), 0);
        assert_eq!(get_retarget_height(2016), 0);
        assert_eq!(get_retarget_height(2017), 2016);

        assert_eq!(get_retarget_height(4031), 2016);
        assert_eq!(get_retarget_height(4032), 2016);
        assert_eq!(get_retarget_height(4033), 4032);

        assert_eq!(get_retarget_height(40319), 38304);
        assert_eq!(get_retarget_height(40320), 38304);
        assert_eq!(get_retarget_height(40321), 40320);
    }

    #[test]
    fn test_get_next_work_required_during_retarget() {
        let headers = get_headers();
        let retarget_height = 38304;
        let previous_height = 40319;
        let next_height = 40320;
        let last_retarget_header = *headers.get(&retarget_height).unwrap();
        let previous_header = *headers.get(&previous_height).unwrap();
        let new_header = *headers.get(&next_height).unwrap();

        let result = get_next_work_required(
            last_retarget_header,
            previous_height,
            previous_header,
            new_header,
        )
        .unwrap();

        // The known target for block 40320
        assert_eq!(result, 0x1c654657);
    }

    #[test]
    fn test_get_next_work_required_right_before_retarget() {
        let headers = get_headers();
        let retarget_height = 38304;
        let previous_height = 40318;
        let next_height = 40319;
        let last_retarget_header = *headers.get(&retarget_height).unwrap();
        let previous_header = *headers.get(&previous_height).unwrap();
        let new_header = *headers.get(&next_height).unwrap();

        let result = get_next_work_required(
            last_retarget_header,
            previous_height,
            previous_header,
            new_header,
        )
        .unwrap();

        assert_eq!(result, 0x1d008cc3);
    }

    #[test]
    fn test_get_next_work_required_right_after_retarget() {
        let headers = get_headers();
        let retarget_height = 38304;
        let previous_height = 40320;
        let next_height = 40321;
        let last_retarget_header = *headers.get(&retarget_height).unwrap();
        let previous_header = *headers.get(&previous_height).unwrap();
        let new_header = *headers.get(&next_height).unwrap();

        let result = get_next_work_required(
            last_retarget_header,
            previous_height,
            previous_header,
            new_header,
        )
        .unwrap();

        assert_eq!(result, 0x1c654657);
    }

    #[test]
    fn test_get_block_proof_genesis_block() {
        let mut expected_proof =
            hex!("0000000000000000000000000000000000000000000000000000000100010001");
        expected_proof.reverse();
        let genesis_header = hex!("0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c");
        let proof = get_block_proof(&genesis_header).unwrap();
        assert_eq!(proof, expected_proof);
    }

    #[test]
    fn test_get_block_proof_standard_block() {
        let mut expected_proof =
            hex!("0000000000000000000000000000000000000000000000000000aa83470b0222");
        expected_proof.reverse();

        let mut previous_header_proof =
            hex!("0000000000000000000000000000000000000000000000000000aa80bfeea100");
        previous_header_proof.reverse();

        // block 40320
        let header = hex!("010000001a231097b6ab6279c80f24674a2c8ee5b9a848e1d45715ad89b6358100000000a822bafe6ed8600e3ffce6d61d10df1927eafe9bbf677cb44c4d209f143c6ba8db8c784b5746651cce222118");
        let proof = get_block_proof(&header).unwrap();

        let calculated_chainwork: [u8; 32] =
            (U256::from_le_bytes(previous_header_proof) + U256::from_le_bytes(proof)).to_le_bytes();

        assert_eq!(calculated_chainwork, expected_proof);
    }
}
