use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum VitalStatus {
    Alive,
    Deceased,
    Unknown,
}

impl VitalStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            VitalStatus::Alive => "alive",
            VitalStatus::Deceased => "deceased",
            VitalStatus::Unknown => "unknown",
        }
    }
}
