use std::io;

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(num);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    println!("Enter an array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let arr: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Please enter valid integers")).collect();

    let max_sum = max_subarray_sum(&arr);
    println!("The maximum subarray sum is {}", max_sum);
}
