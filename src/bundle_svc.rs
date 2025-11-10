use chrono::prelude::*;

use fake::faker::chrono::en::DateTimeAfter;
use fake::{Fake, Faker};
use log::debug;

use crate::models::cli::ResourceType;
use crate::models::enums::id_type::IdType;
use crate::models::enums::syst_therapy_type::SystTherapyType;
use crate::utils::{get_ids, get_min_date_time};
use crate::{
    condition_svc, medication_svc, observation_svc, patient_svc, procedure_svc, specimen_svc,
};

use fhirbolt::model::r4b::resources::{
    Bundle, BundleEntry, Condition, MedicationStatement, Observation, Patient, Procedure, Specimen,
};
use fhirbolt::model::r4b::types::{Code, Id};

pub fn get_bundle() -> Bundle {
    debug!("get_bundle");

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

    let min_date_time = get_min_date_time();
    let effective_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();

    let start_date: DateTime<Utc> = DateTimeAfter(min_date_time).fake();
    let end_date: DateTime<Utc> = DateTimeAfter(start_date).fake();

    let (patient_src_id, _) = get_ids(IdType::Identifier, ResourceType::Patient, i);
    let pt = patient_svc::get_patient(patient_id.as_str(), patient_src_id.as_str());
    // let pt1 = pt.clone();
    // print_fhir_data(pt1, "patient");

    let s = specimen_svc::get_specimen(specimen_id.as_str(), patient_ref_id.as_str());
    // let s1 = s.clone();
    // print_fhir_data(s1, "specimen");

    let c = condition_svc::get_condition(
        condition_id.as_str(),
        patient_ref_id.as_str(),
        "C34.0",
        "C34.0",
    );
    // let c1 = c.clone();
    // print_fhir_data(c1, "condition");

    let ohist = observation_svc::get_histology(
        obs_hist_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        specimen_ref_id.as_str(),
        effective_date.date_naive(),
        "8140/3",
    );
    // let ohist1 = ohist.clone();
    // print_fhir_data(ohist1, "observation-histology");

    let ovs = observation_svc::get_vital_status(
        obs_vital_status_id.as_str(),
        patient_ref_id.as_str(),
        effective_date.date_naive(),
    );
    // let ovs1 = ovs.clone();
    // print_fhir_data(ovs1, "observation-vitalstatus");

    let otnmc = observation_svc::get_tnmc(
        &obs_tnmc_id.as_str(),
        patient_ref_id.as_str(),
        effective_date.date_naive(),
    );
    // let otnmc1 = otnmc.clone();
    // print_fhir_data(otnmc1, "observation-tnmc");

    let prt = procedure_svc::get_procedure(
        proc_rt_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        start_date.date_naive(),
        end_date.date_naive(),
        SystTherapyType::RT,
    );
    // let prt1 = prt.clone();
    // print_fhir_data(prt1, "procedure-radiotherapy");

    let pop = procedure_svc::get_procedure(
        proc_op_id.as_str(),
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        start_date.date_naive(),
        end_date.date_naive(),
        SystTherapyType::OP,
    );
    // let pop1 = pop.clone();
    // print_fhir_data(pop1, "procedure-operation");

    let m = medication_svc::get_med_statement(
        med_stmt_id.as_str(),
        "medicine",
        patient_ref_id.as_str(),
        condition_ref_id.as_str(),
        start_date.date_naive(),
        end_date.date_naive(),
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

pub fn get_condition_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    condition_tuple: (Condition, &str),
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let condition = condition_svc::get_bundle_entry(condition_tuple.0, condition_tuple.1);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: vec![patient, condition],
        ..Default::default()
    }
}

pub fn get_specimen_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    specimen_tuple: (Specimen, &str),
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let specimen = specimen_svc::get_bundle_entry(specimen_tuple.0, specimen_tuple.1);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: vec![patient, specimen],
        ..Default::default()
    }
}

pub fn get_observation_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    observation_tuple: (Observation, &str),
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let observation = observation_svc::get_bundle_entry(observation_tuple.0, observation_tuple.1);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: vec![patient, observation],
        ..Default::default()
    }
}

pub fn get_observation_histology_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    condition_tuple: (Condition, &str),
    specimen_tuple: (Specimen, &str),
    observation_tuple: (Observation, &str),
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let condition = condition_svc::get_bundle_entry(condition_tuple.0, condition_tuple.1);
    let specimen = specimen_svc::get_bundle_entry(specimen_tuple.0, specimen_tuple.1);
    let observation = observation_svc::get_bundle_entry(observation_tuple.0, observation_tuple.1);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: vec![patient, condition, specimen, observation],
        ..Default::default()
    }
}

pub fn get_procedure_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    condition_tuple: (Condition, &str),
    procedure_tuple: (Procedure, &str),
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let condition = condition_svc::get_bundle_entry(condition_tuple.0, condition_tuple.1);
    let procedure = procedure_svc::get_bundle_entry(procedure_tuple.0, procedure_tuple.1);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: vec![patient, condition, procedure],
        ..Default::default()
    }
}

