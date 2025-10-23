//! We have multiple FHIR resources for an Observation. This module has functions to generate XML for these
//! different Observation resources.

use std::ops::Range;

use crate::extensions::option_ext::OptionExt;
use crate::models::cli::ResourceType;
use crate::models::enums::id_type::IdType;
use crate::models::enums::loinc_codes::{
    TnmClassification, TnmmClassification, TnmnClassification, TnmtClassification,
};
use crate::models::enums::tnmm_category::TnmmCategory;
use crate::models::enums::tnmn_category::TnmnCategory;
use crate::models::enums::tnmt_category::TnmtCategory;
use crate::models::enums::uicc_stage::UiccStage;
use crate::models::enums::vital_status::VitalStatus;
use crate::utils::{
    get_bundle_entry_request, get_full_url, get_ids, get_loinc_url, get_tnmm_url, get_tnmn_url, get_tnmt_url, get_uicc_stage_url, get_vital_status_url, HISTOLOGY_BEHAVIOR_CANCER_LOINC_CODE, OBSERVATION_STATUS, VITAL_STATUS_LOINC_CODE
};
use chrono::NaiveDate;
use fake::{Fake, Faker};
use fhirbolt::model::r4b::resources::{
    BundleEntry, Observation, ObservationComponent, ObservationComponentValue,
    ObservationEffective, ObservationValue,
};
use fhirbolt::model::r4b::types::{Code, CodeableConcept, Coding, DateTime, Id, Reference, Uri};
use fhirbolt::model::r4b::Resource;

/// Generates observation histology
pub fn get_histology(
    id: &str,
    subject_ref: &str,
    focus_ref: &str,
    specimen_ref: &str,
    effective_date: NaiveDate,
    code_value: &str,
) -> Observation {
    // TODO: check date, code etc.
    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let subject_rfrnc = Reference {
        reference: Some(subject_ref.into()),
        ..Default::default()
    };
    let focus_rfrnc = Reference {
        reference: Some(focus_ref.into()),
        ..Default::default()
    };
    let speci_rfrnc = Reference {
        reference: Some(specimen_ref.into()),
        ..Default::default()
    };
    let effective = DateTime {
        value: Some(effective_date.to_string()),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(Uri::from("urn:oid:2.16.840.1.113883.6.43.1")),
        version: Some("32".into()),
        code: Some(Code::from(code_value)),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };
    let code_coding = Coding {
        system: Some(get_loinc_url()),
        code: Some(Code::from(HISTOLOGY_BEHAVIOR_CANCER_LOINC_CODE)),
        ..Default::default()
    };
    let code_cod_concept = CodeableConcept {
        coding: vec![code_coding],
        ..Default::default()
    };

    Observation {
        r#id: Some(oid),
        subject: Some(Box::new(subject_rfrnc)),
        focus: vec![focus_rfrnc],
        specimen: Some(Box::new(speci_rfrnc)),
        effective: Some(ObservationEffective::DateTime(effective)),
        // NOTE: status is required by the FHIR lib
        status: OBSERVATION_STATUS.into(),
        value: Some(ObservationValue::CodeableConcept(Box::new(cod_concept))),
        code: Box::new(code_cod_concept),
        ..Default::default()
    }
}

/// Generates observation vitalstatus
pub fn get_vital_status(id: &str, subject_ref: &str, effective_date: NaiveDate) -> Observation {
    // NOTE: VitalStatus is also an Observation
    // TODO: check date, code etc.
    let code_value: VitalStatus = Faker.fake();

    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let subject_rfrnc = Reference {
        reference: Some(subject_ref.into()),
        ..Default::default()
    };
    let effective = DateTime {
        value: Some(effective_date.to_string()),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(get_vital_status_url()),
        // version: Some("31".into()),
        code: Some(Code::from(code_value.as_str())),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };
    let loinc_coding = Coding {
        system: Some(get_loinc_url()),
        code: Some(Code::from(VITAL_STATUS_LOINC_CODE)),
        ..Default::default()
    };
    let loinc_cod_concept = CodeableConcept {
        coding: vec![loinc_coding],
        ..Default::default()
    };

    Observation {
        r#id: Some(oid),
        subject: Some(Box::new(subject_rfrnc)),
        effective: Some(ObservationEffective::DateTime(effective)),
        // NOTE: status is required by the FHIR lib
        status: OBSERVATION_STATUS.into(),
        value: Some(ObservationValue::CodeableConcept(Box::new(cod_concept))),
        code: Box::new(loinc_cod_concept),
        ..Default::default()
    }
}

