#include "bitcoin_core_wrapper.h"
#include "vendor/bitcoin/src/crypto/sha256.h"
#include "vendor/bitcoin/src/pow.h"
#include "vendor/bitcoin/src/streams.h"
#include "vendor/bitcoin/src/primitives/block.h"
#include "vendor/bitcoin/src/chain.h"
#include "vendor/bitcoin/src/kernel/chainparams.h"

#include <memory>
#include <vector>

static const size_t HEADER_LENGTH = 80;

// The config initializers in bitcoin core create global state,
// so we need to duplicate the relevant ones in this pure initializer to avoid that
static const Consensus::Params get_consensus_params()
{
    Consensus::Params consensus;
    consensus.signet_blocks = false;
    consensus.signet_challenge.clear();
    consensus.nSubsidyHalvingInterval = 210000;
    consensus.powLimit = uint256::FromHex("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff").value();
    consensus.nPowTargetTimespan = 14 * 24 * 60 * 60; // two weeks
    consensus.nPowTargetSpacing = 10 * 60;
    consensus.fPowAllowMinDifficultyBlocks = false;
    consensus.enforce_BIP94 = false;
    consensus.fPowNoRetargeting = false;
    return consensus;
}

static bool deserialize_header(const unsigned char *header_bytes, CBlockHeader &header)
{
    std::vector<unsigned char> serialized_header(header_bytes, header_bytes + HEADER_LENGTH);
    DataStream ser_header{serialized_header};
    try
    {
        ser_header >> header;
        return true;
    }
    catch (const std::exception &)
    {
        return false;
    }
}

// Simple fork of GetNextWorkRequired that doesn't require an index, so we don't have to build a skip list
static uint32_t GetNextWorkRequiredNoIndex(const CBlockIndex *pindexLast, const CBlockHeader *pblock, const CBlockIndex *pretarget, const Consensus::Params &params)
{
    assert(pindexLast != nullptr);
    assert(pretarget != nullptr);
    unsigned int nProofOfWorkLimit = UintToArith256(params.powLimit).GetCompact();

    // Only change once per difficulty adjustment interval
    if ((pindexLast->nHeight + 1) % params.DifficultyAdjustmentInterval() != 0)
    {
        if (params.fPowAllowMinDifficultyBlocks)
        {
            // Special difficulty rule for testnet:
            // If the new block's timestamp is more than 2* 10 minutes
            // then allow mining of a min-difficulty block.
            if (pblock->GetBlockTime() > pindexLast->GetBlockTime() + params.nPowTargetSpacing * 2)
                return nProofOfWorkLimit;
            else
            {
                // Return the last non-special-min-difficulty-rules-block
                const CBlockIndex *pindex = pindexLast;
                while (pindex->pprev && pindex->nHeight % params.DifficultyAdjustmentInterval() != 0 && pindex->nBits == nProofOfWorkLimit)
                    pindex = pindex->pprev;
                return pindex->nBits;
            }
        }
        return pindexLast->nBits;
    }

    // Go back by what we want to be 14 days worth of blocks
    int nHeightFirst = pindexLast->nHeight - (params.DifficultyAdjustmentInterval() - 1);
    assert(nHeightFirst >= 0);
    assert(pretarget->nHeight == nHeightFirst);

    return CalculateNextWorkRequired(pindexLast, pretarget->GetBlockTime(), params);
}

extern "C" void sha256_hash(const unsigned char *input, const uint32_t input_len, unsigned char hash_result[32])
{
    CSHA256 sha256;
    sha256.Write(input, input_len);
    sha256.Finalize(hash_result);
}

extern "C" bool get_header_hash(const unsigned char *header_bytes, unsigned char *block_hash)
{
    CBlockHeader header;
    if (!deserialize_header(header_bytes, header))
    {
        return false;
    }
    uint256 hash = header.GetHash();
    std::memcpy(block_hash, hash.data(), 32);
    return true;
}

extern "C" bool check_proof_of_work(const unsigned char *header_bytes)
{
    const Consensus::Params &params = get_consensus_params();
    CBlockHeader header;
    if (!deserialize_header(header_bytes, header))
    {
        return false;
    }
    return CheckProofOfWork(header.GetHash(), header.nBits, params);
}

extern "C" uint32_t get_retarget_height(const uint32_t height)
{
    const Consensus::Params &params = get_consensus_params();
    if (height < params.DifficultyAdjustmentInterval())
    {
        return 0;
    }
    return ((height - 1) / params.DifficultyAdjustmentInterval()) * params.DifficultyAdjustmentInterval();
}

extern "C" bool get_block_proof(const unsigned char *header_bytes, unsigned char *proof)
{
    CBlockHeader header;
    if (!deserialize_header(header_bytes, header))
    {
        return false;
    }
    CBlockIndex header_index(header);
    std::memcpy(proof, ArithToUint256(GetBlockProof(header_index)).data(), 32);
    return true;
}

extern "C" bool validate_next_work_required(
    const unsigned char *last_retarget_header_bytes,
    const uint32_t previous_height,
    const unsigned char *previous_header_bytes,
    const unsigned char *header_bytes,
    unsigned char *next_retarget_header_bytes)
{

    const Consensus::Params &params = get_consensus_params();
    CBlockHeader last_retarget_header;
    CBlockHeader previous_header;
    CBlockHeader header;

    if (!deserialize_header(last_retarget_header_bytes, last_retarget_header) ||
        !deserialize_header(previous_header_bytes, previous_header) ||
        !deserialize_header(header_bytes, header))
    {
        return false;
    }

    // Create previous block index and set its height
    CBlockIndex previous_index(previous_header);
    previous_index.nHeight = previous_height;

    // Create the last retarget block index and set its height
    CBlockIndex last_retarget_index(last_retarget_header);
    last_retarget_index.nHeight = get_retarget_height(previous_height + 1);

    // Calculate the next work required for the header
    uint32_t calculated_next_nbits = GetNextWorkRequiredNoIndex(&previous_index, &header, &last_retarget_index, params);

    if (header.nBits != calculated_next_nbits)
    {
        return false;
    }

    // Only update the next retarget header if we're at a retarget boundary
    // bitcoin/src/pow.cpp:19
    if ((previous_height + 1) % params.DifficultyAdjustmentInterval() == 0)
    {
        std::memcpy(next_retarget_header_bytes, header_bytes, HEADER_LENGTH);
    }
    else
    {
        std::memcpy(next_retarget_header_bytes, last_retarget_header_bytes, HEADER_LENGTH);
    }

    return true;
}

extern "C" bool check_header_connection(
    const unsigned char *header_bytes,
    const unsigned char *previous_header_bytes)
{
    CBlockHeader header;
    CBlockHeader previous_header;
    if (!deserialize_header(header_bytes, header) ||
        !deserialize_header(previous_header_bytes, previous_header))
    {
        return false;
    }
    return header.hashPrevBlock == previous_header.GetHash();
}
