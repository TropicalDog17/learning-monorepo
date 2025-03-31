use ark_ff::{fields::{ Fp64, MontBackend, MontConfig}};
use itertools::Itertools;

#[derive(MontConfig)]
#[modulus = "11"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

fn main() {
    // Example usage of the Fr type
    (0..10).cartesian_product(0..10).for_each(|(x, y)| {
        let fq_x = Fq::from(x);
        let fq_y = Fq::from(y);
        let equation_1 = fq_x + Fq::from(2) * fq_y;
        let equation_2 = Fq::from(6) * fq_x + Fq::from(1) * fq_y;

        if equation_1.eq(&Fq::from(1)) && equation_2.eq(&Fq::from(6)){
            println!("found solution x = {}, y = {}", fq_x, fq_y);
        }
    });
}