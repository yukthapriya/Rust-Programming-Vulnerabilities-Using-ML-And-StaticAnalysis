// dataset/unsafe/unsafe_snippet_5.rs
// Unsafe Command Execution (potential command injection)
use std::process::Command;

fn unsafe_command_exec(command: &str) -> Result<String, String> {
    let output = Command::new(command).output().map_err(|e| 
e.to_string())?;
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}

