//! We have multiple FHIR resources for a Procedure. This module has functions to generate XML for these
//! different Procedure resources.

use std::ops::Range;

use chrono::NaiveDate;
use fake::{Fake, Faker};
use fhirbolt::model::r4b::resources::{BundleEntry, Procedure, ProcedurePerformed};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Period, Reference};
use fhirbolt::model::r4b::Resource;

use crate::extensions::option_ext::OptionExt;
use crate::models::enums::id_type::IdType;
use crate::models::enums::syst_therapy_type::SystTherapyType;
use crate::utils::{get_bundle_entry_request, get_full_url, get_ids, get_syst_therapy_type_url};

pub fn get_procedure(
    id: &str,
    subject_ref: &str,
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
    let subject_rfrnc = Reference {
        reference: Some(subject_ref.into()),
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
        subject: Box::new(subject_rfrnc),
        performed: Some(ProcedurePerformed::Period(Box::new(period))),
        reason_reference: vec![reason_rfrnc],
        ..Default::default()
    }
}

pub fn get_bundle_entry(procedure: Procedure, procedure_ref_id: &str) -> BundleEntry {
    BundleEntry {
        full_url: Some(get_full_url(
            procedure.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Procedure(Box::new(procedure.clone()))),
        request: get_bundle_entry_request("PUT", procedure_ref_id).into_some(),
        ..Default::default()
    }
}

pub fn get_proc_operations(
    src_id: &str,
    reason_ref: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
    range: Range<u8>,
) -> Vec<(Procedure, String)> {
    let res_type = "Operation";
    get_procedures(
        res_type,
        src_id,
        reason_ref,
        start_date,
        end_date,
        SystTherapyType::OP,
        range,
    )
}

pub fn get_proc_radio_therapies(
    src_id: &str,
    reason_ref: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
    range: Range<u8>,
) -> Vec<(Procedure, String)> {
    let res_type = "Radiotherapy";
    get_procedures(
        res_type,
        src_id,
        reason_ref,
        start_date,
        end_date,
        SystTherapyType::RT,
        range,
    )
}

fn get_procedures(
    res_type: &str,
    src_id: &str,
    reason_ref: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
    therapy_type: SystTherapyType,
    range: Range<u8>,
) -> Vec<(Procedure, String)> {
    range
        .map(|_| {
            let i: u16 = Faker.fake();
            let (id, _) = get_ids("Procedure".into_some(), IdType::Id, res_type, i);
            (
                get_procedure(
                    id.as_str(),
                    src_id,
                    reason_ref,
                    start_date,
                    end_date,
                    therapy_type.clone(),
                ),
                id.to_string(),
            )
        })
        .collect()
}
