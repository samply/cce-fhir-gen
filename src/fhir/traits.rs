use fhirbolt::model::r4b::types::Narrative;

use crate::utils::CCE_URL;

/// Trait for FHIR CodeSystem.
/// Any Rust enum containing FHIR CodeSystem values should implement this trait.
/// It contains methods that help in the generation of CodeSystem FHIR resource itself.
pub trait CodeSystemAdapter {
    fn get_fhir_url() -> String {
        format!("{CCE_URL}/fhir/core")
    }

    fn get_code_system_url(name: &str) -> String {
        format!("{}/CodeSystem/{}", Self::get_fhir_url(), name)
    }

    fn get_name() -> String;
    fn get_title() -> String;
    fn get_description() -> String;
    fn get_html_description() -> String;
    fn get_url() -> String;
}

pub struct CodeSystemConceptHelper {
    pub code: String,
    pub display: String,
}

pub trait CodeSystemConceptAdapter {
    fn get_concepts() -> Vec<CodeSystemConceptHelper>;

    fn get_narrative() -> Narrative;
}
