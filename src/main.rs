mod bundle_svc;
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
use fake::faker::chrono::en::DateTimeAfter;
use fake::{Fake, Faker};
use fhirbolt::serde::xml;
use log::info;
use models::cli::{CliArgs, OutputMode, ResourceType};
use models::enums::id_type::IdType;
use models::enums::syst_therapy_type::SystTherapyType;
use utils::get_ids;

const DATA_FOLDER: &str = "generated-data";
// const PROXY_URL: &str = "";

fn main() {
    // initialize colored logger to level Info (change this to Debug for seeing debug stmts in output)
    let mut colog = colog::default_builder();
    colog.filter(None, log::LevelFilter::Info);
    colog.init();

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

    if cli.number > 1 {
        info!("generating a single bundle containing multiple {:?}...", cli.resource_type);
        generate_fhir_bundles(cli.number, cli.resource_type, cli.output_mode);
    } else {
        if cli.resource_type == ResourceType::Bundle {
            info!("generating a single bundle containing all resource types...");
        } else {
            info!("generating a single bundle containing a {:?}...", cli.resource_type);
        }
        generate_fhir_bundle(cli.resource_type, cli.output_mode);
    }
}

fn generate_fhir_bundle(resource_type: ResourceType, output_mode: OutputMode) {
    info!("generate_fhir_bundle");

    let i: u16 = Faker.fake();

    let (bundle_id, _) = get_ids(IdType::Id, ResourceType::Bundle, i);
    let (patient_id, patient_ref_id) = get_ids(IdType::Id, ResourceType::Patient, i);
    let (condition_id, condition_ref_id) = get_ids(IdType::Id, ResourceType::Condition, i);
    let (specimen_id, specimen_ref_id) = get_ids(IdType::Id, ResourceType::Specimen, i);
    let (obs_hist_id, obs_hist_ref_id) = get_ids(IdType::Id, ResourceType::ObservationHistology, i);
    let (obs_vital_status_id, obs_vital_status_ref_id) =
        get_ids(IdType::Id, ResourceType::ObservationVitalStatus, i);
    let (obs_tnmc_id, obs_tnmc_ref_id) = get_ids(IdType::Id, ResourceType::ObservationTNMc, i);
    let (proc_rt_id, proc_rt_ref_id) = get_ids(IdType::Id, ResourceType::ProcedureRadiotherapy, i);
    let (proc_op_id, proc_op_ref_id) = get_ids(IdType::Id, ResourceType::ProcedureOperation, i);
    let (med_stmt_id, med_stmt_ref_id) = get_ids(
        IdType::Id,
        ResourceType::SystemicTherapyMedicationStatement,
        i,
    );

    let min_date_time = Utc.with_ymd_and_hms(1930, 1, 1, 0, 0, 0).unwrap();
    let effective_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();

    let start_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let end_date: DateTime<Utc> = DateTimeAfter(start_date).fake();

    let (xml_data, file_name) = match resource_type {
        ResourceType::Patient => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let b = bundle_svc::get_patients_bundle(&bundle_id, vec![(pt, patient_ref_id)]);
            (utils::get_xml(b, "patient (bundle)"), patient_id)
        }

        ResourceType::Condition => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
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
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
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
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
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
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
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
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
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
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
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
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
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

        ResourceType::SystemicTherapyMedicationStatement => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
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
                start_date.date_naive(),
                end_date.date_naive(),
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

fn generate_fhir_bundles(number: u8, resource_type: ResourceType, output_mode: OutputMode) {
    info!("generate_fhir_bundles");

    let range = 0..number;
    let i: u16 = Faker.fake();

    let (bundle_id, _) = get_ids(IdType::Id, ResourceType::Bundle, i);
    let (patient_id, patient_ref_id) = get_ids(IdType::Id, ResourceType::Patient, i);
    let (condition_id, condition_ref_id) = get_ids(IdType::Id, ResourceType::Condition, i);
    let (specimen_id, specimen_ref_id) = get_ids(IdType::Id, ResourceType::Specimen, i);
    let (obs_hist_id, _) = get_ids(IdType::Id, ResourceType::ObservationHistology, i);
    let (obs_vital_status_id, _) = get_ids(IdType::Id, ResourceType::ObservationVitalStatus, i);
    let (obs_tnmc_id, _) = get_ids(IdType::Id, ResourceType::ObservationTNMc, i);
    let (proc_rt_id, _) = get_ids(IdType::Id, ResourceType::ProcedureRadiotherapy, i);
    let (proc_op_id, _) = get_ids(IdType::Id, ResourceType::ProcedureOperation, i);
    let (med_stmt_id, _) = get_ids(
        IdType::Id,
        ResourceType::SystemicTherapyMedicationStatement,
        i,
    );

    let min_date_time = Utc.with_ymd_and_hms(1930, 1, 1, 0, 0, 0).unwrap();
    let effective_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();

    let start_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let end_date: DateTime<Utc> = DateTimeAfter(start_date).fake();

    let (bundle, file_name) = match resource_type {
        ResourceType::Patient => {
            let patient_tuples = patient_svc::get_patients(range);
            let b = bundle_svc::get_patients_bundle(bundle_id.as_str(), patient_tuples);
            (b, patient_id)
        }

        ResourceType::Condition => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let condition_tuples =
                condition_svc::get_conditions(patient_ref_id.as_str(), "C34.0", "C34.0", range);
            let b = bundle_svc::get_conditions_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                condition_tuples,
            );
            (b, condition_id)
        }

        ResourceType::Specimen => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let specimen_tuples = specimen_svc::get_specimens(patient_ref_id.as_str(), range);
            let b = bundle_svc::get_specimens_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                specimen_tuples,
            );
            (b, specimen_id)
        }

        ResourceType::ObservationHistology => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            let c = condition_svc::get_condition(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
            );
            let s = specimen_svc::get_specimen(specimen_id.as_str(), patient_ref_id.as_str());
            let hist_tuples = observation_svc::get_histologies(
                patient_ref_id.as_str(),
                condition_ref_id.as_str(),
                specimen_ref_id.as_str(),
                effective_date.date_naive(),
                "8140/3",
                range,
            );
            let b = bundle_svc::get_histologies_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (c, condition_ref_id.as_str()),
                (s, specimen_ref_id.as_str()),
                hist_tuples,
            );
            (b, obs_hist_id)
        }

        ResourceType::ObservationVitalStatus => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let vital_status_tuples = observation_svc::get_vital_statuses(
                patient_ref_id.as_str(),
                effective_date.date_naive(),
                range,
            );
            let b = bundle_svc::get_vital_statuses_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                vital_status_tuples,
            );
            (b, obs_vital_status_id)
        }

        ResourceType::ObservationTNMc => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());

            let tnmc_tuples = observation_svc::get_tnmcs(
                patient_ref_id.as_str(),
                effective_date.date_naive(),
                range,
            );
            let b = bundle_svc::get_tnmcs_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                tnmc_tuples,
            );
            (b, obs_tnmc_id)
        }

        ResourceType::ProcedureRadiotherapy => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            let c = condition_svc::get_condition(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
            );
            let prt_tuples = procedure_svc::get_proc_radio_therapies(
                patient_ref_id.as_str(),
                condition_ref_id.as_str(),
                start_date.date_naive(),
                end_date.date_naive(),
                range,
            );
            let b = bundle_svc::get_procedures_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (c, condition_ref_id.as_str()),
                prt_tuples,
            );
            (b, proc_rt_id)
        }

        ResourceType::ProcedureOperation => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            let c = condition_svc::get_condition(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
            );
            let pop_tuples = procedure_svc::get_proc_operations(
                patient_ref_id.as_str(),
                condition_ref_id.as_str(),
                start_date.date_naive(),
                end_date.date_naive(),
                range,
            );
            let b = bundle_svc::get_procedures_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (c, condition_ref_id.as_str()),
                pop_tuples,
            );
            (b, proc_op_id)
        }

        ResourceType::SystemicTherapyMedicationStatement => {
            let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
            let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
            let c = condition_svc::get_condition(
                condition_id.as_str(),
                patient_ref_id.as_str(),
                "C34.0",
                "C34.0",
            );
            let med_stmt_tuples = medication_svc::get_med_statements(
                patient_ref_id.as_str(),
                condition_ref_id.as_str(),
                start_date.date_naive(),
                end_date.date_naive(),
                range,
            );
            let b = bundle_svc::get_med_stmts_bundle(
                &bundle_id,
                (pt, patient_ref_id.as_str()),
                (c, condition_ref_id.as_str()),
                med_stmt_tuples,
            );
            (b, med_stmt_id)
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

// fn get_client() -> Client {
//     let proxy = get_proxy();
//     Client::builder().proxy(proxy).build().unwrap()
// }

// fn get_proxy() -> Proxy {
//     Proxy::all(PROXY_URL).unwrap()
// }
