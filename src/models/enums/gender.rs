use std::any::type_name;

use fake::Dummy;

use crate::models::lens::{
    criteria::Criteria, enums::{ConditionType, CriteriaType}, language::{BiLingualDisplay, BiLingualKey}, traits::{CriteriaConverter, LanguageConverter}
};

#[derive(Debug, Dummy)]
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

    pub fn to_de_str(&self) -> &'static str {
        match self {
            Gender::Male => "männlich",
            Gender::Female => "weiblich",
        }
    }
}

impl LanguageConverter for Gender {
    fn get_bi_lingual_display(&self) -> BiLingualDisplay {
        BiLingualDisplay::new("Geschlecht", type_name::<Self>())
    }

    fn get_bi_lingual_keys(&self) -> Vec<BiLingualKey> {
        let male = BiLingualKey::new("male", Gender::Male.to_de_str(), Gender::Male.as_str());
        let female = BiLingualKey::new(
            "female",
            Gender::Female.to_de_str(),
            Gender::Female.as_str(),
        );
        vec![male, female]
    }
}

impl CriteriaConverter for Gender {
    fn get_criteria(&self) -> Criteria {
        Criteria::new(
            type_name::<Self>().to_lowercase().as_str(),
            self.get_bi_lingual_display(),
            CriteriaType::String,
            ConditionType::In,
            self.get_bi_lingual_keys(),
        )
    }
}
