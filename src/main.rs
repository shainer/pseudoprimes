// Application to compute whether a given number is a pseudoprime number.

use std::env;

fn find_prime_factors(n: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();
    let mut number = n;

    while number % 2 == 0 {
        factors.push(2);
        number /= 2;
    }

    let mut i = 3;
    while i <= (number as f32).sqrt() as i32 {
        while number % i == 0 {
            factors.push(i);
            number /= i;
        }

        i += 2;
    }

    if number > 2 {
        factors.push(number);
    }

    factors
}

// The test. A number is a pseudoprime if all of its prime factors are distinct,
// and if each prime factor minus 1 evenly divides the original number minus 1.
fn is_pseudo_prime(n: i32) -> Result<bool, String> {
    if n <= 0 {
        return Err("N must be a positive number".to_string());
    }

    let mut prime_factors: Vec<i32> = find_prime_factors(n);
    println!("Prime factors are {:?}", prime_factors);
    for f in &mut prime_factors {
        *f -= 1;
    }

    let old_len = prime_factors.len();
    prime_factors.sort();
    prime_factors.dedup();

    if prime_factors.len() != old_len {
        return Ok(false);
    }

    for f in &prime_factors {
        if (n - 1) % f != 0 {
            return Ok(false);
        }
    }

    return Ok(true);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: \"cargo run N\", with N positive integer.");
    }

    let n: i32;

    match args[1].parse::<i32>() {
        Ok(n1) => n = n1,
        Err(_) => panic!("Argument must be an integer."),
    }

    match is_pseudo_prime(n) {
        Ok(b) => {
            if b {
                println!("Number {} is a pseudoprime", n);
            } else {
                println!("Number {} is not a pseudoprime", n);
            }
        }
        Err(e) => panic!("Error occurred: {}", e),
    }
}
