use std::fmt;

use pairing_ce::bn256;

use sapling_crypto_ce::{
    jubjub::JubjubEngine,
};

use ff_ce::{
    Field,
    PrimeField,
    BitIterator,
};

const BITS_IN_BYTE: usize = 8;

pub fn fr_to_usize(fr_a: bn256::Fr) -> usize {
    let a = fr_a.to_hex();
    usize::from_str_radix(a.as_str(), 16).expect("Failed to parse hex")
}

pub fn usize_to_fr(a: usize) -> bn256::Fr {
    bn256::Fr::from_hex(&format!("{:#066x}", a)).expect("Failed to parse hex")
}

pub fn bool_to_fr(cond: bool) -> bn256::Fr {
    if cond {
        bn256::Fr::one()
    } else {
        bn256::Fr::zero()
    }
}

pub fn fr_to_bytes_le(value: bn256::Fr, bytes_len: usize) -> Vec::<u8> {
    let mut field_char = BitIterator::new(bn256::Fr::char());
    let mut value_bits = Vec::with_capacity(bn256::Fr::NUM_BITS as usize);

    let mut found_one = false;
    for b in BitIterator::new(value.into_repr()) {
        found_one |= field_char.next().unwrap();
        if !found_one {
            continue;
        }

        value_bits.push(b);
    }

    value_bits.reverse();

    let mut value_bytes = Vec::with_capacity(bytes_len);
    for (i, byte_chunk) in value_bits.chunks(BITS_IN_BYTE).enumerate() {
        let mut byte = 0u8;
        for (j, bit) in byte_chunk.into_iter().enumerate() {
            if *bit {
                byte += 1 << j;
            }
        }

        value_bytes.push(byte);

        if i == bytes_len - 1 {
            break;
        }
    }

    value_bytes
}

pub fn fs_to_fr<E: JubjubEngine> (num: E::Fs) -> E::Fr {
    let mut num_in_bits_le: Vec<bool> = BitIterator::new(
        num.into_repr()
    ).collect();
    num_in_bits_le.reverse();

    let mut num_converted = E::Fr::zero();
    let mut base = E::Fr::one();

    for bit in num_in_bits_le {
        if bit {
            num_converted.add_assign(&base);
        }
        base.double();
    }

    num_converted
}

pub fn optionalize<T>(array: Vec::<T>) -> Vec::<Option<T>> {
    array.into_iter().map(
        |elem| Some(elem),
    ).collect()
}

pub fn write_formatted_vector<T: fmt::Debug>(
    f: &mut fmt::Formatter<'_>,
    output: Vec::<Vec::<T>>,
    annotation: &str,
) -> fmt::Result {
    write!(f, "{}: [\n", annotation)?;
    for subvector in output.iter() {
        write!(f, "    [\n")?;
        for elem in subvector.into_iter() {
            write!(f, "        {:?},\n", elem)?;
        }
        write!(f, "    ],\n")?;
    }
    write!(f, "]\n")?;

    Ok(())
}

