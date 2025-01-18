fn main() {
    println!("Hello, world!");
}

pub fn greet(name: &str) -> String {
    match name {
        "" => "Hello, world!".to_string(),
        _ => format!("Hello, {}!", name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(""), "Hello, world!".to_string());
        assert_eq!(greet("Alice"), "Hello, Alice!".to_string());
    }
}
