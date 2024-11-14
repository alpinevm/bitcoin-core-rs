//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const PROGRAM_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(
        long,
        default_value = "00606a2a6da096d2b8dbbbed775ac73ebffb4f8005625ff082d902000000000000000000636f25b00a6dba593285caae62bc20cb5c022050efdae664ff52255c1c2e1b754de10867cd0e031739d4a0ef"
    )]
    header_hex: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    let header_bytes = hex::decode(args.header_hex).unwrap();
    stdin.write(&header_bytes);

    if args.execute {
        println!("Executing program...");
        // Execute the program
        let (output, report) = client.execute(PROGRAM_ELF, stdin).run().unwrap();
        println!("Program executed successfully, header valid under PoW...");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(PROGRAM_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
