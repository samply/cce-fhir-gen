use crate::utils::CCE_URL;

/// Trait for FHIR CodeSystem.
/// Any Rust enum containing FHIR CodeSystem values should implement this trait.
/// It contains methods that help in the generation of CodeSystem FHIR resource itself.
pub trait CodeSystem {
    fn get_fhir_url() -> String {
        format!("{CCE_URL}/fhir/core")
    }

    fn get_code_system_url(name: &str) -> String {
        format!("{}/CodeSystem/{}", Self::get_fhir_url(), name)
    }

    fn get_name() -> String;
    fn get_title() -> String;
    fn get_html_title() -> String;
    fn get_html_description() -> String;
    fn get_url() -> String;
}
