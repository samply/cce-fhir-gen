use fhirbolt::model::r4b::resources::{Specimen, SpecimenCollection, SpecimenCollectionCollected};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Reference};

use crate::models::enums::sample_material_type::SampleMaterialType;
use crate::utils::{get_body_site_url, get_sample_mat_type_url};

pub fn get_specimen(id: &str, sub_ref: &str, sample_material_type: SampleMaterialType) -> Specimen {
    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let sub_rfrnc = Reference {
        reference: Some(sub_ref.into()),
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
        subject: Some(Box::new(sub_rfrnc)),
        collection: Some(specimen_collection),
        r#type: Some(Box::new(cod_concept)),
        ..Default::default()
    }
}
