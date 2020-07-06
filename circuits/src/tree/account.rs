use sapling_crypto_ce::{
    poseidon::bn256::Bn256PoseidonParams,
    eddsa::PublicKey,
    alt_babyjubjub::AltJubjubBn256,
    jubjub::{
        edwards::Point,
        Unknown,
    },
};

use ff_ce::Field;

use pairing_ce::{
    bn256,
    bn256::Bn256,
};

use super::{
    balance::Balance,
    merkle_tree::PoseidonMerkleTree,
};

#[derive(Clone)]
pub struct Account {
    pub pubkey: PublicKey::<Bn256>,
    pub nonce: bn256::Fr,
    pub balance: bn256::Fr,
}

impl Account {
    pub fn new(
        sign_params: &AltJubjubBn256,
    ) -> Self {        
        let pubkey = PublicKey::<Bn256>(
            Point::<Bn256, Unknown>::get_for_y(bn256::Fr::zero(), true, sign_params).unwrap()
        );

        Account {
            pubkey,
            nonce: bn256::Fr::zero(),
            balance: bn256::Fr::zero(),
        }
    }

    pub fn compress_to_leaf(&self) -> Vec::<bn256::Fr> {
        let (pubkey_x, pubkey_y) = self.pubkey.0.into_xy();
        vec![pubkey_x, pubkey_y, self.nonce, self.balance]
    }
}

#[derive(Clone)]
pub struct AccountsTree<'a> {
    pub accounts: Vec::<Account>,
    pub accounts_tree: PoseidonMerkleTree::<'a, Bn256>,
}

#[allow(dead_code)]
impl<'a> AccountsTree<'a> {
    pub fn new(
        account_depth: usize,
        hash_params: &'a Bn256PoseidonParams,
        sign_params: &AltJubjubBn256,
    ) -> Self {
        let num_accounts = 1 << account_depth;
        let mut accounts = Vec::with_capacity(num_accounts);
        accounts.resize_with(
            num_accounts,
            || Account::new(sign_params),
        );

        let leaves: Vec<_> = accounts.iter().map(
            |account| account.compress_to_leaf()
        ).collect();
        let accounts_tree = PoseidonMerkleTree::<'a, Bn256>::new(leaves, hash_params);

        AccountsTree { accounts, accounts_tree }
    }

    pub fn update_account(
        &mut self,
        account_id: usize,
        pubkey: PublicKey::<Bn256>,
        nonce: bn256::Fr,
    ) {
        assert!(account_id < self.accounts.len());

        self.accounts[account_id].pubkey = pubkey;
        self.accounts[account_id].nonce = nonce;

        self.accounts_tree.update_leaf(
            account_id,
            self.accounts[account_id].compress_to_leaf(),
        );
    }
    
    pub fn get_pubkey(&self, account_id: usize) -> PublicKey::<Bn256> {
        assert!(account_id < self.accounts.len());        
        self.accounts[account_id].pubkey.clone()
    }

    pub fn get_nonce(&self, account_id: usize) -> bn256::Fr {
        assert!(account_id < self.accounts.len());        
        self.accounts[account_id].nonce
    }

    pub fn update_balance(
        &mut self,
        account_id: usize,
        new_balance: bn256::Fr,
    ) {
        assert!(account_id < self.accounts.len());
        self.accounts[account_id].balance = new_balance;

        self.accounts_tree.update_leaf(
            account_id,
            self.accounts[account_id].compress_to_leaf(),
        );
    }

    pub fn get_balance(&mut self, account_id: usize) -> bn256::Fr {
        assert!(account_id < self.accounts.len());
        self.accounts[account_id].balance 
    }

    pub fn get_root(&self) -> bn256::Fr {
        self.accounts_tree.root()
    }
}

