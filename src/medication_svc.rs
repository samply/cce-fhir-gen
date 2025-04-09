use std::ops::Range;

use chrono::NaiveDate;
use fake::{Fake, Faker};
use fhirbolt::model::r4b::resources::{
    BundleEntry, MedicationStatement, MedicationStatementEffective, MedicationStatementMedication,
};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Period, Reference};
use fhirbolt::model::r4b::Resource;

use crate::extensions::option_ext::OptionExt;
use crate::models::cli::ResourceType;
use crate::models::enums::id_type::IdType;
use crate::models::enums::syst_therapy_type::SystTherapyType;
use crate::utils::{get_bundle_entry_request, get_full_url, get_ids, get_syst_therapy_type_url};

pub fn get_med_statement(
    id: &str,
    med_ref: &str,
    subject_ref: &str,
    reason_ref: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> MedicationStatement {
    let therapy_type: SystTherapyType = Faker.fake();

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
    let subject_rfrnc = Reference {
        reference: Some(subject_ref.into()),
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
            value: Some(start_date.to_string()),
            ..Default::default()
        }),
        end: Some(DateTime {
            value: Some(end_date.to_string()),
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
        subject: Box::new(subject_rfrnc),
        effective: Some(effective),
        reason_reference: vec![reason_rfrnc],
        ..Default::default()
    }
}

pub fn get_bundle_entry(patient: MedicationStatement, patient_ref_id: &str) -> BundleEntry {
    BundleEntry {
        full_url: Some(get_full_url(
            patient.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::MedicationStatement(Box::new(patient.clone()))),
        request: get_bundle_entry_request("PUT", patient_ref_id).into_some(),
        ..Default::default()
    }
}

pub fn get_med_statements(
    src_id: &str,
    reason_ref: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
    range: Range<u8>,
) -> Vec<(MedicationStatement, String)> {
    range
        .map(|_| {
            let i: u16 = Faker.fake();
            let (med_stmt_id, _) = get_ids(
                "MedicationStatement".into_some(),
                IdType::Id,
                ResourceType::MedicationStatementSystemicTherapy,
                i,
            );
            (
                get_med_statement(
                    med_stmt_id.as_str(),
                    "medicine",
                    src_id,
                    reason_ref,
                    start_date,
                    end_date,
                ),
                med_stmt_id,
            )
        })
        .collect()
}
