use chrono::{Months, NaiveDate};
use fhirbolt::model::r4b::resources::{
    Bundle, BundleEntry, Condition, ConditionOnset, MedicationStatement,
    MedicationStatementEffective, MedicationStatementMedication, Observation, ObservationEffective,
    ObservationValue, Patient, PatientDeceased, Procedure, ProcedurePerformed, Specimen,
    SpecimenCollection, SpecimenCollectionCollected,
};
use fhirbolt::model::r4b::types::{
    Code, CodeableConcept, Coding, Date, DateTime, Id, Period, Reference, Uri,
};
use fhirbolt::model::r4b::Resource;

use crate::extensions::option_ext::OptionExt;
use crate::models::enums::gender::Gender;
use crate::models::enums::sample_material_type::SampleMaterialType;
use crate::models::enums::syst_therapy_type::SystTherapyType;
use crate::models::enums::tumor_site_location::TumorSiteLocation;
use crate::utils::{get_bundle_entry_request, get_full_url};

///
/// A service with methods that generate XML using the domain model classes
/// defined in fhirbolt crate.
///

pub fn get_bundle(
    id: &str,
    patient_tuple: (Patient, &str),
    specimen_tuple: (Specimen, &str),
    condition_tuple: (Condition, &str),
    observation_tuple: (Observation, &str),
    procedure_tuple: (Procedure, &str),
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
            observation_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Observation(Box::new(observation_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", observation_tuple.1).into_some(),
        ..Default::default()
    };

    let procedure = BundleEntry {
        full_url: Some(get_full_url(
            procedure_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Procedure(Box::new(procedure_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", procedure_tuple.1).into_some(),
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
            procedure,
            med_stmt,
        ],
        ..Default::default()
    }
}

pub fn get_patient(id: &str, gender: Gender, birthdate: NaiveDate, deceased: bool) -> Patient {
    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };

    let mut patient = Patient {
        r#id: Some(oid),
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
    let specimen_collection = SpecimenCollection {
        collected: Some(SpecimenCollectionCollected::DateTime(collected)),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(Uri::from(
            "https://www.cancercoreeurope.eu/fhir/CodeSystem/CodeSystem-cce-core-CodeSystem-SampleMaterialType",
        )),
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
        version: Some("2004".into()),
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
        system: Some(Uri::from(
            "https://www.cancercoreeurope.eu/fhir/core/CodeSystem/SitelocationCS",
        )),
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

pub fn get_observation(
    id: &str,
    sub_ref: &str,
    focus_ref: &str,
    effective_date: NaiveDate,
    code_value: &str,
) -> Observation {
    // TODO: check date, code etc.
    let oid = Id {
        value: Some(id.to_string()),
        ..Default::default()
    };
    let sub_rfrnc = Reference {
        reference: Some(sub_ref.into()),
        ..Default::default()
    };
    let focus_rfrnc = Reference {
        reference: Some(focus_ref.into()),
        ..Default::default()
    };
    let effective = DateTime {
        value: Some(effective_date.to_string()),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(Uri::from("urn:oid:2.16.840.1.113883.6.43.1")),
        version: Some("31".into()),
        code: Some(Code::from(code_value)),
        ..Default::default()
    };
    let cod_concept = CodeableConcept {
        coding: vec![coding],
        ..Default::default()
    };

    Observation {
        r#id: Some(oid),
        subject: Some(Box::new(sub_rfrnc)),
        focus: vec![focus_rfrnc],
        effective: Some(ObservationEffective::DateTime(effective)),
        status: "final".into(),
        value: Some(ObservationValue::CodeableConcept(Box::new(cod_concept))),
        code: Box::new(CodeableConcept {
            text: Some("some code".into()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn get_procedure(
    id: &str,
    sub_ref: &str,
    reason_ref: &str,
    effective_date: NaiveDate,
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
    let sub_rfrnc = Reference {
        reference: Some(sub_ref.into()),
        ..Default::default()
    };
    let reason_rfrnc = Reference {
        reference: Some(reason_ref.into()),
        ..Default::default()
    };
    let effective = DateTime {
        value: Some(effective_date.to_string()),
        ..Default::default()
    };
    let coding = Coding {
        system: Some(Uri::from(
            "https://www.cancercoreeurope.eu/fhir/core/CodeSystem/SYSTTherapyTypeCS",
        )),
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
        subject: Box::new(sub_rfrnc),
        performed: Some(ProcedurePerformed::DateTime(effective)),
        reason_reference: vec![reason_rfrnc],
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
        system: Some(Uri::from(
            "https://www.cancercoreeurope.eu/fhir/core/CodeSystem/SYSTTherapyTypeCS",
        )),
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
