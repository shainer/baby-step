use std::collections::HashMap;
use std::env;

fn print_help() {
    println!("cargo run <order> <generator> <input>");
    println!("- order:     the (prime) order n of the cyclic group");
    println!("- generator: a generator for the group");
    println!("- input:     the number we want the discrete logarithm of");
    println!("Example: cargo run 31 3 6");
}

// Efficient computation of the modular exponentiation by chaining.
//
// Instead of computing (base ^ exp) mod n, we compute
//   ((base ^ 2 mod n) ^ 2 mod n) ...
// until the product of all exponents is equal to the original exponent.
// When the exponent is not a power of 2, we multiply by the base as many
// times as necessary to make it become one.
//
// For instance
//   base ^ 25 mod n = (base * base^24) mod n = (base * base^8 * base^16) mod n.
fn modular_exponentiation(base: u32, exp: u32, n: u32) -> u32 {
    if base == 0 {
        return 0;
    }

    let mut intermediate = 1;
    let mut b = base;
    let mut e = exp;

    while e != 0 {
        if e % 2 != 0 {
            intermediate = (intermediate * b) % n;
        }
        e /= 2;
        b = (b * b) % n;
    }

    return intermediate;
}

// The modular inverse is the number x such that base*x mod n = 1.
// It does not always exist; it is only guaranteed to exist if the base
// and n are coprimes. Here, this is already required by the setup of
// the algorithm, so we assume it's true (we could do verification too).
//
//   base^tot(n) = 1 (tot(n) is Euler's totient function).
//   base^(tot(n) - 1) = base^-1 mod n.
//
// When n is a prime, tot(n) = n-1, so
//   base^(n-2) = base^-1 mod n.
//
// So we reuse the modular exponentiation algorithm. There are more
// efficient methods too.
fn modular_inverse(base: u32, n: u32) -> u32 {
    return modular_exponentiation(base, n - 2, n);
}

fn baby_step_giant_step(n: u32, alpha: u32, beta: u32) -> Result<u32, &'static str> {
    let m = (n as f64).sqrt().ceil() as u32;
    let mut precomp = HashMap::new();

    for j in 0..m {
        precomp.insert(modular_exponentiation(alpha, j, n), j);
    }

    let invgenerator = modular_inverse(modular_exponentiation(alpha, m, n), n);
    let mut y: u32 = beta;
    let mut found = false;
    let mut res: u32 = 0;

    for i in 0..m {
        if precomp.contains_key(&y) {
            match precomp.get(&y) {
                Some(value) => res = (i * m) + value,
                None => return Err("internal error"),
            }

            found = true;
            break;
        }

        y = y * invgenerator % n;
    }

    if !found {
        return Err("not found");
    }

    Ok(res)
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        print_help();
        return;
    }

    if args.len() != 4 {
        print_help();
        panic!("Wrong number of command line arguments");
    }

    let n: u32 = args[1].parse::<u32>().unwrap();
    let alpha: u32 = args[2].parse::<u32>().unwrap();
    let beta: u32 = args[3].parse::<u32>().unwrap();

    println!("Discrete logarithm through baby-step giant-step algorithm.\n");
    println!("Computing x such that {}^x mod {} = {}", alpha, n, beta);

    match baby_step_giant_step(n, alpha, beta) {
        Result::Ok(value) => println!("Discrete logarithm is {}", value),
        Result::Err(_) => println!("Could not find discrete logarithm."),
    }
}
