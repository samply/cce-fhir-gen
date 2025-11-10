use fake::Dummy;
use strum::Display;

use crate::lens::{catalogue::Criteria, traits::CriteriaConverter};

#[derive(Debug, Display, Dummy)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
        }
    }

    // pub fn to_de_str(&self) -> &'static str {
    //     match self {
    //         Gender::Male => "männlich",
    //         Gender::Female => "weiblich",
    //     }
    // }
}

impl CriteriaConverter for Gender {
    fn get_criteria() -> Vec<Criteria> {
        let male = Criteria::new(Gender::Male.as_str(), Gender::Male.to_string().as_str());
        let female = Criteria::new(Gender::Female.as_str(), Gender::Female.to_string().as_str());
        vec![male, female]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gender_to_str() {
        let female = Gender::Female;

        assert_eq!(Gender::Male.to_string(), "Male");
        assert_eq!(female.as_str(), "female");
    }
}
