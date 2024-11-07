#include "bitcoin_core_wrapper.h"
#include <cstring>
#include "vendor/bitcoin/src/crypto/sha256.h"
#include "vendor/bitcoin/src/primitives/block.h"
#include "vendor/bitcoin/src/consensus/params.h"
#include "vendor/bitcoin/src/chainparams.h"
#include "vendor/bitcoin/src/pow.h"
#include "vendor/bitcoin/src/core_io.h"
#include <vector>

extern "C" void sha256_hash(const unsigned char *input, unsigned int input_len, unsigned char output[32])
{
    CSHA256 sha256;
    sha256.Write(input, input_len);
    sha256.Finalize(output);
}

extern "C" bool check_pow(const char *hex_header)
{
    if (hex_header == nullptr)
    {
        return false; // Invalid input
    }

    // Deserialize into a CBlockHeader
    CBlockHeader header;
    if (!DecodeHexBlockHeader(header, std::string(hex_header)))
    {
        return false;
    }

    // Perform proof-of-work check using `CheckProofOfWork`
    const Consensus::Params &consensusParams = Params().GetConsensus();
    return CheckProofOfWork(header.GetHash(), header.nBits, consensusParams);
}