/// Generates observation TNMc
pub fn get_tnmc(id: &str, subject_ref: &str, effective_date: NaiveDate) -> Observation {
    let uicc_code_value: UiccStage = Faker.fake();
    let tnmm: TnmmCategory = Faker.fake();
    let tnmn: TnmnCategory = Faker.fake();
    let tnmt: TnmtCategory = Faker.fake();
    // TODO: check date, code etc.
    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let subject_rfrnc = Reference {
        reference: Some(subject_ref.into()),
        ..Default::default()
    };
    let effective = DateTime {
        value: Some(effective_date.to_string()),
        ..Default::default()
    };

    let coding = Coding {
        system: Some(get_uicc_stage_url()),
        version: Some("8".into()),
        code: Some(Code::from(uicc_code_value.as_str())),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };

    let tnmm_coding = Coding {
        system: Some(get_tnmm_url()),
        code: Some(Code::from(tnmm.as_str())),
        ..Default::default()
    };
    let tnmm_concept = CodeableConcept {
        coding: vec![tnmm_coding],
        ..Default::default()
    };
    let tnmm_comp = ObservationComponent {
        code: Box::new(get_loinc_code(TnmmClassification::Clinical.as_str())),
        value: Some(ObservationComponentValue::CodeableConcept(Box::new(
            tnmm_concept,
        ))),
        ..Default::default()
    };

    let tnmn_coding = Coding {
        system: Some(get_tnmn_url()),
        code: Some(Code::from(tnmn.as_str())),
        ..Default::default()
    };
    let tnmn_concept = CodeableConcept {
        coding: vec![tnmn_coding],
        ..Default::default()
    };
    let tnmn_comp = ObservationComponent {
        code: Box::new(get_loinc_code(TnmnClassification::Clinical.as_str())),
        value: Some(ObservationComponentValue::CodeableConcept(Box::new(
            tnmn_concept,
        ))),
        ..Default::default()
    };

    let tnmt_coding = Coding {
        system: Some(get_tnmt_url()),
        code: Some(Code::from(tnmt.as_str())),
        ..Default::default()
    };
    let tnmt_concept = CodeableConcept {
        coding: vec![tnmt_coding],
        ..Default::default()
    };
    let tnmt_comp = ObservationComponent {
        code: Box::new(get_loinc_code(TnmtClassification::Clinical.as_str())),
        value: Some(ObservationComponentValue::CodeableConcept(Box::new(
            tnmt_concept,
        ))),
        ..Default::default()
    };

    Observation {
        r#id: Some(oid),
        subject: Some(Box::new(subject_rfrnc)),
        effective: Some(ObservationEffective::DateTime(effective)),
        // NOTE: status is required by the FHIR lib
        status: OBSERVATION_STATUS.into(),
        value: Some(ObservationValue::CodeableConcept(Box::new(cod_concept))),
        code: Box::new(get_loinc_code(TnmClassification::Clinical.as_str())),
        // component: vec![tnmm_comp, tnmn_comp, tnmr_comp, tnmy_comp],
        component: vec![tnmm_comp, tnmn_comp, tnmt_comp],
        ..Default::default()
    }
}

pub fn get_bundle_entry(observation: Observation, observation_ref_id: &str) -> BundleEntry {
    BundleEntry {
        full_url: Some(get_full_url(
            observation.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Observation(Box::new(observation.clone()))),
        request: get_bundle_entry_request("PUT", observation_ref_id).into_some(),
        ..Default::default()
    }
}

pub fn get_histologies(
    subject_ref: &str,
    focus_ref: &str,
    specimen_ref: &str,
    effective_date: NaiveDate,
    code_value: &str,
    range: Range<u8>,
) -> Vec<(Observation, String)> {
    range
        .map(|_| {
            let i: u16 = Faker.fake();
            let (obs_hist_id, obs_hist_ref_id) =
                get_ids(IdType::Id, ResourceType::ObservationHistology, i);
            (
                get_histology(
                    obs_hist_id.as_str(),
                    subject_ref,
                    focus_ref,
                    specimen_ref,
                    effective_date,
                    code_value,
                ),
                obs_hist_ref_id,
            )
        })
        .collect()
}

pub fn get_vital_statuses(
    subject_ref: &str,
    effective_date: NaiveDate,
    range: Range<u8>,
) -> Vec<(Observation, String)> {
    range
        .map(|_| {
            let i: u16 = Faker.fake();
            let (ovs_id, ovs_ref_id) = get_ids(IdType::Id, ResourceType::ObservationVitalStatus, i);
            (
                get_vital_status(ovs_id.as_str(), subject_ref, effective_date),
                ovs_ref_id,
            )
        })
        .collect()
}

pub fn get_tnmcs(
    subject_ref: &str,
    effective_date: NaiveDate,
    range: Range<u8>,
) -> Vec<(Observation, String)> {
    range
        .map(|_| {
            let i: u16 = Faker.fake();
            let (obs_tnmc_id, obs_tnmc_ref_id) =
                get_ids(IdType::Id, ResourceType::ObservationTNMc, i);
            (
                get_tnmc(obs_tnmc_id.as_str(), subject_ref, effective_date),
                obs_tnmc_ref_id,
            )
        })
        .collect()
}

fn get_loinc_code(code_val: &str) -> CodeableConcept {
    let loinc_coding = Coding {
        system: Some(get_loinc_url()),
        code: Some(Code::from(code_val)),
        ..Default::default()
    };

    CodeableConcept {
        coding: vec![loinc_coding],
        ..Default::default()
    }
}
