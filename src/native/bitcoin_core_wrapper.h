#ifndef BITCOIN_CORE_WRAPPER_H
#define BITCOIN_CORE_WRAPPER_H

#ifdef __cplusplus
extern "C"
{
#endif

    // Perform SHA256 hash on input data
    // input: pointer to input data
    // input_len: length of input data in bytes
    // output: pointer to output buffer (must be at least 32 bytes)
    void sha256_hash(const unsigned char *input, unsigned int input_len, unsigned char output[32]);

    // Check if the proof of work for a given block header is valid
    // hex_header: block header in hex format
    // returns: true if proof of work is valid, false otherwise
    bool check_pow(const char *hex_header);

#ifdef __cplusplus
}
#endif

#endif // BITCOIN_CORE_WRAPPER_H
