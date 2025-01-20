use regex::Regex;

fn main() {
    let user = UserProfile {
        name: "Alice".to_string(),
        email: "alice.a@mail.com".to_string(),
        age: 25,
    };
    match validate_user(&user) {
        Ok(_) => println!("User profile is valid"),
        Err(err) => eprintln!("Validation error: {}", err),
    }
}

#[derive(Debug, PartialEq)]
struct UserProfile {
    name: String,
    email: String,
    age: u8,
}

fn validate_user(profile: &UserProfile) -> Result<(), String> {
    if profile.name.is_empty() {
        return Err("Name cannot be empty".into());
    }
    if profile.name.len() > 50 {
        return Err("Name cannot exceed 50 characters".into());
    }
    let email_regex = Regex::new(r"^[^@]+@[^@]+\.[^@]+$").unwrap();
    if !email_regex.is_match(&profile.email) {
        return Err("Invalid email format".into());
    }
    if profile.age <= 15 {
        return Err("Age must be above 15".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn valid_user() -> UserProfile {
        UserProfile {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            age: 25,
        }
    }

    #[rstest]
    #[case("", "alice@example.com", 25, Err("Name cannot be empty".into()))]
    #[case("A very long name that exceeds fifty characters ............", "alice@example.com", 25, Err("Name cannot exceed 50 characters".into()))]
    #[case("Alice", "invalid-email", 25, Err("Invalid email format".into()))]
    #[case("Alice", "alice@example.com", 15, Err("Age must be above 15".into()))]
    #[case("Alice", "alice@example.com", 25, Ok(()))]
    fn test_validate_user(
        #[from(valid_user)] mut user: UserProfile,
        #[case] name: &str,
        #[case] email: &str,
        #[case] age: u8,
        #[case] expected: Result<(), String>,
    ) {
        user.name = name.to_string();
        user.email = email.to_string();
        user.age = age;

        let result = validate_user(&user);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("", "alice@example.com", 25, Err("Name cannot be empty".into()))]
    #[case("A very long name that exceeds fifty characters ............", "alice@example.com", 25, Err("Name cannot exceed 50 characters".into()))]
    #[case("Alice", "invalid-email", 25, Err("Invalid email format".into()))]
    #[case("Alice", "alice@example.com", 15, Err("Age must be above 15".into()))]
    #[case("Alice", "alice@example.com", 25, Ok(()))]
    fn test_validate_user_without_fixture(
        #[case] name: &str,
        #[case] email: &str,
        #[case] age: u8,
        #[case] expected: Result<(), String>,
    ) {
        let user = UserProfile {
            name: name.to_string(),
            email: email.to_string(),
            age,
        };

        let result = validate_user(&user);
        assert_eq!(result, expected);
    }
}
