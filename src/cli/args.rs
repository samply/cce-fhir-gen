use clap::{command, Parser, ValueEnum};

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

    /// Generate Medication Statement
    MedicationStatement,
}

/// A program to generate synthetic XML data (conforming to CCE FHIR profiles)
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Number of resources to generate
    #[arg(short, long, default_value_t = 1)]
    pub number: u8,

    /// Type of resource to generate
    #[arg(short, long, value_enum)]
    pub resource_type: ResourceType,

    /// Where to store the resources
    #[arg(short, long, value_enum)]
    pub output_mode: OutputMode,
}
