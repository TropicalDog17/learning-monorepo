use std::{
    collections::HashMap,
    fmt::Display,
    time::{Duration, Instant},
};

use ark_ff::{
    AdditiveGroup, BigInt, BigInteger, FftField, Field, Fp, MontConfig, PrimeField, UniformRand,
};
use ark_secp256k1::{Fq, Fr};
use ark_std::rand::{thread_rng, RngCore};

// Get unsafe scalar for cryptography
// Only in range [0, 2^b) where b is the upper_bound_bit
fn get_unsafe_scalar(upper_bound_bit: usize) -> Fq {
    // Create a random number generator
    let mut rng = thread_rng();

    // Initialize random number to 0
    let mut random_u32 = 0u32;

    // Set each bit up to the upper bound
    for i in 0..upper_bound_bit {
        // Generate a random bit (0 or 1)
        let random_bit = rng.next_u32() % 2;

        // Set the i-th bit of the random number if the random bit is 1
        if random_bit == 1 {
            random_u32 |= (1 << i);
        }
    }

    // Convert the random number to Fq
    Fq::from(random_u32 as u64)
}

// Naive brute force implementation
fn naive_brute_force(target: Fq, max_iterations: u32) -> BenchmarkResult {
    let start = Instant::now();

    let mut solution = None;
    for i in 0..max_iterations {
        let x = Fq::from(i as u64);
        if x.mul_by_base_prime_field(&Fq::GENERATOR) == target {
            solution = Some(x);
            break;
        }
    }

    let duration = start.elapsed();

    BenchmarkResult {
        method_name: "Naive Brute Force".to_string(),
        solution,
        execution_time: duration,
        success: solution.is_some(),
    }
}
fn bsgs(target: Fq, max_iterations: u32) -> BenchmarkResult {
    let start = Instant::now();

    let g = Fq::GENERATOR;
    let order = Fq::MODULUS;

    // Calculate m = ceil(sqrt(order)) more precisely
    let m = (order.num_bits() + 1) / 2;
    let m: BigInt<4> = BigInt::from(1u64) << m;

    let mut lookup_table = HashMap::<Fq, BigInt<4>>::new();

    // Pre-compute baby steps
    let mut current = Fq::ONE;

    for j in 0..m.0[0] {
        lookup_table.insert(current, BigInt::from(j));
        current *= g;
    }

    let g_pow_minus_m = g.inverse().unwrap().pow(m);
    let mut current = target;

    // Giant steps
    for i in 0..m.0[0] {
        if let Some(j) = lookup_table.get(&current) {
            let result = i * m.as_ref()[0] + j.as_ref()[0];

            let result = Fq::from(result);
            let computed_target = g.mul_by_base_prime_field(&result);
            if computed_target == target {
                return BenchmarkResult {
                    method_name: "Baby Step Giant Step".to_string(),
                    solution: Some(result),
                    execution_time: start.elapsed(),
                    success: true,
                };
            }
        }

        current.mul_by_base_prime_field(&g_pow_minus_m);
    }

    BenchmarkResult {
        method_name: "Baby Step Giant Step".to_string(),
        solution: None,
        execution_time: start.elapsed(),
        success: false,
    }
}
enum Method {
    Naive,
    BabyStepGiantStep,
    PollardsRho,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Naive => write!(f, "Naive Brute Force"),
            Method::BabyStepGiantStep => write!(f, "Baby-step Giant-step"),
            Method::PollardsRho => write!(f, "Pollard's Rho"),
        }
    }
}

// Benchmark result struct to hold all timing and solution data
struct BenchmarkResult {
    method_name: String,
    solution: Option<Fq>,
    execution_time: Duration,
    success: bool,
}
// Function to run a single benchmark method
fn run_benchmark(
    method: Method,
    target: Fq,
    expected_result: Fq,
    bit_size: usize,
) -> BenchmarkResult {
    println!(
        "Running benchmark for discrete logarithm with {} bit input",
        bit_size
    );
    println!("Target value: {:?}", target);
    println!("Expected result: {:?}", expected_result);
    println!("--------------------------------------------------------------");

    // Set limitations for the algorithms
    let max_brute_force = if bit_size <= 28 {
        2u32.pow(bit_size as u32)
    } else {
        2u32.pow(28)
    };
    let max_iterations = if bit_size <= 28 {
        10_000_000
    } else {
        1_000_000
    };

    // Run the selected method
    let result = match method {
        Method::Naive => {
            println!("Running Naive Brute Force...");
            naive_brute_force(target, max_brute_force)
        }
        Method::BabyStepGiantStep => {
            println!("Running Baby-step Giant-step...");
            // baby_step_giant_step(target, 2u32.pow(bit_size as u32 / 2 + 1))
            bsgs(target, max_iterations)
        }
        Method::PollardsRho => {
            println!("Running Pollard's Rho...");
            // pollards_rho(target, max_iterations)
            unimplemented!()
        }
        _ => {
            println!(
                "Unknown method: {}, defaulting to Naive Brute Force",
                method
            );
            naive_brute_force(target, max_brute_force)
        }
    };

    // Print result
    println!("\nBenchmark Result:");
    println!("--------------------------------------------------------------");
    println!(
        "{:<20} | {:<15} | {:<20} | {:<10}",
        "Method", "Time (ms)", "Solution Found", "Correct"
    );
    println!("--------------------------------------------------------------");

    let is_correct = match result.solution {
        Some(sol) => sol == expected_result,
        None => false,
    };

    println!(
        "{:<20} | {:<15.2} | {:<20} | {:<10}",
        result.method_name,
        result.execution_time.as_millis(),
        result.success,
        is_correct
    );

    // Print detailed result
    println!("\nDetailed Result:");
    println!("  Method: {}", result.method_name);
    println!("  Execution time: {:?}", result.execution_time);
    println!("  Solution found: {}", result.success);
    if let Some(sol) = result.solution {
        println!("  Solution value: {:?}", sol);
        println!("  Correct solution: {}", sol == expected_result);

        // Verify the solution
        let computed_target = sol.mul_by_base_prime_field(&Fq::GENERATOR);
        println!(
            "  Verification: target == g^solution: {}",
            computed_target == target
        );
    }

    result
}

fn main() {
    // Config
    let bit_size = 24; // Change this to test different bit sizes
    let method = Method::Naive; // Change this to run different methods: "naive", "bsgs", or "pollard"

    // Generate random scalar
    let scalar_gen_start = Instant::now();
    let bounded_scalar = get_unsafe_scalar(bit_size);
    let scalar_gen_time = scalar_gen_start.elapsed();

    // Compute target
    let target_comp_start = Instant::now();
    let target = Fq::GENERATOR.mul_by_base_prime_field(&bounded_scalar);
    let target_comp_time = target_comp_start.elapsed();

    println!("Scalar generation time: {:?}", scalar_gen_time);
    println!("Target computation time: {:?}", target_comp_time);

    // Run all benchmark methods
    let methods = vec![Method::Naive, Method::BabyStepGiantStep];
    for method in methods {
        run_benchmark(method, target, bounded_scalar, bit_size);
    }
    println!("--------------------------------------------------------------");
    println!("All benchmarks completed.");
}
