pub trait OptionExt {
    fn as_some(&self) -> Option<&Self> {
        Option::Some(self)
    }

    fn into_some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Option::Some(self)
    }

    fn as_none(&self) -> Option<&Self> {
        Option::None
    }

    fn into_none(self) -> Option<Self>
    where
        Self: Sized,
    {
        Option::None
    }
}

impl<A> OptionExt for A {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_convert_to_some() {
        let i = 2;
        let as_i = i.as_some();
        assert_eq!(as_i, Some(&i), "could not convert to Some");

        let into_i = i.into_some();
        assert_eq!(into_i, Some(i), "could not convert to Some");
    }

    #[test]
    fn test_should_convert_to_none() {
        let s = "";
        let as_s = s.as_none();
        assert_eq!(as_s, None::<&&str>, "could not convert to None");

        let into_s = s.into_none();
        assert_eq!(into_s, None::<&str>, "could not convert to None");
    }
}