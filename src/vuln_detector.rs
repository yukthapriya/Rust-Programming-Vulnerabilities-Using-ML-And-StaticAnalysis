use std::fs::File;
use std::io::{self, Read, Write};
use regex::Regex;
use std::process::Command;

pub fn detect_vulnerabilities(file_path: &str) -> io::Result<()> {
    // Open the file
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Regex pattern to match 'unsafe' keyword in the code
    let unsafe_pattern = Regex::new(r"unsafe").unwrap();
    
    // Search for the pattern in the content
    if unsafe_pattern.is_match(&content) {
        println!("Warning: Unsafe code detected in file: {}", file_path);
       // Generate LLVM IR for the unsafe code
        let llvm_ir_path = file_path.replace(".rs", ".ll");
        let status = Command::new("rustc")
            .arg("--emit=llvm-ir")
            .arg(file_path)
            .arg("-o")
            .arg(&llvm_ir_path)
            .status()?;
        
        if status.success() {
            println!("LLVM IR generated at: {}", llvm_ir_path);
        } else {
            eprintln!("Failed to generate LLVM IR for file: {}", 
file_path);
        }    
   } else {
        println!("No unsafe code detected in file: {}", file_path);
    }

    Ok(())
}

