use std::io;

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        return 0.0; // or handle empty array case as needed
    }

    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
   
    println!("Enter a sorted array of integers with space seperation");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

   
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter integers"))
        .collect();

   
    let med = median(&arr);
    println!("The median of the sorted array is {}", med);
}
