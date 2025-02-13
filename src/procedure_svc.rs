use chrono::NaiveDate;
use fhirbolt::model::r4b::resources::{Procedure, ProcedurePerformed};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Reference, Uri};

use crate::models::enums::syst_therapy_type::SystTherapyType;

pub fn get_procedure(
    id: &str,
    sub_ref: &str,
    reason_ref: &str,
    effective_date: NaiveDate,
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
    let effective = DateTime {
        value: Some(effective_date.to_string()),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(Uri::from(
            "https://www.cancercoreeurope.eu/fhir/core/CodeSystem/SYSTTherapyTypeCS",
        )),
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
        performed: Some(ProcedurePerformed::DateTime(effective)),
        reason_reference: vec![reason_rfrnc],
        ..Default::default()
    }
}
