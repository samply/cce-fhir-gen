mod data_gen_svc;
mod models;

use chrono::prelude::*;
use data_gen_svc::get_bundle;
use fake::faker::chrono::en::DateTimeAfter;
use fake::{Fake, Faker};
use fhirbolt::{serde::SerializeResource, xml};

fn main() {
    println!("Hello, world!");
    println!("");

    use_fhir_models();
}

fn use_fhir_models() {
    // Use Default::default() or constructing new resources by yourself
    let patient_id = "Patient-identifier-1";
    let patient_ref_id = "Patient/Patient-identifier-1";

    let condition_id = "Condition-identifier-1";
    let condition_ref_id = "Condition/Condition-identifier-1";

    let specimen_id = "Specimen-identifier-1";
    let specimen_ref_id = "Specimen/Specimen-identifier-1";

    let observation_id = "Histology-identifier-1";
    let observation_ref_id = "Observation/Histology-identifier-1";

    let procedure_id = "Procedure-identifier-1";
    let procedure_ref_id = "Procedure/Procedure-identifier-1";

    let med_stmt_id = "SystemicTherapy-identifier-1";
    let med_stmt_ref_id = "MedicationStatement/SystemicTherapy-identifier-1";

    // let sample_id = "Sample-identifier-1";
    // let operation_id = "Operation-identifier-1";

    let min_date_time = Utc.with_ymd_and_hms(1930, 1, 1, 0, 0, 0).unwrap();
    let bd: DateTime<Utc> = DateTimeAfter(min_date_time).fake();

    let pt = data_gen_svc::get_patient(patient_id, Faker.fake(), bd.date_naive(), Faker.fake());
    let pt1 = pt.clone();
    print_fhir_data(pt, "patient");

    let s = data_gen_svc::get_specimen(specimen_id, patient_ref_id, Faker.fake());
    let s1 = s.clone();
    print_fhir_data(s, "specimen");

    let c =
        data_gen_svc::get_condition(condition_id, patient_ref_id, "C34.0", "C34.0", Faker.fake());
    let c1 = c.clone();
    print_fhir_data(c, "condition");

    let o = data_gen_svc::get_observation(
        observation_id,
        patient_ref_id,
        condition_ref_id,
        Faker.fake(),
        "8140/3",
    );
    let o1 = o.clone();
    print_fhir_data(o, "observation");

    let ed: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let p = data_gen_svc::get_procedure(
        procedure_id,
        patient_ref_id,
        condition_ref_id,
        ed.date_naive(),
        Faker.fake(),
    );
    let p1 = p.clone();
    print_fhir_data(p, "procedure");

    let m = data_gen_svc::get_med_statement(
        med_stmt_id,
        "medicine",
        patient_ref_id,
        condition_ref_id,
        Faker.fake(),
        "2021-06-12",
        "2021-06-21",
    );
    let m1 = m.clone();
    print_fhir_data(m, "medication statement");

    let b = get_bundle(
        "752",
        pt1,
        patient_ref_id,
        s1,
        specimen_ref_id,
        c1,
        condition_ref_id,
        o1,
        observation_ref_id,
        p1,
        procedure_ref_id,
        m1,
        med_stmt_ref_id
    );
    print_fhir_data(b, "bundle");
}

fn print_fhir_data<T>(t: T, name: &str)
where
    T: SerializeResource,
{
    let serialized_data = xml::to_string(&t, None).unwrap();
    println!("{name}: {serialized_data:#?}");
    println!("");
}
