use fhirbolt::model::r4b::resources::{CodeSystem, CodeSystemConcept};
use fhirbolt::model::r4b::types::Code;

use crate::fhir::traits::{CodeSystemAdapter, CodeSystemConceptAdapter}; // Bring the trait into scope
use crate::models::lens::traits::CriteriaConverter; // Bring the trait into scope
use crate::utils::{CCE, FHIR_COMPLETION_STATUS, FHIR_RESOURCE_STATUS, FHIR_RESOURCE_VERSION};
use crate::models::enums::vital_status::VitalStatus;

use super::globals::get_contact_details;

pub fn get_vital_status_code_system() -> CodeSystem {
    let status_code = Code {
        value: Some(FHIR_RESOURCE_STATUS.to_string()),
        ..Default::default()
    };
    let content_code = Code {
        value: Some(FHIR_COMPLETION_STATUS.to_string()),
        ..Default::default()
    };

    CodeSystem {
        text: Some(Box::new(VitalStatus::get_narrative())),
        url: Some(VitalStatus::get_url().into()),
        version: Some(FHIR_RESOURCE_VERSION.to_string().into()),
        name: Some(VitalStatus::get_name().into()),
        title: Some(VitalStatus::get_title().into()),
        status: status_code,
        publisher: Some(CCE.to_string().into()),
        contact: get_contact_details(),
        description: Some(VitalStatus::get_description().into()),
        case_sensitive: Some(true.into()),
        compositional: Some(false.into()),
        content: content_code,
        count: Some((VitalStatus::get_criteria().len() as u32).into()),
        concept: get_code_system_concepts(),
        ..Default::default()
    }
}

fn get_code_system_concepts() -> Vec<CodeSystemConcept> {
    VitalStatus::get_concepts()
        .into_iter()
        .map(|concept| CodeSystemConcept {
            code: Code {
                value: Some(concept.code),
                ..Default::default()
            },
            display: Some(concept.display.into()),
            ..Default::default()
        })
        .collect()
}
