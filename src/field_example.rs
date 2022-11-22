use ark_ff::Field;
use ark_bls12_381::Fq2 as F;
use ark_std::{One, UniformRand};

pub fn print_example(){
    let mut rng = ark_std::test_rng();
    let a = F::rand(&mut rng);
    let b = F::rand(&mut rng);

    let c = a + b;

    //let d = a.double();
    //let f = b.double();

    let a_inverse = a.inverse().unwrap();

    assert_eq!(a_inverse * a, F::one());

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}