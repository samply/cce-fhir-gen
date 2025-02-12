/// Extension methods for working with &str.
pub trait StrExt {
    fn to_option_str(&self) -> Option<&str>;
}

impl StrExt for str {
    /// Converts &str to Option<&str>
    fn to_option_str(&self) -> Option<&str> {
        match self.trim() {
            "" => None,
            x => Some(x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_convert_a_non_empty_str_to_some() {
        let s = "hello";
        assert_eq!(s.to_option_str(), Some("hello"), "some option does not match");
    }

    #[test]
    fn test_should_convert_an_empty_str_to_none() {
        let s = "  ";
        assert_eq!(s.to_option_str(), None, "none option does not match");
    }
}
