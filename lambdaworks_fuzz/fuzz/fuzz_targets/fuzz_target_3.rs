#![no_main]
use libfuzzer_sys::fuzz_target;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_377::curve::BLS12377Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::traits::IsShortWeierstrass;
use lambdaworks_math::elliptic_curve::short_weierstrass::element::ProjectivePoint;
use lambdaworks_math::cyclic_group::IsCyclicGroup;
use arbitrary::Arbitrary;

fuzz_target!(|point_a: ProjectivePoint::<BLS12377Curve>| {
    /// Computes the addition of `self` and `self`.
    let operate = point_a.operate_with(&point_a);

    // Returns the sum of projective points `p` and `q`
    let sum = <BLS12377Curve as IsShortWeierstrass>::add(&point_a.value, &point_a.value);

    // The value of the ProjectivePoint.value when operate with itself shoould be the 
    // same as the addition of ProjectivePoint.value with itself
    assert_eq!(sum, operate.value);
});
