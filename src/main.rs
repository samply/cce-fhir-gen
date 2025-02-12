mod data_gen_svc;
mod extensions;
mod models;
mod utils;

use chrono::prelude::*;
use data_gen_svc::get_bundle;
use extensions::option_ext::OptionExt;
use fake::faker::chrono::en::DateTimeAfter;
use fake::{Fake, Faker};
use utils::{get_ids, print_fhir_data};

fn main() {
    println!("Hello, world!");
    println!("");

    use_fhir_models();
}

fn use_fhir_models() {
    // Use Default::default() or constructing new resources by yourself
    let i: i8 = Faker.fake();
    let (bundle_id, _) = get_ids(None, "Bundle", i);
    let (patient_id, patient_ref_id) = get_ids(None, "Patient", i);
    let (condition_id, condition_ref_id) = get_ids(None, "Condition", i);
    let (specimen_id, specimen_ref_id) = get_ids(None, "Specimen", i);
    let (observation_id, observation_ref_id) = get_ids("Observation".into_some(), "Histology", i);
    let (procedure_id, procedure_ref_id) = get_ids(None, "Procedure", i);
    let (med_stmt_id, med_stmt_ref_id) =
        get_ids("MedicationStatement".into_some(), "SystemicTherapy", i);

    let min_date_time = Utc.with_ymd_and_hms(1930, 1, 1, 0, 0, 0).unwrap();
    let bd: DateTime<Utc> = DateTimeAfter(min_date_time).fake();

    let pt = data_gen_svc::get_patient(
        patient_id.as_str(),
        Faker.fake(),
        bd.date_naive(),
        Faker.fake(),
    );
    let pt1 = pt.clone();
    print_fhir_data(pt, "patient");

    let s = data_gen_svc::get_specimen(specimen_id.as_str(), patient_ref_id.as_str(), Faker.fake());
    let s1 = s.clone();
    print_fhir_data(s, "specimen");

    let c = data_gen_svc::get_condition(
        condition_id.as_str(),
        patient_ref_id.as_str(),
        "C34.0",
        "C34.0",
        Faker.fake(),
    );
    let c1 = c.clone();
    print_fhir_data(c, "condition");

    let o = data_gen_svc::get_observation(
        observation_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        Faker.fake(),
        "8140/3",
    );
    let o1 = o.clone();
    print_fhir_data(o, "observation");

    let ed: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let p = data_gen_svc::get_procedure(
        procedure_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        ed.date_naive(),
        Faker.fake(),
    );
    let p1 = p.clone();
    print_fhir_data(p, "procedure");

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
    print_fhir_data(m, "medication statement");

    let b = get_bundle(
        bundle_id.as_str(),
        (pt1, patient_ref_id.as_str()),
        (s1, specimen_ref_id.as_str()),
        (c1, condition_ref_id.as_str()),
        (o1, observation_ref_id.as_str()),
        (p1, procedure_ref_id.as_str()),
        (m1, med_stmt_ref_id.as_str()),
    );
    print_fhir_data(b, "bundle");
}
