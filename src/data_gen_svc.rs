use chrono::{Months, NaiveDate};
use fhirbolt::model::r4b::resources::{
    Bundle, BundleEntry, Condition, ConditionOnset, MedicationStatement,
    MedicationStatementEffective, MedicationStatementMedication, Observation, Patient, PatientDeceased, Procedure, Specimen,
    SpecimenCollection, SpecimenCollectionCollected,
};
use fhirbolt::model::r4b::types::{
    Code, CodeableConcept, Coding, Date, DateTime, Id, Identifier, Period, Reference, String, Uri
};
use fhirbolt::model::r4b::Resource;

use crate::extensions::option_ext::OptionExt;
use crate::models::enums::gender::Gender;
use crate::models::enums::sample_material_type::SampleMaterialType;
use crate::models::enums::syst_therapy_type::SystTherapyType;
use crate::models::enums::tumor_site_location::TumorSiteLocation;
use crate::utils::{get_body_site_url, get_bundle_entry_request, get_full_url, get_sample_mat_type_url, get_site_location_url, get_syst_therapy_type_url};

///
/// A service with methods that generate XML using the domain model classes
/// defined in fhirbolt crate.
///

pub fn get_bundle(
    id: &str,
    patient_tuple: (Patient, &str),
    specimen_tuple: (Specimen, &str),
    condition_tuple: (Condition, &str),
    obs_histology_tuple: (Observation, &str),
    obs_vital_status_tuple: (Observation, &str),
    proc_op_tuple: (Procedure, &str),
    proc_rt_tuple: (Procedure, &str),
    med_stmt_tuple: (MedicationStatement, &str),
) -> Bundle {
    let id = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = BundleEntry {
        full_url: Some(get_full_url(
            patient_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Patient(Box::new(patient_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", patient_tuple.1).into_some(),
        ..Default::default()
    };

    let specimen = BundleEntry {
        full_url: Some(get_full_url(
            specimen_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Specimen(Box::new(specimen_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", specimen_tuple.1).into_some(),
        ..Default::default()
    };

    let condition = BundleEntry {
        full_url: Some(get_full_url(
            condition_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Condition(Box::new(condition_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", condition_tuple.1).into_some(),
        ..Default::default()
    };

    let observation = BundleEntry {
        full_url: Some(get_full_url(
            obs_histology_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Observation(Box::new(obs_histology_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", obs_histology_tuple.1).into_some(),
        ..Default::default()
    };

    let vital_status = BundleEntry {
        full_url: Some(get_full_url(
            obs_vital_status_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Observation(Box::new(obs_vital_status_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", obs_vital_status_tuple.1).into_some(),
        ..Default::default()
    };

    let procedure = BundleEntry {
        full_url: Some(get_full_url(
            proc_rt_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Procedure(Box::new(proc_rt_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", proc_rt_tuple.1).into_some(),
        ..Default::default()
    };

    let operation = BundleEntry {
        full_url: Some(get_full_url(
            proc_op_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Procedure(Box::new(proc_op_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", proc_op_tuple.1).into_some(),
        ..Default::default()
    };

    let med_stmt = BundleEntry {
        full_url: Some(get_full_url(
            med_stmt_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::MedicationStatement(Box::new(med_stmt_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", med_stmt_tuple.1).into_some(),
        ..Default::default()
    };

    Bundle {
        id: Some(id),
        r#type: code,
        entry: vec![
            patient,
            specimen,
            condition,
            observation,
            vital_status,
            procedure,
            operation,
            med_stmt,
        ],
        ..Default::default()
    }
}

pub fn get_patient(id: &str, src_id: &str, gender: Gender, birthdate: NaiveDate, deceased: bool) -> Patient {
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

pub fn get_condition(
    id: &str,
    sub_ref: &str,
    code_value: &str,
    bs_code_value1: &str,
    bs_code_value2: TumorSiteLocation,
) -> Condition {
    let cid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let sub_rfrnc = Reference {
        reference: Some(sub_ref.into()),
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
        subject: Box::new(sub_rfrnc),
        onset: Some(ConditionOnset::DateTime(effective.clone())),
        recorded_date: Some(effective.clone()),
        ..Default::default()
    }
}

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
