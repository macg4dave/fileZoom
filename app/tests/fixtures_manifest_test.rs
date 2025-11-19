use std::fs;
use std::path::Path;

#[test]
fn fixtures_manifest_contains_expected_entries() {
    // Manifest path relative to crate
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("fixtures_manifest.txt");
    assert!(manifest_path.exists(), "Manifest file should exist: {:?}", manifest_path);

    let content = fs::read_to_string(&manifest_path).expect("read manifest");
    let lines: Vec<&str> = content.lines().collect();

    // There should be a lot of entries
    let count = lines.len();
    assert!(count >= 200, "Expected at least 200 fixtures, found {}", count);

    // Check for a few exemplar filenames we intentionally create
    let has_emoji = lines.iter().any(|l| l.contains("emoji-😊"));
    let has_complex = lines.iter().any(|l| l.contains("COMPLEX.name.with.many.dots.log"));
    let has_spaces = lines.iter().any(|l| l.contains("spaces and tabs.txt"));

    assert!(has_emoji, "Manifest should list emoji file");
    assert!(has_complex, "Manifest should list complex-named log");
    assert!(has_spaces, "Manifest should list spaces-and-tabs file");

    // Ensure at least one nested directory file exists
    let has_nested = lines.iter().any(|l| l.contains("deep/level1/level2") || l.contains("deep\\level1\\level2"));
    assert!(has_nested, "Expected at least one nested file under deep/level1/level2");
}
