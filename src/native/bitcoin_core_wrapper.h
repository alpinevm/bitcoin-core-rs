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
    void sha256_hash(const unsigned char *input, const uint32_t input_len, unsigned char *hash_result);

    /**
     * Gets the double SHA256 hash of a Bitcoin block header
     * @param[in]  header_bytes   Pointer to the 80-byte Bitcoin block header
     * @param[out] block_hash     Pointer to a pre-allocated 32-byte buffer where the hash will be stored
     * @return                    Returns true if header was successfully deserialized and hashed,
     *                           false otherwise
     */
    bool get_header_hash(const unsigned char *header_bytes, unsigned char *block_hash);

    /**
     * Validates a Bitcoin block header's proof of work using its embedded nBits
     * @param[in] header_bytes   Pointer to the 80-byte Bitcoin block header
     * @return                  Returns true if the header's proof of work is valid,
     *                         false otherwise
     */
    bool check_proof_of_work(const unsigned char *header_bytes);

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
    bool get_block_proof(const unsigned char *header_bytes, unsigned char *proof);

    /**
     * Validates the next required proof of work (nBits) for a block
     *
     * @param[in]  last_retarget_header_bytes  Pointer to the 80-byte header at last retarget
     * @param[in]  previous_height             Height of the previous block
     * @param[in]  previous_header_bytes       Pointer to the 80-byte previous block header
     * @param[in]  header_bytes                Pointer to the 80-byte new block header
     * @param[out] next_retarget_header_bytes  Pointer to a pre-allocated 80-byte buffer for the next retarget header
     * @return                                 Returns true if the header's nBits are valid,
     *                                         false otherwise
     */
    bool validate_next_work_required(
        const unsigned char *last_retarget_header_bytes,
        const uint32_t previous_height,
        const unsigned char *previous_header_bytes,
        const unsigned char *header_bytes,
        unsigned char *next_retarget_header_bytes);

    /**
     * Checks if a header properly connects to its previous header
     * @param[in]  header_bytes           Pointer to the 80-byte Bitcoin block header
     * @param[in]  previous_header_bytes  Pointer to the 80-byte previous block header
     * @return                           Returns true if the headers are properly connected,
     *                                   false otherwise
     */
    bool check_header_connection(
        const unsigned char *header_bytes,
        const unsigned char *previous_header_bytes);

#ifdef __cplusplus
}
#endif

#endif // BITCOIN_CORE_WRAPPER_H
