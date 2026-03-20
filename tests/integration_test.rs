use jacoco_filter::process;

const SAMPLE_XML: &str = include_str!("fixtures/sample.xml");

#[test]
fn test_constructor_is_filtered() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    let names: Vec<&str> = methods.iter().map(|m| m.method.as_str()).collect();
    assert!(!names.contains(&"<init>"), "constructor <init> should be filtered");
}

#[test]
fn test_getter_is_filtered() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    let names: Vec<&str> = methods.iter().map(|m| m.method.as_str()).collect();
    assert!(!names.contains(&"getDeductionRule"), "getter getDeductionRule should be filtered");
}

#[test]
fn test_non_trivial_methods_present() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    let names: Vec<&str> = methods.iter().map(|m| m.method.as_str()).collect();
    assert!(names.contains(&"resourceType"), "resourceType should be in output");
    assert!(names.contains(&"startUseResourceActually"), "startUseResourceActually should be in output");
    assert!(names.contains(&"finishUseResource"), "finishUseResource should be in output");
}

#[test]
fn test_method_count() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    assert_eq!(methods.len(), 3, "expected exactly 3 non-trivial partially-covered methods");
}

#[test]
fn test_scores() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    for m in &methods {
        assert!(
            (m.score - 1.0).abs() < 1e-9,
            "method {} has unexpected score {}",
            m.method,
            m.score
        );
    }
}

#[test]
fn test_resource_type_missed_lines() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    let m = methods.iter().find(|m| m.method == "resourceType").unwrap();
    assert_eq!(m.missed_lines, vec![32]);
}

#[test]
fn test_start_use_resource_missed_lines() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    let m = methods
        .iter()
        .find(|m| m.method == "startUseResourceActually")
        .unwrap();
    let expected = vec![38, 39, 40, 41, 42, 43, 44, 45, 46, 48, 49];
    assert_eq!(m.missed_lines, expected);
}

#[test]
fn test_finish_use_resource_missed_lines() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    let m = methods.iter().find(|m| m.method == "finishUseResource").unwrap();
    let expected = vec![53, 54, 55, 56, 57, 58, 59, 60, 61, 63, 64];
    assert_eq!(m.missed_lines, expected);
}

#[test]
fn test_class_and_source_file() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    for m in &methods {
        assert_eq!(
            m.class,
            "com.newland.modules.billing.domain.business.service.resource.level.LevelResource"
        );
        assert_eq!(m.source_file, "LevelResource.java");
    }
}

#[test]
fn test_min_score_filter() {
    // With min_score > 1.0, nothing should pass
    let methods = process(SAMPLE_XML, 1.1).unwrap();
    assert!(methods.is_empty(), "no methods should pass min_score > all scores");
}

#[test]
fn test_sorted_by_score_desc() {
    let methods = process(SAMPLE_XML, 0.0).unwrap();
    for window in methods.windows(2) {
        assert!(
            window[0].score >= window[1].score,
            "results should be sorted by score descending"
        );
    }
}

#[test]
fn test_scorer_unit() {
    use jacoco_filter::scorer::score;
    assert!((score(2, 5, 10) - 1.0).abs() < 1e-9);
    assert!((score(3, 1, 3) - 1.0).abs() < 1e-9);
    assert_eq!(score(5, 0, 10), 0.0);
    assert_eq!(score(5, 5, 0), 0.0);
}

#[test]
fn test_filter_unit() {
    use jacoco_filter::filter::is_trivial;
    assert!(is_trivial("<init>"));
    assert!(is_trivial("getValue"));
    assert!(is_trivial("setName"));
    assert!(!is_trivial("calculate"));
    assert!(!is_trivial("resourceType"));
}
