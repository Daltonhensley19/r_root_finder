use std::io;

fn calc_factors(mut num: i64) -> Vec<f64> {
    let mut factors = Vec::new();

    // If number is negative, make it positive and proceed
    if num < 0 {
        num = -num;
    }

    // Find all of the factors of the number.
    let mut incre = 1;
    while incre != num {
        if num % incre == 0 {
            let factor = num as f64 / incre as f64;
            factors.push(factor);
        }

        incre += 1;
    }

    // Append the factor of 1 at the end
    factors.push(1.0);

    factors
}

fn ra_root(p: i64, q: i64) -> Vec<(f64, f64)> {
    let mut pq_possible = Vec::new();

    let q_factors = calc_factors(q);
    let p_factors = calc_factors(p);

    println!("Factors of p: {:?}", p_factors);
    println!("Factors of q: {:?}", q_factors);

    for i in 0..q_factors.len() {
        for j in 0..p_factors.len() {
            pq_possible.push((
                q_factors[i as usize] / p_factors[j as usize],
                -q_factors[i as usize] / p_factors[j as usize],
            ));
        }
    }

    pq_possible
}

fn main() {
    let input = io::stdin();


    let mut p_string = String::new();
    let mut q_string = String::new();

    println!("Enter value for p: ");
    input.read_line(&mut p_string).unwrap();

    println!("Enter value for q: ");
    input.read_line(&mut q_string).unwrap();
    
    let p: i64 = p_string.trim().parse().expect("Invalid input for p.");
    let q: i64 = q_string.trim().parse().expect("Invaild input for q."); 


    // Get inital possible rational roots
    let mut r_poss = ra_root(p, q);

    // Sort list
    r_poss.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Remove duplicates
    r_poss.dedup_by_key(|k| k.0);

    println!("Possible rational roots are (+q/p, -q/p): {:?}", r_poss);
}
