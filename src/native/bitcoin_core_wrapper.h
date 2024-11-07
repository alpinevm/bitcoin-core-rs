#ifndef BITCOIN_CORE_WRAPPER_H
#define BITCOIN_CORE_WRAPPER_H

#include "vendor/bitcoin/src/uint256.h"

#ifdef __cplusplus
extern "C"
{
#endif

    // Perform SHA256 hash on input data
    // input: pointer to input data
    // input_len: length of input data in bytes
    // output: pointer to output buffer (must be at least 32 bytes)
    void sha256_hash(const unsigned char *input, unsigned int input_len, unsigned char output[32]);
    bool check_pow(uint256 hash, unsigned int nBits, uint256 powLimit);

#ifdef __cplusplus
}
#endif

#endif // BITCOIN_CORE_WRAPPER_H
