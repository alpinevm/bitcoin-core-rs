// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]

sp1_zkvm::entrypoint!(main);
pub fn main() {
    // Read an input to the program.
    let serialized_header = sp1_zkvm::io::read::<Vec<u8>>();
    let serialized_header_array: [u8; 80] = serialized_header.try_into().unwrap();

    println!("Testing all Bitcoin functions...");

    // Test sha256
    let hash = bitcoin_core_rs::sha256(&serialized_header_array);
    println!("SHA256 hash computed");

    // Test get_block_hash
    let block_hash = bitcoin_core_rs::get_block_hash(&serialized_header_array).unwrap();
    println!("Block hash computed");

    // Test check_proof_of_work
    assert!(bitcoin_core_rs::check_proof_of_work(
        &serialized_header_array
    ));
    println!("Proof of work verified");

    // Test get_retarget_height (using a sample height)
    let height = 40320u32;
    let retarget_height = bitcoin_core_rs::get_retarget_height(height);
    println!("Retarget height computed: {}", retarget_height);

    // Test get_block_proof
    let block_proof = bitcoin_core_rs::get_block_proof(&serialized_header_array).unwrap();
    println!("Block proof computed");

    // Encode and commit the public values
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&hash);
    bytes.extend_from_slice(&block_hash);
    bytes.extend_from_slice(&block_proof);
    bytes.extend_from_slice(&serialized_header_array);
    bytes.extend_from_slice(&retarget_height.to_le_bytes());

    sp1_zkvm::io::commit_slice(&bytes);
}
