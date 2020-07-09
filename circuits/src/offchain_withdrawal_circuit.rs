use std::mem;

use bellman_ce::{
    Circuit,
    ConstraintSystem,
    SynthesisError,
};

use sapling_crypto_ce::{
    jubjub::{
        Unknown,
        JubjubEngine,
        edwards::Point,
    },
    poseidon::{
        PoseidonEngine,
        QuinticSBox,
    },
    circuit::{
        baby_eddsa::EddsaSignature,
        poseidon_hash::poseidon_hash,
        num::AllocatedNum,
        boolean::{ Boolean, AllocatedBit },
        ecc::EdwardsPoint,
    },  
    eddsa::Signature,
};

use crate::utils::{
    sign::verify_signature,
    calc::boolean_to_allocated_num,
};

use super::account::{ AccountState, AccountCircuit };
use super::utils::calc::check_decomposition_le;

const BITS_IN_BYTE: usize = 8;
const NUM_BYTES_TO_SIGN: usize = 31;

#[derive(Clone)]
pub struct OffchainWithdrawalCircuit<E: JubjubEngine + PoseidonEngine> {
    pub account_state: AccountState<E>,
    pub account_id: Option::<E::Fr>,
    pub amount: Option::<E::Fr>,
    pub nonce: Option::<E::Fr>,
    pub sign: Option::<Signature<E>>,
    pub pubkey: Option::<Point<E, Unknown>>,
}

impl<E> OffchainWithdrawalCircuit<E>
    where E: JubjubEngine + PoseidonEngine<SBox = QuinticSBox<E>>,
{
    pub fn process<'a, CS: ConstraintSystem<E>> (
        &self,
        mut cs: CS,
        account_depth: usize,
        hash_params: &'a <E as PoseidonEngine>::Params,
        sign_params: &'a <E as JubjubEngine>::Params,
        old_root: &AllocatedNum<E>,
    ) -> Result<AllocatedNum<E>, SynthesisError> {
        
        // allocate avariables ----------------------------------------------------------
        
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

        let nonce_alloc = AllocatedNum::alloc(
            cs.namespace(|| "allocate nonce"),
            || self.nonce.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // check signature --------------------------------------------------------------

        let withdrawal_hash = {
            let hash_vec = poseidon_hash(
                cs.namespace(|| "calculate message hash"),
                &[
                    account_id_alloc.clone(),
                    amount_alloc.clone(),
                    nonce_alloc.clone(),
                ],
                hash_params,
            )?;
            hash_vec[0].clone()
        };

        let sign_alloc = verify_signature(
            cs.namespace(|| "verify signature"),
            self.sign.clone(),
            self.pubkey.clone(),
            &withdrawal_hash,
            NUM_BYTES_TO_SIGN,
            sign_params,
        )?;

        // check changes validity -------------------------------------------------------

        // check pubkey consistency

        Self::check_pubkey(
            cs.namespace(|| "public key consistence"),
            &sign_alloc.pk,
            &account_circuit,
        );
        
        // check account id, asset id consistency

        check_decomposition_le(
            cs.namespace(|| "account id consistence"),
            &account_id_alloc,
            &account_circuit.accounts_tree.indices_alloc,
        )?;

        // check amount

        cs.enforce(
            || "check amount withdrawal",
            |lc| lc + account_circuit.accounts_tree.old_leaf_alloc[3].get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + account_circuit.accounts_tree.new_leaf_alloc[3].get_variable()
                + amount_alloc.get_variable(),
        );

        // check balance for overflow

        account_circuit.accounts_tree.new_leaf_alloc[3].limit_number_of_bits(
            cs.namespace(|| "check buy balance overflow"),
            mem::size_of::<usize>() * BITS_IN_BYTE,
        )?;

        // check nonce

        cs.enforce(
            || "nonce consistence",
            |lc| lc + account_circuit.accounts_tree.old_leaf_alloc[2].get_variable() + CS::one(),
            |lc| lc + CS::one(),
            |lc| lc + nonce_alloc.get_variable(),
        );

        cs.enforce(
            || "check nonce + 1",
            |lc| lc + account_circuit.accounts_tree.old_leaf_alloc[2].get_variable() + CS::one(),
            |lc| lc + CS::one(),
            |lc| lc + account_circuit.accounts_tree.new_leaf_alloc[2].get_variable(),
        );

        // verify old root & calculate new root -----------------------------------------

        account_circuit.accounts_tree.verify_old_root(
            cs.namespace(|| "verify old root"),
            old_root,
        )?;

        let new_root = account_circuit.accounts_tree.calc_new_root(
            cs.namespace(|| "calculate new root"),
        )?;

        Ok(new_root)
    }

    pub fn check_pubkey<CS: ConstraintSystem<E>> (
        mut cs: CS,
        pubkey: &EdwardsPoint<E>,
        account_circuit: &AccountCircuit<E>,
    ) {
        cs.enforce(
            || "enforce pubkey x and old leaf equivalence",
            |lc| lc + pubkey.get_x().get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + account_circuit.accounts_tree.old_leaf_alloc[0].get_variable(),
        );

        cs.enforce(
            || "enforce pubkey x and new leaf equivalence",
            |lc| lc + pubkey.get_x().get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + account_circuit.accounts_tree.new_leaf_alloc[0].get_variable(),
        );

        cs.enforce(
            || "enforce pubkey y and old leaf equivalence",
            |lc| lc + pubkey.get_y().get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + account_circuit.accounts_tree.old_leaf_alloc[1].get_variable(),
        );

        cs.enforce(
            || "enforce pubkey y and new leaf equivalence",
            |lc| lc + pubkey.get_y().get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + account_circuit.accounts_tree.new_leaf_alloc[1].get_variable(),
        );
    }
}

#[derive(Clone)]
pub struct OffchainWithdrawalBatchCircuit<'a, E: JubjubEngine + PoseidonEngine> {
    pub batch_size: usize,
    pub account_depth: usize,
    pub hash_params: &'a <E as PoseidonEngine>::Params,
    pub sign_params: &'a <E as JubjubEngine>::Params,

    pub queue: Vec::<OffchainWithdrawalCircuit<E>>,
    pub old_account_root: Option::<E::Fr>,
    pub new_account_root: Option::<E::Fr>,
}

impl<'a, E> Circuit<E> for OffchainWithdrawalBatchCircuit<'a, E>
    where E: JubjubEngine + PoseidonEngine<SBox = QuinticSBox<E>>,
{
    fn synthesize<CS: ConstraintSystem<E>> (
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        assert_eq!(self.batch_size, self.queue.len());

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
            let root = withdrawal.process(
                cs.namespace(|| format!("verify withdrawal {}", i)),
                self.account_depth,
                self.hash_params,
                self.sign_params,
                &prev_root,
            )?;

            prev_root = root;
        }

        cs.enforce(
            || "enforce new root equivalence",
            |lc| lc + prev_root.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + new_root.get_variable(),
        );

        Ok(())
    }
}
