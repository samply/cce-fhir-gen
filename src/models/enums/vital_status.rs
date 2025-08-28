use fake::Dummy;
use strum::Display;

use crate::models::{lens::{catalogue::Criteria, traits::CriteriaConverter}, traits::CodeSystem};

#[derive(Debug, Display, Dummy)]
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

    // pub fn to_de_str(&self) -> &'static str {
    //     match self {
    //         VitalStatus::Alive => "lebend",
    //         VitalStatus::Deceased => "verstorben",
    //         VitalStatus::Unknown => "unbekannt",
    //     }
    // }
}

impl CodeSystem for VitalStatus {
    fn get_name() -> String {
        "VitalStatusCS".to_string()
    }

    fn get_title() -> String {
        "Vital Status CS".to_string()
    }

    fn get_html_title() -> String {
        "VitalStatus".to_string()
    }
    
    fn get_html_description() -> String {
        "VitalStatus CodeSystem".to_string()
    }

    fn get_url() -> String {
        Self::get_code_system_url(Self::get_name().as_str())
    }
}

impl CriteriaConverter for VitalStatus {

    fn get_criteria() -> Vec<Criteria> {
        let alive = Criteria::new(VitalStatus::Alive.as_str(), VitalStatus::Alive.to_string().as_str());
        let deceased = Criteria::new(VitalStatus::Deceased.as_str(), VitalStatus::Deceased.to_string().as_str());
        let unknown = Criteria::new(VitalStatus::Unknown.as_str(), VitalStatus::Unknown.to_string().as_str());
        vec![alive, deceased, unknown]
    }
}
