fn unsafe_array_access(arr: &[i32], index: usize) -> i32 {
    arr[index] // Potential out-of-bounds access
}
