use ark_ec::{ProjectiveCurve, AffineCurve};
use ark_ff::{PrimeField, Field};
use ark_bls12_381::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField};
use ark_std::{Zero, UniformRand};

pub fn print_example() {
    let mut rng = ark_std::test_rng();

    let a = G::rand(&mut rng);
    let b = G::rand(&mut rng);

    let c = a + b;
    let d = a - b;

    assert_eq!(c + d, a.double());
    // assert_eq!(c + d, a); // unsuccessful try

    let e = -a;
    assert_eq!(e + a, G::zero());

    // Scalar multiplication
    let scalar = ScalarField::rand(&mut rng);
    let e = c.mul(&scalar.into_repr()); // sA + sB
    let f = e.mul(&scalar.inverse().unwrap().into_repr()); // s^-1(sA) + s^-1(sB) = a + b => c
    assert_eq!(f, c);

    println!("{}", c);
}