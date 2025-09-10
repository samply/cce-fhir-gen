// use crate::extensions::str_ext::StrExt;

// /// Extension methods for working with Option<&str>.
// pub trait OptionStrExt {
//     fn to_option_string(&self) -> Option<String>;
// }

// impl OptionStrExt for Option<&str> {    
//     /// Converts Option<&str> to Option<String>
//     fn to_option_string(&self) -> Option<String> {
//         self.map(StrExt::to_option_str).flatten().map(str::to_string)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_should_convert_a_non_empty_str_to_some() {
//         let s = Some("hello");
//         assert_eq!(s.to_option_string(), Some("hello".to_string()), "some option does not match");
//     }

//     #[test]
//     fn test_should_convert_an_empty_str_to_none() {
//         let s = Some("  ");
//         assert_eq!(s.to_option_string(), None, "none option does not match");
//     }
// }