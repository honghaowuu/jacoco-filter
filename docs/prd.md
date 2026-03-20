# PRD: JaCoCo XML Filter Tool for Claude Code

## 1. Overview

The **JaCoCo XML Filter Tool** is a cross-platform CLI tool written in Rust that parses JaCoCo XML coverage reports, filters out low-value or fully-covered methods, calculates a priority score based on complexity and uncovered lines, and outputs a concise JSON report. This output will be consumed by **Claude Code** to guide automated testing and test generation.

---

## 2. Goals

1. Reduce token usage for Claude Code by filtering irrelevant coverage data.
2. Highlight methods with **partial coverage** and **high complexity**.
3. Provide line-level detail to allow precise test generation.
4. Be fully cross-platform (Windows, Mac, Linux) as a standalone Rust binary.

---

## 3. Inputs

* **Input file:** JaCoCo XML report (`jacoco.xml`) generated from `mvn test jacoco:report`
* **XML structure:**

  * `<class>` → contains `name` and `sourcefilename`
  * `<method>` → contains `name`, `<counter>` elements, optional `<line>` elements
  * `<line>` → attributes `nr`, `mi`, `ci`, `mb`, `cb`
* **XML EXAMPLE:**
```
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<!DOCTYPE report PUBLIC "-//JACOCO//DTD Report 1.1//EN" "report.dtd">
<report name="billing">
    <sessioninfo id="DESKTOP-THCQOVB-e5b134dd" start="1773815425198" dump="1773815434070"/>
    <sessioninfo id="DESKTOP-THCQOVB-20e8c752" start="1773815547985" dump="1773815557284"/>
    <package name="com/newland/modules/billing/domain/business/service/resource/level">
        <class name="com/newland/modules/billing/domain/business/service/resource/level/LevelResource" sourcefilename="LevelResource.java">
            <method name="&lt;init&gt;" desc="(Lcom/newland/modules/billing/domain/business/service/deduction/IDeductionRule;)V" line="21">
                <counter type="INSTRUCTION" missed="0" covered="6"/>
                <counter type="LINE" missed="0" covered="3"/>
                <counter type="COMPLEXITY" missed="0" covered="1"/>
                <counter type="METHOD" missed="0" covered="1"/>
            </method>
            <method name="getDeductionRule" desc="()Lcom/newland/modules/billing/domain/business/service/deduction/IDeductionRule;" line="27">
                <counter type="INSTRUCTION" missed="3" covered="0"/>
                <counter type="LINE" missed="1" covered="0"/>
                <counter type="COMPLEXITY" missed="1" covered="0"/>
                <counter type="METHOD" missed="1" covered="0"/>
            </method>
            <method name="resourceType" desc="()Ljava/lang/String;" line="32">
                <counter type="INSTRUCTION" missed="2" covered="0"/>
                <counter type="LINE" missed="1" covered="0"/>
                <counter type="COMPLEXITY" missed="1" covered="0"/>
                <counter type="METHOD" missed="1" covered="0"/>
            </method>
            <method name="startUseResourceActually" desc="(Lcom/newland/modules/billing/domain/business/service/biz/dto/BusinessOwner;)V" line="38">
                <counter type="INSTRUCTION" missed="31" covered="0"/>
                <counter type="LINE" missed="11" covered="0"/>
                <counter type="COMPLEXITY" missed="1" covered="0"/>
                <counter type="METHOD" missed="1" covered="0"/>
            </method>
            <method name="finishUseResource" desc="(Lcom/newland/modules/billing/domain/business/service/biz/dto/BusinessOwner;Lcom/newland/modules/billing/domain/business/model/SubjectBizConsumptionDetail;)V" line="53">
                <counter type="INSTRUCTION" missed="31" covered="0"/>
                <counter type="LINE" missed="11" covered="0"/>
                <counter type="COMPLEXITY" missed="1" covered="0"/>
                <counter type="METHOD" missed="1" covered="0"/>
            </method>
            <counter type="INSTRUCTION" missed="67" covered="6"/>
            <counter type="LINE" missed="24" covered="3"/>
            <counter type="COMPLEXITY" missed="4" covered="1"/>
            <counter type="METHOD" missed="4" covered="1"/>
            <counter type="CLASS" missed="0" covered="1"/>
        </class>
        <sourcefile name="LevelResource.java">
            <line nr="21" mi="0" ci="2" mb="0" cb="0"/>
            <line nr="22" mi="0" ci="3" mb="0" cb="0"/>
            <line nr="23" mi="0" ci="1" mb="0" cb="0"/>
            <line nr="27" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="32" mi="2" ci="0" mb="0" cb="0"/>
            <line nr="38" mi="2" ci="0" mb="0" cb="0"/>
            <line nr="39" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="40" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="41" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="42" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="43" mi="2" ci="0" mb="0" cb="0"/>
            <line nr="44" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="45" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="46" mi="4" ci="0" mb="0" cb="0"/>
            <line nr="48" mi="4" ci="0" mb="0" cb="0"/>
            <line nr="49" mi="1" ci="0" mb="0" cb="0"/>
            <line nr="53" mi="2" ci="0" mb="0" cb="0"/>
            <line nr="54" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="55" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="56" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="57" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="58" mi="2" ci="0" mb="0" cb="0"/>
            <line nr="59" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="60" mi="3" ci="0" mb="0" cb="0"/>
            <line nr="61" mi="4" ci="0" mb="0" cb="0"/>
            <line nr="63" mi="4" ci="0" mb="0" cb="0"/>
            <line nr="64" mi="1" ci="0" mb="0" cb="0"/>
            <counter type="INSTRUCTION" missed="67" covered="6"/>
            <counter type="LINE" missed="24" covered="3"/>
            <counter type="COMPLEXITY" missed="4" covered="1"/>
            <counter type="METHOD" missed="4" covered="1"/>
            <counter type="CLASS" missed="0" covered="1"/>
        </sourcefile>
        <counter type="INSTRUCTION" missed="67" covered="6"/>
        <counter type="LINE" missed="24" covered="3"/>
        <counter type="COMPLEXITY" missed="4" covered="1"/>
        <counter type="METHOD" missed="4" covered="1"/>
        <counter type="CLASS" missed="0" covered="1"/>
    </package>
</report>
```

