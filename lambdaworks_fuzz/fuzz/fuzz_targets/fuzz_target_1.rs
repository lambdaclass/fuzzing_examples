#![no_main]
use libfuzzer_sys::fuzz_target;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_377::curve::BLS12377Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::traits::IsShortWeierstrass;
use lambdaworks_math::elliptic_curve::short_weierstrass::element::ProjectivePoint;
use lambdaworks_math::cyclic_group::IsCyclicGroup;
use arbitrary::Unstructured;
use arbitrary::Arbitrary;

fuzz_target!(|curve: ProjectivePoint::<BLS12377Curve>| {
    
    let curve2 = curve.operate_with(&curve);
    assert_ne!(&curve2, &curve);
    assert_eq!(&curve, &curve);

});