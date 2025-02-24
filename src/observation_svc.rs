use crate::models::enums::tnmm_category::TnmmCategory;
use crate::models::enums::tnmn_category::TnmnCategory;
use crate::models::enums::tnmr_symbol::TnmrSymbol;
use crate::models::enums::tnmy_symbol::TnmySymbol;
use crate::models::enums::vital_status::VitalStatus;
use crate::utils::{
    get_loinc_url, get_tnmm_url, get_tnmn_url, get_tnmr_symbol_url, get_tnmt_url, get_tnmy_symbol_url, get_uicc_stage_url, get_vital_status_url, OBSERVATION_STATUS
};
use chrono::NaiveDate;
use fhirbolt::model::r4b::resources::{
    Observation, ObservationComponent, ObservationComponentValue, ObservationEffective,
    ObservationValue,
};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Reference, Uri};

/// Generates observation histology
pub fn get_histology(
    id: &str,
    sub_ref: &str,
    focus_ref: &str,
    specimen_ref: &str,
    effective_date: NaiveDate,
    code_value: &str,
) -> Observation {
    // TODO: check date, code etc.
    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let sub_rfrnc = Reference {
        reference: Some(sub_ref.into()),
        ..Default::default()
    };
    let focus_rfrnc = Reference {
        reference: Some(focus_ref.into()),
        ..Default::default()
    };
    let speci_rfrnc = Reference {
        reference: Some(specimen_ref.into()),
        ..Default::default()
    };
    let effective = DateTime {
        value: Some(effective_date.to_string()),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(Uri::from("urn:oid:2.16.840.1.113883.6.43.1")),
        version: Some("32".into()),
        code: Some(Code::from(code_value)),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };
    let code_coding = Coding {
        system: Some(get_loinc_url()),
        code: Some(Code::from("59847-4")),
        ..Default::default()
    };
    let code_cod_concept = CodeableConcept {
        coding: vec![code_coding],
        ..Default::default()
    };

    Observation {
        r#id: Some(oid),
        subject: Some(Box::new(sub_rfrnc)),
        focus: vec![focus_rfrnc],
        specimen: Some(Box::new(speci_rfrnc)),
        effective: Some(ObservationEffective::DateTime(effective)),
        // NOTE: status is required by the FHIR lib
        status: OBSERVATION_STATUS.into(),
        value: Some(ObservationValue::CodeableConcept(Box::new(cod_concept))),
        code: Box::new(code_cod_concept),
        ..Default::default()
    }
}

/// Generates observation vitalstatus
pub fn get_vital_status(
    id: &str,
    sub_ref: &str,
    effective_date: NaiveDate,
    code_value: VitalStatus,
) -> Observation {
    // NOTE: VitalStatus is also an Observation
    // TODO: check date, code etc.
    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let sub_rfrnc = Reference {
        reference: Some(sub_ref.into()),
        ..Default::default()
    };
    let effective = DateTime {
        value: Some(effective_date.to_string()),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(get_vital_status_url()),
        // version: Some("31".into()),
        code: Some(Code::from(code_value.as_str())),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };
    let loinc_coding = Coding {
        system: Some(get_loinc_url()),
        code: Some(Code::from("75186-7")),
        ..Default::default()
    };
    let loinc_cod_concept = CodeableConcept {
        coding: vec![loinc_coding],
        ..Default::default()
    };

    Observation {
        r#id: Some(oid),
        subject: Some(Box::new(sub_rfrnc)),
        effective: Some(ObservationEffective::DateTime(effective)),
        // NOTE: status is required by the FHIR lib
        status: OBSERVATION_STATUS.into(),
        value: Some(ObservationValue::CodeableConcept(Box::new(cod_concept))),
        code: Box::new(loinc_cod_concept),
        ..Default::default()
    }
}

