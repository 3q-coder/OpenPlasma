use sapling_crypto_ce::eddsa::PublicKey;
use pairing_ce::bn256::Bn256;

use crate::account::AccountState;

use super::super::{
    tree::account::AccountsTree,
};

use crate::utils::utils::{
    optionalize,
    fr_to_usize,
    usize_to_fr,
};

#[derive(Clone)]
pub struct Deposit {
    pub pubkey: Option::<PublicKey::<Bn256>>,
    pub account_id: usize,
    pub amount: usize,
}

impl Deposit {
    pub fn update_tree_and_record_state(
        &self,
        tree: &mut AccountsTree,
    ) -> AccountState::<Bn256> {
        assert!(self.account_id < tree.accounts.len());

        // count balances
        let old_balance = tree.accounts[self.account_id].balance;
        let new_balance = usize_to_fr(fr_to_usize(old_balance) + self.amount);

        // prepare paths, indices, pubkeys, nonces
        let old_pubkey = tree.accounts[self.account_id].pubkey.clone();
        let nonce = tree.accounts[self.account_id].nonce;
        let new_pubkey = self.pubkey.clone().unwrap();
        let account_path = tree.accounts_tree.get_leaf_path(self.account_id);
        let account_indices = tree.accounts_tree.get_leaf_indices(self.account_id);

        // update balance & account
        tree.update_balance(
            self.account_id,
            new_balance,
        );

        tree.update_account(
            self.account_id,
            new_pubkey.clone(),
            nonce,
        );

        // record account state
        AccountState::<Bn256> {
            old_balance: Some(old_balance),
            new_balance: Some(new_balance),
            old_pubkey: Some(old_pubkey.0),
            new_pubkey: Some(new_pubkey.0),
            old_nonce: Some(nonce),
            new_nonce: Some(nonce),
            account_path: optionalize(account_path),
            account_indices: optionalize(account_indices),
        }
    }
}
