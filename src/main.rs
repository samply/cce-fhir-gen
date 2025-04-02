mod cli;
mod data_gen_svc;
mod extensions;
mod models;
mod observation_svc;
mod procedure_svc;
mod utils;

use std::fs;

use chrono::prelude::*;
use cli::args::{CliArgs, OutputMode};
use clap::Parser;
use data_gen_svc::get_bundle;
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

    let msg = format!("write to a file in /{}", DATA_FOLDER);
    let storage = match cli.output_mode {
        OutputMode::Screen => "show on terminal",
        OutputMode::File => msg.as_str(),
        OutputMode::ApiCall => "call API endpoint (WIP)",
    };
    
    println!("Generating {} {:?} and {}...", cli.number, cli.resource_type, storage);
    println!("");

    // TODO: directly post a request to an endpoint
    // TODO: for curl, we need a server name, user name, pwd, proxy url
    generate_fhir_bundles(cli.number, cli.output_mode);
}

fn generate_fhir_bundles(number: u8, output_mode: OutputMode) {
    let range = 1..(number + 1);
    // println!("current dir: {}", env::current_dir().unwrap().display());

    let server_name = "";
    let user_name = "";
    let pwd = "";

    let mut client: Option<Client> = None;
    if output_mode == OutputMode::ApiCall {
        // let http_proxy = reqwest::Proxy::http(http_proxy_svr).unwrap();
        // let https_proxy = reqwest::Proxy::http(https_proxy_svr).unwrap();
        client = Some(get_client());
        // client = Some(Client::new());
    }

    for _i in range {
        // Use Default::default() or constructing new resources by yourself
        let i: u16 = Faker.fake();
        let (bundle_id, _) = get_ids(None, IdType::Id, "Bundle", i);
        let (patient_id, patient_ref_id) = get_ids(None, IdType::Id, "Patient", i);
        let (condition_id, condition_ref_id) = get_ids(None, IdType::Id, "Condition", i);
        let (specimen_id, specimen_ref_id) = get_ids(None, IdType::Id, "Specimen", i);
        let (obs_hist_id, obs_hist_ref_id) =
            get_ids("Observation".into_some(), IdType::Id, "Histology", i);
        let (obs_vital_status_id, obs_vital_status_ref_id) =
            get_ids("Observation".into_some(), IdType::Id, "VitalStatus", i);
        let (obs_tnmc_id, obs_tnmc_ref_id) =
            get_ids("Observation".into_some(), IdType::Id, "TNMc", i);
        let (proc_rt_id, proc_rt_ref_id) =
            get_ids("Procedure".into_some(), IdType::Id, "Radiotherapy", i);
        let (proc_op_id, proc_op_ref_id) =
            get_ids("Procedure".into_some(), IdType::Id, "Operation", i);
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
        let pt = data_gen_svc::get_patient(
            patient_id.as_str(),
            patient_src_id.as_str(),
            Faker.fake(),
            bd.date_naive(),
            Faker.fake(),
        );
        // let pt1 = pt.clone();
        // print_fhir_data(pt1, "patient");

        let s =
            data_gen_svc::get_specimen(specimen_id.as_str(), patient_ref_id.as_str(), Faker.fake());
        // let s1 = s.clone();
        // print_fhir_data(s1, "specimen");

        let c = data_gen_svc::get_condition(
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

        let m = data_gen_svc::get_med_statement(
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

        let b = get_bundle(
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
        let data =
            xml::to_string(&b, None).unwrap_or("Cannot serialize bundle to XML.".to_string());

        match output_mode {
            OutputMode::Screen => print_fhir_data(b, "bundle"),

            OutputMode::File => {
                let dir_path = format!("./{DATA_FOLDER}");
                if fs::exists(&dir_path).expect("dir exists error") {
                    println!("{} already exists.", dir_path);
                } else {
                    println!("creating {}.", &dir_path);
                    fs::create_dir(&dir_path).expect("failed to create dir");
                }

                let file_name = format!("Bundle-{}.xml", i);
                let file_path = format!("{}/{}", &dir_path, file_name);
                fs::write(file_path, data).expect("Unable to create XML file");
            }

            OutputMode::ApiCall => {
                let url = utils::get_bh_fhir_api_url(server_name);
                println!("api url: {}", &url);
                let res = client
                    .as_mut()
                    .unwrap()
                    .post(url)
                    .basic_auth(user_name, Some(pwd))
                    .header(ACCEPT, "application/xml")
                    .header(CONTENT_TYPE, "application/xml") // this fixed it
                    .header(USER_AGENT, "Rust-Reqwest-App")
                    .body(data)
                    .send();

                match res {
                    Ok(resp) => println!("Successfully posted bundle({}): {}", i, resp.status().as_str()),
                    Err(e) => println!("Error in posting bundle({}): connect: {}, body: {}, timeout: {}", i, e.is_connect(), e.is_body(), e.is_timeout()),
                }
            }
        }
    }
}

fn get_client() -> Client {
    let proxy = get_proxy();
    Client::builder().proxy(proxy).build().unwrap()
}

fn get_proxy() -> Proxy {
    Proxy::all(PROXY_URL).unwrap()
}
