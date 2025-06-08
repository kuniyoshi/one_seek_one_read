# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Building and Testing
- **Build**: `cargo build`
- **Build release**: `cargo build --release` or `make release`
- **Run tests**: `make test` (runs tests sequentially to avoid race conditions)
- **Run benchmarks**: `make bench` (requires nightly Rust)
- **Run main binary**: `cargo run --bin main -- <mode> <count> <seq_type> <optimization>`
  - Example: `make run MODE=archive COUNT=100 SEQ_TYPE=random OPTIMIZATION=true`

### Data Preparation
The project requires test data to be generated before running:
1. **Create resource list**: `make list_resource` (processes access.log)
2. **Generate dummy files**: `make create_dummy` (creates test files)
3. **Build archive**: `make archive` (creates single archive file)
4. **Generate test data**: `make test_data` (runs test.pl script)

### Clean Up
- **Clean all**: `make clean` (removes resource directory and cargo artifacts)

## Architecture Overview

This project benchmarks different file reading strategies to compare performance between consolidated archive reading vs traditional file-by-file reading.

### Core Components

1. **Reading Strategies**:
   - **Archive** (`src/archive.rs`): Implements "one seek, one read" strategy using a single pre-built archive file containing all resources. Supports seek optimization for sequential access.
   - **Normal** (`src/normal.rs`): Traditional file reading, opening each file individually. Has two modes: standard (multiple reads) and one-read (single read with pre-allocated buffer).

2. **Data Structures**:
   - **Index** (`src/index.rs`): Manages file metadata (path, size, hash) and archive offsets. Reads TSV index files and creates path-to-index mappings.
   - **Utilities** (`src/util.rs`): Provides SHA1 hashing for verification and iterators for random/sequential access patterns.

3. **Performance Optimizations**:
   - Pre-allocated buffers using unsafe Rust for minimal overhead
   - Sequential read optimization to skip unnecessary seeks
   - Single file handle for archive strategy vs multiple handles for normal strategy

### Key Design Principles

- Files are accessed by index rather than path for efficiency
- All strategies verify data integrity using SHA1 hashes
- Deterministic random access patterns for reproducible benchmarks
- The archive approach significantly reduces system calls (3 vs 148+ open calls)

### Typical Workflow

1. Process access logs to identify resources
2. Generate dummy test files based on resource patterns
3. Build a consolidated archive from individual files
4. Run benchmarks comparing different reading strategies
5. Analyze system call counts and execution times