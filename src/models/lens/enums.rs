use chrono::NaiveDate;

use super::{
    condition::Condition,
    language::BiLingualKey,
    operation::Operation,
    ranges::{DateRange, NumberRange},
};

pub enum Operand {
    And,
    Or,
    Not,
    Xor,
}

impl Operand {
    pub fn as_str(&self) -> &'static str {
        match self {
            Operand::And => "AND",
            Operand::Or => "OR",
            Operand::Not => "NOT",
            Operand::Xor => "XOR",
        }
    }
}

pub enum ConditionType {
    Equals,
    NotEquals,
    In,
    Between,
    LowerThan,
    GreaterThan,
    Contains,
}

impl ConditionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConditionType::Equals => "EQUALS",
            ConditionType::NotEquals => "NOT_EQUALS",
            ConditionType::In => "IN",
            ConditionType::Between => "BETWEEN",
            ConditionType::LowerThan => "LOWER_THAN",
            ConditionType::GreaterThan => "GREATER_THAN",
            ConditionType::Contains => "CONTAINS",
        }
    }
}

pub enum CriteriaType {
    Str(String),
    Number(f32),
    Boolean(bool),
    Date(NaiveDate),
    Predefined,
}

impl CriteriaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CriteriaType::Str(_s) => "string",
            CriteriaType::Number(_n) => "number",
            CriteriaType::Boolean(_b) => "boolean",
            CriteriaType::Date(_d) => "date",
            CriteriaType::Predefined => "predefined",
        }
    }
}

pub enum ValueType {
    Str(String),
    Number(f32),
    Boolean(bool),
    StrArray(Vec<String>),
    NumRange(NumberRange),
    DateRange(DateRange),
}

pub enum OperationsConditions {
    Operations(Vec<Operation>),
    Conditions(Vec<Condition>),
}

pub enum BiLingualKeysOperations {
    BiLingualKeys(Vec<BiLingualKey>),
    Operations(Vec<Operation>),
}
