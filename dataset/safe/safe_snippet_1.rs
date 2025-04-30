fn safe_add(a: i32, b: i32) -> i32 {
    a.checked_add(b).unwrap_or(0)
}
