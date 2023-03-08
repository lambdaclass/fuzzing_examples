#[cfg(test)]
mod tests {
    use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_377::curve::BLS12377Curve;
    use lambdaworks_math::elliptic_curve::short_weierstrass::traits::IsShortWeierstrass;
    use lambdaworks_math::elliptic_curve::short_weierstrass::element::ProjectivePoint;
    use lambdaworks_math::cyclic_group::IsCyclicGroup;
    use arbitrary::Unstructured;
    use arbitrary::Arbitrary;


    #[test]
    fn fuzz_regression_01() {
        let raw_data = include_bytes!(
            "../fuzz/artifacts/fuzz_target_1/crash-e0939b1011098054c350958a42f3df1bb93f5f58"
        );

        let mut unstructured = Unstructured::new(raw_data);

        // this creates an ProjectivePoint::<BLS12377Curve> using the arbitrary trait and the raw data file, then 
        // operates the curve with itself and should give a different curve but the curve should remain the same.
        if let Ok(curve) = ProjectivePoint::<BLS12377Curve>::arbitrary(&mut unstructured) {

            println!("curve: {:?}", curve);
            let curve2 = curve.operate_with(&curve);
            println!("curve2: {:?}", curve2);
            assert_ne!(&curve2, &curve);
            assert_eq!(&curve, &curve);

        }    
    }
}
