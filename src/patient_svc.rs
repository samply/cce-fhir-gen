use std::ops::Range;

use chrono::{prelude::*, Months};
use fake::faker::chrono::en::DateTimeAfter;
use fake::{Fake, Faker};
use fhirbolt::model::r4b::resources::{BundleEntry, Patient, PatientDeceased};
use fhirbolt::model::r4b::types::{Code, Date, DateTime, Id, Identifier, String};
use fhirbolt::model::r4b::Resource;

use crate::extensions::option_ext::OptionExt;
use crate::models::enums::gender::Gender;
use crate::models::enums::id_type::IdType;
use crate::utils::{get_bundle_entry_request, get_full_url, get_ids};

pub fn get_patient(id: &str, src_id: &str) -> Patient {
    let gender: Gender = Faker.fake();
    let min_date_time: chrono::DateTime<Utc> = Faker.fake();
    let birthdate: chrono::DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let deceased: bool = Faker.fake();

    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };

    let id_val = String {
        value: Some(src_id.to_string()),
        ..Default::default()
    };
    let identifier = Identifier {
        value: Some(id_val),
        ..Default::default()
    };

    let mut patient = Patient {
        r#id: Some(oid),
        r#identifier: vec![identifier],
        gender: Some(Code {
            value: Some(gender.as_str().to_string()),
            ..Default::default()
        }),
        birth_date: Some(Date {
            value: Some(birthdate.to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    if deceased {
        let deceased_date = birthdate.checked_add_months(Months::new(600)).unwrap();
        let deceased_date_time =
            PatientDeceased::DateTime(DateTime::from(deceased_date.to_string()));

        patient.deceased = Some(deceased_date_time);
        patient
    } else {
        patient
    }
}

pub fn get_bundle_entry(patient: Patient, patient_ref_id: &str) -> BundleEntry {
    BundleEntry {
        full_url: Some(get_full_url(
            patient.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Patient(Box::new(patient.clone()))),
        request: get_bundle_entry_request("PUT", patient_ref_id).into_some(),
        ..Default::default()
    }
}

pub fn get_patients(range: Range<u8>) -> Vec<(Patient, std::string::String)> {
    range
        .map(|_| {
            let i: u16 = Faker.fake();
            let (patient_id, _) = get_ids(None, IdType::Id, "Patient", i);
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            (get_patient(patient_id.as_str(), patient_src_id.as_str()), patient_id)
        })
        .collect()
}
