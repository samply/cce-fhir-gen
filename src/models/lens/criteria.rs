use super::{
    enums::{BiLingualKeysOperations, ConditionType, CriteriaType},
    language::{BiLingualDisplay, BiLingualKey},
};

pub struct Criteria {
    pub key: String,
    pub display_name: BiLingualDisplay,
    pub r#type: CriteriaType,
    pub system: String,
    pub allowed_condition_types: Vec<ConditionType>,
    pub values: BiLingualKeysOperations,
}

impl Criteria {
    pub fn new(
        key: &str,
        display_name: BiLingualDisplay,
        criteria_type: CriteriaType,
        condition_type: ConditionType,
        keys: Vec<BiLingualKey>,
    ) -> Self {
        Criteria {
            key: key.to_string(),
            display_name,
            r#type: criteria_type,
            system: "".to_string(),
            allowed_condition_types: vec![condition_type],
            values: BiLingualKeysOperations::BiLingualKeys(keys),
        }
    }
}
