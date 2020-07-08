use openplasma_circuits::{
    data_structs::{
        transfer::Transfer,
        deposit::Deposit,
        onchain_withdrawal::OnchainWithdrawal,
        offchain_withdrawal::OffchainWithdrawal,
    },
    operator::Operator,
    utils::utils::{fr_to_usize, usize_to_fr},
    account::AccountState,
    deposit_circuit::{ DepositCircuit, DepositBatchCircuit },
    onchain_withdrawal_circuit::{ OnchainWithdrawalCircuit, OnchainWithdrawalBatchCircuit },
    offchain_withdrawal_circuit::{ OffchainWithdrawalCircuit, OffchainWithdrawalBatchCircuit },
    transfer_circuit::{ TransferCircuit, TransferBatchCircuit },
};

use bellman_ce::{
    SynthesisError,
    groth16::{
        Parameters,
        generate_random_parameters,
        prepare_verifying_key,
        verify_proof,
    },
};

use sapling_crypto_ce::{
    poseidon::{bn256::Bn256PoseidonParams, poseidon_hash},
    group_hash::BlakeHasher,
    jubjub::FixedGenerators,
    alt_babyjubjub::AltJubjubBn256,
    eddsa::{ PublicKey, PrivateKey },
};

use pairing_ce::bn256::Bn256;

use rand::{ Rng, thread_rng };

// circuit params generation ------------------------------------------------------------
// --------------------------------------------------------------------------------------

fn setup_deposit_circuit<'a>(
    deposit_batch: usize,
    account_depth: usize,
    hash_params: &'a Bn256PoseidonParams,
) -> Result<Parameters<Bn256>, SynthesisError> {
    let account_state = AccountState::<Bn256> {
        old_balance: None,
        new_balance: None,
        old_pubkey: None,
        old_nonce: None,
        new_pubkey: None,
        new_nonce: None,
        account_path: vec![None; account_depth],
        account_indices: vec![None; account_depth],
    };

    let deposit_gen = || {
        DepositCircuit::<Bn256> {
            account_state: account_state.clone(),
            pubkey: None,
            account_id: None,
            amount: None,
        }
    };

    let mut deposit_queue = Vec::with_capacity(deposit_batch);
    deposit_queue.resize_with(deposit_batch, deposit_gen);

    let circuit = DepositBatchCircuit {
        deposit_batch,
        account_depth,
        hash_params,
        deposit_queue,
        old_accum_hash: None,
        new_accum_hash: None,
        old_account_root: None,
        new_account_root: None,
    };

    let mut rng = thread_rng();
    generate_random_parameters(circuit, &mut rng)
}

fn setup_onchain_withdraw_circuit<'a>(
    batch_size: usize,
    account_depth: usize,
    hash_params: &'a Bn256PoseidonParams,
) -> Result<Parameters<Bn256>, SynthesisError> {
    let account_state = AccountState::<Bn256> {
        old_balance: None,
        new_balance: None,
        old_pubkey: None,
        old_nonce: None,
        new_pubkey: None,
        new_nonce: None,
        account_path: vec![None; account_depth],
        account_indices: vec![None; account_depth],
    };

    let withdrawal_gen = || {
        OnchainWithdrawalCircuit::<Bn256> {
            account_state: account_state.clone(),
            account_id: None,
            amount: None,
        }
    };

    let mut queue = Vec::with_capacity(batch_size);
    queue.resize_with(batch_size, withdrawal_gen);

    let circuit = OnchainWithdrawalBatchCircuit {
        batch_size,
        account_depth,
        hash_params,
        queue,
        old_accum_hash: None,
        new_accum_hash: None,
        old_account_root: None,
        new_account_root: None,
    };

    let mut rng = thread_rng();
    generate_random_parameters(circuit, &mut rng)
}

fn setup_offchain_withdraw_circuit<'a>(
    batch_size: usize,
    account_depth: usize,
    hash_params: &'a Bn256PoseidonParams,
    sign_params: &'a AltJubjubBn256,
) -> Result<Parameters<Bn256>, SynthesisError> {
    let account_state = AccountState::<Bn256> {
        old_balance: None,
        new_balance: None,
        old_pubkey: None,
        old_nonce: None,
        new_pubkey: None,
        new_nonce: None,
        account_path: vec![None; account_depth],
        account_indices: vec![None; account_depth],
    };

    let withdrawal_gen = || {
        OffchainWithdrawalCircuit::<Bn256> {
            account_state: account_state.clone(),
            account_id: None,
            amount: None,
            nonce: None,
            sign: None,
            pubkey: None,
        }
    };

    let mut queue = Vec::with_capacity(batch_size);
    queue.resize_with(batch_size, withdrawal_gen);

    let circuit = OffchainWithdrawalBatchCircuit {
        batch_size,
        account_depth,
        hash_params,
        sign_params,
        queue,
        old_account_root: None,
        new_account_root: None,
    };

    let mut rng = thread_rng();
    generate_random_parameters(circuit, &mut rng)
}

fn setup_transfer_circuit<'a>(
    batch_size: usize,
    account_depth: usize,
    hash_params: &'a Bn256PoseidonParams,
    sign_params: &'a AltJubjubBn256,
) -> Result<Parameters<Bn256>, SynthesisError> {
    let account_state = AccountState::<Bn256> {
        old_balance: None,
        new_balance: None,
        old_pubkey: None,
        old_nonce: None,
        new_pubkey: None,
        new_nonce: None,
        account_path: vec![None; account_depth],
        account_indices: vec![None; account_depth],
    };

    let transfer_gen = || {
        TransferCircuit::<Bn256> {
            account_state_from: account_state.clone(),
            account_state_to: account_state.clone(),
            account_id_from: None,
            account_id_to: None,
            amount: None,
            nonce: None,
            sign: None,
            pubkey: None,
        }
    };

    let mut queue = Vec::with_capacity(batch_size);
    queue.resize_with(batch_size, transfer_gen);

    let circuit = TransferBatchCircuit::<'a, Bn256> {
        batch_size,
        account_depth,
        hash_params,
        sign_params,
        queue,
        old_account_root: None,
        new_account_root: None,
    };

    let mut rng = thread_rng();
    generate_random_parameters(circuit, &mut rng)
}

