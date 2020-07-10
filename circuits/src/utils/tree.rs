use bellman_ce::{
    ConstraintSystem,
    SynthesisError,
};

use sapling_crypto_ce::{
    jubjub::JubjubEngine,
    poseidon::{
        PoseidonEngine,
        QuinticSBox,
    },
    circuit::{
        num::AllocatedNum,
        boolean::Boolean,
        poseidon_hash::poseidon_hash,
    },
};

use super::{
    alloc::{ alloc_nums, alloc_bits },
};

#[derive(Clone)]
pub struct TreeState<E: JubjubEngine> {
    pub old_leaf: Vec::<Option<E::Fr>>,
    pub new_leaf: Vec::<Option<E::Fr>>,
    pub path: Vec::<Option<E::Fr>>,
    pub indices: Vec::<Option<bool>>,
}

#[derive(Clone)]
pub struct TreeCircuit<'a, E: JubjubEngine + PoseidonEngine> {
    pub params: &'a <E as PoseidonEngine>::Params,
    pub old_leaf_alloc: Vec::<AllocatedNum<E>>,
    pub new_leaf_alloc: Vec::<AllocatedNum<E>>,
    pub path_alloc: Vec::<AllocatedNum<E>>,
    pub indices_alloc: Vec::<Boolean>,
}

impl<'a, E> TreeCircuit<'a, E> 
    where E: JubjubEngine + PoseidonEngine<SBox = QuinticSBox<E>>,
{
    pub fn new<CS: ConstraintSystem<E>> (
        mut cs: CS,
        leaf_size: usize,
        tree_depth: usize,
        params: &'a <E as PoseidonEngine>::Params,
        tree_state: &TreeState<E>,
    ) -> Result<Self, SynthesisError> {
        assert_eq!(tree_state.old_leaf.len(), leaf_size);
        assert_eq!(tree_state.new_leaf.len(), leaf_size);
        assert_eq!(tree_state.indices.len(), tree_depth);
        assert_eq!(tree_state.path.len(), tree_depth);

        let old_leaf_alloc = alloc_nums(
            cs.namespace(|| "allocate old leaf"),
            &tree_state.old_leaf,
        )?;

        let new_leaf_alloc = alloc_nums(
            cs.namespace(|| "allocate new leaf"),
            &tree_state.new_leaf,
        )?;

        let path_alloc = alloc_nums(
            cs.namespace(|| "allocate leaf path"),
            &tree_state.path,
        )?;

        let indices_alloc = alloc_bits(
            cs.namespace(|| "allocate leaf path indices"),
            &tree_state.indices,
        )?;

        let tree = TreeCircuit {
            params,
            old_leaf_alloc,
            new_leaf_alloc,
            path_alloc,
            indices_alloc,
        };

        Ok(tree)
    }

    pub fn calc_old_root<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
    ) -> Result<AllocatedNum<E>, SynthesisError> {
        calc_root(
            cs.namespace(|| "calculate old root"),
            self.params,
            &self.old_leaf_alloc,
            &self.path_alloc,
            &self.indices_alloc,
        )
    }

    pub fn calc_new_root<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
    ) -> Result<AllocatedNum<E>, SynthesisError> {
        calc_root(
            cs.namespace(|| "calculate new root"),
            self.params,
            &self.new_leaf_alloc,
            &self.path_alloc,
            &self.indices_alloc,
        )
    }

    pub fn verify_old_root<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        old_root: &AllocatedNum<E>,
    ) -> Result<(), SynthesisError> {
        verify(
            cs.namespace(|| "verify old root"),
            self.params,
            &self.old_leaf_alloc,
            &self.path_alloc,
            &self.indices_alloc,
            old_root,
        )
    }
}

// TODO the same logic in utils/tree ???
pub fn calc_root<E, CS> (
    mut cs: CS,
    params: &<E as PoseidonEngine>::Params,
    leaf: &[AllocatedNum<E>],
    path: &[AllocatedNum<E>],
    indices: &[Boolean],
) -> Result<AllocatedNum<E>, SynthesisError>
    where E: JubjubEngine + PoseidonEngine<SBox = QuinticSBox<E>>,
          CS: ConstraintSystem<E>,
{
    let mut prev_hash = {
        let hashes_vec = poseidon_hash(
            cs.namespace(|| "calculate leaf hash"),
            leaf,
            params,
        )?;
        hashes_vec[0].clone()
    };

    for (i, (index, neighbor_hash)) in indices.iter()
        .zip(path.iter())
        .enumerate()
    {
        let (left, right) = AllocatedNum::conditionally_reverse(
            cs.namespace(|| format!("conditionally reversing node children {}", i)),
            &prev_hash,
            neighbor_hash,
            index,
        )?;

        prev_hash = {
            let hashes_vec = poseidon_hash(
                cs.namespace(|| format!("calculate level hash {}", i)),
                &[left, right],
                params,
            )?;
            hashes_vec[0].clone()
        };
    }

    Ok(prev_hash)
}

pub fn verify<E, CS> (
    mut cs: CS,
    params: &<E as PoseidonEngine>::Params,
    leaf: &[AllocatedNum<E>],
    path: &[AllocatedNum<E>],
    indices: &[Boolean],
    root: &AllocatedNum<E>,
) -> Result<(), SynthesisError>
    where E: JubjubEngine + PoseidonEngine<SBox = QuinticSBox<E>>,
          CS: ConstraintSystem<E>,
{
    let last_hash = calc_root(
        cs.namespace(|| "calculate root"),
        params,
        leaf,
        path,
        indices,
    )?;

    cs.enforce(
        || "enforce root equivalence",
        |lc| lc + last_hash.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + root.get_variable(),
    );

    Ok(())
}

