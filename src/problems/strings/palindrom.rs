pub fn is_palindrome(str: &str) -> bool {
    let reversed = str.chars().rev().collect::<String>();
    str == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pal() {
        assert_eq!(is_palindrome("hello"), "olleh");
        assert_eq!(is_palindrome("a", "a"));
    }
}