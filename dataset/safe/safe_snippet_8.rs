// dataset/safe/safe_snippet_8.rs
fn main() {
    let x = Box::new(10);
    let ptr = &*x;
    println!("{}", *ptr);
}
