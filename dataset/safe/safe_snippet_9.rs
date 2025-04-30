// dataset/safe/safe_snippet_9.rs
fn main() {
    unsafe {
        let ptr = std::alloc::alloc(std::alloc::Layout::new::<i32>()) as 
*mut i32;
        *ptr = 42;
        std::alloc::dealloc(ptr as *mut u8, 
std::alloc::Layout::new::<i32>());
    }
}
