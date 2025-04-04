use chrono::prelude::*;

use fake::faker::chrono::en::DateTimeAfter;
use fake::{Fake, Faker};

use crate::extensions::option_ext::OptionExt;
use crate::models::enums::id_type::IdType;
use crate::models::enums::syst_therapy_type::SystTherapyType;
use crate::utils::{get_bundle_entry_request, get_full_url, get_ids};
use crate::{
    condition_svc, medication_svc, observation_svc, patient_svc, procedure_svc, specimen_svc,
};

use fhirbolt::model::r4b::resources::{
    Bundle, BundleEntry, Condition, MedicationStatement, Observation, Patient, Procedure, Specimen,
};
use fhirbolt::model::r4b::types::{Code, Id};
use fhirbolt::model::r4b::Resource;

pub fn get_bundle() -> Bundle {
    let i: u16 = Faker.fake();
    let (bundle_id, _) = get_ids(None, IdType::Id, "Bundle", i);
    let (patient_id, patient_ref_id) = get_ids(None, IdType::Id, "Patient", i);
    let (condition_id, condition_ref_id) = get_ids(None, IdType::Id, "Condition", i);
    let (specimen_id, specimen_ref_id) = get_ids(None, IdType::Id, "Specimen", i);
    let (obs_hist_id, obs_hist_ref_id) =
        get_ids("Observation".into_some(), IdType::Id, "Histology", i);
    let (obs_vital_status_id, obs_vital_status_ref_id) =
        get_ids("Observation".into_some(), IdType::Id, "VitalStatus", i);
    let (obs_tnmc_id, obs_tnmc_ref_id) = get_ids("Observation".into_some(), IdType::Id, "TNMc", i);
    let (proc_rt_id, proc_rt_ref_id) =
        get_ids("Procedure".into_some(), IdType::Id, "Radiotherapy", i);
    let (proc_op_id, proc_op_ref_id) = get_ids("Procedure".into_some(), IdType::Id, "Operation", i);
    let (med_stmt_id, med_stmt_ref_id) = get_ids(
        "MedicationStatement".into_some(),
        IdType::Id,
        "SystemicTherapy",
        i,
    );

    let min_date_time = Utc.with_ymd_and_hms(1930, 1, 1, 0, 0, 0).unwrap();
    let bd: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let ed: DateTime<Utc> = DateTimeAfter(min_date_time).fake();

    let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
    let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str(), Faker.fake());
    // let pt1 = pt.clone();
    // print_fhir_data(pt1, "patient");

    let s = specimen_svc::get_specimen(specimen_id.as_str(), patient_ref_id.as_str(), Faker.fake());
    // let s1 = s.clone();
    // print_fhir_data(s1, "specimen");

    let c = condition_svc::get_condition(
        condition_id.as_str(),
        patient_ref_id.as_str(),
        "C34.0",
        "C34.0",
        Faker.fake(),
    );
    // let c1 = c.clone();
    // print_fhir_data(c1, "condition");

    let ohist = observation_svc::get_histology(
        obs_hist_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        specimen_ref_id.as_str(),
        ed.date_naive(),
        "8140/3",
    );
    // let ohist1 = ohist.clone();
    // print_fhir_data(ohist1, "observation-histology");

    let ovs = observation_svc::get_vital_status(
        obs_vital_status_id.as_str(),
        patient_ref_id.as_str(),
        ed.date_naive(),
        Faker.fake(),
    );
    // let ovs1 = ovs.clone();
    // print_fhir_data(ovs1, "observation-vitalstatus");

    let otnmc = observation_svc::get_tnmc(
        &obs_tnmc_id.as_str(),
        patient_ref_id.as_str(),
        ed.date_naive(),
        Faker.fake(),
        Faker.fake(),
        Faker.fake(),
        Faker.fake(),
    );
    // let otnmc1 = otnmc.clone();
    // print_fhir_data(otnmc1, "observation-tnmc");

    let sd: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let ed: DateTime<Utc> = DateTimeAfter(sd).fake();
    let prt = procedure_svc::get_procedure(
        proc_rt_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        sd.date_naive(),
        ed.date_naive(),
        SystTherapyType::RT,
    );
    // let prt1 = prt.clone();
    // print_fhir_data(prt1, "procedure-radiotherapy");

    let pop = procedure_svc::get_procedure(
        proc_op_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        sd.date_naive(),
        ed.date_naive(),
        SystTherapyType::OP,
    );
    // let pop1 = pop.clone();
    // print_fhir_data(pop1, "procedure-operation");

    let m = medication_svc::get_med_statement(
        med_stmt_id.as_str(),
        "medicine",
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        Faker.fake(),
        "2021-06-12",
        "2021-06-21",
    );
    // let m1 = m.clone();
    // print_fhir_data(m1, "medication statement");

    let b = assemble_bundle(
        bundle_id.as_str(),
        (pt, patient_ref_id.as_str()),
        (s, specimen_ref_id.as_str()),
        (c, condition_ref_id.as_str()),
        (ohist, obs_hist_ref_id.as_str()),
        (ovs, obs_vital_status_ref_id.as_str()),
        (otnmc, obs_tnmc_ref_id.as_str()),
        (pop, proc_op_ref_id.as_str()),
        (prt, proc_rt_ref_id.as_str()),
        (m, med_stmt_ref_id.as_str()),
    );
    // let b1 = b.clone();
    // print_fhir_data(b1, "bundle");

    b
}

