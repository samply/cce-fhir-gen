use fhirbolt::model::r4b::resources::Patient;

use crate::{
    fhir::traits::CodeSystemAdapter,
    models::enums::{gender::Gender, vital_status::VitalStatus},
    utils::VITAL_STATUS_LOINC_CODE,
};

use crate::lens::{
    catalogue::{Category, CategoryGroup, SingleSelectCategory},
    traits::{CategoryConverter, CriteriaConverter},
};

impl CategoryConverter for Patient {
    fn get_category() -> Category {
        let gender = SingleSelectCategory::new("gender", "Gender", "", Gender::get_criteria());

        let vital_status = SingleSelectCategory::new(
            VITAL_STATUS_LOINC_CODE,
            "Vital Status",
            VitalStatus::get_url().as_str(),
            VitalStatus::get_criteria(),
        );

        let child_categories = vec![
            Category::SingleSelect(gender),
            Category::SingleSelect(vital_status),
        ];
        let category_group = CategoryGroup::new("patient", "Patient", child_categories);
        Category::Group(category_group)
    }
}

#[cfg(test)]
mod test_patient_converter {
    use super::*;

    #[test]
    fn test_get_category() {
        let category = Patient::get_category();
        let categories = vec![category];
        let json = serde_json::to_string_pretty(&categories)
            .expect("Failed to serialize categories to JSON");
        println!("Catalog of patient categories:\n{json}");
    }
}
