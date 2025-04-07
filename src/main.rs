mod bundle_svc;
mod cli;
mod condition_svc;
mod extensions;
mod medication_svc;
mod models;
mod observation_svc;
mod patient_svc;
mod procedure_svc;
mod specimen_svc;
mod utils;

use std::fs;

use chrono::prelude::*;
use clap::Parser;
use cli::args::{CliArgs, OutputMode, ResourceType};
use extensions::option_ext::OptionExt;
use fake::faker::chrono::en::DateTimeAfter;
use fake::{Fake, Faker};
use fhirbolt::serde::xml;
use models::enums::id_type::IdType;
use models::enums::syst_therapy_type::SystTherapyType;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::Proxy;
use utils::{get_ids, print_fhir_data};

const DATA_FOLDER: &str = "generated-data";
const PROXY_URL: &str = "";

fn main() {
    let cli = CliArgs::parse();

    let file_msg = format!("write to a file in /{}", DATA_FOLDER);
    let storage = match cli.output_mode {
        OutputMode::Screen => "show on terminal",
        OutputMode::File => file_msg.as_str(),
        OutputMode::ApiCall => "call API endpoint (WIP)",
    };

    println!(
        "Generating {} {:?} and {}...",
        cli.number, cli.resource_type, storage
    );
    println!("");

    // TODO: directly post a request to an endpoint
    // TODO: for curl, we need a server name, user name, pwd, proxy url
    generate_fhir_bundles(cli.number, cli.resource_type, cli.output_mode);
}

fn generate_fhir_bundle(resource_type: ResourceType, output_mode: OutputMode) {
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
    let effective_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();

    let start_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let end_date: DateTime<Utc> = DateTimeAfter(start_date).fake();

    let (xml_data, file_name) = match resource_type {
        ResourceType::Patient => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            (utils::get_xml(pt, "patient"), patient_id)
        }

        ResourceType::Condition => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let c = condition_svc::get_condition(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
            );
            let b = bundle_svc::get_condition_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (c, condition_ref_id.as_str()),
            );
            (utils::get_xml(b, "condition (bundle)"), condition_id)
        }

        ResourceType::Specimen => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let s = specimen_svc::get_specimen(specimen_id.as_str(), patient_ref_id.as_str());
            let b = bundle_svc::get_specimen_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (s, specimen_ref_id.as_str()),
            );
            (utils::get_xml(b, "specimen (bundle)"), specimen_id)
        }

        ResourceType::ObservationHistology => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            let c = condition_svc::get_condition(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
            );
            let s = specimen_svc::get_specimen(specimen_id.as_str(), patient_ref_id.as_str());
            let ohist = observation_svc::get_histology(
                obs_hist_id.as_str(),
                patient_ref_id.as_str(),
                condition_ref_id.as_str(),
                specimen_ref_id.as_str(),
                effective_date.date_naive(),
                "8140/3",
            );
            let b = bundle_svc::get_observation_histology_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (c, condition_ref_id.as_str()),
                (s, specimen_ref_id.as_str()),
                (ohist, obs_hist_ref_id.as_str()),
            );
            (
                utils::get_xml(b, "observation histology (bundle)"),
                obs_hist_id,
            )
        }

        ResourceType::ObservationVitalStatus => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let ovs = observation_svc::get_vital_status(
                obs_vital_status_id.as_str(),
                patient_ref_id.as_str(),
                effective_date.date_naive(),
            );
            let b = bundle_svc::get_observation_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (ovs, obs_vital_status_ref_id.as_str()),
            );
            (
                utils::get_xml(b, "observation vital-status (bundle)"),
                obs_vital_status_id,
            )
        }

        ResourceType::ObservationTNMc => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let otnmc = observation_svc::get_tnmc(
                &obs_tnmc_id.as_str(),
                patient_ref_id.as_str(),
                effective_date.date_naive(),
            );
            let b = bundle_svc::get_observation_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (otnmc, obs_tnmc_ref_id.as_str()),
            );
            (utils::get_xml(b, "observation tnmc (bundle)"), obs_tnmc_id)
        }

        ResourceType::ProcedureRadiotherapy => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            let c = condition_svc::get_condition(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
            );
            let prt = procedure_svc::get_procedure(
                proc_rt_id.as_str(),
                patient_ref_id.as_str(),
                condition_ref_id.as_str(),
                start_date.date_naive(),
                end_date.date_naive(),
                SystTherapyType::RT,
            );
            let b = bundle_svc::get_procedure_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (c, condition_ref_id.as_str()),
                (prt, proc_rt_ref_id.as_str()),
            );
            (
                utils::get_xml(b, "procedure radiotherapy (bundle)"),
                proc_rt_id,
            )
        }

        ResourceType::ProcedureOperation => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            let c = condition_svc::get_condition(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
            );
            let pop = procedure_svc::get_procedure(
                proc_op_id.as_str(),
                patient_ref_id.as_str(),
                condition_ref_id.as_str(),
                start_date.date_naive(),
                end_date.date_naive(),
                SystTherapyType::OP,
            );
            let b = bundle_svc::get_procedure_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (c, condition_ref_id.as_str()),
                (pop, proc_op_ref_id.as_str()),
            );
            (
                utils::get_xml(b, "procedure operation (bundle)"),
                proc_op_id,
            )
        }

        ResourceType::MedicationStatement => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            let c = condition_svc::get_condition(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
            );
            let m = medication_svc::get_med_statement(
                med_stmt_id.as_str(),
                "medicine",
                patient_ref_id.as_str(),
                condition_ref_id.as_str(),
                "2021-06-12",
                "2021-06-21",
            );
            let b = bundle_svc::get_med_stmt_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (c, condition_ref_id.as_str()),
                (m, med_stmt_ref_id.as_str()),
            );
            (utils::get_xml(b, "medication stmt (bundle)"), med_stmt_id)
        }

        ResourceType::Bundle => {
            let b = bundle_svc::get_bundle();
            (
                xml::to_string(&b, None).unwrap_or("Cannot serialize bundle to XML.".to_string()),
                bundle_id,
            )
        }
    };

    match output_mode {
        OutputMode::Screen => {
            println!("{}:", resource_type.as_str());
            println!("{xml_data}");
            println!();
        }

        OutputMode::File => {
            let dir_path = format!("./{DATA_FOLDER}");
            if fs::exists(&dir_path).expect("dir exists error") {
                println!("{} already exists.", dir_path);
            } else {
                println!("creating {}.", &dir_path);
                fs::create_dir(&dir_path).expect("failed to create dir");
            }

            let with_extn = format!("{}.xml", file_name);
            let file_path = format!("{}/{}", &dir_path, with_extn);
            fs::write(file_path, xml_data).expect("Unable to create XML file");
        }

        OutputMode::ApiCall => todo!(),
    }
}

