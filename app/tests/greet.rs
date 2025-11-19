use fileZoom::greet;

#[test]
fn greet_returns_expected() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}
