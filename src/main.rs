use rstest::rstest;

fn main() {
    println!("Hello, world!");
}

pub fn greet(name: &str) -> String {
    match name {
        "" => "Hello, world!".to_string(),
        _ => format!("Hello, {}!", name),
    }
}

#[rstest]
#[case("Alice", "Hello, Alice!".to_string())]
#[case("", "Hello, world!".to_string())]
fn test_greet(#[case] input: &str, #[case] expected: String) {
    assert_eq!(expected, greet(input))
}
