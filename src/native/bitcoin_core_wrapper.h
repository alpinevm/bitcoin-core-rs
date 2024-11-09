#ifndef BITCOIN_CORE_WRAPPER_H
#define BITCOIN_CORE_WRAPPER_H

#include "vendor/bitcoin/src/uint256.h"

#ifdef __cplusplus
extern "C"
{
#endif

    /**
     * Performs SHA256 hash on input data
     * @param input      Pointer to the input data to be hashed
     * @param input_len  Length of the input data in bytes
     * @param output     Pointer to a pre-allocated 32-byte buffer where the hash will be stored
     */
    void sha256_hash(const unsigned char *input, unsigned int input_len, unsigned char output[32]);

    /**
     * Validates a Bitcoin block header's proof of work
     * @param header_bytes   Pointer to the 80-byte Bitcoin block header
     * @param pow_limit      The proof of work limit (target threshold) to check against
     * @return             Returns true if the header's proof of work is valid and below the target,
     *                     false otherwise
     */

    bool check_header_pow(const unsigned char *header_bytes, uint256 pow_limit);

#ifdef __cplusplus
}
#endif

#endif // BITCOIN_CORE_WRAPPER_H
