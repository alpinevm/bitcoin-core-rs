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

    println!("Checking header...");

    assert!(bitcoin_core_rs::check_pow(serialized_header_array));

    // Encode the public values of the program.
    let bytes = serialized_header_array.to_vec();

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
