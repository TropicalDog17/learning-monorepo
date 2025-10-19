// The Schnoor Identification Protocol allow a party A to prove to another party B
// That they know a secret x, without revealing any knowledge of x to B
// This protocol is an interactive version of ZKP

use std::hash::RandomState;

use ark_bls12_381::{Config, Fr, G1Affine};
use ark_ec::{bls12::Bls12Config, short_weierstrass::Projective, AffineRepr, CurveGroup, PrimeGroup};
use ark_ff::{Field, PrimeField, UniformRand};
use rand::Rng;

fn honest_alice_phase_1(x: Fr) -> (Fr, G1Affine, G1Affine) {
    let mut rng = ark_std::test_rng();
    let r = Fr::rand(&mut rng);
    let g = G1Affine::generator();
    let X = g.mul_bigint(x.into_bigint()).into();
    let R = g.mul_bigint(r.into_bigint()).into();
    (r, X, R)
}
fn honest_alice_phase_2(x: Fr, r: Fr, c: Fr) -> Fr {
    r + c.mul_by_base_prime_field(&x)
}

fn dishonest_alice_phase_1() -> (Fr, G1Affine) {
    let mut rng = ark_std::test_rng();

    let r: ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4> = Fr::rand(&mut rng);
    let g = G1Affine::generator();
    let R = g.mul_bigint(r.into_bigint()).into();
    (r, R)
}

fn dishonest_alice_phase_2(r: Fr) -> Fr {
    r
}

fn b_verify(X: G1Affine, R: G1Affine, e: Fr, c: Fr) -> bool {
    let g = G1Affine::generator();
    g.mul_bigint(e.into_bigint()) == R + X.mul_bigint(c.into_bigint())
}
fn main() {
    let x = Fr::from(1702);
    let X = G1Affine::generator().mul_bigint(x.into_bigint());
    let mut rng =  rand::rng();

    // Honest first
    for i in 0..15 {
        let (r, X, R) = honest_alice_phase_1(x);
        let c = Fr::from(rng.random_range(0..1));
        let e = honest_alice_phase_2(x, r, c);

        assert!(b_verify(X, R, e, c))
    }

    let mut successes = 0;
    for _ in 0..100 {
        let (r, R) = dishonest_alice_phase_1();
        let c = if rng.random_bool(0.5) { Fr::from(1u64) } else { Fr::from(0u64) };
        let e = dishonest_alice_phase_2(r);
        if b_verify(X.into(), R, e, c) {
            successes += 1;
        }
    }
    println!("Dishonest prover success rate: {}%", successes);
    
}