use std::collections::HashMap;
use std::env;

fn print_help() {
    println!("cargo run <order> <generator> <input>");
    println!("- order:     the (prime) order n of the cyclic group");
    println!("- generator: a generator for the group");
    println!("- input:     the number we want the discrete logarithm of");
}

fn baby_step_giant_step(n : u32, alpha : u32, beta : u32) -> Result<u32, &'static str> {
    let m = (n as f64).sqrt().ceil() as u32;
    let mut precomp = HashMap::new();

    for j in 0..m {
        precomp.insert((alpha.pow(j) as u32), j);
    }

    // This should be an invmod, g^-m.
    let invgenerator : u32 = 2;
    let mut y : u32 = beta;
    let mut found = false;
    let mut res : u32 = 0;

    for i in 0..m {
        if precomp.contains_key(&y) {
            match precomp.get(&y) {
                Some(value) => res = (i * m) + value,
                None => return Err("internal error")
            }

            found = true;
            break;
        }

        y = y * invgenerator % n;
    }

    if !found {
        return Err("not found")
    }

    Ok(res)
}

fn main() {
    // TODO: input parameters from command line.
    // let p : u32 = 31;
    // let g : u32 = 3;
    // let h = 6;
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        print_help();
        return;
    }

    if args.len() != 4 {
        print_help();
        panic!("Wrong number of command line arguments");
    }

    let n : u32 = args[1].parse::<u32>().unwrap();
    let alpha : u32 = args[2].parse::<u32>().unwrap();
    let beta : u32 = args[3].parse::<u32>().unwrap();

    println!("Discrete logarithm through baby-step giant-step algorithm.\n");
    println!("Computing x such that {}^x mod {} = {}", alpha, n, beta);

    match baby_step_giant_step(n, alpha, beta) {
        Result::Ok(value) => println!("Discrete logarithm is {}", value),
        Result::Err(_) => println!("Could not find discrete logarithm.")
    }
}
