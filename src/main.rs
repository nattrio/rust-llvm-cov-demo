use rstest::rstest;

fn main() {
    println!("Hello, world!");
    let result = congrats(0);
    println!("{}", result);
}

pub fn greet(name: &str) -> String {
    match name {
        "" => "Hello, world!".to_string(),
        _ => format!("Hello, {}!", name),
    }
}

pub fn congrats(score: u32) -> String {
    match score {
        0 => "You scored no points".to_string(),
        1 => "You scored 1 point".to_string(),
        _ => format!("You scored {} points", score),
    }
}

#[rstest]
#[case("Alice", "Hello, Alice!".to_string())]
#[case("", "Hello, world!".to_string())]
fn test_greet(#[case] input: &str, #[case] expected: String) {
    assert_eq!(expected, greet(input))
}

#[rstest]
#[case(0, "You scored no points")]
#[case(1, "You scored 1 point")]
#[case(10, "You scored 10 points")]
fn test_congrats(#[case] input: u32, #[case] expected: String) {
    assert_eq!(expected, congrats(input))
}
