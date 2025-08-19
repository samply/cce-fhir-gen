use std::any::type_name;

use fake::Dummy;

use crate::models::lens::{
    criteria::Criteria,
    enums::{ConditionType, CriteriaType},
    language::{BiLingualDisplay, BiLingualKey},
    traits::{CriteriaConverter, LanguageConverter},
};

#[derive(Debug, Dummy)]
pub enum VitalStatus {
    Alive,
    Deceased,
    Unknown,
}

impl VitalStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            VitalStatus::Alive => "alive",
            VitalStatus::Deceased => "deceased",
            VitalStatus::Unknown => "unknown",
        }
    }

    pub fn to_de_str(&self) -> &'static str {
        match self {
            VitalStatus::Alive => "lebend",
            VitalStatus::Deceased => "verstorben",
            VitalStatus::Unknown => "unbekannt",
        }
    }
}

impl LanguageConverter for VitalStatus {
    fn get_bi_lingual_display(&self) -> BiLingualDisplay {
        BiLingualDisplay::new("Vitalstatus", type_name::<Self>())
    }

    fn get_bi_lingual_keys(&self) -> Vec<BiLingualKey> {
        let alive = BiLingualKey::new(
            "alive",
            VitalStatus::Alive.to_de_str(),
            VitalStatus::Alive.as_str(),
        );
        let deceased = BiLingualKey::new(
            "deceased",
            VitalStatus::Deceased.to_de_str(),
            VitalStatus::Deceased.as_str(),
        );
        let unknown = BiLingualKey::new(
            "unknown",
            VitalStatus::Unknown.to_de_str(),
            VitalStatus::Unknown.as_str(),
        );
        vec![alive, deceased, unknown]
    }
}

impl CriteriaConverter for VitalStatus {
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
