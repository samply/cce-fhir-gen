use super::catalogue::{Category, Criteria};

/// Any Rust enum representing FHIR CodeSystem values should implement this trait.
pub trait CriteriaConverter {
    /// Each value of the enum represents a Criteria (which ultimately becomes a part of the Catalogue).
    /// This method returns a vector of Criteria for the enum.
    fn get_criteria() -> Vec<Criteria>;
}

/// A FHIR resource represents one or the other type of [Category].
pub trait CategoryConverter {
    /// Each FHIR resource represents one or more Categories (which ultimately become a part of the Catalogue).
    /// This method returns the Category for the FHIR resource.
    fn get_category() -> Category;
}