---

## 4. Filtering Rules

1. **Skip trivial methods**:

   * Constructors (`<init>`)
   * Getters (`get*`)
   * Setters (`set*`)

2. **Skip fully-covered methods**:

   * Methods where `line coverage = 100%` (no missed lines)

3. **Score calculation**:

   ```
   score = complexity * (missed lines / total lines)
   ```

   * Complexity derived from `<counter type="COMPLEXITY">`
   * Missed lines = number of lines where `mi > 0` or `mb > 0`

---

## 5. Output

* **Format:** JSON
* **Per method entry:**

```json
{
  "class": "com.newland.modules.billing.LevelResource",
  "source_file": "LevelResource.java",
  "method": "doSomething",
  "score": 2.5,
  "missed_lines": [22, 25, 28]
}
```

* **All methods** in an array:

```json
[
  {
    "class": "com.newland.modules.billing.LevelResource",
    "source_file": "LevelResource.java",
    "method": "doSomething",
    "score": 2.5,
    "missed_lines": [22, 25, 28]
  },
  {
    "class": "com.newland.modules.billing.DeductionService",
    "source_file": "DeductionService.java",
    "method": "calculate",
    "score": 4.0,
    "missed_lines": [50, 51, 54]
  }
]
```

* **Optional:** can include CLI flag to output **pretty JSON** vs compact JSON.

---

## 6. CLI Interface

```bash
jacoco-filter <input_file> [--output <output_file>] [--min-score <float>] [--pretty]
```

* `input_file` → required JaCoCo XML path
* `--output` → optional JSON output path (stdout if omitted)
* `--min-score` → filter out methods below threshold (default: 0.0)
* `--pretty` → pretty-print JSON

---

## 7. Functional Requirements

1. **Parse JaCoCo XML** reliably across platforms.
2. **Identify trivial methods** and skip them.
3. **Collect missed lines** (`mi > 0` or `mb > 0`) per method.
4. **Compute score** per method.
5. **Output JSON** array containing filtered methods with score and missed lines.
6. **Cross-platform Rust binary**: single executable, no external dependencies.
7. **Optional flags** for filtering and output formatting.

---

## 8. Non-functional Requirements

* **Performance:** Should process large multi-module JaCoCo XML reports within seconds.
* **Reliability:** Fail gracefully if XML is malformed or missing.
* **Cross-platform:** Works on Windows, Mac, Linux.
* **Token-efficient:** Only output relevant fields (`class`, `source_file`, `method`, `score`, `missed_lines`).

---

## 9. Example Workflow

```bash
# Generate XML coverage
mvn clean test jacoco:report

# Run filter tool
jacoco-filter target/site/jacoco/jacoco.xml --output coverage-summary.json --min-score 0.1 --pretty

# Feed output to Claude Code
claude-code analyze --input coverage-summary.json
```

---

## 10. Future Enhancements

1. Aggregate multi-module reports.
2. Include branch coverage weighting in score calculation.
3. Support additional output formats (CSV, Markdown).
4. Provide optional **interactive CLI** to explore coverage summary.

