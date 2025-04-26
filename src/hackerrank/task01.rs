//Simple Array Sum

use std::io;

fn simple_array_sum(ar: Vec<i32>) -> i32 {
    ar.iter().sum()
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let _n: usize = input.trim().parse().expect("Expected a number");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let ar: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected an integer"))
        .collect();

    let result = simple_array_sum(ar);
    println!("{}", result);
}

