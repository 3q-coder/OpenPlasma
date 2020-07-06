use bellman_ce::{
    ConstraintSystem,
    SynthesisError,
};

use sapling_crypto_ce::{
    jubjub::JubjubEngine,
    circuit::{
        num::AllocatedNum,
        boolean::Boolean,
    },  
};

use ff_ce::Field;

pub fn add<E, CS> (
    mut cs: CS,
    a: &AllocatedNum<E>,
    b: &AllocatedNum<E>,
) -> Result<AllocatedNum<E>, SynthesisError>
    where E: JubjubEngine,
          CS: ConstraintSystem<E>,
{
    let a_val = a.get_value();
    let b_val = b.get_value();

    let sum = match (a_val, b_val) {
        (Some(a_val), Some(b_val)) => {
            let mut tmp = a_val;
            tmp.add_assign(&b_val);
            Some(tmp)
        },
        _ => None,
    };

    let sum = AllocatedNum::alloc(
        cs.namespace(|| "allocate sum"),
        || sum.ok_or(SynthesisError::AssignmentMissing),
    )?;

    cs.enforce(
        || "enforce sum",
        |lc| lc + a.get_variable() + b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + sum.get_variable(),
    );

    Ok(sum)
}

pub fn sub<E, CS> (
    mut cs: CS,
    a: &AllocatedNum<E>,
    b: &AllocatedNum<E>,
) -> Result<AllocatedNum<E>, SynthesisError>
    where E: JubjubEngine,
          CS: ConstraintSystem<E>,
{   let a_val = a.get_value();
    let b_val = b.get_value();

    let diff = match (a_val, b_val) {
        (Some(a_val), Some(b_val)) => {
            let mut tmp = a_val;
            tmp.sub_assign(&b_val);
            Some(tmp)
        },
        _ => None,
    };

    let diff = AllocatedNum::alloc(
        cs.namespace(|| "allocate diff"),
        || diff.ok_or(SynthesisError::AssignmentMissing),
    )?;

    cs.enforce(
        || "enforce diff",
        |lc| lc + a.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + diff.get_variable() + b.get_variable(),
    );

    Ok(diff)
}

pub fn div<E, CS> (
    mut cs: CS,
    a: &AllocatedNum<E>,
    b: &AllocatedNum<E>,
) -> Result<AllocatedNum<E>, SynthesisError>
    where E: JubjubEngine,
          CS: ConstraintSystem<E>,
{
    let a_val = a.get_value();
    let b_val = b.get_value();

    let b_inv_val = match b_val {
        Some(b_val) => {
            let b_inv_val = b_val.inverse().ok_or(SynthesisError::DivisionByZero)?;
            Some(b_inv_val)
        },
        None => None,
    };

    let quot = match (a_val, b_inv_val) {
        (Some(a_val), Some(b_inv_val)) => {
                let mut tmp = a_val;
            tmp.mul_assign(&b_inv_val);
            Some(tmp)
        },
        _ => None,
    };

    let quot = AllocatedNum::alloc(
        cs.namespace(|| "allocate quot"),
        || quot.ok_or(SynthesisError::AssignmentMissing),
    )?;

    cs.enforce(
        || "enforce quot",
        |lc| lc + b.get_variable(),
        |lc| lc + quot.get_variable(),
        |lc| lc + a.get_variable(),
    );

    Ok(quot)
}

pub fn boolean_to_allocated_num<E, CS> (
    mut cs: CS,
    cond: &Boolean,
) -> Result<AllocatedNum<E>, SynthesisError>
    where E: JubjubEngine,
          CS: ConstraintSystem<E>,
{   let zero = AllocatedNum::alloc(
        cs.namespace(|| "allocate zero"),
        || Ok(E::Fr::zero()),
    )?;

    let one = AllocatedNum::alloc(
        cs.namespace(|| "allocate one"),
        || Ok(E::Fr::one()),
    )?;

    zero.assert_zero(cs.namespace(|| "check zero"))?;
    cs.enforce(
        || "check one",
        |lc| lc + CS::one(),
        |lc| lc + CS::one(),
        |lc| lc + one.get_variable(),
    );

    AllocatedNum::conditionally_select(
        cs.namespace(|| "select zero or one"),
        &one,
        &zero,
        cond,
    )
}

pub fn check_decomposition_le<E, CS> (
    mut cs: CS,
    num: &AllocatedNum<E>,
    bits: &Vec::<Boolean>,
) -> Result<(), SynthesisError>
    where E: JubjubEngine,
          CS: ConstraintSystem<E>,
{
    let decomp = num.into_bits_le(
        cs.namespace(|| "decompose number")
    )?;

    for i in 0..bits.len() {
        Boolean::enforce_equal(
            cs.namespace(|| format!("check bit {}", i)),
            &decomp[i],
            &bits[i],
        )?;
    }

    Ok(())
}
