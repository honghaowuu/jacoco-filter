pub mod filter;
pub mod model;
pub mod parser;
pub mod scorer;

use model::FilteredMethod;

/// Parse the JaCoCo XML, filter trivial/fully-covered methods, score the
/// remainder, drop entries below `min_score`, and return sorted by score desc.
pub fn process(xml: &str, min_score: f64) -> Result<Vec<FilteredMethod>, Box<dyn std::error::Error>> {
    let classes = parser::parse(xml)?;

    let mut output: Vec<FilteredMethod> = Vec::new();

    for class in classes {
        for method in class.methods {
            if filter::is_trivial(&method.name) {
                continue;
            }
            if filter::is_fully_covered(&method) {
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
                method: method.name,
                score: s,
                missed_lines: method.missed_lines,
            });
        }
    }

    output.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    Ok(output)
}
