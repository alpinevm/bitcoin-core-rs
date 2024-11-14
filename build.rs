use std::env;

fn main() {
    let mut base_config = cc::Build::new();
    const DEFAULT_RISCV_GNU_TOOLCHAIN: &str = "/opt/riscv";
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "riscv32" {
        println!("cargo:rerun-if-env-changed=RISCV_GNU_TOOLCHAIN");

        let riscv_gnu_toolchain_path =
            env::var("RISCV_GNU_TOOLCHAIN").unwrap_or_else(|_| DEFAULT_RISCV_GNU_TOOLCHAIN.into());

        base_config
            .compiler("clang++")
            .no_default_flags(true)
            .flag(&format!(
                "--sysroot={riscv_gnu_toolchain_path}/riscv32-unknown-elf"
            ))
            .flag(&format!("--gcc-toolchain={riscv_gnu_toolchain_path}"))
            .flag("--target=riscv32-unknown-none-elf")
            .flag("-march=rv32im")
            .flag("-mabi=ilp32")
            .flag("-mcmodel=medany")
            .flag("-Os")
            .flag("-fdata-sections")
            .flag("-ffunction-sections")
            .flag("-flto")
            .flag("-fno-threadsafe-statics")
            .flag("-D_POSIX_TIMERS")
            .flag("-include")
            .flag("sys/time.h")
            .target("riscv32im-unknown-none-elf");
    }

    base_config
        .flag("-std=c++20")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        .flag("-fvisibility=default")
        .include("src/native/vendor/bitcoin/src")
        .include("src/native/vendor/bitcoin/src/univalue/include")
        .file("src/native/vendor/bitcoin/src/crypto/sha256.cpp")
        .file("src/native/vendor/bitcoin/src/pow.cpp")
        .file("src/native/vendor/bitcoin/src/uint256.cpp")
        .file("src/native/vendor/bitcoin/src/arith_uint256.cpp")
        .file("src/native/vendor/bitcoin/src/primitives/block.cpp")
        .file("src/native/vendor/bitcoin/src/streams.cpp")
        .file("src/native/vendor/bitcoin/src/support/cleanse.cpp")
        .file("src/native/vendor/bitcoin/src/chain.cpp")
        .file("src/native/vendor/bitcoin/src/chainparams.cpp")
        .file("src/native/vendor/bitcoin/src/kernel/chainparams.cpp")
        .file("src/native/vendor/bitcoin/src/chainparamsbase.cpp")
        .file("src/native/vendor/bitcoin/src/consensus/merkle.cpp")
        .file("src/native/vendor/bitcoin/src/util/chaintype.cpp")
        .file("src/native/vendor/bitcoin/src/util/strencodings.cpp")
        .file("src/native/vendor/bitcoin/src/util/string.cpp")
        .file("src/native/vendor/bitcoin/src/util/time.cpp")
        .file("src/native/vendor/bitcoin/src/deploymentinfo.cpp")
        .file("src/native/vendor/bitcoin/src/hash.cpp")
        .file("src/native/vendor/bitcoin/src/primitives/transaction.cpp")
        .file("src/native/vendor/bitcoin/src/script/interpreter.cpp")
        .file("src/native/vendor/bitcoin/src/script/script.cpp")
        .file("src/native/vendor/bitcoin/src/crypto/hex_base.cpp")
        .file("src/native/vendor/bitcoin/src/common/args.cpp")
        .file("src/native/vendor/bitcoin/src/util/threadnames.cpp")
        .file("src/native/vendor/bitcoin/src/common/settings.cpp")
        .file("src/native/vendor/bitcoin/src/univalue/lib/univalue.cpp")
        .file("src/native/vendor/bitcoin/src/univalue/lib/univalue_get.cpp")
        .file("src/native/vendor/bitcoin/src/univalue/lib/univalue_write.cpp")
        .file("src/native/vendor/bitcoin/src/univalue/lib/univalue_read.cpp")
        .file("src/native/bitcoin_core_wrapper.cpp")
        .compile("bitcoin_core_lib");

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        println!("cargo:rustc-link-lib=c++");
    } else if env::var("CARGO_CFG_TARGET_OS").unwrap() == "linux" {
        println!("cargo:rustc-link-lib=stdc++");
    } else if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "riscv32" {
        let riscv_gnu_toolchain_path =
            env::var("RISCV_GNU_TOOLCHAIN").unwrap_or_else(|_| DEFAULT_RISCV_GNU_TOOLCHAIN.into());

        println!("cargo:rustc-link-search={}/lib", riscv_gnu_toolchain_path);
        println!(
            "cargo:rustc-link-search={}/riscv32-unknown-elf/lib",
            riscv_gnu_toolchain_path
        );
        println!(
            "cargo:rustc-link-search={}/lib/gcc/riscv32-unknown-elf/14.2.0",
            riscv_gnu_toolchain_path
        );

        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=c");
        println!("cargo:rustc-link-lib=gcc");
        println!("cargo:rustc-link-lib=nosys");
    }

    println!("cargo:rerun-if-changed=src/native");
}
