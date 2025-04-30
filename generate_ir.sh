#!/bin/bash

echo "Generating LLVM IR for Safe and Unsafe Rust files..."

# Create output directory
mkdir -p ir

# Process safe Rust code
for file in dataset/safe/*.rs; do
    base=$(basename "$file" .rs)
    echo "Generating IR for safe code: $base.rs"
    rustc --emit=llvm-ir "$file" -o "ir/${base}_safe.ll"
done

# Process unsafe Rust code
for file in dataset/unsafe/*.rs; do
    base=$(basename "$file" .rs)
    echo "Generating IR for unsafe code: $base.rs"
    rustc --emit=llvm-ir "$file" -o "ir/${base}_unsafe.ll"
done

echo "✅ LLVM IR generation completed. Check the 'ir/' directory."

