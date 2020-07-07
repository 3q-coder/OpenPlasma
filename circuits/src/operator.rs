use std::fmt;
use std::io;
use std::error::Error;

#[allow(unused_imports)]
use sapling_crypto_ce::{
    poseidon::{
        bn256::Bn256PoseidonParams,
        poseidon_hash,
    },
    alt_babyjubjub::AltJubjubBn256,
    circuit::test::TestConstraintSystem,
};

#[allow(unused_imports)]
use bellman_ce::{
    SynthesisError,
    groth16::{
        Proof,
        Parameters,
        create_random_proof,
    },
    Circuit,
};

use pairing_ce::{
    bn256,
    bn256::Bn256,
};

use ff_ce::Field;
use rand::thread_rng;

use super::{
    data_structs::transfer::Transfer,
    data_structs::deposit::Deposit,
    data_structs::onchain_withdrawal::OnchainWithdrawal,
    data_structs::offchain_withdrawal::OffchainWithdrawal,
    tree::account::AccountsTree,
};

use crate::utils::utils::{
    usize_to_fr,
    fr_to_usize,
};

use crate::{
    deposit_circuit::{ DepositCircuit, DepositBatchCircuit },
    onchain_withdrawal_circuit:: { OnchainWithdrawalCircuit, OnchainWithdrawalBatchCircuit },
    offchain_withdrawal_circuit:: { OffchainWithdrawalCircuit, OffchainWithdrawalBatchCircuit },
    transfer_circuit::{ TransferCircuit, TransferBatchCircuit },
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum OperatorError {
    Unknown,
    NotEnoughObjects,
    InvalidSignature,
    CircuitError(SynthesisError),
    IoError(std::io::Error),
}

impl Error for OperatorError {
    fn description(&self) -> &str {
        match *self {
            OperatorError::Unknown => "Unknown error",
            OperatorError::NotEnoughObjects => "Not enough objects for batch",
            OperatorError::InvalidSignature => "Invalid order signature",
            OperatorError::CircuitError(_) => "Encountered a circuit error",
            OperatorError::IoError(_) => "Encountered an I/O error",
        }
    }
}

impl fmt::Display for OperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let &OperatorError::IoError(ref e) = self {
            write!(f, "I/O error: ")?;
            e.fmt(f)
        } else {
            write!(f, "{}", self.description())
        }
    }
}

impl From<io::Error> for OperatorError {
    fn from(err: io::Error) -> Self {
        OperatorError::IoError(err)
    }
}

impl From<SynthesisError> for OperatorError {
    fn from(err: SynthesisError) -> Self {
        OperatorError::CircuitError(err)
    }
}

#[derive(Clone)]
pub struct Operator<'a> {
    // requests
    pub transfer_batch: usize,
    pub transfer_queue: Vec<Transfer>,
    pub deposit_batch: usize,
    pub deposit_queue: Vec<Deposit>,
    pub onchain_withdrawal_batch: usize,
    pub onchain_withdrawal_queue: Vec<OnchainWithdrawal>,
    pub offchain_withdrawal_batch: usize,
    pub offchain_withdrawal_queue: Vec<OffchainWithdrawal>,

    pub tree: AccountsTree<'a>,
    pub deposit_accum_hash: bn256::Fr,
    pub withdrawal_accum_hash: bn256::Fr,

    pub account_depth: usize,
    pub hash_params: &'a Bn256PoseidonParams,
    pub sign_params: &'a AltJubjubBn256,
    pub deposit_circuit_params: &'a Parameters::<Bn256>,
    pub onchain_withdrawal_circuit_params: &'a Parameters::<Bn256>,
    pub offchain_withdrawal_circuit_params: &'a Parameters::<Bn256>,
    pub transfer_circuit_params: &'a Parameters::<Bn256>,
}

