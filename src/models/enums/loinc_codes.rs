use fake::Dummy;

use crate::utils::{
    CLINICAL_METASTASES_LOINC_CODE, CLINICAL_NODES_LOINC_CODE, CLINICAL_STAGE_GROUP_LOINC_CODE,
    CLINICAL_TUMOR_LOINC_CODE,
};

#[derive(Debug, Dummy)]
pub enum TnmClassification {
    Pathologic,
    Clinical,
}

impl TnmClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmClassification::Pathologic => "21902-2",
            // TODO: ... check if this is correct
            TnmClassification::Clinical => CLINICAL_STAGE_GROUP_LOINC_CODE,
        }
    }
}

#[derive(Debug, Dummy)]
pub enum TnmtClassification {
    Pathologic,
    Clinical,
}

impl TnmtClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmtClassification::Pathologic => "21899-0",
            TnmtClassification::Clinical => CLINICAL_TUMOR_LOINC_CODE,
        }
    }
}

#[derive(Debug, Dummy)]
pub enum TnmnClassification {
    Pathologic,
    Clinical,
}

impl TnmnClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmnClassification::Pathologic => "21900-6",
            TnmnClassification::Clinical => CLINICAL_NODES_LOINC_CODE,
        }
    }
}

#[derive(Debug, Dummy)]
pub enum TnmmClassification {
    Pathologic,
    Clinical,
}

impl TnmmClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmmClassification::Pathologic => "21901-4",
            TnmmClassification::Clinical => CLINICAL_METASTASES_LOINC_CODE,
        }
    }
}
