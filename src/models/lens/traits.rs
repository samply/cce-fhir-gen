use super::{criteria::Criteria, language::{BiLingualDisplay, BiLingualKey}};

pub trait LanguageConverter {
    fn get_bi_lingual_display(&self) -> BiLingualDisplay;

    fn get_bi_lingual_keys(&self) -> Vec<BiLingualKey>;
}

pub trait CriteriaConverter {
    fn get_criteria(&self) -> Criteria;
}