#[allow(dead_code)]
impl<'a> Operator<'a> {
    pub fn new(
        transfer_batch: usize,
        onchain_withdrawal_batch: usize,
        offchain_withdrawal_batch: usize,
        deposit_batch: usize,
        account_depth: usize,
        hash_params: &'a Bn256PoseidonParams,
        sign_params: &'a AltJubjubBn256,
        deposit_circuit_params: &'a Parameters::<Bn256>,
        onchain_withdrawal_circuit_params: &'a Parameters::<Bn256>,
        offchain_withdrawal_circuit_params: &'a Parameters::<Bn256>,
        transfer_circuit_params: &'a Parameters::<Bn256>,
    ) -> Self {
        Operator {
            transfer_batch,
            transfer_queue: Vec::new(),
            deposit_batch,
            deposit_queue: Vec::new(),
            onchain_withdrawal_batch,
            onchain_withdrawal_queue: Vec::new(),
            offchain_withdrawal_batch,
            offchain_withdrawal_queue: Vec::new(),
            tree: AccountsTree::new(
                account_depth,
                hash_params,
                sign_params,
            ),
            deposit_accum_hash: bn256::Fr::zero(),
            withdrawal_accum_hash: bn256::Fr::zero(),
            account_depth,
            hash_params,
            sign_params,
            deposit_circuit_params,
            onchain_withdrawal_circuit_params,
            offchain_withdrawal_circuit_params,
            transfer_circuit_params,
        }
    }

    pub fn add_deposit(
        &mut self,
        deposit: Deposit,
    ) -> Result<(), OperatorError> {
        // TODO check deposit correctnes
        self.deposit_queue.push(deposit);

        Ok(())
    }

    pub fn add_onchain_withdrawal(
        &mut self,
        withdrawal: OnchainWithdrawal,
    ) -> Result<(), OperatorError> {
        // TODO check withdrawal correctnes
        self.onchain_withdrawal_queue.push(withdrawal);

        Ok(())
    }

    pub fn add_offchain_withdrawal(
        &mut self,
        withdrawal: OffchainWithdrawal,
    ) -> Result<(), OperatorError> {
        // TODO check withdrawal correctnes
        self.offchain_withdrawal_queue.push(withdrawal);

        Ok(())
    }

    pub fn add_transfer(
        &mut self,
        transfer: Transfer,
    ) -> Result<(), OperatorError> {
        // TODO assert correctness - recheck matcher: orders not cancelled, enough balances, prices correspond, price integer
        self.transfer_queue.push(transfer);

        Ok(())
    }

    pub fn execute_deposit_batch(
        &mut self,
    ) -> Result<(Vec::<bn256::Fr>, Proof<Bn256>), OperatorError> { 
        if self.deposit_queue.len() < self.deposit_batch {
            return Err(OperatorError::NotEnoughObjects);
        }

        // update local tree --------------------------------------------------

        let old_hash = self.deposit_accum_hash;
        let old_root = self.tree.get_root();
        let mut executed_deposits = Vec::new();

        for _ in 0..self.deposit_batch {
            let deposit = self.deposit_queue.remove(0);
            // update account

            let pubkey = deposit.pubkey.clone().unwrap();
            let (pubkey_x, pubkey_y) = pubkey.0.into_xy();

            // update accumulate hash

            self.deposit_accum_hash = {
                let hashes_vec = poseidon_hash::<Bn256>(
                    self.hash_params,
                    &[
                        self.deposit_accum_hash,
                        pubkey_x,
                        pubkey_y,
                        usize_to_fr(deposit.account_id),
                        usize_to_fr(deposit.amount),
                    ],
                );
                hashes_vec[0]
            };

            let account_state = deposit.update_tree_and_record_state(&mut self.tree);

            let executed_deposit = DepositCircuit {
                account_state,
                pubkey: Some(pubkey.0),
                account_id: Some(usize_to_fr(deposit.account_id)),
                amount: Some(usize_to_fr(deposit.amount)),
            };

            executed_deposits.push(executed_deposit);
        }

        let new_hash = self.deposit_accum_hash;
        let new_root = self.tree.get_root();

        // prepare snark input

        let circuit = DepositBatchCircuit {
            deposit_batch: self.deposit_batch,
            account_depth: self.account_depth,
            hash_params: self.hash_params,

            deposit_queue: executed_deposits,
            old_accum_hash: Some(old_hash),
            new_accum_hash: Some(new_hash),
            old_account_root: Some(old_root),
            new_account_root: Some(new_root),
        };
        /*
        {
            // for debug purposes only

            let mut cs = TestConstraintSystem::<Bn256>::new();
            circuit.clone().synthesize(&mut cs).expect("circuit must synthesize");

            let unconstrained = cs.find_unconstrained();
            println!("{}", unconstrained);
            assert!(unconstrained == "");

            let unsatisfied = cs.which_is_unsatisfied();
            if unsatisfied.is_some() {
                panic!("{}", unsatisfied.unwrap());
            }
        }
        */
        // generate proof

        let mut rng = thread_rng();
        let proof = create_random_proof(circuit, self.deposit_circuit_params, &mut rng)?;
        let public_inputs = vec![old_hash, new_hash, old_root, new_root];

        // TODO send new state to smart contract

        Ok((public_inputs, proof))
    }

