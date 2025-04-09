use fake::Dummy;

/// Represents whether the type of identifying (unique) value is
/// from the FHIR DB store or the source system.
#[derive(Debug, Dummy)]
pub enum IdType {
    /// Unique value in the FHIR DB or system
    Id,
    /// Unique value coming from the source system
    Identifier,
}

impl IdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IdType::Id => "id",
            IdType::Identifier => "identifier"
        }
    }
}
