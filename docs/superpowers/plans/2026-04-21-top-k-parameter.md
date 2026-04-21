# top-k Parameter Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `--top-k` CLI flag (default 5, 0=unlimited) that truncates output to the top-k highest-scoring methods.

**Architecture:** Single-file change in `src/main.rs` — add a `top_k: usize` field to `Cli`, then truncate the methods vec in both code paths (with and without `--summary`) before serializing to JSON. No changes to `lib.rs` or any other file.

**Tech Stack:** Rust, clap ~4.4 (derive), serde_json

---

## File Map

| File | Change |
|------|--------|
| `src/main.rs` | Add `top_k` field to `Cli`; truncate in both branches |
| `tests/integration_test.rs` | Add 4 CLI-level tests using `std::process::Command` |

---

### Task 1: Write failing CLI tests for `--top-k`

**Files:**
- Modify: `tests/integration_test.rs`

The existing tests call `jacoco_filter::process()` directly. For `--top-k` (a `main.rs`-only flag), use `std::process::Command` with `env!("CARGO_BIN_EXE_jacoco-filter")` to invoke the compiled binary. This macro expands to the binary path at compile time — no extra dependencies needed.

The sample XML has exactly 3 non-trivial methods after filtering (verified by `test_method_count`).

- [ ] **Step 1: Add the following tests to the bottom of `tests/integration_test.rs`**

```rust
use std::process::Command;

fn sample_xml_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sample.xml")
}

#[test]
fn test_top_k_limits_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_jacoco-filter"))
        .args([sample_xml_path().to_str().unwrap(), "--top-k", "2"])
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json.as_array().unwrap().len(), 2);
}

#[test]
fn test_top_k_zero_returns_all() {
    let output = Command::new(env!("CARGO_BIN_EXE_jacoco-filter"))
        .args([sample_xml_path().to_str().unwrap(), "--top-k", "0"])
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json.as_array().unwrap().len(), 3);
}

#[test]
fn test_top_k_larger_than_results_returns_all() {
    let output = Command::new(env!("CARGO_BIN_EXE_jacoco-filter"))
        .args([sample_xml_path().to_str().unwrap(), "--top-k", "100"])
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json.as_array().unwrap().len(), 3);
}

#[test]
fn test_top_k_with_summary() {
    let output = Command::new(env!("CARGO_BIN_EXE_jacoco-filter"))
        .args([sample_xml_path().to_str().unwrap(), "--top-k", "1", "--summary"])
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid JSON");
    assert_eq!(json["methods"].as_array().unwrap().len(), 1);
}
```

Note: `serde_json` is already a dev-dependency via `Cargo.toml` (used transitively). If the compiler complains, add `serde_json = "1"` under `[dev-dependencies]` in `Cargo.toml`.

- [ ] **Step 2: Build the binary first, then run the new tests to confirm they fail**

```bash
cargo build && cargo test test_top_k -- --nocapture
```

Expected: 4 test failures mentioning unknown argument `--top-k` or wrong array lengths. (The binary doesn't have the flag yet, so `output.status.success()` will be false and the assert will panic.)

---

### Task 2: Implement `--top-k` in `main.rs`

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Add `top_k` field to the `Cli` struct**

In `src/main.rs`, after the `summary` field (line 26), add:

```rust
    /// Limit output to the top-k highest-scoring methods (0 = no limit)
    #[arg(long, default_value = "5")]
    top_k: usize,
```

The full `Cli` struct should now look like:

```rust
#[derive(Parser)]
#[command(name = "jacoco-filter", about = "Filter and score JaCoCo XML coverage reports")]
struct Cli {
    /// JaCoCo XML input file
    input_file: String,

    /// Output JSON file (defaults to stdout)
    #[arg(long)]
    output: Option<String>,

    /// Minimum score threshold (methods below this are excluded)
    #[arg(long, default_value = "0.0")]
    min_score: f64,

    /// Pretty-print JSON output
    #[arg(long)]
    pretty: bool,

    /// Include a line-coverage summary alongside the filtered methods
    #[arg(long)]
    summary: bool,

    /// Limit output to the top-k highest-scoring methods (0 = no limit)
    #[arg(long, default_value = "5")]
    top_k: usize,
}
```

- [ ] **Step 2: Add truncation in the `--summary` branch**

In `main.rs`, locate the `--summary` branch (around line 41). After the `Ok(r)` match that binds `report`, add:

```rust
if cli.top_k > 0 {
    report.methods.truncate(cli.top_k);
}
```

The branch should look like:

```rust
let json = if cli.summary {
    let mut report = match jacoco_filter::process_with_summary(&xml, cli.min_score) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error processing XML: {}", e);
            std::process::exit(1);
        }
    };
    if cli.top_k > 0 {
        report.methods.truncate(cli.top_k);
    }
    if cli.pretty {
        serde_json::to_string_pretty(&report)
    } else {
        serde_json::to_string(&report)
    }
```

Note: `report` must be declared `mut` to allow truncation.

- [ ] **Step 3: Add truncation in the non-summary branch**

In `main.rs`, locate the `else` branch (around line 54). After the `Ok(m)` match that binds `methods`, add:

```rust
if cli.top_k > 0 {
    methods.truncate(cli.top_k);
}
```

The branch should look like:

```rust
} else {
    let mut methods = match jacoco_filter::process(&xml, cli.min_score) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error processing XML: {}", e);
            std::process::exit(1);
        }
    };
    if cli.top_k > 0 {
        methods.truncate(cli.top_k);
    }
    if cli.pretty {
        serde_json::to_string_pretty(&methods)
    } else {
        serde_json::to_string(&methods)
    }
};
```

Note: `methods` must be declared `mut`.

---

### Task 3: Verify and commit

- [ ] **Step 1: Run the full test suite**

```bash
cargo test -- --nocapture
```

Expected: all tests pass, including the 4 new `test_top_k_*` tests.

- [ ] **Step 2: Run clippy**

```bash
cargo clippy
```

Expected: no warnings.

- [ ] **Step 3: Smoke-test the default behavior**

```bash
cargo build && ./target/debug/jacoco-filter tests/fixtures/sample.xml --pretty
```

Expected: JSON array with at most 5 entries (sample has 3, so all 3 appear).

- [ ] **Step 4: Commit**

```bash
git add src/main.rs tests/integration_test.rs
git commit -m "feat: add --top-k flag to limit output (default 5, 0=unlimited)"
```
