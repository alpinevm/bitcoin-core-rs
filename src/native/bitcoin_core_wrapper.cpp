#include "bitcoin_core_wrapper.h"
#include "vendor/bitcoin/src/crypto/sha256.h"
#include "vendor/bitcoin/src/pow.h"

extern "C" void sha256_hash(const unsigned char *input, unsigned int input_len, unsigned char output[32])
{
    CSHA256 sha256;
    sha256.Write(input, input_len);
    sha256.Finalize(output);
}

extern "C" bool check_pow(uint256 hash, unsigned int nBits, uint256 powLimit)
{
    Consensus::Params params;
    uint16_t sack = 1;
    params.powLimit = powLimit;
    bool result = CheckProofOfWork(hash, nBits, params);
    return result;
}
