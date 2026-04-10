pub mod filter;
pub mod model;
pub mod parser;
pub mod scorer;

use model::{ClassCoverage, CoverageSummary, FilteredMethod, Report};

/// Parse the JaCoCo XML, filter trivial/fully-covered methods, score the
/// remainder, drop entries below `min_score`, and return sorted by score desc.
pub fn process(xml: &str, min_score: f64) -> Result<Vec<FilteredMethod>, Box<dyn std::error::Error>> {
    let classes = parser::parse(xml)?;
    let mut output: Vec<FilteredMethod> = Vec::new();

    for class in &classes {
        for method in &class.methods {
            if filter::is_trivial(&method.name) {
                continue;
            }
            if filter::is_fully_covered(method) {
                continue;
            }

            let total = (method.line_missed + method.line_covered) as usize;
            let missed = method.missed_lines.len();
            let s = scorer::score(method.complexity, missed, total);

            if s < min_score {
                continue;
            }

            output.push(FilteredMethod {
                class: class.class_name.clone(),
                source_file: class.source_file.clone(),
                method: method.name.clone(),
                score: s,
                missed_lines: method.missed_lines.clone(),
            });
        }
    }

    output.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    Ok(output)
}

/// Build a line-coverage summary from all parsed classes (before filtering).
fn summarize(classes: &[model::ParsedClass]) -> CoverageSummary {
    let mut total_covered: u32 = 0;
    let mut total_missed: u32 = 0;
    let mut by_class: Vec<ClassCoverage> = Vec::new();

    for class in classes {
        let covered = class.line_covered;
        let missed = class.line_missed;
        let pct = pct(covered, missed);
        by_class.push(ClassCoverage {
            class: class.class_name.clone(),
            source_file: class.source_file.clone(),
            line_coverage_pct: pct,
            lines_covered: covered,
            lines_missed: missed,
        });
        total_covered += covered;
        total_missed += missed;
    }

    by_class.sort_by(|a, b| a.line_coverage_pct.partial_cmp(&b.line_coverage_pct).unwrap_or(std::cmp::Ordering::Equal));

    CoverageSummary {
        line_coverage_pct: pct(total_covered, total_missed),
        lines_covered: total_covered,
        lines_missed: total_missed,
        by_class,
    }
}

fn pct(covered: u32, missed: u32) -> f64 {
    let total = covered + missed;
    if total == 0 {
        100.0
    } else {
        (covered as f64 / total as f64) * 100.0
    }
}

/// Like `process`, but also returns a coverage summary in a `Report` wrapper.
pub fn process_with_summary(xml: &str, min_score: f64) -> Result<Report, Box<dyn std::error::Error>> {
    let classes = parser::parse(xml)?;
    let summary = summarize(&classes);

    let mut methods: Vec<FilteredMethod> = Vec::new();
    for class in &classes {
        for method in &class.methods {
            if filter::is_trivial(&method.name) {
                continue;
            }
            if filter::is_fully_covered(method) {
                continue;
            }

            let total = (method.line_missed + method.line_covered) as usize;
            let missed = method.missed_lines.len();
            let s = scorer::score(method.complexity, missed, total);

            if s < min_score {
                continue;
            }

            methods.push(FilteredMethod {
                class: class.class_name.clone(),
                source_file: class.source_file.clone(),
                method: method.name.clone(),
                score: s,
                missed_lines: method.missed_lines.clone(),
            });
        }
    }

    methods.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    Ok(Report { summary, methods })
}
