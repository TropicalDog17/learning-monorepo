use ark_ff::{
    fields::{Fp64, MontBackend, MontConfig},
    BigInteger, FftField, Field, Fp256, PrimeField,
};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial};
use itertools::Itertools;

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "5"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;

fn main() {
    // A dev creates a circuit with the polynomial x² + 2x + 3 === 11 and proves that 2 is a solution. What is the other solution? Hint: write the circuit as x² + 2x - 8 === 0 then factor the polynomial by hand to find the roots. Finally, compute the congruent element of the roots in the finite field to find the other solution.

    // x = 2 and x = -4.

    let solution_4 = Fq::from(-4);

    assert_eq!(
        solution_4 * solution_4 + Fq::from(2) * solution_4 + Fq::from(3),
        Fq::from(11)
    );

    let mut congruent_of_4 = Fq::MODULUS;
    congruent_of_4.sub_with_borrow(&Fq::from(4).into_bigint());
    assert_eq!(solution_4, Fq::from(congruent_of_4));
}
