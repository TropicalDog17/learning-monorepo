use ark_ec::PrimeGroup;
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::Field;
use ark_std::UniformRand;

use ark_test_curves::bls12_381::Fr as ScalarField;
use ark_test_curves::bls12_381::{Bls12_381, Fq12, G1Projective as G1, G2Projective as G2};

fn main() {
    // The pairing engine is parameterized by the scalar field of the curve.
    let mut rng = ark_std::test_rng();
    let s = ScalarField::rand(&mut rng);
    let a = G1::rand(&mut rng);
    let b = G2::rand(&mut rng);

    // We can compute the pairing of two points on the curve, either monolithically...
    let e1 = Bls12_381::pairing(a, b);
    // ... or in two steps. First, we compute the Miller loop...
    let ml_result = Bls12_381::miller_loop(a, b);
    // ... and then the final exponentiation.
    let e2 = Bls12_381::final_exponentiation(ml_result).unwrap();
    assert_eq!(e1, e2);

    let p = ScalarField::rand(&mut rng);
    let q = ScalarField::rand(&mut rng);

    // f(P, Q)
    let left = Bls12_381::pairing(a * p, b * q);
    let right = e1 * p * q;

    // Bilinearity check: f(P, Q) = f(pA, qB) = pq * f(A, B)

    assert_eq!(left, right);
}
