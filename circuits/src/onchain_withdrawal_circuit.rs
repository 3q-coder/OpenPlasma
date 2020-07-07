use std::mem;

use bellman_ce::{
    Circuit,
    ConstraintSystem,
    SynthesisError,
};

use sapling_crypto_ce::{
    poseidon::{
        PoseidonEngine,
        QuinticSBox,
    },
    jubjub::{
        JubjubEngine,
    },
    circuit::{
        num::AllocatedNum,
        poseidon_hash::poseidon_hash,
    },
};

use super::account::{ AccountState, AccountCircuit };
use super::utils::calc::check_decomposition_le;

const BITS_IN_BYTE: usize = 8;

#[derive(Clone)]
pub struct OnchainWithdrawalCircuit<E: JubjubEngine + PoseidonEngine> {
    pub account_state: AccountState<E>,
    pub account_id: Option::<E::Fr>,
    pub amount: Option::<E::Fr>,
}

impl<E> OnchainWithdrawalCircuit<E>
    where E: JubjubEngine + PoseidonEngine<SBox = QuinticSBox<E>>,
{
    pub fn process<'a, CS: ConstraintSystem<E>> (
        &self,
        mut cs: CS,
        account_depth: usize,
        hash_params: &'a <E as PoseidonEngine>::Params,
        old_hash: &AllocatedNum<E>,
        old_root: &AllocatedNum<E>,
    ) -> Result<(AllocatedNum<E>, AllocatedNum<E>), SynthesisError> {
        
        // allocate avariables --------------------------------------
        
        let account_circuit = AccountCircuit::new(
            cs.namespace(|| "allocate account circuit"),
            account_depth,
            hash_params,
            &self.account_state,
        )?;

        let account_id_alloc = AllocatedNum::alloc(
            cs.namespace(|| "allocate account id"),
            || self.account_id.ok_or(SynthesisError::AssignmentMissing),
        )?;
        account_id_alloc.inputize(cs.namespace(|| "input account id"))?;

        let amount_alloc = AllocatedNum::alloc(
            cs.namespace(|| "allocate amount"),
            || self.amount.ok_or(SynthesisError::AssignmentMissing),
        )?;
        amount_alloc.inputize(cs.namespace(|| "input amount"))?;

        // check changes validity -----------------------------------
        
        // check account id, asset id consistency

        check_decomposition_le(
            cs.namespace(|| "account id consistence"),
            &account_id_alloc,
            &account_circuit.accounts_tree.indices_alloc,
        )?;

        // check amount

        cs.enforce(
            || "check amount withdrawal",
            |lc| lc + account_circuit.accounts_tree.old_leaf_alloc[0].get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + account_circuit.accounts_tree.new_leaf_alloc[0].get_variable()
                + amount_alloc.get_variable(),
        );

        // check balance for overflow

        account_circuit.accounts_tree.new_leaf_alloc[0].limit_number_of_bits(
            cs.namespace(|| "check buy balance overflow"),
            mem::size_of::<usize>() * BITS_IN_BYTE,
        )?;

        // check nonce the same TODO ???

        cs.enforce(
            || "check nonce the same",
            |lc| lc + account_circuit.accounts_tree.old_leaf_alloc[2].get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + account_circuit.accounts_tree.new_leaf_alloc[2].get_variable(),
        );

        // calculate new hash ---------------------------------------

        let new_hash = {
            let hashes_vec = poseidon_hash(
                cs.namespace(|| "calculate new accum hash"),
                &[
                    old_hash.clone(),
                    account_id_alloc,
                ],
                hash_params,
            )?;
            hashes_vec[0].clone()
        };

        // verify old root & calculate new root ---------------------

        account_circuit.accounts_tree.verify_old_root(
            cs.namespace(|| "verify old root"),
            old_root,
        )?;

        let new_root = account_circuit.accounts_tree.calc_new_root(
            cs.namespace(|| "calculate new root"),
        )?;

        Ok((new_hash, new_root))
    }
}

#[derive(Clone)]
pub struct OnchainWithdrawalBatchCircuit<'a, E: JubjubEngine + PoseidonEngine> {
    pub batch_size: usize,
    pub account_depth: usize,
    pub balance_depth: usize,
    pub hash_params: &'a <E as PoseidonEngine>::Params,

    pub queue: Vec::<OnchainWithdrawalCircuit<E>>,
    pub old_accum_hash: Option::<E::Fr>,
    pub new_accum_hash: Option::<E::Fr>,
    pub old_account_root: Option::<E::Fr>,
    pub new_account_root: Option::<E::Fr>,
}

impl<'a, E> Circuit<E> for OnchainWithdrawalBatchCircuit<'a, E>
    where E: JubjubEngine + PoseidonEngine<SBox = QuinticSBox<E>>,
{
    fn synthesize<CS: ConstraintSystem<E>> (
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        assert_eq!(self.batch_size, self.queue.len());

        let mut prev_hash = AllocatedNum::alloc(
            cs.namespace(|| "allocate old accum hash"),
            || self.old_accum_hash.ok_or(SynthesisError::AssignmentMissing),
        )?;
        prev_hash.inputize(cs.namespace(|| "input old accum hash"))?;

        let new_hash = AllocatedNum::alloc(
            cs.namespace(|| "allocate new accum hash"),
            || self.new_accum_hash.ok_or(SynthesisError::AssignmentMissing),
        )?;
        new_hash.inputize(cs.namespace(|| "input new accum hash"))?;

        let mut prev_root = AllocatedNum::alloc(
            cs.namespace(|| "allocate old root"),
            || self.old_account_root.ok_or(SynthesisError::AssignmentMissing),
        )?;
        prev_root.inputize(cs.namespace(|| "input old root"))?;

        let new_root = AllocatedNum::alloc(
            cs.namespace(|| "allocate new root"),
            || self.new_account_root.ok_or(SynthesisError::AssignmentMissing),
        )?;
        new_root.inputize(cs.namespace(|| "input new root"))?;

        for (i, withdrawal) in self.queue.iter().enumerate() {
            let (hash, root) = withdrawal.process(
                cs.namespace(|| format!("verify withdrawal {}", i)),
                self.account_depth,
                self.hash_params,
                &prev_hash,
                &prev_root,
            )?;

            prev_hash = hash;
            prev_root = root;
        }

        cs.enforce(
            || "enforce new accum hash equivalence",
            |lc| lc + prev_hash.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + new_hash.get_variable(),
        );

        cs.enforce(
            || "enforce new root equivalence",
            |lc| lc + prev_root.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + new_root.get_variable(),
        );

        Ok(())
    }
}
