// dataset/safe/safe_snippet_5.rs
// Safe Command Execution
use std::process::Command;

fn safe_command_exec(filename: &str) -> Result<String, String> {
    let output = 
Command::new("ls").arg("-l").arg(filename).output().map_err(|e| 
e.to_string())?;
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
