# Rust <> Bitcoin Core 
Bitcoin Core consensus logic exposed to Rust via C++ FFI<br>

*Supports SP1! (riscv32im)*

## Why?
`rust-bitcoin` is not designed for consensus validation according to their own [docs](https://github.com/rust-bitcoin/rust-bitcoin?tab=readme-ov-file#consensus). Instead of reimplementing the consensus critical parts of Bitcoin, just expose the internal Bitcoin Core functions needed for block validation over a minimal FFI!

## Limitations
Currently, this library only exposes the functionality necessary for a Bitcoin *zk* light client being built for [Rift](https://rift.exchange)<br>
This means that only the logic for validating block header proof of work is exposed. Support for full block validation may be added in the future

## TODO:
- expose chainwork calculation
- add pure AcceptBlockHeader like function to FFI?
- create patch file of bitcoin diff
- validate build README instructions for Ubuntu and MacOs
- exclude vendored files and sp1-test code in crate
- documentation

## Tests
Unit tests
```
cargo test
```
Smoke test RISCV target using SP1
```
cargo run --release --manifest-path sp1-test/Cargo.toml -- --execute
```

## Usage 

On typical Unix-like platforms, the build script should handle linking by default.
For RISCV, continue as follows:

## RISCV Build Instructions
If you want to use this library within an SP1 program for example, you'll need to ensure you have a valid riscv toolchain:

### Requirements
- Rust (1.81.0 tested)
- Clang++ (15.0.0 tested)

### Install RISCV GNU Toolchain

Run the following before then follow the installation instructions for your OS:
```
git clone https://github.com/riscv-collab/riscv-gnu-toolchain && cd riscv-gnu-toolchain
```

#### Ubuntu 
##### Prerequisites
```
sudo apt-get install autoconf automake autotools-dev curl python3 python3-pip libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev ninja-build git cmake libglib2.0-dev libslirp-dev
```


##### Configure Toolchain for SP1
```
./configure --prefix="/opt/riscv" --with-arch=rv32im
```

##### Build Toolchain 
```
sudo make -j<num_cores>
```

#### MacOs
##### Prerequisites
```
brew install python3 gawk gnu-sed make gmp mpfr libmpc isl zlib expat texinfo flock libslirp llvm@19
```

##### Symlink `gmp` and `mpfr` 
```
sudo ln -s /opt/homebrew/opt/gmp/include/gmp.h /usr/local/include/gmp.h
sudo ln -s /opt/homebrew/opt/gmp/lib/libgmp.dylib /usr/local/lib/libgmp.dylib

sudo ln -s /opt/homebrew/opt/mpfr/include/mpfr.h /usr/local/include/mpfr.h
sudo ln -s /opt/homebrew/opt/mpfr/lib/libmpfr.dylib /usr/local/lib/libmpfr.dylib
```

##### Configure Toolchain for SP1
```
./configure --prefix="/opt/riscv" --with-arch=rv32im
```

##### Build Toolchain 
```
sudo gmake -j<num_cores>
```

Using as library on MacOs
Clang 15 or higher is known to function properly, instead of changing your default-installed clang version, temporarily use this alias, assuming you installed `llvm` as instructed above. 
```
alias clang++="/opt/homebrew/opt/llvm/bin/clang++"
```



