use ark_ff::fields::{Fp64, MontBackend, MontConfig};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial};

#[derive(MontConfig)]
#[modulus = "103"]
#[generator = "5"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

fn main() {
    let p1 = DensePolynomial::from_coefficients_vec(vec![Fq::from(102), Fq::from(2), Fq::from(1)]);

    let p2 = DensePolynomial::from_coefficients_vec(vec![Fq::from(1), Fq::from(1), Fq::from(1)]);

    println!("{:?}", (p1 + p2).coeffs);

    let p3 = DensePolynomial::from_coefficients_vec(vec![Fq::from(1), Fq::from(-1)]);

    let p4 = DensePolynomial::from_coefficients_vec(vec![Fq::from(2), Fq::from(-1)]);

    println!("{:?}", (p3.naive_mul(&p4)));
}
