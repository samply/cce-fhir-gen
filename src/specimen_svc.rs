use std::ops::Range;

use fake::{Fake, Faker};
use fhirbolt::model::r4b::resources::{
    BundleEntry, Specimen, SpecimenCollection, SpecimenCollectionCollected,
};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Reference};
use fhirbolt::model::r4b::Resource;

use crate::extensions::option_ext::OptionExt;
use crate::models::enums::sample_material_type::SampleMaterialType;
use crate::utils::{
    get_body_site_url, get_bundle_entry_request, get_full_url, get_sample_mat_type_url,
};

pub fn get_specimen(id: &str, subject_ref: &str) -> Specimen {
    let sample_material_type: SampleMaterialType = Faker.fake();

    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let subject_rfrnc = Reference {
        reference: Some(subject_ref.into()),
        ..Default::default()
    };
    let collected = DateTime {
        value: Some("2021-02-02".to_string()),
        ..Default::default()
    };

    let bs_coding = Coding {
        system: Some(get_body_site_url()),
        version: None,
        code: Some(Code::from("C26.8")),
        ..Default::default()
    };
    let bs_cod_concept = CodeableConcept {
        coding: vec![bs_coding],
        ..Default::default()
    };
    let specimen_collection = SpecimenCollection {
        collected: Some(SpecimenCollectionCollected::DateTime(collected)),
        body_site: Some(Box::new(bs_cod_concept)),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(get_sample_mat_type_url()),
        version: None,
        code: Some(Code::from(sample_material_type.as_str())),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };

    Specimen {
        r#id: Some(oid),
        subject: Some(Box::new(subject_rfrnc)),
        collection: Some(specimen_collection),
        r#type: Some(Box::new(cod_concept)),
        ..Default::default()
    }
}

pub fn get_bundle_entry(specimen: Specimen, specimen_ref_id: &str) -> BundleEntry {
    BundleEntry {
        full_url: Some(get_full_url(
            specimen.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Specimen(Box::new(specimen.clone()))),
        request: get_bundle_entry_request("PUT", specimen_ref_id).into_some(),
        ..Default::default()
    }
}

pub fn get_specimens(id: &str, src_id: &str, range: Range<u8>) -> Vec<Specimen> {
    range.map(|_| get_specimen(id, src_id)).collect()
}
