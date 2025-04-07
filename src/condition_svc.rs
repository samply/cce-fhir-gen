use std::ops::Range;

use fake::{Fake, Faker};
use fhirbolt::model::r4b::resources::{BundleEntry, Condition, ConditionOnset};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Reference, Uri};
use fhirbolt::model::r4b::Resource;

use crate::extensions::option_ext::OptionExt;
use crate::models::enums::tumor_site_location::TumorSiteLocation;
use crate::utils::{get_bundle_entry_request, get_full_url, get_site_location_url};

pub fn get_condition(
    id: &str,
    subject_ref: &str,
    code_value: &str,
    bs_code_value1: &str,
) -> Condition {
    let bs_code_value2: TumorSiteLocation = Faker.fake();

    let cid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let subject_rfrnc = Reference {
        reference: Some(subject_ref.into()),
        ..Default::default()
    };
    let effective = DateTime {
        value: Some("2021-02-02".to_string()),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(Uri::from("http://fhir.de/CodeSystem/bfarm/icd-10-gm")),
        version: Some("2019".into()),
        code: Some(Code::from(code_value)),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };
    let body_site_coding1 = Coding {
        system: Some(Uri::from("urn:oid:2.16.840.1.113883.6.43.1")),
        version: Some("31".into()),
        code: Some(Code::from(bs_code_value1)),
        ..Default::default()
    };
    let body_site_coding2 = Coding {
        system: Some(get_site_location_url()),
        version: None,
        code: Some(Code::from(bs_code_value2.to_string())),
        ..Default::default()
    };
    let body_site = CodeableConcept {
        coding: vec![body_site_coding1, body_site_coding2],
        ..Default::default()
    };

    Condition {
        r#id: Some(cid),
        code: Some(Box::new(cod_concept)),
        body_site: vec![body_site],
        subject: Box::new(subject_rfrnc),
        onset: Some(ConditionOnset::DateTime(effective.clone())),
        recorded_date: Some(effective.clone()),
        ..Default::default()
    }
}

pub fn get_bundle_entry(condition: Condition, condition_ref_id: &str) -> BundleEntry {
    BundleEntry {
        full_url: Some(get_full_url(
            condition.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Condition(Box::new(condition.clone()))),
        request: get_bundle_entry_request("PUT", condition_ref_id).into_some(),
        ..Default::default()
    }
}

pub fn get_conditions(
    id: &str,
    subject_ref: &str,
    code_value: &str,
    bs_code_value1: &str,
    range: Range<u8>,
) -> Vec<Condition> {
    range
        .map(|_| get_condition(id, subject_ref, code_value, bs_code_value1))
        .collect()
}
