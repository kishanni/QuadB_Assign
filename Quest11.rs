use std::io;

fn merging_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);

    merged
}

fn main() {
    
    println!("Enter the first sorted array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr1: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Please enter valid integers")).collect();

    println!("Enter the second sorted array of integers (space-separated):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr2: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Please enter valid integers")).collect();

    let merged = merging_arrays(&arr1, &arr2);

    println!("Merged sorted array: {:?}", merged);
}
