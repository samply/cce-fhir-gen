use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female"
        }
    }
}
