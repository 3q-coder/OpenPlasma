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
  const user0 = accounts[1];
  const user1 = accounts[2];

  before(async () => {
    [Plasma] = await Promise.all([
      contracts.Plasma.deployed()
    ]);
  });

  describe("Testing Plasma", function () {
    this.timeout(0);
    describe("Plasma operator", () => {
      it("should be able to initialize the exchange", async () => {
        await Plasma.initialize();
      });
      it("should be able to register a deposit circuit", async () => {
        await Plasma.registerCircuit(BlockType.DEPOSIT, depositData.vkDeposit);
      });
      it("should be able to register a transfer circuit", async () => {
        await Plasma.registerCircuit(BlockType.TRANSFER, transferData.vk);
      });
      it("should be able to register an offchain withdrawal circuit", async () => {
        await Plasma.registerCircuit(BlockType.OFFCHAIN_WITHDRAWAL, offWithdrData.vk);
      });
      it("should be able to register an onchain withdrawal circuit", async () => {
        await Plasma.registerCircuit(BlockType.ONCHAIN_WITHDRAWAL, onWithdrData.vk);
      });
      it("should be able to create user0", async () => {
        await Plasma.createAccount(
          depositData.deposits[0].pubkey[0],
          depositData.deposits[0].pubkey[1],
          { from: user0 }
        );
      });
      it("should be able to create user1", async () => {
        await Plasma.createAccount(
          depositData.deposits[1].pubkey[0],
          depositData.deposits[1].pubkey[1],
          { from: user1 }
        );
      });
      it("should be able to deposit ether for user0", async () => {
        await Plasma.deposit(
          depositData.fakeDepositHashes[1],
          { 
            from: user0,
            value: 100,
          }
        );
      });
      it("should be able to deposit tokens for user1", async () => {
        await Plasma.deposit(
          depositData.fakeDepositHashes[2],
          { 
            from: user1,
            value: 100,
          }
        );
      });
      it("should be able to commit a deposit block", async () => {
        let merkleRootOld = depositData.publicInputsDeposit[2];
        let merkleRootNew = depositData.publicInputsDeposit[3];
        let startHash = await Plasma.getDepositHash(0);
        let endHash = await Plasma.getDepositHash(2);
        await Plasma.commitBlock(
          BlockType.DEPOSIT,
          merkleRootOld,
          merkleRootNew,
          0,
          2,
          startHash,
          endHash,
          []
        );
      });
      it("should be able to verify a deposit block", async () => {
        await Plasma.verifyBlock(
          1,
          depositData.proofDeposit
        );
      });
      it("should be able to commit a transfer block", async () => {
        let merkleRootOld = transferData.publicInputs[0];
        let merkleRootNew = transferData.publicInputs[1];
        await Plasma.commitBlock(
          BlockType.TRANSFER,
          merkleRootOld,
          merkleRootNew,
          0,
          0,
          0,
          0,
          []
        );
      });
      it("should be able to verify a transfer block", async () => {
        await Plasma.verifyBlock(
          2,
          transferData.proof
        );
      });
      it("should be able to commit an offchain withdrawal block", async () => {
        let merkleRootOld = offWithdrData.publicInputs[0];
        let merkleRootNew = offWithdrData.publicInputs[1];
        await Plasma.commitBlock(
          BlockType.OFFCHAIN_WITHDRAWAL,
          merkleRootOld,
          merkleRootNew,
          0,
          0,
          0,
          0,
          [
            offWithdrData.offchainWithdrawals[0]
          ]
        );
      });
      it("should be able to verify an offchain withdrawal block", async () => {
        await Plasma.verifyBlock(
          3,
          offWithdrData.proof
        );
      });
      it("should be able to create onchain withdrawal requests", async () => {
        await Plasma.withdraw(
          onWithdrData.fakeOnWithdrHashes[1],
          { from: user0 }
        );
        await Plasma.withdraw(
          onWithdrData.fakeOnWithdrHashes[2],
          { from: user0 }
        );
      });
      it("should be able to commit an onchain withdrawal block", async () => {
        let merkleRootOld = onWithdrData.publicInputs[2];
        let merkleRootNew = onWithdrData.publicInputs[3];
        let startHash = await Plasma.getWithdrawalHash(0);
        let endHash = await Plasma.getWithdrawalHash(2);
        // console.log("Public inputs onchain withdrawal SC: ", startHash, endHash, merkleRootOld, merkleRootNew);
        // console.log("Public inputs onchain withdrawal operator: ", onWithdrData.publicInputsOnWithdr);
        await Plasma.commitBlock(
          BlockType.ONCHAIN_WITHDRAWAL,
          merkleRootOld,
          merkleRootNew,
          0,
          2,
          startHash,
          endHash,
          [
            onWithdrData.onchainWithdrawals[0],
            onWithdrData.onchainWithdrawals[1]
          ]
        );
      });
      it("should be able to verify an onchain withdrawal block", async () => {
        // let withdrawal1 = await Plasma.getBlockWithdrawal(3, 0);
        // let withdrawal2 = await Plasma.getBlockWithdrawal(3, 1);
        // console.log("Withdrawals: ", withdrawal1, withdrawal2);
        // console.log("Public inputs: ", onWithdrData.publicInputsOffWithdr);
        await Plasma.verifyBlock(
          4,
          onWithdrData.proof
        );
      });
      it("should be able to distribute withdrawals", async () => {
        await Plasma.distributeWithdrawals(3, 2);
      });
      it("should be able to withdraw from approved withdrawal", async () => {
        await Plasma.withdrawFromApprovedWithdrawal(4, 0, 1);
        await Plasma.withdrawFromApprovedWithdrawal(4, 1, 0);
      });
    });
  });
});