fn generate_fhir_bundle_mult(number: u8, resource_type: ResourceType, output_mode: OutputMode) {
    let range = 0..number;
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
    let effective_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();

    let start_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let end_date: DateTime<Utc> = DateTimeAfter(start_date).fake();

    let (bundle, file_name) = match resource_type {
        ResourceType::Patient => {
            let patients_tuple = patient_svc::get_patients(range);
            let b = bundle_svc::get_patients_bundle(bundle_id.as_str(), patients_tuple);
            (b, patient_id)
        }

        ResourceType::Condition => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let conditions = condition_svc::get_conditions(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
                range,
            );
            let b = bundle_svc::get_conditions_bundle(
                &bundle_id,
                (conditions, condition_ref_id.as_str()),
            );
            (b, condition_id)
        }

        ResourceType::Specimen => {
            let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let specimens =
                specimen_svc::get_specimens(specimen_id.as_str(), patient_ref_id.as_str(), range);
            let b =
                bundle_svc::get_specimens_bundle(&bundle_id, (specimens, specimen_ref_id.as_str()));
            (b, specimen_id)
        }

        ResourceType::ObservationHistology => {
            todo!()
            // let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            // let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            // let c = condition_svc::get_condition(
            //     condition_id.as_str(),
            //     patient_ref_id.as_str(),
            //     "C34.0",
            //     "C34.0",
            // );
            // let s = specimen_svc::get_specimen(specimen_id.as_str(), patient_ref_id.as_str());
            // let ohist = observation_svc::get_histology(
            //     obs_hist_id.as_str(),
            //     patient_ref_id.as_str(),
            //     condition_ref_id.as_str(),
            //     specimen_ref_id.as_str(),
            //     effective_date.date_naive(),
            //     "8140/3",
            // );
            // let b = bundle_svc::get_observation_histology_bundle(
            //     &bundle_id,
            //     (pt, patient_ref_id.as_str()),
            //     (c, condition_ref_id.as_str()),
            //     (s, specimen_ref_id.as_str()),
            //     (ohist, obs_hist_ref_id.as_str()),
            // );
            // (
            //     utils::get_xml(b, "observation histology (bundle)"),
            //     obs_hist_id,
            // )
        }

        ResourceType::ObservationVitalStatus => {
            todo!()
            // let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            // let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            // let ovs = observation_svc::get_vital_status(
            //     obs_vital_status_id.as_str(),
            //     patient_ref_id.as_str(),
            //     effective_date.date_naive(),
            // );
            // let b = bundle_svc::get_observation_bundle(
            //     &bundle_id,
            //     (pt, patient_ref_id.as_str()),
            //     (ovs, obs_vital_status_ref_id.as_str()),
            // );
            // (
            //     utils::get_xml(b, "observation vital-status (bundle)"),
            //     obs_vital_status_id,
            // )
        }

        ResourceType::ObservationTNMc => {
            todo!()
            // let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            // let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            // let otnmc = observation_svc::get_tnmc(
            //     &obs_tnmc_id.as_str(),
            //     patient_ref_id.as_str(),
            //     effective_date.date_naive(),
            // );
            // let b = bundle_svc::get_observation_bundle(
            //     &bundle_id,
            //     (pt, patient_ref_id.as_str()),
            //     (otnmc, obs_tnmc_ref_id.as_str()),
            // );
            // (utils::get_xml(b, "observation tnmc (bundle)"), obs_tnmc_id)
        }

        ResourceType::ProcedureRadiotherapy => {
            todo!()
            // let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            // let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            // let c = condition_svc::get_condition(
            //     condition_id.as_str(),
            //     patient_ref_id.as_str(),
            //     "C34.0",
            //     "C34.0",
            // );
            // let prt = procedure_svc::get_procedure(
            //     proc_rt_id.as_str(),
            //     patient_ref_id.as_str(),
            //     condition_ref_id.as_str(),
            //     start_date.date_naive(),
            //     end_date.date_naive(),
            //     SystTherapyType::RT,
            // );
            // let b = bundle_svc::get_procedure_bundle(
            //     &bundle_id,
            //     (pt, patient_ref_id.as_str()),
            //     (c, condition_ref_id.as_str()),
            //     (prt, proc_rt_ref_id.as_str()),
            // );
            // (
            //     utils::get_xml(b, "procedure radiotherapy (bundle)"),
            //     proc_rt_id,
            // )
        }

        ResourceType::ProcedureOperation => {
            todo!()
            // let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            // let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            // let c = condition_svc::get_condition(
            //     condition_id.as_str(),
            //     patient_ref_id.as_str(),
            //     "C34.0",
            //     "C34.0",
            // );
            // let pop = procedure_svc::get_procedure(
            //     proc_op_id.as_str(),
            //     patient_ref_id.as_str(),
            //     condition_ref_id.as_str(),
            //     start_date.date_naive(),
            //     end_date.date_naive(),
            //     SystTherapyType::OP,
            // );
            // let b = bundle_svc::get_procedure_bundle(
            //     &bundle_id,
            //     (pt, patient_ref_id.as_str()),
            //     (c, condition_ref_id.as_str()),
            //     (pop, proc_op_ref_id.as_str()),
            // );
            // (
            //     utils::get_xml(b, "procedure operation (bundle)"),
            //     proc_op_id,
            // )
        }

        ResourceType::MedicationStatement => {
            todo!()
            // let (patient_src_id, _) = get_ids(None, IdType::Identifier, "Patient", i);
            // let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            // let c = condition_svc::get_condition(
            //     condition_id.as_str(),
            //     patient_ref_id.as_str(),
            //     "C34.0",
            //     "C34.0",
            // );
            // let m = medication_svc::get_med_statement(
            //     med_stmt_id.as_str(),
            //     "medicine",
            //     patient_ref_id.as_str(),
            //     condition_ref_id.as_str(),
            //     "2021-06-12",
            //     "2021-06-21",
            // );
            // let b = bundle_svc::get_med_stmt_bundle(
            //     &bundle_id,
            //     (pt, patient_ref_id.as_str()),
            //     (c, condition_ref_id.as_str()),
            //     (m, med_stmt_ref_id.as_str()),
            // );
            // (utils::get_xml(b, "medication stmt (bundle)"), med_stmt_id)
        }

        ResourceType::Bundle => {
            todo!()
            // let b = bundle_svc::get_bundle();
            // (
            //     xml::to_string(&b, None).unwrap_or("Cannot serialize bundle to XML.".to_string()),
            //     bundle_id,
            // )
        }
    };

    let bundle_xml = utils::get_xml(bundle, "bundle");
    match output_mode {
        OutputMode::Screen => {
            println!("{}:", resource_type.as_str());
            println!("{bundle_xml}");
            println!();
        }

        OutputMode::File => {
            let dir_path = format!("./{DATA_FOLDER}");
            if fs::exists(&dir_path).expect("dir exists error") {
                println!("{} already exists.", dir_path);
            } else {
                println!("creating {}.", &dir_path);
                fs::create_dir(&dir_path).expect("failed to create dir");
            }

            let with_extn = format!("{}.xml", file_name);
            let file_path = format!("{}/{}", &dir_path, with_extn);
            fs::write(file_path, bundle_xml).expect("Unable to create XML file");
        }

        OutputMode::ApiCall => todo!(),
    }
}

fn generate_fhir_bundles(number: u8, resource_type: ResourceType, output_mode: OutputMode) {
    if number > 1 {
        generate_fhir_bundle_mult(number, resource_type, output_mode);
    } else {
        generate_fhir_bundle(resource_type, output_mode);
    }
}

fn get_client() -> Client {
    let proxy = get_proxy();
    Client::builder().proxy(proxy).build().unwrap()
}

fn get_proxy() -> Proxy {
    Proxy::all(PROXY_URL).unwrap()
}