    pub fn execute_onchain_withdrawal_batch(
        &mut self,
    ) -> Result<(Vec::<bn256::Fr>, Proof<Bn256>), OperatorError> {
        
        if self.onchain_withdrawal_queue.len() < self.onchain_withdrawal_batch {
            return Err(OperatorError::NotEnoughObjects);
        }

        // update local tree ----------------------------------------

        let old_hash = self.withdrawal_accum_hash;
        let old_root = self.tree.get_root();
        let mut executed = Vec::new();

        for _ in 0..self.onchain_withdrawal_batch {
            let mut withdrawal = self.onchain_withdrawal_queue.remove(0);

            // update accumulate hash
            self.withdrawal_accum_hash = {
                let hashes_vec = poseidon_hash::<Bn256>(
                    self.hash_params,
                    &[
                        self.withdrawal_accum_hash,
                        usize_to_fr(withdrawal.account_id),
                    ],
                );
                hashes_vec[0]
            };

            // calculate withdrawal amount (onchain withdrawal takes all value)
            withdrawal.amount = Some(fr_to_usize(
                self.tree.get_balance(withdrawal.account_id)
            ));

            let account_state = withdrawal.update_tree_and_record_state(&mut self.tree);

            let executed_withdrawal = OnchainWithdrawalCircuit {
                account_state,
                account_id: Some(usize_to_fr(withdrawal.account_id)),
                amount: Some(usize_to_fr(withdrawal.amount.unwrap())),
            };

            executed.push(executed_withdrawal);
        }

        let new_hash = self.withdrawal_accum_hash;
        let new_root = self.tree.get_root();

        // prepare snark input

        let circuit = OnchainWithdrawalBatchCircuit {
            batch_size: self.onchain_withdrawal_batch,
            account_depth: self.account_depth,
            hash_params: self.hash_params,
            queue: executed.clone(),
            old_accum_hash: Some(old_hash),
            new_accum_hash: Some(new_hash),
            old_account_root: Some(old_root),
            new_account_root: Some(new_root),
        };
        /*
        {
            // for debug purposes only

            let mut cs = TestConstraintSystem::<Bn256>::new();
            circuit.clone().synthesize(&mut cs).expect("circuit must synthesize");

            let unconstrained = cs.find_unconstrained();
            println!("{}", unconstrained);
            assert!(unconstrained == "");

            let unsatisfied = cs.which_is_unsatisfied();
            if unsatisfied.is_some() {
                panic!("{}", unsatisfied.unwrap());
            }
        }
        */
        // generate proof -------------------------------------------

        let mut rng = thread_rng();
        let proof = create_random_proof(circuit, self.onchain_withdrawal_circuit_params, &mut rng)?;
        
        let mut public_inputs = vec![old_hash, new_hash, old_root, new_root];
        for i in 0..self.onchain_withdrawal_batch {
            let mut inputs = vec![
                executed[i].account_id.unwrap(), 
                executed[i].amount.unwrap(),
            ];
            public_inputs.append(&mut inputs);
        }

        // TODO send new state to smart contract --------------------

        Ok((public_inputs, proof))
    }

