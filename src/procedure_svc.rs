use chrono::NaiveDate;
use fhirbolt::model::r4b::resources::{Procedure, ProcedurePerformed};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Period, Reference};

use crate::models::enums::syst_therapy_type::SystTherapyType;
use crate::utils::get_syst_therapy_type_url;

pub fn get_procedure(
    id: &str,
    sub_ref: &str,
    reason_ref: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
    therapy_type: SystTherapyType,
) -> Procedure {
    let pid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let status = Code {
        id: Some("successful".to_string()),
        ..Default::default()
    };
    let sub_rfrnc = Reference {
        reference: Some(sub_ref.into()),
        ..Default::default()
    };
    let reason_rfrnc = Reference {
        reference: Some(reason_ref.into()),
        ..Default::default()
    };

    let start = DateTime {
        value: Some(start_date.to_string()),
        ..Default::default()
    };
    let end = DateTime {
        value: Some(end_date.to_string()),
        ..Default::default()
    };
    let period = Period {
        start: Some(start),
        end: Some(end),
        ..Default::default()
    };

    let coding = Coding {
        system: Some(get_syst_therapy_type_url()),
        code: Some(Code::from(therapy_type.to_string())),
        display: Some(therapy_type.as_str().into()),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };

    Procedure {
        r#id: Some(pid),
        status,
        category: Some(Box::new(cod_concept)),
        subject: Box::new(sub_rfrnc),
        performed: Some(ProcedurePerformed::Period(Box::new(period))),
        reason_reference: vec![reason_rfrnc],
        ..Default::default()
    }
}
