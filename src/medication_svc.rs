use fhirbolt::model::r4b::resources::{
    MedicationStatement, MedicationStatementEffective, MedicationStatementMedication,
};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Period, Reference};

use crate::models::enums::syst_therapy_type::SystTherapyType;
use crate::utils::get_syst_therapy_type_url;

pub fn get_med_statement(
    id: &str,
    med_ref: &str,
    sub_ref: &str,
    reason_ref: &str,
    therapy_type: SystTherapyType,
    start: &str,
    end: &str,
) -> MedicationStatement {
    let pid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let status = Code {
        id: Some("ongoing".to_string()),
        ..Default::default()
    };
    let med_rfrnc = Reference {
        reference: Some(med_ref.into()),
        ..Default::default()
    };
    let medication = MedicationStatementMedication::Reference(Box::new(med_rfrnc));
    let sub_rfrnc = Reference {
        reference: Some(sub_ref.into()),
        ..Default::default()
    };
    let reason_rfrnc = Reference {
        reference: Some(reason_ref.into()),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(get_syst_therapy_type_url()),
        code: Some(Code::from(therapy_type.to_string())),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };
    let period = Period {
        start: Some(DateTime {
            value: Some(start.to_string()),
            ..Default::default()
        }),
        end: Some(DateTime {
            value: Some(end.to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let effective = MedicationStatementEffective::Period(Box::new(period));

    MedicationStatement {
        r#id: Some(pid),
        status,
        medication,
        category: Some(Box::new(cod_concept)),
        subject: Box::new(sub_rfrnc),
        effective: Some(effective),
        reason_reference: vec![reason_rfrnc],
        ..Default::default()
    }
}
