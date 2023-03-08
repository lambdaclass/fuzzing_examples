#![no_main]
use libfuzzer_sys::fuzz_target;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_377::curve::BLS12377Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::traits::IsShortWeierstrass;
use lambdaworks_math::elliptic_curve::short_weierstrass::element::ProjectivePoint;
use arbitrary::Arbitrary;

fuzz_target!(|point: ProjectivePoint::<BLS12377Curve>| {
    // check coordinate getters
    let coordinates = point.coordinates();
        assert_eq!(point.x(), &coordinates[0]);
        assert_eq!(point.y(), &coordinates[1]);
        assert_eq!(point.z(), &coordinates[2]);
});
