use std::ops::Range;

use chrono::{prelude::*, Months};
use fake::faker::chrono::en::DateTimeAfter;
use fake::{Fake, Faker};
use fhirbolt::model::r4b::resources::{Patient, PatientDeceased};
use fhirbolt::model::r4b::types::{Code, Date, DateTime, Id, Identifier, String};

use crate::models::enums::gender::Gender;

pub fn get_patient(id: &str, src_id: &str, min_date_time: chrono::DateTime<Utc>) -> Patient {
    let gender: Gender = Faker.fake();
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

pub fn get_patients(
    id: &str,
    src_id: &str,
    min_date_time: chrono::DateTime<Utc>,
    range: Range<u8>,
) -> Vec<Patient> {
    range
        .map(|_| get_patient(id, src_id, min_date_time))
        .collect()
}
