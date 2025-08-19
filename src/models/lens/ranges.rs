use chrono::NaiveDate;

pub struct NumberRange {
    pub min: f32,
    pub max: f32,
}

pub struct DateRange {
    pub min: Option<NaiveDate>,
    pub max: Option<NaiveDate>,
}
