use super::utils::{
    fs_to_fr,
};

use sapling_crypto_ce::{
    jubjub::{
        Unknown,
        JubjubEngine,
        JubjubParams,
        FixedGenerators,
        edwards::Point,
    },
   circuit::{
        num::AllocatedNum,
        baby_eddsa::EddsaSignature,
        ecc::EdwardsPoint,
    },
    eddsa::Signature,
};

use bellman_ce::{
    ConstraintSystem,
    SynthesisError,
};

const BITS_IN_BYTE: usize = 8;

pub fn alloc_signature<E, CS>(
    mut cs: CS,
    sign: Option::<Signature<E>>,
    pk: Option::<Point<E, Unknown>>,
    params: &E::Params,
) -> Result<EddsaSignature<E>, SynthesisError>
    where E: JubjubEngine,
          CS: ConstraintSystem<E>,
{
    let (r, s) = match sign {
        Some(sign) => (Some(sign.r), Some(fs_to_fr::<E>(sign.s))),
        None => (None, None),
    };

    let r_alloc = EdwardsPoint::witness(
        cs.namespace(|| "allocate signature.r"),
        r,
        params,
    )?; 

    let s_alloc = AllocatedNum::alloc(
        cs.namespace(|| "allocate signature.s"),
        || s.ok_or(SynthesisError::AssignmentMissing),
    )?; 

    let pk_alloc = EdwardsPoint::witness(
        cs.namespace(|| "allocate public key"),
        pk,
        params,
    )?;

    let sign_alloc = EddsaSignature {
        r: r_alloc,
        s: s_alloc,
        pk: pk_alloc,
    };

    Ok(sign_alloc)
}

pub fn verify_signature<E, CS>(
    mut cs: CS,
    sign: Option::<Signature<E>>,
    pk: Option::<Point<E, Unknown>>,
    leaf_hash: &AllocatedNum<E>,
    msg_bytes_len: usize,
    params: &E::Params,
) -> Result<EddsaSignature<E>, SynthesisError>
    where E: JubjubEngine,
          CS: ConstraintSystem<E>,
{
    let sign_alloc = alloc_signature(
        cs.namespace(|| "allocate signature"),
        sign,
        pk,
        params,
    )?;

    let msg_alloc = leaf_hash.into_bits_le(
        cs.namespace(|| "convert hash to bits")
    )?;

    let public_generator = params.generator(
            FixedGenerators::SpendingKeyGenerator
    ).clone();
    let generator = EdwardsPoint::witness(
        cs.namespace(|| "allocate public generator"),
        Some(public_generator),
        params
    )?;

    sign_alloc.verify_raw_message_signature(
        cs.namespace(|| "verify signature"),
        params,
        &msg_alloc[..(msg_bytes_len * BITS_IN_BYTE)],
        generator,
        msg_bytes_len,
    )?;

    Ok(sign_alloc)
}

