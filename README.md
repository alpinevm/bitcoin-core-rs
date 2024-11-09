# Rust bitcoin-core
Rust bindings with a minimal C++ FFI to access the validation engine of `bitcoin-core`<br>
**supports SP1! (riscv32im)**

## Requirements

- Rust (1.81 tested)
- Clang (15.0.0 tested)

## Install RISCV GNU Toolchain

Run the following before then follow the installation instructions for your OS:
```
git clone https://github.com/riscv-collab/riscv-gnu-toolchain && cd riscv-gnu-toolchain
```

### Ubuntu 
#### Prerequisites
```
sudo apt-get install autoconf automake autotools-dev curl python3 python3-pip libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev ninja-build git cmake libglib2.0-dev libslirp-dev
```


#### Configure Toolchain for SP1
```
./configure --prefix="/opt/riscv" --with-arch=rv32im
```

#### Build Toolchain 
```
sudo gmake -j<num_cores>
```

### MacOs
#### Prerequisites
```
brew install python3 gawk gnu-sed make gmp mpfr libmpc isl zlib expat texinfo flock libslirp llvm@19
```

#### Symlink `gmp` and `mpfr` 
```
sudo ln -s /opt/homebrew/opt/gmp/include/gmp.h /usr/local/include/gmp.h
sudo ln -s /opt/homebrew/opt/gmp/lib/libgmp.dylib /usr/local/lib/libgmp.dylib

sudo ln -s /opt/homebrew/opt/mpfr/include/mpfr.h /usr/local/include/mpfr.h
sudo ln -s /opt/homebrew/opt/mpfr/lib/libmpfr.dylib /usr/local/lib/libmpfr.dylib
```

#### Configure Toolchain for SP1
```
./configure --prefix="/opt/riscv" --with-arch=rv32im
```

#### Build Toolchain 
```
sudo gmake -j<num_cores>
```

Using as library on MacOs
Clang 15 or higher is known to function properly, instead of changing your default-installed clang version, temporarily use this alias, assuming you installed `llvm` as instructed above. 
```
alias clang++="/opt/homebrew/opt/llvm/bin/clang++"
```