fn assemble_bundle(
    id: &str,
    patient_tuple: (Patient, &str),
    specimen_tuple: (Specimen, &str),
    condition_tuple: (Condition, &str),
    obs_histology_tuple: (Observation, &str),
    obs_vital_status_tuple: (Observation, &str),
    obs_tnmc_tuple: (Observation, &str),
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
            condition_tuple
                .0
                .clone()
                .id
                .unwrap()
                .value
                .unwrap()
                .as_str(),
        )),
        resource: Some(Resource::Condition(Box::new(condition_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", condition_tuple.1).into_some(),
        ..Default::default()
    };

    let observation = BundleEntry {
        full_url: Some(get_full_url(
            obs_histology_tuple
                .0
                .clone()
                .id
                .unwrap()
                .value
                .unwrap()
                .as_str(),
        )),
        resource: Some(Resource::Observation(Box::new(
            obs_histology_tuple.0.clone(),
        ))),
        request: get_bundle_entry_request("PUT", obs_histology_tuple.1).into_some(),
        ..Default::default()
    };

    let vital_status = BundleEntry {
        full_url: Some(get_full_url(
            obs_vital_status_tuple
                .0
                .clone()
                .id
                .unwrap()
                .value
                .unwrap()
                .as_str(),
        )),
        resource: Some(Resource::Observation(Box::new(
            obs_vital_status_tuple.0.clone(),
        ))),
        request: get_bundle_entry_request("PUT", obs_vital_status_tuple.1).into_some(),
        ..Default::default()
    };

    let tnmc = BundleEntry {
        full_url: Some(get_full_url(
            obs_tnmc_tuple.0.clone().id.unwrap().value.unwrap().as_str(),
        )),
        resource: Some(Resource::Observation(Box::new(obs_tnmc_tuple.0.clone()))),
        request: get_bundle_entry_request("PUT", obs_tnmc_tuple.1).into_some(),
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
        resource: Some(Resource::MedicationStatement(Box::new(
            med_stmt_tuple.0.clone(),
        ))),
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
            tnmc,
            procedure,
            operation,
            med_stmt,
        ],
        ..Default::default()
    }
}

fn assemble_bundles(
    id: &str,
    patient_tuples: Vec<(Patient, &str)>,
    specimen_tuples: Vec<(Specimen, &str)>,
    condition_tuples: Vec<(Condition, &str)>,
    observation_tuples: Vec<(Observation, &str)>,
    procedure_tuples: Vec<(Procedure, &str)>,
    med_stmt_tuples: Vec<(MedicationStatement, &str)>,
) -> Vec<Bundle> {
    todo!()
}
