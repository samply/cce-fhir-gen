use fake::Dummy;
use strum::Display;

use crate::lens::{
    catalogue::{Category, CategoryGroup, Criteria, SingleSelectCategory},
    traits::{CategoryConverter, CriteriaConverter},
};

#[derive(Clone, Debug, Display, Dummy)]
pub enum SystTherapyType {
    CH,
    HO,
    IM,
    // BM,
    // WS,
    // AS,
    // TS,
    // MI,
    RT,
    OP,
    // CI,
    // CT,
    // CIT,
    // IT,
    SC,
    // WW,
}

impl SystTherapyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystTherapyType::CH => "Chemotherapy",
            SystTherapyType::HO => "Hormone therapy",
            SystTherapyType::IM => "Immunotherapy and antibody therapy",
            // SystTherapyType::BM => "Bone marrow transplantation",
            // SystTherapyType::WS => "Wait and see",
            // SystTherapyType::AS => "Active Surveillance",
            // SystTherapyType::TS => "Targeted substances",
            // SystTherapyType::MI => "Miscellaneous",
            SystTherapyType::RT => "Radiotherapy",
            SystTherapyType::OP => "Operation",
            // SystTherapyType::CI => "Chemo- + Immuno-/Antibody therapy",
            // SystTherapyType::CT => "Chemotherapy + Targeted substances",
            // SystTherapyType::CIT => "Chemo- + Immuno-/Antibody therapy + Targeted substances",
            // SystTherapyType::IT => "Immuno-/Antibody therapy + Targeted substances",
            SystTherapyType::SC => "Stem cell transplantation (incl. bone marrow transplantation)",
            // SystTherapyType::WW => "Watchful Waiting",
        }
    }
}

impl CategoryConverter for SystTherapyType {
    fn get_category() -> Category {
        let op = Criteria::new(
            SystTherapyType::OP.to_string().as_str(),
            SystTherapyType::OP.as_str(),
        );
        let rt = Criteria::new(
            SystTherapyType::RT.to_string().as_str(),
            SystTherapyType::RT.as_str(),
        );

        let operation_radiotherapy_category =
            SingleSelectCategory::new("procedure", "Operation / Radiotherapy", "", vec![op, rt]);

        let ch = Criteria::new(
            SystTherapyType::CH.to_string().as_str(),
            SystTherapyType::CH.as_str(),
        );
        let ho = Criteria::new(
            SystTherapyType::HO.to_string().as_str(),
            SystTherapyType::HO.as_str(),
        );
        let im = Criteria::new(
            SystTherapyType::IM.to_string().as_str(),
            SystTherapyType::IM.as_str(),
        );
        let sc = Criteria::new(
            SystTherapyType::SC.to_string().as_str(),
            SystTherapyType::SC.as_str(),
        );

        let other_therapy_category = SingleSelectCategory::new(
            "medicationStatement",
            "Other Therapies",
            "",
            vec![ch, ho, im, sc],
        );
        let child_categories = vec![
            Category::SingleSelect(operation_radiotherapy_category),
            Category::SingleSelect(other_therapy_category),
        ];
        let category_group =
            CategoryGroup::new("therapy_of_tumor", "Therapy of tumor", child_categories);
        Category::Group(category_group)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_get_category() {
        let category = SystTherapyType::get_category();
        let categories = vec![category];
        let json = serde_json::to_string_pretty(&categories)
            .expect("Failed to serialize categories to JSON");
        println!("Catalog of tumor therapy categories:\n{json}");
    }
}
