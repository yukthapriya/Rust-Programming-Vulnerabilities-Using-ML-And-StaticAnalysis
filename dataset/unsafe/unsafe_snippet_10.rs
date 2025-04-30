// dataset/unsafe/unsafe_snippet_10.rs
fn main() {
    let format_string = "%s%s%s"; // Simulated format string vulnerability
    unsafe {
        println!(format_string); // Potentially dangerous if format_string 
is user-controlled
    }
}
