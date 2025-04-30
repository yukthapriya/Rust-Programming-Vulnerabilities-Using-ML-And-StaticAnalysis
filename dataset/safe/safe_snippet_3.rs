// dataset/safe/safe_snippet_3.rs
// Safe File I/O
use std::fs::File;
use std::path::Path;

fn safe_file_read(filename: &str) -> Result<String, String> {
    let path = Path::new(filename);
    let canonical_path = path.canonicalize().map_err(|e| e.to_string())?;

    let mut file = File::open(canonical_path).map_err(|e| e.to_string())?;
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).map_err(|e| 
e.to_string())?;

    Ok(content)
}

