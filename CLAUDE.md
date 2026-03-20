# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**jacoco-filter** is a cross-platform Rust CLI tool that parses JaCoCo XML coverage reports, filters trivial/fully-covered methods, scores remaining methods by complexity × coverage gap, and outputs compact JSON for consumption by Claude Code to guide test generation.

## Build & Run Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release binary in target/release/
cargo run -- <input.xml>       # Run with args
cargo test                     # Run all tests
cargo test <test_name>         # Run a single test
cargo test -- --nocapture      # Show stdout during tests
cargo clippy                   # Lint
cargo fmt                      # Format code
```

## Architecture

This is a greenfield project. The PRD (`docs/prd.md`) is the authoritative spec.

### Recommended module layout

```
src/
├── main.rs      # CLI entrypoint (clap argument parsing)
├── lib.rs       # Public API surface
├── parser.rs    # JaCoCo XML → internal model (quick-xml crate)
├── filter.rs    # Filtering rules (skip trivial methods, skip 100%-covered)
├── scorer.rs    # score = complexity × (missed_lines / total_lines)
└── model.rs     # Data structs: Method, Class, CoverageReport, FilteredResult
```

### Key data flow

1. Parse `jacoco.xml` → `CoverageReport` (classes → methods → line-level counters)
2. Filter: skip constructors (`<init>`), getters (`get*`), setters (`set*`), fully-covered methods
3. Score each method: `complexity * (missed_lines / total_lines)`
   - Complexity from `<counter type="COMPLEXITY">`
   - Missed lines = lines where `mi > 0` or `mb > 0`
4. Apply `--min-score` threshold
5. Serialize to JSON array (compact by default, pretty with `--pretty`)

### XML structure to parse

- `<class name="..." sourcefilename="...">` — convert `/` to `.` for class name
- `<method name="..." desc="..." line="...">` — nested under class
- `<counter type="LINE|COMPLEXITY" missed="..." covered="...">` — nested under method
- `<line nr="..." mi="..." ci="..." mb="..." cb="...">` — nested under `<sourcefile>` (not `<method>`)

**Important:** Line-level detail (`<line>` elements) is under `<sourcefile>`, not under `<method>`. You must correlate `<sourcefile>` lines with method line ranges to assign missed lines to the correct method.

### Output JSON shape

```json
[
  {
    "class": "com.example.ClassName",
    "source_file": "ClassName.java",
    "method": "methodName",
    "score": 2.5,
    "missed_lines": [22, 25, 28]
  }
]
```

## Suggested Dependencies (Cargo.toml)

- `quick-xml` — fast XML streaming parser
- `serde` + `serde_json` — JSON serialization
- `clap` — CLI argument parsing (derive feature)

## CLI Interface

```bash
jacoco-filter <input_file> [--output <path>] [--min-score <float>] [--pretty]
```
