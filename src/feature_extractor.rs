// src/feature_extractor.rs

use syn::{parse_file, visit::Visit, ItemFn};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn count_functions(file_path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut src = String::new();
    file.read_to_string(&mut src)?;

    let ast = parse_file(&src)?;

    let mut visitor = FunctionVisitor { function_count: 0 };
    visitor.visit_file(&ast);

    Ok(visitor.function_count)
}

struct FunctionVisitor {
    function_count: usize,
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_fn(&mut self, _i: &'ast ItemFn) {
        self.function_count += 1;
    }
}