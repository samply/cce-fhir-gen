use super::enums::{ConditionType, ValueType};

pub struct Condition {
    pub key: String,
    pub r#type: ConditionType,
    pub system: Option<String>,
    pub value: Option<ValueType>,
    pub de: Option<String>
}