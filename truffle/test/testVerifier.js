import { BlockType } from "./helpers/types";
import expectThrow from "./helpers/expectThrow";
import Artifacts from "./helpers/artifacts";
import * as depositData from "./helpers/depositData";
import * as transferData from "./helpers/transferData";
import * as offWithdrData from "./helpers/offWithdrData";
import * as onWithdrData from "./helpers/onWithdrData";

contract("Plasma", (accounts) => {
  const contracts = new Artifacts(artifacts);
  let Plasma;

  before(async () => {
    [Plasma] = await Promise.all([contracts.Plasma.deployed()]);
  });

  describe("Testing verifier", function () {
    this.timeout(0);
    describe("Exchange owner", () => {
      it("should be able to initialize the exchange", async () => {
        await Plasma.initialize();
      });
      it("should be able to register a deposit circuit", async () => {
        await Plasma.registerCircuit(BlockType.DEPOSIT, depositData.vkDeposit);
      });
      it("should be able to verify a deposit proof", async () => {
        let result = await Plasma.verifyProof(
            BlockType.DEPOSIT,
            depositData.publicInputsDeposit,
            depositData.proofDeposit
          );
        assert.equal(result, true);
      });
      it("should be able to register a transfer circuit", async () => {
        await Plasma.registerCircuit(BlockType.TRANSFER, transferData.vk);
      });
      it("should be able to verify a transfer proof", async () => {
        let result = await Plasma.verifyProof(
            BlockType.TRANSFER,
            transferData.publicInputs,
            transferData.proof
          );
        assert.equal(result, true);
      });
      it("should be able to register an offchain withdrawal circuit", async () => {
        await Plasma.registerCircuit(BlockType.OFFCHAIN_WITHDRAWAL, offWithdrData.vk);
      });
      // it("should be able to verify an offchain withdrawal proof", async () => {
      //   let result = await Plasma.verifyProof(
      //       BlockType.OFFCHAIN_WITHDRAWAL,
      //       offWithdrData.publicInputsOffWithdr,
      //       offWithdrData.proofOffWithdr
      //     );
      //   assert.equal(result, true);
      // });
      it("should be able to verify an offchain withdrawal proof", async () => {
        let merkleRootOld = offWithdrData.publicInputs[0];
        let merkleRootNew = offWithdrData.publicInputs[1];
        let result = await Plasma.verifyProof(
            BlockType.OFFCHAIN_WITHDRAWAL,
            offWithdrData.publicInputs,
            offWithdrData.proof
          );
        assert.equal(result, true);
      });
      it("should be able to register an onchain withdrawal circuit", async () => {
        await Plasma.registerCircuit(BlockType.ONCHAIN_WITHDRAWAL, onWithdrData.vk);
      });
      it("should be able to verify an onchain withdrawal proof", async () => {
        let result = await Plasma.verifyProof(
            BlockType.ONCHAIN_WITHDRAWAL,
            onWithdrData.publicInputs,
            onWithdrData.proof
          );
        assert.equal(result, true);
      });
    });
  });
});