// tests --------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------

#[test]
pub fn execute_trade() {
    let hash_params = Bn256PoseidonParams::new_for_params::<BlakeHasher>(5,6,52,126);
    let sign_params = AltJubjubBn256::new();

    let dep_params = setup_deposit_circuit(4, 2, &hash_params).unwrap();
    let on_w_params = setup_onchain_withdraw_circuit(2, 2, &hash_params).unwrap();
    let of_w_params = setup_offchain_withdraw_circuit(2, 2, &hash_params, &sign_params).unwrap();
    let transfer_params = setup_transfer_circuit(2, 2, &hash_params, &sign_params).unwrap();

    let mut oper = Operator::new(2, 2, 2, 4, 2,
        &hash_params, &sign_params, &dep_params, &on_w_params, 
        &of_w_params, &transfer_params);
    
    let mut rng = thread_rng();

    let seckey_maker = PrivateKey::<Bn256>(rng.gen());
    let pubkey_maker = PublicKey::from_private(
        &seckey_maker,
        FixedGenerators::SpendingKeyGenerator,
        &sign_params,
    );

    let seckey_taker = PrivateKey::<Bn256>(rng.gen());
    let pubkey_taker = PublicKey::from_private(
        &seckey_taker,
        FixedGenerators::SpendingKeyGenerator,
        &sign_params,
    );

    // check deposit execution ----------------------------------------------------------

    let deposit_maker = Deposit {
        pubkey: Some(pubkey_maker.clone()),
        account_id: 0,
        amount: 100,
    };
    oper.add_deposit(deposit_maker).unwrap();

    let deposit_taker = Deposit {
        pubkey: Some(pubkey_taker.clone()),
        account_id: 1,
        amount: 100,
    };
    oper.add_deposit(deposit_taker).unwrap();

    let (public_inputs, proof) = oper.execute_deposit_batch().unwrap();

    // check deposit proof

    let verifying_key = prepare_verifying_key(&dep_params.vk);

    let is_valid = verify_proof(&verifying_key, &proof, &public_inputs).unwrap();
    assert!(is_valid);

    // check after deposit execution

    assert_eq!(fr_to_usize(oper.tree.get_balance(0)), 100);
    assert_eq!(fr_to_usize(oper.tree.get_balance(1)), 100);

    // check transfer execution ------------------------------------------------------------

    let mut transfer = Transfer {
        account_id_from: 0,
        account_id_to: 1,
        amount: 1,
        nonce: 1,
        sign: None,
    };

    transfer.sign(&seckey_maker, &hash_params, &sign_params);
    oper.add_transfer(transfer.clone()).unwrap();

    let (public_inputs, proof) = oper.execute_transfer_batch().unwrap();

    // check transfer batch proof

    let verifying_key = prepare_verifying_key(&transfer_params.vk);

    let is_valid = verify_proof(&verifying_key, &proof, &public_inputs).unwrap();
    assert!(is_valid);

    // check after trade execution

    assert_eq!(oper.transfer_queue.len(), 0);

    assert_eq!(fr_to_usize(oper.tree.get_balance(0)), 99);
    assert_eq!(fr_to_usize(oper.tree.get_balance(1)), 101);

    // check offchain withdrawal execution ----------------------------------------------

    let mut withdrawal = OffchainWithdrawal {
        account_id: 0,
        amount: 10,
        nonce: 2,
        sign: None,
    };

    withdrawal.sign(&seckey_maker, &hash_params, &sign_params);
    oper.add_offchain_withdrawal(withdrawal.clone()).unwrap();

    withdrawal.nonce = 3;

    withdrawal.sign(&seckey_maker, &hash_params, &sign_params);
    oper.add_offchain_withdrawal(withdrawal.clone()).unwrap();

    let (public_inputs, proof) = oper.execute_offchain_withdrawal_batch().unwrap();

    // check deposit proof

    let verifying_key = prepare_verifying_key(&of_w_params.vk);

    let is_valid = verify_proof(&verifying_key, &proof, &public_inputs).unwrap();
    assert!(is_valid);

    // check withdrawal execution

    assert_eq!(fr_to_usize(oper.tree.get_balance(0)), 79);

    // check onchain withdrawal ---------------------------------------------------------

    let mut withdrawal = OnchainWithdrawal {
        account_id: 0,
        amount: None,
    };
    oper.add_onchain_withdrawal(withdrawal.clone()).unwrap();

    withdrawal.account_id = 1;

    oper.add_onchain_withdrawal(withdrawal.clone()).unwrap();

    let (public_inputs, proof) = oper.execute_onchain_withdrawal_batch().unwrap();

    // check deposit proof

    let verifying_key = prepare_verifying_key(&on_w_params.vk);

    let is_valid = verify_proof(&verifying_key, &proof, &public_inputs).unwrap();
    assert!(is_valid);

    // check withdrawal execution

    assert_eq!(fr_to_usize(oper.tree.get_balance(0)), 0);
    assert_eq!(fr_to_usize(oper.tree.get_balance(1)), 0);
}
