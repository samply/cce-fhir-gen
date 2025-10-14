use fhirbolt::model::r4b::resources::Observation;

use crate::{
    fhir::traits::CodeSystemAdapter,
    models::enums::{
        gender::Gender, tnmm_category::TnmmCategory, tnmn_category::TnmnCategory,
        tnmt_category::TnmtCategory, tumor_site_location::TumorSiteLocation, uicc_stage::UiccStage,
        vital_status::VitalStatus,
    },
    utils::{
        CLINICAL_METASTASES_LOINC_CODE, CLINICAL_NODES_LOINC_CODE, CLINICAL_STAGE_GROUP_LOINC_CODE,
        CLINICAL_TUMOR_LOINC_CODE,
    },
};

use crate::lens::{
    catalogue::{Category, CategoryGroup, SingleSelectCategory},
    traits::{CategoryConverter, CriteriaConverter},
};

impl CategoryConverter for Observation {
    fn get_category() -> Category {
        let site_location = SingleSelectCategory::new(
            "bodySite",
            "Side Location",
            "",
            TumorSiteLocation::get_criteria(),
        );
        let uicc_stage = SingleSelectCategory::new(
            CLINICAL_STAGE_GROUP_LOINC_CODE,
            "UICC Stage",
            UiccStage::get_url().as_str(),
            UiccStage::get_criteria(),
        );
        let tnmt = SingleSelectCategory::new(
            CLINICAL_TUMOR_LOINC_CODE,
            "TNM-T",
            "",
            TnmtCategory::get_criteria(),
        );
        let tnmn = SingleSelectCategory::new(
            CLINICAL_NODES_LOINC_CODE,
            "TNM-N",
            "",
            TnmnCategory::get_criteria(),
        );
        let tnmm = SingleSelectCategory::new(
            CLINICAL_METASTASES_LOINC_CODE,
            "TNM-M",
            "",
            TnmmCategory::get_criteria(),
        );
        let tnm_group = CategoryGroup::new(
            "tnm",
            "TNM(c)",
            vec![
                Category::SingleSelect(tnmt),
                Category::SingleSelect(tnmn),
                Category::SingleSelect(tnmm),
            ],
        );

        let child_categories = vec![
            Category::SingleSelect(site_location),
            Category::SingleSelect(uicc_stage),
            Category::Group(tnm_group),
        ];
        let category_group = CategoryGroup::new(
            "tumor_classification",
            "Tumor classification",
            child_categories,
        );
        Category::Group(category_group)
    }
}
