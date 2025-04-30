// use std::fs;

// pub fn analyze_file(file_path: &str) -> (i32, i32, i32, String) {
//     let content = fs::read_to_string(file_path).expect("Unable to read 
// file");

//     // Static analysis checks
//     let unsafe_block = if content.contains("unsafe") { 1 } else { 0 };
//     let path_traversal = if content.contains("File::open") { 1 } else { 0 
// };
//     let command_injection = if content.contains("Command::new") { 1 } else 
// { 0 };

//     // Get the file's label (safe or unsafe based on the directory)
//     let label = if file_path.contains("unsafe") {
//         "unsafe".to_string()
//     } else {
//         "safe".to_string()
//     };

//     // Print detected vulnerabilities
//     if unsafe_block == 1 {
//         println!("Unsafe block found in {}", file_path);
//     }
//     if path_traversal == 1 {
//         println!("Potential path traversal in {}", file_path);
//     }
//     if command_injection == 1 {
//         println!("Potential command injection in {}", file_path);
//     }

//     (unsafe_block, path_traversal, command_injection, label)
// }

// pub fn analyze_files_in_directory(directory_path: &str) -> Vec<(i32, i32, 
// i32, String)> {
//     let paths = fs::read_dir(directory_path).expect("Unable to read 
// directory");
//     let mut features = Vec::new();

//     for path in paths {
//         let path = path.expect("Unable to read path").path();
//         if path.extension().and_then(|e| e.to_str()) == Some("rs") {
//             println!("Analyzing file: {:?}", path);
//             let file_features = analyze_file(path.to_str().unwrap());
//             features.push(file_features);
//         }
//     }

//     features
// }




use std::fs;

pub fn analyze_file(file_path: &str) -> (f64, f64, f64, String) {
    let content = fs::read_to_string(file_path).expect("Unable to read file");

    // Static analysis checks (converted to f64)
    let unsafe_block = if content.contains("unsafe") { 1.0 } else { 0.0 };
    let path_traversal = if content.contains("File::open") { 1.0 } else { 0.0 };
    let command_injection = if content.contains("Command::new") { 1.0 } else { 0.0 };

    // Get the file's label (safe or unsafe based on the directory)
    let label = if file_path.contains("unsafe") {
        "unsafe".to_string()
    } else {
        "safe".to_string()
    };

    // Print detected vulnerabilities
    if unsafe_block == 1.0 {
        println!("Unsafe block found in {}", file_path);
    }
    if path_traversal == 1.0 {
        println!("Potential path traversal in {}", file_path);
    }
    if command_injection == 1.0 {
        println!("Potential command injection in {}", file_path);
    }

    (unsafe_block, path_traversal, command_injection, label)
}

pub fn analyze_files_in_directory(directory_path: &str) -> Vec<(f64, f64, f64, String)> {
    let paths = fs::read_dir(directory_path).expect("Unable to read directory");
    let mut features = Vec::new();

    for path in paths {
        let path = path.expect("Unable to read path").path();
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            println!("Analyzing file: {:?}", path);
            let file_features = analyze_file(path.to_str().unwrap());
            features.push(file_features);
        }
    }

    features
}

pub fn analyze_code_snippet(snippet: &str) -> (f64, f64, f64) {
    // Static analysis checks for a single code snippet
    let unsafe_block = if snippet.contains("unsafe") { 1.0 } else { 0.0 };
    let path_traversal = if snippet.contains("File::open") { 1.0 } else { 0.0 };
    let command_injection = if snippet.contains("Command::new") { 1.0 } else { 0.0 };

    // Print detected vulnerabilities
    if unsafe_block == 1.0 {
        println!("Unsafe block found in snippet");
    }
    if path_traversal == 1.0 {
        println!("Potential path traversal in snippet");
    }
    if command_injection == 1.0 {
        println!("Potential command injection in snippet");
    }

    (unsafe_block, path_traversal, command_injection)
}