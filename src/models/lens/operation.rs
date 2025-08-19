use super::{
    enums::{Operand, OperationsConditions},
    language::BiLingualKey,
};

pub struct Operation {
    pub operand: Operand,
    pub children: OperationsConditions,
    pub bi_lingual_key: BiLingualKey,
}
