fn safe_array_access(arr: &[i32], index: usize) -> Option<i32> {
    arr.get(index).copied()
}
