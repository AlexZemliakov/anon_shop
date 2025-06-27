mod tor;

pub fn hello() -> String {
    "Hello from backend!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello from backend!");
    }
}