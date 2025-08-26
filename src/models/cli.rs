use clap::{command, Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputMode {
    /// Show the generated XML in the terminal
    #[default]
    Screen,

    /// Store the generated XML in a file
    File,

    /// Call the given API endpoint (WIP)
    ApiCall,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ResourceType {
    /// Generate whole Bundle
    #[default]
    Bundle,

    /// Generate Patient
    Patient,

    /// Generate Condition
    Condition,

    /// Generate Specimen
    Specimen,

    /// Generate Observation Histology
    ObservationHistology,

    /// Generate Observation VitalStatus
    ObservationVitalStatus,

    /// Generate Observation TNMc
    ObservationTNMc,

    /// Generate Procedure Radiotherapy
    ProcedureRadiotherapy,

    /// Generate Procedure Operation
    ProcedureOperation,

    /// Generate Systemic Therapy Medication Statement
    SystemicTherapyMedicationStatement,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Bundle => "Bundle",
            ResourceType::Patient => "Patient",
            ResourceType::Condition => "Condition",
            ResourceType::Specimen => "Specimen",
            ResourceType::ObservationHistology => "Histology",
            ResourceType::ObservationVitalStatus => "VitalStatus",
            ResourceType::ObservationTNMc => "TNMc",
            ResourceType::ProcedureRadiotherapy => "Radiotherapy",
            ResourceType::ProcedureOperation => "Operation",
            ResourceType::SystemicTherapyMedicationStatement => "SystemicTherapy",
        }
    }

    pub fn get_resource_group(&self) -> &'static str {
        match self {
            ResourceType::Bundle => "Bundle",
            ResourceType::Patient => "Patient",
            ResourceType::Condition => "Condition",
            ResourceType::Specimen => "Specimen",
            ResourceType::ObservationHistology => "Observation",
            ResourceType::ObservationVitalStatus => "Observation",
            ResourceType::ObservationTNMc => "Observation",
            ResourceType::ProcedureRadiotherapy => "Procedure",
            ResourceType::ProcedureOperation => "Procedure",
            ResourceType::SystemicTherapyMedicationStatement => "MedicationStatement",
        }
    }
}

/// A program to generate synthetic XML data (conforming to CCE FHIR profiles)
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    SyntheticData {
        /// Number of resources to generate
        #[arg(short, long, default_value_t = 1)]
        number: u8,

        /// Type of resource to generate
        #[arg(short, long, value_enum, default_value_t=ResourceType::Bundle)]
        resource_type: ResourceType,

        /// Where to store the resources
        #[arg(short, long, value_enum, default_value_t=OutputMode::Screen)]
        output_mode: OutputMode,
    },
    Catalog,
}
