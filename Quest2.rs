use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let mut input = String::new();

    
    println!("Enter the sorted array of integers with space seperation:");
    io::stdin()
        .read_line(&mut input);
       
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    input.clear();

   
    println!("Enter the target number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let target: i32 = input.trim().parse().expect("Please enter a valid integer");

   
    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} is not in the array", target),
    }
}
