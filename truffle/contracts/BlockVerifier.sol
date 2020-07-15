pragma solidity ^0.5.16;

import "./lib/Verifier.sol";
import "./lib/Claimable.sol";
import "./PlasmaData.sol";


contract BlockVerifier is Claimable
{
    struct Circuit
    {
        bool registered;
        uint[36] verificationKey;
    }

    mapping (uint => Circuit) public circuits;

    constructor() Claimable() public {}

    function registerCircuit(
        PlasmaData.BlockType blockType,
        uint[] calldata vk
        )
        external
        // FIXME как сделать, чтоб распознавал оунера при вызове через DEX.sol?
        // onlyOwner
    {
        // FIXME remove registration order
        Circuit storage circuit = circuits[uint(blockType)];
        require(circuit.registered == false, "ALREADY_REGISTERED");

        for (uint i = 0; i < vk.length; i++) {
            circuit.verificationKey[i] = vk[i];
        }
        circuit.registered = true;
    }

    function verifyProof(
        PlasmaData.BlockType blockType,
        uint[] calldata publicInputs,
        uint[] calldata proof
        )
        external
        view
        returns (bool)
    {
        Circuit storage circuit = circuits[uint(blockType)];
        require(circuit.registered == true, "NOT_REGISTERED");
        uint[36] storage vk = circuit.verificationKey;

        uint[14] memory vk_ = [
            vk[0], vk[1], vk[2], vk[3], vk[4], vk[5], vk[6],
            vk[7], vk[8], vk[9], vk[10], vk[11], vk[12], vk[13]
        ];

        uint vkGammaAbcLength;
        if (blockType == PlasmaData.BlockType.DEPOSIT) {
            vkGammaAbcLength = 10;
        } else if (blockType == PlasmaData.BlockType.TRANSFER) {
            vkGammaAbcLength = 6;
        } else if (blockType == PlasmaData.BlockType.OFFCHAIN_WITHDRAWAL) {
            vkGammaAbcLength = 18;
        } else if (blockType == PlasmaData.BlockType.ONCHAIN_WITHDRAWAL) {
            vkGammaAbcLength = 22;
        } else {
            revert("UNSUPPORTED_BLOCK_TYPE");
        }
        uint[] memory vkGammaAbc = new uint[](vkGammaAbcLength);
        for (uint i = 0; i < vkGammaAbc.length; i++) {
            vkGammaAbc[i] = vk[i+14];
        }

        return Verifier.Verify(vk_, vkGammaAbc, proof, publicInputs);
    }
}
