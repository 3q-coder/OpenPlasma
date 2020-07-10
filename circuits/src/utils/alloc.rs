use bellman_ce::{
    ConstraintSystem,
    SynthesisError,
};

use sapling_crypto_ce::{
    jubjub::JubjubEngine,
    circuit::{
        num::AllocatedNum,
        boolean::{
            AllocatedBit,
            Boolean,
        },
    },
};

pub fn alloc_nums<E, CS> (
    mut cs: CS,
    array: &Vec::<Option<E::Fr>>,
) -> Result<Vec::<AllocatedNum<E>>, SynthesisError>
    where E: JubjubEngine,
          CS: ConstraintSystem<E>,
{   
    let mut allocated_array = Vec::with_capacity(array.len());
    
    for (i, x) in array.iter().enumerate() {
        let value = AllocatedNum::alloc(
            cs.namespace(|| format!("allocate number {}", i)),
            || x.ok_or(SynthesisError::AssignmentMissing),
        )?;
        allocated_array.push(value);
    }
    
    Ok(allocated_array)
}

pub fn alloc_bits<E, CS> (
    mut cs: CS,
    array: &Vec::<Option<bool>>,
) -> Result<Vec::<Boolean>, SynthesisError>
    where E: JubjubEngine,
          CS: ConstraintSystem<E>,
{
    let mut allocated_array = Vec::with_capacity(array.len());

    for (i, x) in array.into_iter().enumerate() {
        let value = Boolean::from(
            AllocatedBit::alloc(
                cs.namespace(|| format!("allocate bit {}", i)),
                x.clone(),
            )?
        );
        allocated_array.push(value);
    }

    Ok(allocated_array)
}

