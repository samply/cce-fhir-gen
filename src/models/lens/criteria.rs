use super::{enums::{BiLingualKeysOperations, ConditionType, CriteriaType}, language::BiLingualDisplay};

pub struct Criteria {
    pub key: String,
    pub display_name: BiLingualDisplay,
    pub r#type: CriteriaType,
    pub system: String,
    pub allowed_condition_types: Vec<ConditionType>,
    pub values: BiLingualKeysOperations
}