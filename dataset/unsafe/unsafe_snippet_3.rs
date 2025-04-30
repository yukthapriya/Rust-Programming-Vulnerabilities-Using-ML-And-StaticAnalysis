// dataset/unsafe/unsafe_snippet_3.rs
// Unsafe File I/O (potential path traversal)
use std::fs::File;
use std::path::Path;

fn unsafe_file_read(filename: &str) -> Result<String, String> {
    let path = Path::new(filename);
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).map_err(|e| 
e.to_string())?;

    Ok(content)
}

