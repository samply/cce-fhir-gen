use fhirbolt::model::r4b::resources::Patient;

use crate::{
    models::{
        enums::{gender::Gender, vital_status::VitalStatus},
        lens::{
            catalogue::{Category, CategoryGroup, SingleSelectCategory},
            traits::{CategoryConverter, CriteriaConverter},
        },
    },
    utils::{get_vital_status_url, VITAL_STATUS_LOINC_CODE},
};

impl CategoryConverter for Patient {
    fn get_category() -> Category {
        let gender = SingleSelectCategory::new("gender", "Gender", "", Gender::get_criteria());

        let vital_status = SingleSelectCategory::new(
            VITAL_STATUS_LOINC_CODE,
            "Vital Status",
            get_vital_status_url().value.unwrap_or_default().as_str(),
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
