#include "bitcoin_core_wrapper.h"
#include "vendor/bitcoin/src/crypto/sha256.h"
#include "vendor/bitcoin/src/pow.h"
#include "vendor/bitcoin/src/streams.h"
#include "vendor/bitcoin/src/primitives/block.h"

extern "C" void sha256_hash(const unsigned char *input, unsigned int input_len, unsigned char output[32])
{
    CSHA256 sha256;
    sha256.Write(input, input_len);
    sha256.Finalize(output);
}

extern "C" bool check_header_pow(const unsigned char *header_bytes, uint256 pow_limit)
{
    std::vector<unsigned char> serialized_header(header_bytes, header_bytes + 80);
    CBlockHeader header;
    DataStream ser_header{serialized_header};
    try
    {
        ser_header >> header;
    }
    catch (const std::exception &)
    {
        return false;
    }

    Consensus::Params params;
    params.powLimit = pow_limit;
    return CheckProofOfWork(header.GetHash(), header.nBits, params);
}