    pub fn execute_offchain_withdrawal_batch(
        &mut self,
    ) -> Result<(Vec::<bn256::Fr>, Proof<Bn256>), OperatorError> {
        
        if self.offchain_withdrawal_queue.len() < self.offchain_withdrawal_batch {
            return Err(OperatorError::NotEnoughObjects);
        }

        // update local tree ----------------------------------------

        let old_root = self.tree.get_root();
        let mut executed = Vec::new();

        for _ in 0..self.offchain_withdrawal_batch {
            let withdrawal = self.offchain_withdrawal_queue.remove(0);

            self.check_offchain_withdrawal_signature(&withdrawal)?;

            let account_state = withdrawal.update_tree_and_record_state(&mut self.tree);

            let pubkey = self.tree.get_pubkey(withdrawal.account_id);

            let executed_withdrawal = OffchainWithdrawalCircuit {
                account_state,
                account_id: Some(usize_to_fr(withdrawal.account_id)),
                amount: Some(usize_to_fr(withdrawal.amount)),
                nonce: Some(usize_to_fr(withdrawal.nonce)),
                sign: Some(withdrawal.sign.unwrap()),
                pubkey: Some(pubkey.0),
            };

            executed.push(executed_withdrawal);
        }

        let new_root = self.tree.get_root();

        // prepare snark input

        let circuit = OffchainWithdrawalBatchCircuit {
            batch_size: self.offchain_withdrawal_batch,
            account_depth: self.account_depth,
            hash_params: self.hash_params,
            sign_params: self.sign_params,

            queue: executed.clone(),
            old_account_root: Some(old_root),
            new_account_root: Some(new_root),
        };
        
        // generate proof -------------------------------------------

        let mut rng = thread_rng();
        let proof = create_random_proof(circuit, self.offchain_withdrawal_circuit_params, &mut rng)?;
        
        let mut public_inputs = vec![old_root, new_root];
        for i in 0..self.offchain_withdrawal_batch {
            let mut inputs = vec![
                executed[i].account_id.unwrap(),
                executed[i].amount.unwrap(),
            ];
            public_inputs.append(&mut inputs);
        }

        // TODO send new state to smart contract --------------------

        Ok((public_inputs, proof))
    }

    fn check_transfer_signature(
        &self,
        transfer: &Transfer
    ) -> Result<(), OperatorError> {
        let pubkey = &self.tree.get_pubkey(transfer.account_id_from);

        if !transfer.verify_signature(
            pubkey,
            self.hash_params,
            self.sign_params,
        ) {
            return Err(OperatorError::InvalidSignature);
        }

        Ok(())
    }

    fn check_offchain_withdrawal_signature(
        &self,
        withdrawal: &OffchainWithdrawal
    ) -> Result<(), OperatorError> {
        let pubkey = &self.tree.get_pubkey(withdrawal.account_id);

        if !withdrawal.verify_signature(
            pubkey,
            self.hash_params,
            self.sign_params,
        ) {
            return Err(OperatorError::InvalidSignature);
        }

        Ok(())
    }

    pub fn execute_transfer_batch(
        &mut self,
    ) -> Result<(Vec::<bn256::Fr>, Proof<Bn256>), OperatorError> {
        
        if self.transfer_queue.len() < self.transfer_batch {
            return Err(OperatorError::NotEnoughObjects);
        }

        // update local tree ----------------------------------------

        let old_root = self.tree.get_root();
        let mut executed = Vec::new();

        for _ in 0..self.transfer_batch {
            let transfer = self.transfer_queue.remove(0);

            self.check_transfer_signature(&transfer)?;

            let (account_state_from, account_state_to) = transfer.update_tree_and_record_state(&mut self.tree);

            let pubkey = self.tree.get_pubkey(transfer.account_id_from);

            let executed_withdrawal = TransferCircuit {
                account_state_from,
                account_state_to,
                account_id_from: Some(usize_to_fr(transfer.account_id_from)),
                account_id_to: Some(usize_to_fr(transfer.account_id_to)),
                amount: Some(usize_to_fr(transfer.amount)),
                nonce: Some(usize_to_fr(transfer.nonce)),
                sign: Some(transfer.sign.unwrap()),
                pubkey: Some(pubkey.0),
            };

            executed.push(executed_withdrawal);
        }

        let new_root = self.tree.get_root();

        // prepare snark input

        let circuit = TransferBatchCircuit {
            batch_size: self.transfer_batch,
            account_depth: self.account_depth,
            hash_params: self.hash_params,
            sign_params: self.sign_params,
            queue: executed.clone(),
            old_account_root: Some(old_root),
            new_account_root: Some(new_root),
        };
        
        // generate proof -------------------------------------------

        let mut rng = thread_rng();
        let proof = create_random_proof(circuit, self.transfer_circuit_params, &mut rng)?;
        let public_inputs = vec![old_root, new_root];

        // TODO send new state to smart contract --------------------

        Ok((public_inputs, proof))
    }
}
