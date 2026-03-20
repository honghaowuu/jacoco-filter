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

`jacoco-filter` parses a JaCoCo XML coverage report and outputs a prioritized JSON array of methods that need tests. It:

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
# Basic
jacoco-filter path/to/jacoco.xml

# Filter to high-priority methods only
jacoco-filter path/to/jacoco.xml --min-score 2.0

# Save output
jacoco-filter path/to/jacoco.xml --output gaps.json
```

---

## Output Format

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

- **score** — `complexity × (missed_lines / total_lines)`. Higher = write tests here first.
- **missed_lines** — exact line numbers with no coverage. Target these in your tests.
- Sorted descending by score.

---

## Workflow: Coverage → Tests

### Step 1: Generate coverage

```bash
mvn test        # or: gradle test
```

### Step 2: Run jacoco-filter

```bash
jacoco-filter target/site/jacoco/jacoco.xml --min-score 1.0
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
jacoco-filter target/site/jacoco/jacoco.xml --min-score 1.0
```

Confirm scores dropped for the methods you covered.

---

## Process output with jq

```bash
# Top 10 by score
jacoco-filter jacoco.xml | jq 'sort_by(-.score) | .[0:10]'

# Methods in a specific class
jacoco-filter jacoco.xml | jq '[.[] | select(.class | contains("PaymentService"))]'

# Just class, method, score
jacoco-filter jacoco.xml | jq '[.[] | {class, method, score}]'
```
