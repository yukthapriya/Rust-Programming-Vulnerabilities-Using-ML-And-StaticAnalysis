// dataset/unsafe/unsafe_snippet_11.rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let data_clone = data.clone();

    let handle = thread::spawn(move || {
        let mut guard = data_clone.lock().unwrap();
        *guard += 1;
    });

    let mut guard = data.lock().unwrap();
    *guard += 1;

    handle.join().unwrap();
}
