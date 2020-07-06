use sapling_crypto_ce::{
    eddsa::{
        PrivateKey,
        PublicKey,
    },
    poseidon::{
        poseidon_hash,
        bn256::Bn256PoseidonParams,
    },
    jubjub::FixedGenerators,
    alt_babyjubjub::AltJubjubBn256,
};

use pairing_ce::{
    bn256,
    bn256::Bn256,
};

use rand::thread_rng;

use super::order::{ Order, SignedOrder };

use crate::utils::utils::{
    usize_to_fr,
    bool_to_fr,
    fr_to_bytes_le,
};

pub const NUM_BYTES_TO_SIGN: usize = 31;

pub fn order_hash(order: &Order, hash_params: &Bn256PoseidonParams) -> bn256::Fr {
    let leaf = vec![
        usize_to_fr(order.order_id),
        usize_to_fr(order.account_id),
        usize_to_fr(order.token_s),
        usize_to_fr(order.token_b),
        usize_to_fr(order.amount_s),
        usize_to_fr(order.amount_b),
        bool_to_fr(order.buy),
        bool_to_fr(order.asset_b_basic),
    ];

    let hash_vec = poseidon_hash::<Bn256>(hash_params, &leaf);
    hash_vec[0]
}

pub fn sign_order(
    order: Order,
    seckey: &PrivateKey::<Bn256>,
    hash_params: &Bn256PoseidonParams,
    sign_params: &AltJubjubBn256,
) -> SignedOrder {
    let hash = order_hash(&order, hash_params);
    let hash_bytes: Vec<_> = fr_to_bytes_le(hash, NUM_BYTES_TO_SIGN);
    let mut rng = thread_rng();

    let sign = seckey.sign_raw_message(
        &hash_bytes,
        &mut rng,
        FixedGenerators::SpendingKeyGenerator,
        sign_params,
        NUM_BYTES_TO_SIGN,
    );

    SignedOrder { order, sign }
}

pub fn verify_order_signature(
    signed_order: &SignedOrder,
    pubkey: &PublicKey::<Bn256>,
    hash_params: &Bn256PoseidonParams,
    sign_params: &AltJubjubBn256,
) -> bool {
    let hash = order_hash(&signed_order.order, hash_params);
    let hash_bytes: Vec<_> = fr_to_bytes_le(hash, NUM_BYTES_TO_SIGN);

    pubkey.verify_for_raw_message(
        &hash_bytes,
        &signed_order.sign,
        FixedGenerators::SpendingKeyGenerator,
        sign_params,
        NUM_BYTES_TO_SIGN,
    )
}
