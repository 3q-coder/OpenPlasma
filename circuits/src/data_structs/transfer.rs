use std::fmt;

use sapling_crypto_ce::eddsa::Signature;
use pairing_ce::bn256::Bn256;

use crate::{
    account::AccountState,
    utils::tree::TreeState,
};

use super::super::{
    tree::account::AccountsTree,
};

use crate::utils::utils::{
    optionalize,
    fr_to_usize,
    usize_to_fr,
};

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Transfer {
    pub account_id_from: usize,
    pub account_id_to: usize,
    pub amount: usize,
    pub nonce: usize,
}

#[derive(Clone)]
pub struct SignedTransfer {
    pub transfer: Transfer,
    pub sign: Signature::<Bn256>,
}

impl Transfer {
    pub fn update_tree_and_record_state(
        & self,
        tree: &mut AccountsTree,
    ) -> (AccountState::<Bn256>, AccountState::<Bn256>) {
        
        assert!(self.account_id_from < tree.accounts.len());
        assert!(self.account_id_to < tree.accounts.len());

        // account from ------------------------------------------------------------

        // count balances
        let old_balance = tree.accounts[self.account_id].balance;
        let new_balance = {
            let old_balance = fr_to_usize(old_balance);
            assert!(old_balance >= self.amount);
            usize_to_fr(old_balance - amount_sell)
        };

        // prepare paths, indices, pubkeys, nonces
        let pubkey = tree.accounts[self.account_id].pubkey.clone();
        let old_nonce = tree.accounts[self.account_id].nonce;
        assert!(fr_to_usize(old_nonce) == self.nonce - 1);
        let new_nonce = usize_to_fr(self.nonce);
        let account_path = tree.accounts_tree.get_leaf_path(self.account_id);
        let account_indices = tree.accounts_tree.get_leaf_indices(self.account_id);

        // update balance
        tree.update_balance(
            self.account_id_from,
            new_balance,
        );

        // record account state
        let account_state_from = AccountState::<Bn256> {
            old_balance: Some(old_balance),
            new_balance: Some(new_balance),
            old_pubkey: Some(pubkey.0.clone()),
            new_pubkey: Some(pubkey.0),
            old_nonce: Some(nonce),
            new_nonce: Some(nonce),
            account_path: optionalize(account_path),
            account_indices: optionalize(account_indices),
        };

        // account to --------------------------------------------------------------

        // count balances
        let old_balance = tree.accounts[self.account_id_to].balance;
        let new_balance = usize_to_fr(fr_to_usize(old_balance) + self.amount);

        // prepare paths, indices, pubkeys, nonces
        let pubkey = tree.accounts[self.account_id].pubkey.clone();
        let nonce = tree.accounts[self.account_id].nonce;
        let account_path = tree.accounts_tree.get_leaf_path(self.account_id);
        let account_indices = tree.accounts_tree.get_leaf_indices(self.account_id);

        // update balance
        tree.update_balance(
            self.account_id,
            new_balance,
        );

        // record account state
        let account_state_to = AccountState::<Bn256> {
            old_balance: Some(old_balance),
            new_balance: Some(new_balance),
            old_pubkey: Some(pubkey.0.clone()),
            new_pubkey: Some(pubkey.0),
            old_nonce: Some(nonce),
            new_nonce: Some(nonce),
            account_path: optionalize(account_path),
            account_indices: optionalize(account_indices),
        };

        (account_state_from, account_state_to)
    }
}

impl fmt::Debug for SignedTransfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "SignedTransfer: {:?}", self.transfer)
    }
}

