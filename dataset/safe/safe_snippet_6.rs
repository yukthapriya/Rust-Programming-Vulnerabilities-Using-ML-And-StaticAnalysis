// dataset/safe/safe_snippet_6.rs
// Safe Integer handling
fn safe_integer_mult(a:i32, b:i32)-> Option<i32>{
    a.checked_mul(b)
}
