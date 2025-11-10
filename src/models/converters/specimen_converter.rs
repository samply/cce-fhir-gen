use fhirbolt::model::r4b::resources::Specimen;

use crate::models::enums::sample_material_type::SampleMaterialType;

use crate::lens::{
    catalogue::{Category, CategoryGroup, SingleSelectCategory},
    traits::{CategoryConverter, CriteriaConverter},
};

impl CategoryConverter for Specimen {
    fn get_category() -> Category {
        let sample_material_type = SingleSelectCategory::new(
            "sample_kind",
            "Sample Type",
            "",
            SampleMaterialType::get_criteria(),
        );

        let child_categories = vec![Category::SingleSelect(sample_material_type)];
        let category_group = CategoryGroup::new("biosamples", "BioSamples", child_categories);
        Category::Group(category_group)
    }
}

#[cfg(test)]
mod test_specimen_converter {
    use super::*;

    #[test]
    fn test_get_category() {
        let specimen_category = Specimen::get_category();
        let categories = vec![specimen_category];
        let json = serde_json::to_string_pretty(&categories)
            .expect("Failed to serialize categories to JSON");
        println!("Catalog of specimen categories:\n{json}");
    }
}
