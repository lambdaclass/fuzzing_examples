use lambdaworks_math::{
    fft::{
        bit_reversing::in_place_bit_reverse_permute, errors::FFTError,
        fft_iterative::in_place_nr_2radix_fft,
    },
    field::test_fields::u64_test_field::U64TestField,
    field::{element::FieldElement, traits::IsTwoAdicField},
    polynomial::Polynomial,
};
use libfuzzer_sys::fuzz_target;
use std::convert::TryInto;

const MODULUS: u64 = 0xFFFFFFFF00000001;
type F = U64TestField<MODULUS>;
type FE = FieldElement<F>;

fn log2(n: usize) -> Result<u64, FFTError> {
    if !n.is_power_of_two() {
        return Err(FFTError::InvalidOrder(
            "The order of polynomial + 1 should a be power of 2".to_string(),
        ));
    }
    Ok(n.trailing_zeros() as u64)
}

fuzz_target!(|data: &[u8]| {
    if data.len() != 8 || u64::from_be_bytes(data.try_into().unwrap()) == 0 {
        return;
    }

    let mut powers_of_two = 1 << (7 & data[0]);
    if powers_of_two == 1 {
        powers_of_two = 2;
    }

    let field_element = FE::from(u64::from_be_bytes(data.try_into().unwrap()));
    let field_vec = vec![field_element; powers_of_two];

    let root = F::get_primitive_root_of_unity(log2(field_vec.len()).unwrap()).unwrap();
    let mut twiddles = (0..field_vec.len() as u64)
        .map(|i| root.pow(i))
        .collect::<Vec<FE>>();
    in_place_bit_reverse_permute(&mut twiddles[..]); // required for NR

    let poly = Polynomial::new(&field_vec[..]);
    let expected: Vec<FE> = twiddles.iter().map(|x| poly.evaluate(x)).collect();

    let mut result = field_vec;
    in_place_nr_2radix_fft(&mut result, &twiddles[..]);
    in_place_bit_reverse_permute(&mut result);

    assert_eq!(result, expected);
});
