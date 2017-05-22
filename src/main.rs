use std::collections::HashMap;

fn main() {
    // TODO: input parameters from command line.
    let p : u32 = 31;
    let g : u32 = 3;
    let h = 6;

    // TODO: verify the parameters and print them
    println!("My generator is {}", g);

    let m = (p as f64).sqrt().ceil() as u32;
    println!("M is {}", m);
    let mut precomp = HashMap::new();

    for j in 0..m {
        precomp.insert((g.pow(j) as u32), j);
    }

    // This should be an invmod, g^-m.
    let y : u32 = 2;
    let mut start : u32 = h;
    let mut found = false;

    for i in 0..m {
        if precomp.contains_key(&start) {
            match precomp.get(&start) {
                Some(value) => println!("Found discrete log: {}", (i * m) + value),
                // TODO: raise an error when this happens.
                None => println!("Unrecoverable error!!!")
            }

            found = true;
            break;
        }

        start = start * y % p;
    }

    if !found {
        println!("Error: discrete logarithm not found.")
    }
}
