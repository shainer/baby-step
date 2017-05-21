use std::collections::HashMap;

fn main() {
    let p = 31;
    let g = 3;
    let h = 6;

    // TODO: verify the parameters and print them
    println!("My generator is {}", g);

    let m = (p as f64).sqrt().ceil() as i32;
    let mut precomp = HashMap::new();

    for j in 0..m {
        precomp.insert(j, g.pow(j))
    }

    let y = g.pow(-m);
}
