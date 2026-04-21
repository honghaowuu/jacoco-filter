# Design: `--top-k` Parameter for jacoco-filter

**Date:** 2026-04-21

## Problem

When test coverage is very low, jacoco-filter can output hundreds of methods. This makes the JSON output too large for practical consumption by Claude Code or other tools.

## Solution

Add a `--top-k` CLI parameter (default: 5) that limits output to the top-k highest-scoring methods. Since output is already sorted by score descending, this is a simple truncation after scoring. `--top-k 0` disables the limit and returns all results.

## CLI Interface

```bash
jacoco-filter <input_file> [--top-k <n>] [--min-score <f>] [--pretty] [--summary] [--output <path>]
```

- `--top-k 5` (default) — return top 5 methods by score
- `--top-k 0` — return all methods (no limit)

## Design

**Where:** `src/main.rs` only. `lib.rs` is unchanged.

**Why main.rs:** Truncation is a presentation/output concern, not a scoring concern. All other output decisions (`--pretty`, `--output`) already live in `main.rs`.

**Changes:**

1. Add field to `Cli` struct:
   ```rust
   #[arg(long, default_value = "5")]
   top_k: usize,
   ```

2. After `process()`, before serializing:
   ```rust
   if cli.top_k > 0 { methods.truncate(cli.top_k); }
   ```

3. After `process_with_summary()`, before serializing:
   ```rust
   if cli.top_k > 0 { report.methods.truncate(cli.top_k); }
   ```

## Scope

- No changes to `lib.rs`, `filter.rs`, `scorer.rs`, `parser.rs`, or `model.rs`
- No new dependencies
- Existing tests unaffected; add integration test cases for `--top-k`
