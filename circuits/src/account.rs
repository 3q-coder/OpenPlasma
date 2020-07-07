use bellman_ce::{
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
        edwards::Point,
        Unknown,
    },
};

use super::utils::tree::{
    TreeCircuit,
    TreeState,
};

pub const ACCOUNT_LEAF_SIZE: usize = 4;

#[derive(Clone)]
pub struct AccountState<E: JubjubEngine> {
    pub old_balance: Option<E::Fr>,
    pub new_balance: Option<E::Fr>,
    pub old_pubkey: Option<Point<E, Unknown>>,
    pub new_pubkey: Option<Point<E, Unknown>>,
    pub old_nonce: Option<E::Fr>,
    pub new_nonce: Option<E::Fr>,
    pub account_path: Vec::<Option<E::Fr>>,
    pub account_indices: Vec::<Option<bool>>,
}

#[derive(Clone)]
pub struct AccountCircuit<'a, E: JubjubEngine + PoseidonEngine> {
    pub accounts_tree: TreeCircuit<'a, E>,
}

impl<'a, E> AccountCircuit<'a, E>
    where E: JubjubEngine + PoseidonEngine<SBox = QuinticSBox<E>>,
{
    pub fn new<CS: ConstraintSystem<E>> (
        mut cs: CS,
        account_depth: usize,
        params: &'a <E as PoseidonEngine>::Params,
        state: &AccountState<E>,
    ) -> Result<Self, SynthesisError> {

        let (old_pubkey_x, old_pubkey_y) = match &state.old_pubkey {
            Some(point) => {
                let (x, y) = point.into_xy();
                (Some(x), Some(y))
            },
            None => (None, None),
        };

        let (new_pubkey_x, new_pubkey_y) = match &state.new_pubkey {
            Some(point) => {
                let (x, y) = point.into_xy();
                (Some(x), Some(y))
            },
            None => (None, None),
        };

        let account_old_leaf = vec![
            old_pubkey_x,
            old_pubkey_y,
            state.old_nonce.clone(),
            state.old_balance.clone(),
        ];

        let account_new_leaf = vec![
            new_pubkey_x,
            new_pubkey_y,
            state.new_nonce.clone(),
            state.new_balance.clone(),
        ];

        let tree_state = TreeState {
            old_leaf: account_old_leaf,
            new_leaf: account_new_leaf,
            path: state.account_path.clone(),
            indices: state.account_indices.clone(),
        };

        let accounts_tree = TreeCircuit::<'a, E>::new(
            cs.namespace(|| "allocate accounts tree"),
            ACCOUNT_LEAF_SIZE,
            account_depth,
            params,
            &tree_state,
        )?;

        let circuit = AccountCircuit {
            accounts_tree,
        };

        Ok(circuit)
    }
}
