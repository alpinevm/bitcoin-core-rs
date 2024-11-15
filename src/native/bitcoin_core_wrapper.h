#ifndef BITCOIN_CORE_WRAPPER_H
#define BITCOIN_CORE_WRAPPER_H

#include "vendor/bitcoin/src/uint256.h"

#ifdef __cplusplus
extern "C"
{
#endif

    /**
     * Performs SHA256 hash on input data
     * @param[in]  input      Pointer to the input data to be hashed
     * @param[in]  input_len  Length of the input data in bytes
     * @param[out] hash_result Pointer to a pre-allocated 32-byte buffer where the hash will be stored
     */
    void sha256_hash(const unsigned char *input, const uint32_t input_len, unsigned char hash_result[32]);

    /**
     * Gets the double SHA256 hash of a Bitcoin block header
     * @param[in]  header_bytes   Pointer to the 80-byte Bitcoin block header
     * @param[out] block_hash     Pointer to a pre-allocated 32-byte buffer where the hash will be stored
     * @return                    Returns true if header was successfully deserialized and hashed,
     *                           false otherwise
     */
    bool get_header_hash(const unsigned char header_bytes[80], unsigned char block_hash[32]);

    /**
     * Validates a Bitcoin block header's proof of work using its embedded nBits
     * @param[in] header_bytes   Pointer to the 80-byte Bitcoin block header
     * @return                  Returns true if the header's proof of work is valid,
     *                         false otherwise
     */
    bool check_proof_of_work(const unsigned char header_bytes[80]);

    /**
     * Gets the height of the last difficulty retarget for a given height
     * @param[in] height       The block height to check
     * @return                The height of the last difficulty retarget
     */
    uint32_t get_retarget_height(const uint32_t height);

    /**
     * Gets the proof of a Bitcoin block header
     * @param[in]  header_bytes   Pointer to the 80-byte Bitcoin block header
     * @param[out] proof          Pointer to a pre-allocated 32-byte buffer where the proof will be stored
     * @return                    Returns true if header was successfully deserialized and proof calculated,
     *                           false otherwise
     */
    bool get_block_proof(const unsigned char header_bytes[80], unsigned char proof[32]);

    /**
     * Calculates the next required proof of work (nBits) for a new block
     *
     * @param[in]  last_retarget_header_bytes  Pointer to the 80-byte header at last retarget
     * @param[in]  previous_height             Height of the previous block
     * @param[in]  previous_header_bytes       Pointer to the 80-byte previous block header
     * @param[in]  header_bytes                Pointer to the 80-byte new block header
     * @param[out] next_nbits                  Pointer where the calculated nBits will be stored
     * @return                                Returns true if calculation was successful,
     *                                        false otherwise
     */
    bool get_next_work_required(
        const unsigned char last_retarget_header_bytes[80],
        const uint32_t previous_height,
        const unsigned char previous_header_bytes[80],
        const unsigned char header_bytes[80],
        uint32_t *next_nbits);

#ifdef __cplusplus
}
#endif

#endif // BITCOIN_CORE_WRAPPER_H
