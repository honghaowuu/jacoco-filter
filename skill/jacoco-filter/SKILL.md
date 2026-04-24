---
name: jacoco-filter
version: 1.0.0
description: |
  Guide for using the jacoco-filter tool to analyze JaCoCo coverage reports and
  identify methods needing tests. Use when the user asks to "analyze coverage",
  "find uncovered methods", "improve test coverage", "check coverage gaps",
  "generate tests from coverage report", or mentions a jacoco.xml file.
  CRITICAL: Never read jacoco.xml directly — always run jacoco-filter instead.
---

# jacoco-filter Skill

## Critical Rule

**NEVER read `jacoco.xml` directly.** The raw XML is thousands of lines of noise. Always run `jacoco-filter` to get a compact, scored JSON summary of exactly which methods need tests.

---

## What jacoco-filter Does

`jacoco-filter` parses a JaCoCo XML coverage report and outputs either:

- A **prioritized JSON array** of methods that need tests (default), or
- A **full report** with a line-coverage summary + methods (with `--summary`)

It:

1. Skips trivial methods (constructors, getters, setters, fully-covered methods)
2. Scores each remaining method: `complexity × (missed_lines / total_lines)`
3. Returns only methods above a configurable score threshold, sorted highest-first

---

## Locate the coverage report

JaCoCo XML is typically generated after `mvn test` or `gradle test`:

```bash
# Maven
find . -name "jacoco.xml" -path "*/site/jacoco/*"

# Gradle
find . -name "jacoco.xml" -path "*/reports/jacoco/*"
```

---

## Run jacoco-filter

```bash
# Basic (methods array only, top 5 by default)
jacoco-filter path/to/jacoco.xml

# With line-coverage summary (overall % + per-class breakdown)
jacoco-filter path/to/jacoco.xml --summary

# Filter to high-priority methods only
jacoco-filter path/to/jacoco.xml --min-score 2.0

# Show more results (or all with 0)
jacoco-filter path/to/jacoco.xml --top-k 10
jacoco-filter path/to/jacoco.xml --top-k 0

# Save output
jacoco-filter path/to/jacoco.xml --summary --output gaps.json
```

---

## Output Format

**Default** — plain array, sorted by score descending:

```json
[
  {
    "class": "com.example.ClassName",
    "source_file": "ClassName.java",
    "method": "methodName",
    "score": 4.5,
    "missed_lines": [42, 45, 51]
  }
]
```

**With `--summary`** — object with coverage summary + methods:

```json
{
  "summary": {
    "line_coverage_pct": 72.4,
    "lines_covered": 842,
    "lines_missed": 321
  },
  "methods": [ ... ]
}
```

- **score** — `complexity × (missed_lines / total_lines)`. Higher = write tests here first.
- **missed_lines** — exact line numbers with no coverage. Target these in your tests.

---

## Workflow: Coverage → Tests

### Step 1: Generate coverage

```bash
mvn test        # or: gradle test
```

### Step 2: Check overall coverage and get gaps

```bash
jacoco-filter target/site/jacoco/jacoco.xml --summary --min-score 1.0
```

### Step 3: Find and read the source

Use `source_file` from the output to locate the Java file:

```bash
find . -name "ClassName.java" -not -path "*/target/*" -not -path "*/build/*"
```

Read the method, focusing on the `missed_lines` line numbers.

### Step 4: Write tests

Write tests that exercise the missed lines. Work top-down by score.

### Step 5: Verify

```bash
mvn test
jacoco-filter target/site/jacoco/jacoco.xml --summary --min-score 1.0
```

Confirm overall `line_coverage_pct` increased and scores dropped for covered methods.

---

## Process output with jq

### Check overall line coverage (one number)

```bash
# Overall line coverage percentage — use this to check if the target is met
jacoco-filter jacoco.xml --summary | jq '.summary.line_coverage_pct'

# Check against a threshold (exits 0 if met, 1 if not)
jacoco-filter jacoco.xml --summary | jq 'if .summary.line_coverage_pct >= 80 then "PASS" else "FAIL" end'
```

### Work with the methods array

```bash
# Top 10 by score (pass --top-k 10, or use --top-k 0 and slice with jq)
jacoco-filter jacoco.xml --top-k 10
jacoco-filter jacoco.xml --top-k 0 | jq '.[0:10]'

# Top 10 by score from summary output
jacoco-filter jacoco.xml --summary --top-k 10
jacoco-filter jacoco.xml --summary --top-k 0 | jq '.methods[0:10]'

# Methods in a specific class
jacoco-filter jacoco.xml | jq '[.[] | select(.class | contains("PaymentService"))]'

# Just class, method, score
jacoco-filter jacoco.xml | jq '[.[] | {class, method, score}]'
```
