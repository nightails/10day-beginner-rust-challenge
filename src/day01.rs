// Day 1: Hello, Rustacean + Variables and Mutability

fn greet() -> String {
    "Hello, Rustacean!".to_string()
}

fn double_counter() -> i32 {
    let mut counter = 0;
    for _ in 0..32 {
        counter += 1;
    }
    return counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, Rustacean!");
    }

    #[test]
    fn test_double_counter() {
        assert_eq!(double_counter(), 32);
    }
}
