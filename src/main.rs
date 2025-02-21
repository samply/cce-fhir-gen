mod data_gen_svc;
mod extensions;
mod models;
mod observation_svc;
mod procedure_svc;
mod utils;

use chrono::prelude::*;
use data_gen_svc::get_bundle;
use extensions::option_ext::OptionExt;
use fake::faker::chrono::en::DateTimeAfter;
use fake::{Fake, Faker};
use models::enums::id_type::IdType;
use utils::{get_ids, print_fhir_data};

fn main() {
    println!("Hello, world!");
    println!("");

    use_fhir_models();
}

fn use_fhir_models() {
    // Use Default::default() or constructing new resources by yourself
    let i: i8 = Faker.fake();
    let (bundle_id, _) = get_ids(None, IdType::Id, "Bundle", i);
    let (patient_id, patient_ref_id) = get_ids(None, IdType::Id, "Patient", i);
    let (condition_id, condition_ref_id) = get_ids(None, IdType::Id, "Condition", i);
    let (specimen_id, specimen_ref_id) = get_ids(None, IdType::Id, "Specimen", i);
    let (observation_id, observation_ref_id) =
        get_ids("Observation".into_some(), IdType::Id, "Histology", i);
    let (vital_status_id, vital_status_ref_id) =
        get_ids("Observation".into_some(), IdType::Id, "VitalStatus", i);
    let (procedure_id, procedure_ref_id) =
        get_ids("Procedure".into_some(), IdType::Id, "Radiotherapy", i);
    let (operation_id, operation_ref_id) =
        get_ids("Procedure".into_some(), IdType::Id, "Operation", i);
    let (med_stmt_id, med_stmt_ref_id) = get_ids(
        "MedicationStatement".into_some(),
        IdType::Id,
        "SystemicTherapy",
        i,
    );

    let min_date_time = Utc.with_ymd_and_hms(1930, 1, 1, 0, 0, 0).unwrap();
    let bd: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let od: DateTime<Utc> = DateTimeAfter(min_date_time).fake();

    let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
    let pt = data_gen_svc::get_patient(
        patient_id.as_str(),
        patient_src_id.as_str(),
        Faker.fake(),
        bd.date_naive(),
        Faker.fake(),
    );
    let pt1 = pt.clone();
    print_fhir_data(pt1, "patient");

    let s = data_gen_svc::get_specimen(specimen_id.as_str(), patient_ref_id.as_str(), Faker.fake());
    let s1 = s.clone();
    print_fhir_data(s1, "specimen");

    let c = data_gen_svc::get_condition(
        condition_id.as_str(),
        patient_ref_id.as_str(),
        "C34.0",
        "C34.0",
        Faker.fake(),
    );
    let c1 = c.clone();
    print_fhir_data(c1, "condition");

    let o = observation_svc::get_observation(
        observation_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        od.date_naive(),
        "8140/3",
    );
    let o1 = o.clone();
    print_fhir_data(o1, "observation-histology");

    let v = observation_svc::get_vital_status(
        vital_status_id.as_str(),
        patient_ref_id.as_str(),
        od.date_naive(),
        Faker.fake(),
    );
    let v1 = v.clone();
    print_fhir_data(v1, "observation-vitalstatus");

    let ed: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let prt = procedure_svc::get_procedure(
        procedure_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        ed.date_naive(),
        models::enums::syst_therapy_type::SystTherapyType::RT,
    );
    let prt1 = prt.clone();
    print_fhir_data(prt1, "procedure-radiotherapy");

    let pop = procedure_svc::get_procedure(
        operation_id.as_str(),
        operation_ref_id.as_str(),
        condition_ref_id.as_str(),
        ed.date_naive(),
        models::enums::syst_therapy_type::SystTherapyType::OP,
    );
    let pop1 = pop.clone();
    print_fhir_data(pop1, "procedure-operation");

    let m = data_gen_svc::get_med_statement(
        med_stmt_id.as_str(),
        "medicine",
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        Faker.fake(),
        "2021-06-12",
        "2021-06-21",
    );
    let m1 = m.clone();
    print_fhir_data(m1, "medication statement");

    let b = get_bundle(
        bundle_id.as_str(),
        (pt, patient_ref_id.as_str()),
        (s, specimen_ref_id.as_str()),
        (c, condition_ref_id.as_str()),
        (o, observation_ref_id.as_str()),
        (v, vital_status_ref_id.as_str()),
        (prt, procedure_ref_id.as_str()),
        (pop, operation_ref_id.as_str()),
        (m, med_stmt_ref_id.as_str()),
    );
    print_fhir_data(b, "bundle");
}
