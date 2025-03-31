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
    let fq_x = Fq::from(10);
    let mut congruent_of_10 = Fq::MODULUS;
    congruent_of_10.sub_with_borrow(&Fq::from(10).into_bigint());

    let fq_y = Fq::from(congruent_of_10);

    let sum = fq_x + fq_y + Fq::from(0);
    let product = fq_x * fq_y * Fq::from(0);

    assert_eq!(sum, Fq::from(0));
    assert_eq!(product, Fq::from(0));

    println!("sum and product is 0, yet not all elements are 0");
    println!("x = {}, y = {}", fq_x, fq_y);
}