/// Generates observation TNMc
pub fn get_tnmc(
    id: &str,
    sub_ref: &str,
    effective_date: NaiveDate,
    uicc_code_value: &str,
    tnmm: TnmmCategory,
    tnmn: TnmnCategory,
    tnmt: &str,
    tnmr: TnmrSymbol,
    tnmy: TnmySymbol,
) -> Observation {
    // TODO: check date, code etc.
    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let sub_rfrnc = Reference {
        reference: Some(sub_ref.into()),
        ..Default::default()
    };
    let effective = DateTime {
        value: Some(effective_date.to_string()),
        ..Default::default()
    };

    
    let coding = Coding {
        system: Some(get_uicc_stage_url()),
        version: Some("8".into()),
        code: Some(Code::from(uicc_code_value)),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };

    let tnmm_coding = Coding {
        system: Some(get_tnmm_url()),
        code: Some(Code::from(tnmm.as_str())),
        ..Default::default()
    };
    let tnmm_concept = CodeableConcept {
        coding: vec![tnmm_coding],
        ..Default::default()
    };
    let tnmm_comp = ObservationComponent {
        code: Box::new(get_loinc_code("21907-1")),
        value: Some(ObservationComponentValue::CodeableConcept(Box::new(
            tnmm_concept,
        ))),
        ..Default::default()
    };

    let tnmn_coding = Coding {
        system: Some(get_tnmn_url()),
        code: Some(Code::from(tnmn.as_str())),
        ..Default::default()
    };
    let tnmn_concept = CodeableConcept {
        coding: vec![tnmn_coding],
        ..Default::default()
    };
    let tnmn_comp = ObservationComponent {
        code: Box::new(get_loinc_code("201906-3")),
        value: Some(ObservationComponentValue::CodeableConcept(Box::new(
            tnmn_concept,
        ))),
        ..Default::default()
    };

    let tnmt_coding = Coding {
        system: Some(get_tnmt_url()),
        code: Some(Code::from(tnmt)),
        ..Default::default()
    };
    let tnmt_concept = CodeableConcept {
        coding: vec![tnmt_coding],
        ..Default::default()
    };
    let tnmt_comp = ObservationComponent {
        code: Box::new(get_loinc_code("21907-1")),
        value: Some(ObservationComponentValue::CodeableConcept(Box::new(
            tnmt_concept,
        ))),
        ..Default::default()
    };

    // let tnmr_coding = Coding {
    //     system: Some(get_tnmr_symbol_url()),
    //     code: Some(Code::from(tnmr.code())),
    //     ..Default::default()
    // };
    // let tnmr_concept = CodeableConcept {
    //     coding: vec![tnmr_coding],
    //     ..Default::default()
    // };
    // let tnmr_comp = ObservationComponent {
    //     code: Box::new(get_loinc_code("21983-2")),
    //     value: Some(ObservationComponentValue::CodeableConcept(Box::new(
    //         tnmr_concept,
    //     ))),
    //     ..Default::default()
    // };

    // let tnmy_coding = Coding {
    //     system: Some(get_tnmy_symbol_url()),
    //     code: Some(Code::from(tnmy.code())),
    //     ..Default::default()
    // };
    // let tnmy_concept = CodeableConcept {
    //     coding: vec![tnmy_coding],
    //     ..Default::default()
    // };
    // let tnmy_comp = ObservationComponent {
    //     code: Box::new(get_loinc_code("59479-6")),
    //     value: Some(ObservationComponentValue::CodeableConcept(Box::new(
    //         tnmy_concept,
    //     ))),
    //     ..Default::default()
    // };

    Observation {
        r#id: Some(oid),
        subject: Some(Box::new(sub_rfrnc)),
        effective: Some(ObservationEffective::DateTime(effective)),
        // NOTE: status is required by the FHIR lib
        status: OBSERVATION_STATUS.into(),
        value: Some(ObservationValue::CodeableConcept(Box::new(cod_concept))),
        code: Box::new(get_loinc_code("21908-9")),
        // component: vec![tnmm_comp, tnmn_comp, tnmr_comp, tnmy_comp],
        component: vec![tnmm_comp, tnmn_comp, tnmt_comp],
        ..Default::default()
    }
}

fn get_loinc_code(code_val: &str) -> CodeableConcept {
    let loinc_coding = Coding {
        system: Some(get_loinc_url()),
        code: Some(Code::from(code_val)),
        ..Default::default()
    };

    CodeableConcept {
        coding: vec![loinc_coding],
        ..Default::default()
    }
}
