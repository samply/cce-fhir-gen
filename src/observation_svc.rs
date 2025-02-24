use chrono::NaiveDate;
use fhirbolt::model::r4b::resources::{
    Observation, ObservationEffective,
    ObservationValue,
};
use fhirbolt::model::r4b::types::{
    Code, CodeableConcept, Coding, DateTime, Id, Reference, Uri,
};
use crate::models::enums::vital_status::VitalStatus;
use crate::utils::{get_loinc_url, get_vital_status_url, OBSERVATION_STATUS};

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
        // version: Some("31".into()),
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
