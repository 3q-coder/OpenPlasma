pragma solidity ^0.5.16;
pragma experimental ABIEncoderV2;


import "./PlasmaData.sol";
import "./BlockVerifier.sol";
import "./lib/MathUint.sol";
import "./lib/BytesUtil.sol";

contract DEX is BlockVerifier
{
    PlasmaData.State private state;

    using BytesUtil         for bytes;
    using MathUint          for uint;

    // No aruments in contructor for easier deploy
    constructor() public {}

    function initialize(
        // uint genesisBlockHash
        )
        external
        onlyOwner
    {
        // require(genesisBlockHash != 0, "ZERO_GENESIS_BLOCK_HASH");
        uint genesisBlockHash = 0x2bac0e85c5856a4045bec41f225b07b24f6130f6a6d242f49d6afe9feb0b9922;

        // create genesis block
        PlasmaData.Block memory genesisBlock = PlasmaData.Block(
            PlasmaData.BlockState.VERIFIED,
            PlasmaData.BlockType.TRANSFER,
            0,
            PlasmaData.BlockData(
                0x0,
                genesisBlockHash,
                0,
                0,
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0
                // withdrawals
            )
        );
        state.blocks.push(genesisBlock);
        state.numBlocksFinalized = 1;

        // initialize withdrawal and deposit chains
        PlasmaData.Request memory genesisRequest = PlasmaData.Request(
            0,
            0xFFFFFFFF
        );
        state.depositChain.push(genesisRequest);
        state.withdrawalChain.push(genesisRequest);
    }

    // Users' functions ---------------------------------------------

    function getAccountId(address account)
        public
        view
        returns (uint)
    {
        return state.ownerToAccountId[account];
    }

    function createAccount(
        uint  pubKeyX,
        uint  pubKeyY
        )
        external
        returns (uint accountId)
    {
        require(state.accounts.length < PlasmaData.MAX_NUM_ACCOUNTS(), "MAX_ACCOUNTS_REACHED");
        require(state.ownerToAccountId[msg.sender] == 0, "ACCOUNT_EXISTS");
        // We allow invalid public keys to be set for accounts to
        // disable offchain request signing.
        // Make sure we can detect accounts that were not yet created in the circuits
        // by forcing the pubKeyX to be non-zero.
        require(pubKeyX > 0, "INVALID_PUBKEY");
        // Make sure the public key can be stored in the SNARK field
        require(pubKeyX < PlasmaData.SNARK_SCALAR_FIELD(), "INVALID_PUBKEY");
        require(pubKeyY < PlasmaData.SNARK_SCALAR_FIELD(), "INVALID_PUBKEY");

        PlasmaData.Account memory account = PlasmaData.Account(
            msg.sender,
            pubKeyX,
            pubKeyY
        );

        state.accounts.push(account);
        accountId = uint(state.accounts.length);
        state.ownerToAccountId[msg.sender] = accountId;
        return accountId;
    }

    function deposit(
        // FIXME временная мера, пока не сделаем хеш
        uint    hash
        )
        external
        payable
    {
        depositTo(msg.sender, hash);
    }

    function depositTo(
        address recipient,
        // FIXME временная мера, пока не сделаем хеш
        uint    hash
        )
        public
        payable
    {
        require(msg.value > 0, "INVALID_AMOUNT");
        require(recipient != address(0), "ZERO_ADDRESS");
        uint accountId = state.ownerToAccountId[recipient];
        require(accountId > 0, "ACCOUNT_NOT_REGISTERED");

        // PlasmaData.Account storage account = state.accounts[accountId-1];

        // Add the request to the deposit chain
        // PlasmaData.Request storage prevRequest = state.depositChain[state.depositChain.length - 1];
        PlasmaData.Request memory request = PlasmaData.Request(
            // FIXME пока не вычисляем хеш, а передаём его снаружи
            // sha256(
            //     abi.encodePacked(
            //         prevRequest.accumulatedHash,
            //         account.pubKeyX,  // Include the pubKey to allow using the same circuit for
            //                           // account creation, account updating and depositing.
            //                           // In the circuit we always overwrite the public keys in
            //                           // the Account leaf with the data given onchain.
            //         account.pubKeyY,
            //         accountId,
            //         tokenId,
            //         amount
            //     )
            // ),
            hash,
            uint(now)
        );
        state.depositChain.push(request);
    }

    function withdraw(
        // FIXME временная мера, пока не сделаем хеш
        uint    hash
        )
        external
    {
        uint accountId = state.ownerToAccountId[msg.sender];
        require(accountId > 0, "ACCOUNT_NOT_REGISTERED");

        // Add the withdraw to the withdraw chain
        // PlasmaData.Request storage prevRequest = state.withdrawalChain[state.withdrawalChain.length - 1];
        PlasmaData.Request memory request = PlasmaData.Request(
            hash,
            uint(block.timestamp)
        );
        state.withdrawalChain.push(request);
    }

    function withdrawFromApprovedWithdrawal(
        uint blockId,
        uint slotIdx,
        bool allowFailure
        )
        public
        returns (bool success)
    {
        // Only allow withdrawing on finalized blocks
        require(blockId < state.numBlocksFinalized, "BLOCK_NOT_FINALIZED");

        PlasmaData.Block storage withdrawBlock = state.blocks[blockId];

        require(slotIdx < PlasmaData.BLOCK_SIZE(), "INVALID_SLOT_IDX");
        require(slotIdx < withdrawBlock.blockData.withdrawalsLength, "INVALID_SLOT_IDX_");

        PlasmaData.Withdrawal memory withdrawal = withdrawBlock.blockData.withdrawals[slotIdx];
        // Extract the withdrawal data
        uint accountId = withdrawal.accountId;
        uint amount = withdrawal.amount;

        // Transfer the tokens
        success = sendEther(
            accountId,
            amount,
            allowFailure
        );

        if (success && amount > 0) {
            // Set everything to 0 for this withdrawal so it cannot be used anymore
            withdrawal.accountId = 0;
            withdrawal.amount = 0;

            withdrawBlock.blockData.withdrawals[slotIdx] = withdrawal;
        }
    }

    // Operator functions -------------------------------------------

    function distributeWithdrawals(
        uint blockId,
        uint maxNumWithdrawals
        )
        external
        onlyOwner
    {
        require(blockId < state.blocks.length, "INVALID_BLOCK_IDX");
        require(maxNumWithdrawals > 0, "INVALID_MAX_NUM_WITHDRAWALS");
        PlasmaData.Block storage withdrawBlock = state.blocks[blockId];

        // Check if this is a withdrawal block
        require(
            withdrawBlock.blockType == PlasmaData.BlockType.ONCHAIN_WITHDRAWAL ||
            withdrawBlock.blockType == PlasmaData.BlockType.OFFCHAIN_WITHDRAWAL,
            "INVALID_BLOCK_TYPE"
        );

        // Only allow withdrawing on finalized blocks
        require(blockId < state.numBlocksFinalized, "BLOCK_NOT_FINALIZED");
        // Check if the withdrawals were already completely distributed
        require(withdrawBlock.numWithdrawalsDistributed < PlasmaData.BLOCK_SIZE(), "WITHDRAWALS_ALREADY_DISTRIBUTED");

        // Calculate the range of withdrawals we'll do
        uint start = withdrawBlock.numWithdrawalsDistributed;
        uint end = start.add(maxNumWithdrawals);
        if (end > PlasmaData.BLOCK_SIZE()) {
            end = PlasmaData.BLOCK_SIZE();
        }
        if (end > withdrawBlock.blockData.withdrawalsLength) {
            end = withdrawBlock.blockData.withdrawalsLength;
        }

        // Do the withdrawals
        uint gasLimit = PlasmaData.MIN_GAS_TO_DISTRIBUTE_WITHDRAWALS();
        uint totalNumWithdrawn = start;
        while (totalNumWithdrawn < end && gasleft() >= gasLimit) {
            // Don't check the return value here, the withdrawal is allowed to fail.
            // The automatic token disribution by the operator is a best effort only.
            // The account owner can always manually withdraw without any limits.
            withdrawFromApprovedWithdrawal(
                blockId,
                totalNumWithdrawn,
                true
            );
            totalNumWithdrawn++;
        }
        withdrawBlock.numWithdrawalsDistributed = uint(totalNumWithdrawn);
    }

    function getBlockMerkleRoot(uint blockId)
        public
        view
        returns (uint)
    {
        require(blockId >= 0 && blockId < state.blocks.length, "INVALID_BLOCK_ID");
        PlasmaData.Block storage specifiedBlock = state.blocks[blockId];
        return specifiedBlock.blockData.merkleRootAfter;
    }

    function getBlockWithdrawal(uint blockId, uint withdrawalId)
        public
        view
        returns (PlasmaData.Withdrawal memory withdrawal)
    {
        require(blockId >= 0 && blockId < state.blocks.length, "INVALID_BLOCK_ID");
        PlasmaData.Block storage specifiedBlock = state.blocks[blockId];
        require(withdrawalId >= 0 && withdrawalId < specifiedBlock.blockData.withdrawalsLength, "INVALID_WITHDRAWAL_ID");
        withdrawal = specifiedBlock.blockData.withdrawals[withdrawalId];
        return withdrawal;
    }

    function getDepositHash(uint index)
        public
        view
        returns (uint)
    {
        require(index >= 0 && index < state.depositChain.length, "INVALID_DEPOSIT_ID");
        return state.depositChain[index].accumulatedHash;
    }

    function getWithdrawalHash(uint index)
        public
        view
        returns (uint)
    {
        require(index >= 0 && index < state.withdrawalChain.length, "INVALID_WITHDRAWAL_ID");
        return state.withdrawalChain[index].accumulatedHash;
    }

    function commitBlock(
        PlasmaData.BlockType blockType,
        uint merkleRootBefore,
        uint merkleRootAfter,
        uint startIdx,
        uint count,
        uint inputStartingHash,
        uint inputEndingHash,
        PlasmaData.Withdrawal[] memory inputWithdrawals
        )
        public
        onlyOwner
    {
        // Get the current block
        PlasmaData.Block storage prevBlock = state.blocks[state.blocks.length - 1];

        require(merkleRootBefore == prevBlock.blockData.merkleRootAfter, "INVALID_MERKLE_ROOT");
        require(merkleRootAfter < PlasmaData.SNARK_SCALAR_FIELD(), "INVALID_MERKLE_ROOT");

        if (blockType == PlasmaData.BlockType.DEPOSIT) {
            require (startIdx == state.numDepositRequestsCommitted, "INVALID_REQUEST_RANGE");
            require (count <= PlasmaData.BLOCK_SIZE(), "INVALID_REQUEST_RANGE");
            require (startIdx + count <= state.depositChain.length, "INVALID_REQUEST_RANGE");

            uint startingHash = state.depositChain[startIdx].accumulatedHash;
            uint endingHash = state.depositChain[startIdx + count].accumulatedHash;

            require(inputStartingHash == startingHash, "INVALID_STARTING_HASH");
            require(inputEndingHash == endingHash, "INVALID_ENDING_HASH");

            state.numDepositRequestsCommitted += uint(count);
        } else if (blockType == PlasmaData.BlockType.ONCHAIN_WITHDRAWAL) {
            require (startIdx == state.numWithdrawalRequestsCommitted, "INVALID_REQUEST_RANGE");
            require (count <= PlasmaData.BLOCK_SIZE(), "INVALID_REQUEST_RANGE");
            require (startIdx + count <= state.withdrawalChain.length, "INVALID_REQUEST_RANGE");

            require (count > 0, "INVALID_WITHDRAWAL_COUNT");
            uint startingHash = state.withdrawalChain[startIdx].accumulatedHash;
            uint endingHash = state.withdrawalChain[startIdx + count].accumulatedHash;

            require(inputStartingHash == startingHash, "INVALID_STARTING_HASH");
            require(inputEndingHash == endingHash, "INVALID_ENDING_HASH");
            state.numWithdrawalRequestsCommitted += uint(count);
        } else if (
            blockType != PlasmaData.BlockType.OFFCHAIN_WITHDRAWAL &&
            blockType != PlasmaData.BlockType.TRANSFER) {
            revert("UNSUPPORTED_BLOCK_TYPE");
        }

        // Create a new block with the updated merkle roots
        PlasmaData.Block memory newBlock = PlasmaData.Block(
            PlasmaData.BlockState.COMMITTED,
            blockType,
            0,
            PlasmaData.BlockData(
                merkleRootBefore,
                merkleRootAfter,
                startIdx,
                count,
                inputStartingHash,
                inputEndingHash,
                inputWithdrawals.length
                // withdrawals
            )
        );

        state.blocks.push(newBlock);

        PlasmaData.Block storage lastBlock = state.blocks[state.blocks.length - 1];

        // Store the approved withdrawal data onchain
        if (blockType == PlasmaData.BlockType.ONCHAIN_WITHDRAWAL ||
            blockType == PlasmaData.BlockType.OFFCHAIN_WITHDRAWAL) {
            for (uint i = 0; i < inputWithdrawals.length; i++) {
                lastBlock.blockData.withdrawals[i] = inputWithdrawals[i];
            }
        }
    }

    function verifyBlock(
        uint blockId,
        // FIXME понять, зачем calldata
        // uint[] calldata publicInputs,
        uint[] calldata proof
        )
        external
        onlyOwner
    {
        // Check input data
        require(blockId > 0, "INVALID_ID");
        require(blockId < state.blocks.length, "INVALID_BLOCK_ID");
        require(proof.length == 8, "INVALID_PROOF_ARRAY");

        PlasmaData.Block storage specifiedBlock = state.blocks[blockId];
        require(
            specifiedBlock.state == PlasmaData.BlockState.COMMITTED,
            "BLOCK_VERIFIED_ALREADY"
        );

        PlasmaData.BlockType blockType = specifiedBlock.blockType;

        if (blockType == PlasmaData.BlockType.DEPOSIT) {
            uint[] memory publicInputs = new uint[](4);
            publicInputs[0] = specifiedBlock.blockData.inputStartingHash;
            publicInputs[1] = specifiedBlock.blockData.inputEndingHash;
            publicInputs[2] = specifiedBlock.blockData.merkleRootBefore;
            publicInputs[3] = specifiedBlock.blockData.merkleRootAfter;
            require(
                this.verifyProof(
                    blockType,
                    publicInputs,
                    proof
                ),
                "INVALID_PROOF"
            );
        } else if (blockType == PlasmaData.BlockType.TRANSFER) {
            uint[] memory publicInputs = new uint[](2);
            publicInputs[0] = specifiedBlock.blockData.merkleRootBefore;
            publicInputs[1] = specifiedBlock.blockData.merkleRootAfter;
            require(
                this.verifyProof(
                    blockType,
                    publicInputs,
                    proof
                ),
                "INVALID_PROOF"
            );
        } else if (blockType == PlasmaData.BlockType.OFFCHAIN_WITHDRAWAL) {
            uint publicInputsLength = 2 + 3*specifiedBlock.blockData.withdrawalsLength;
            uint[] memory publicInputs = new uint[](publicInputsLength);
            publicInputs[0] = specifiedBlock.blockData.merkleRootBefore;
            publicInputs[1] = specifiedBlock.blockData.merkleRootAfter;
            for (uint i = 0; i < specifiedBlock.blockData.withdrawalsLength; i++) {
                // we substract 1 since the SC accountId starts from 1, operator accountId starts from 0
                publicInputs[2+2*i] = specifiedBlock.blockData.withdrawals[i].accountId - 1;
                publicInputs[2+2*i+1] = specifiedBlock.blockData.withdrawals[i].amount;
            }
            require(
                this.verifyProof(
                    blockType,
                    publicInputs,
                    proof
                ),
                "INVALID_PROOF"
            );
        } else if (blockType == PlasmaData.BlockType.ONCHAIN_WITHDRAWAL) {
            uint publicInputsLength = 4 + 3*specifiedBlock.blockData.withdrawalsLength;
            uint[] memory publicInputs = new uint[](publicInputsLength);
            publicInputs[0] = state.withdrawalChain[specifiedBlock.blockData.startIdx].accumulatedHash;
            publicInputs[1] = state.withdrawalChain[specifiedBlock.blockData.startIdx + specifiedBlock.blockData.count].accumulatedHash;
            publicInputs[2] = specifiedBlock.blockData.merkleRootBefore;
            publicInputs[3] = specifiedBlock.blockData.merkleRootAfter;
            for (uint i = 0; i < specifiedBlock.blockData.withdrawalsLength; i++) {
                // we substract 1 since the SC accountId starts from 1, operator accountId starts from 0
                publicInputs[4+2*i] = specifiedBlock.blockData.withdrawals[i].accountId - 1;
                publicInputs[4+2*i+1] = specifiedBlock.blockData.withdrawals[i].amount;
            }
            require(
                this.verifyProof(
                    blockType,
                    publicInputs,
                    proof
                ),
                "INVALID_PROOF"
            );
        } else {
            revert("UNSUPPORTED_BLOCK_TYPE");
        }

        specifiedBlock.state = PlasmaData.BlockState.VERIFIED;
        state.numBlocksFinalized += 1;
    }

    // Internal functions -------------------------------------------

    function sendEther(
        uint accountId,
        uint amount,
        bool allowFailure
        )
        private
        returns (bool success)
    {
        address payable to = address(uint160(state.accounts[accountId-1].owner));

        uint gasLimit = allowFailure ? PlasmaData.GAS_LIMIT_SEND_ETHER() : gasleft();

        if (amount > 0) {
            (success, ) = to.call.value(amount).gas(gasLimit)("");
        } else {
            success = true;
        }

        if (!allowFailure) {
            require(success, "TRANSFER_FAILURE");
        }
    }
}
