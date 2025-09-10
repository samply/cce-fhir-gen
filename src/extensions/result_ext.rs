// pub trait ResultExt {
//     fn as_ok<E>(&self) -> Result<&Self, E> {
//         Result::Ok(self)
//     }

//     fn into_ok<E>(self) -> Result<Self, E>
//     where
//         Self: Sized,
//     {
//         Result::Ok(self)
//     }

//     fn as_err<T>(&self) -> Result<T, &Self> {
//         Result::Err(self)
//     }

//     fn into_err<T>(self) -> Result<T, Self>
//     where
//         Self: Sized,
//     {
//         Result::Err(self)
//     }
// }

// impl<A> ResultExt for A {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_should_convert_to_ok() {
//         let i = 2;
//         let as_i = i.as_ok::<&str>();
//         assert_eq!(as_i, Ok(&i), "could not convert to Ok");

//         let into_i = i.into_ok::<&str>();
//         assert_eq!(into_i, Ok(i), "could not convert to Ok");
//     }

//     #[test]
//     fn test_should_convert_to_err() {
//         let s = "";
//         let as_s = s.as_err::<&str>();
//         assert_eq!(as_s, Err(&s), "could not convert to Err");

//         let into_s = s.into_err::<&str>();
//         assert_eq!(into_s, Err(s), "could not convert to Err");
//     }
// }
