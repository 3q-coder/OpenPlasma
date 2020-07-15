pragma solidity ^0.5.16;
pragma experimental ABIEncoderV2;


library PlasmaData {
    struct Account {
        address owner;
        uint    pubKeyX;
        uint    pubKeyY;
    }

    struct Request {
        uint accumulatedHash;
        uint timestamp;
    }

    struct Withdrawal {
        uint accountId;
        uint amount;
    }

    enum BlockType {
        TRANSFER,
        DEPOSIT,
        ONCHAIN_WITHDRAWAL,
        OFFCHAIN_WITHDRAWAL
    }

    enum BlockState {
        COMMITTED,     // = 0
        VERIFIED       // = 1
    }

    struct BlockData {
        uint merkleRootBefore;
        uint merkleRootAfter;

        uint startIdx;
        uint count;

        uint inputStartingHash;
        uint inputEndingHash;

        uint withdrawalsLength;
        mapping(uint => Withdrawal) withdrawals;
    }

    struct Block {
        BlockState state;
        BlockType blockType;
        uint numWithdrawalsDistributed;
        BlockData blockData;
    }

    struct State {
        Block[]     blocks;
        Account[]   accounts;
        Request[]   depositChain;
        Request[]   withdrawalChain;
        uint numDepositRequestsCommitted;
        uint numWithdrawalRequestsCommitted;

        mapping (address => uint) ownerToAccountId;
        mapping (address => uint) tokenToTokenId;

        mapping (address => uint) tokenBalances;

        uint numBlocksFinalized;
    }

    function blockDataToBytes(PlasmaData.BlockData memory data)
        internal
        pure
        returns (bytes memory)
    {
        bytes memory dataBytes = abi.encode(
            data.merkleRootBefore,
            data.merkleRootAfter,
            data.startIdx,
            data.count,
            data.inputStartingHash,
            data.inputEndingHash);
        return dataBytes;
    }

    // Constants getters --------------------------------------------

    function BLOCK_SIZE() internal pure returns (uint) { return 8; }
    function SNARK_SCALAR_FIELD() internal pure returns (uint) {
        // This is the prime number that is used for the alt_bn128 elliptic curve, see EIP-196.
        return 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    }
    function MAX_NUM_ACCOUNTS() internal pure returns (uint) { return 2 ** 20 - 1; }
    function MIN_GAS_TO_DISTRIBUTE_WITHDRAWALS() internal pure returns (uint) { return 150000; }
    function GAS_LIMIT_SEND_ETHER() internal pure returns (uint) { return 30000; }
}
