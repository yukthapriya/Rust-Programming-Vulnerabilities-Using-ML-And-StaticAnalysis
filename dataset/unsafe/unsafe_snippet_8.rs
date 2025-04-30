// dataset/unsafe/unsafe_snippet_8.rs
fn main() {
    let mut x = Box::new(10);
    let ptr = &*x;
    drop(x);
    unsafe {
        println!("{}", *ptr); // Use-after-free
    }
}
