use std::io;

fn KthSmallest(arr: &mut [i32], k: usize) -> Option<i32> {
    arr.sort();
    arr.get(k - 1).copied()
}

fn main() {
    
    println!("Enter an array of integers with space seperation:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

   
    let mut arr: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Please enter valid integers")).collect();

    
    println!("Enter the value of k:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Please enter a valid integer");

   
    match KthSmallest(&mut arr, k) {
        Some(value) => println!("The {}-th smallest element is {}", k, value),
        None => println!("The value of k is out of bounds."),
    }
}