pub fn get_med_stmt_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    condition_tuple: (Condition, &str),
    med_stmt_tuple: (MedicationStatement, &str),
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let condition = condition_svc::get_bundle_entry(condition_tuple.0, condition_tuple.1);
    let med_stmt = medication_svc::get_bundle_entry(med_stmt_tuple.0, med_stmt_tuple.1);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: vec![patient, condition, med_stmt],
        ..Default::default()
    }
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

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let specimen = specimen_svc::get_bundle_entry(specimen_tuple.0, specimen_tuple.1);
    let condition = condition_svc::get_bundle_entry(condition_tuple.0, condition_tuple.1);

    let observation =
        observation_svc::get_bundle_entry(obs_histology_tuple.0, obs_histology_tuple.1);
    let vital_status =
        observation_svc::get_bundle_entry(obs_vital_status_tuple.0, obs_vital_status_tuple.1);
    let tnmc = observation_svc::get_bundle_entry(obs_tnmc_tuple.0, obs_tnmc_tuple.1);

    let procedure = procedure_svc::get_bundle_entry(proc_rt_tuple.0, proc_rt_tuple.1);
    let operation = procedure_svc::get_bundle_entry(proc_op_tuple.0, proc_op_tuple.1);

    let med_stmt = medication_svc::get_bundle_entry(med_stmt_tuple.0, med_stmt_tuple.1);

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

pub fn get_patients_bundle(bundle_id: &str, patient_tuples: Vec<(Patient, String)>) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient_entries: Vec<BundleEntry> = patient_tuples
        .iter()
        .map(|pt_tuple| patient_svc::get_bundle_entry(pt_tuple.0.clone(), pt_tuple.1.as_str()))
        .collect();

    Bundle {
        id: Some(id),
        r#type: code,
        entry: patient_entries,
        ..Default::default()
    }
}

pub fn get_conditions_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    condition_tuples: Vec<(Condition, String)>,
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let condition_entries: Vec<BundleEntry> = condition_tuples
        .iter()
        .map(|c_tuple| condition_svc::get_bundle_entry(c_tuple.0.clone(), c_tuple.1.as_str()))
        .collect();

    let mut entries = vec![patient];
    entries.extend(condition_entries);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: entries,
        ..Default::default()
    }
}

pub fn get_specimens_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    specimen_tuples: Vec<(Specimen, String)>,
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let specimen_entries: Vec<BundleEntry> = specimen_tuples
        .iter()
        .map(|s_tuple| specimen_svc::get_bundle_entry(s_tuple.0.clone(), s_tuple.1.as_str()))
        .collect();

    let mut entries = vec![patient];
    entries.extend(specimen_entries);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: entries,
        ..Default::default()
    }
}

pub fn get_histologies_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    condition_tuple: (Condition, &str),
    specimen_tuple: (Specimen, &str),
    observation_tuples: Vec<(Observation, String)>,
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let condition = condition_svc::get_bundle_entry(condition_tuple.0, condition_tuple.1);
    let specimen = specimen_svc::get_bundle_entry(specimen_tuple.0, specimen_tuple.1);
    // let observation = observation_svc::get_bundle_entry(observation_tuple.0, observation_tuple.1);
    let hist_entries: Vec<BundleEntry> = observation_tuples
        .iter()
        .map(|hist_tuple| {
            observation_svc::get_bundle_entry(hist_tuple.0.clone(), hist_tuple.1.as_str())
        })
        .collect();

    let mut entries = vec![patient, condition, specimen];
    entries.extend(hist_entries);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: entries,
        ..Default::default()
    }
}

pub fn get_vital_statuses_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    obs_vs_tuples: Vec<(Observation, String)>,
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let vs_entries: Vec<BundleEntry> = obs_vs_tuples
        .iter()
        .map(|vs_tuple| observation_svc::get_bundle_entry(vs_tuple.0.clone(), vs_tuple.1.as_str()))
        .collect();

    let mut entries = vec![patient];
    entries.extend(vs_entries);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: entries,
        ..Default::default()
    }
}

pub fn get_tnmcs_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    obs_tnmc_tuples: Vec<(Observation, String)>,
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let vs_entries: Vec<BundleEntry> = obs_tnmc_tuples
        .iter()
        .map(|tnmc_tuple| {
            observation_svc::get_bundle_entry(tnmc_tuple.0.clone(), tnmc_tuple.1.as_str())
        })
        .collect();

    let mut entries = vec![patient];
    entries.extend(vs_entries);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: entries,
        ..Default::default()
    }
}

pub fn get_procedures_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    condition_tuple: (Condition, &str),
    procedure_tuples: Vec<(Procedure, String)>,
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let condition = condition_svc::get_bundle_entry(condition_tuple.0, condition_tuple.1);
    let proc_entries: Vec<BundleEntry> = procedure_tuples
        .iter()
        .map(|proc_tuple| {
            procedure_svc::get_bundle_entry(proc_tuple.0.clone(), proc_tuple.1.as_str())
        })
        .collect();

    let mut entries = vec![patient, condition];
    entries.extend(proc_entries);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: entries,
        ..Default::default()
    }
}

pub fn get_med_stmts_bundle(
    bundle_id: &str,
    patient_tuple: (Patient, &str),
    condition_tuple: (Condition, &str),
    med_stmt_tuples: Vec<(MedicationStatement, String)>,
) -> Bundle {
    let id = Id {
        value: Some(bundle_id.to_string()),
        ..Default::default()
    };
    let code = Code {
        value: Some("transaction".to_string()),
        ..Default::default()
    };

    let patient = patient_svc::get_bundle_entry(patient_tuple.0, patient_tuple.1);
    let condition = condition_svc::get_bundle_entry(condition_tuple.0, condition_tuple.1);
    let ms_entries: Vec<BundleEntry> = med_stmt_tuples
        .iter()
        .map(|ms_tuple| medication_svc::get_bundle_entry(ms_tuple.0.clone(), ms_tuple.1.as_str()))
        .collect();

    let mut entries = vec![patient, condition];
    entries.extend(ms_entries);

    Bundle {
        id: Some(id),
        r#type: code,
        entry: entries,
        ..Default::default()
    }
}
