use std::vec;

use fake::Dummy;
use strum::Display;

use crate::{fhir::traits::CodeSystemAdapter, lens::{catalogue::Criteria, traits::CriteriaConverter}};

#[derive(Debug, Display, Dummy)]
pub enum SampleMaterialType {
    WholeBlood,
    BoneMarrow,
    // BuffyCoat,
    // DriedWholeBlood,
    // PeripheralBloodMononuclearCells,
    BloodPlasma,
    // PlasmaEdta,
    // PlasmaCitrat,
    // PlasmaHeparin,
    // PlasmaCellFree,
    // PlasmaOther,
    BloodSerum,
    // Ascites,
    CsfLiquor,
    // Saliva,
    StoolFaeces,
    Urine,
    // Swab,
    // OtherLiquid,
    // TissueFfpe,
    TumorTissueFfpe,
    NormalTissueFfpe,
    // OtherTissueFfpe,
    // TissueFrozen,
    TumorTissueFrozen,
    NormalTissueFrozen,
    // OtherTissueFrozen,
    // OtherTissueStorage,
    Dna,
    // CfDna,
    // GDna,
    Rna,
    // OtherDerivative,
}

impl SampleMaterialType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SampleMaterialType::WholeBlood => "whole-blood",
            SampleMaterialType::BoneMarrow => "bone-marrow",
            // SampleMaterialType::BuffyCoat => "buffy-coat",
            // SampleMaterialType::DriedWholeBlood => "dried-whole-blood",
            // SampleMaterialType::PeripheralBloodMononuclearCells => "peripheral-blood-cells-vital",
            SampleMaterialType::BloodPlasma => "blood-plasma",
            // SampleMaterialType::PlasmaEdta => "plasma-edta",
            // SampleMaterialType::PlasmaCitrat => "plasma-citrat",
            // SampleMaterialType::PlasmaHeparin => "plasma-heparin",
            // SampleMaterialType::PlasmaCellFree => "plasma-cell-free",
            // SampleMaterialType::PlasmaOther => "plasma-other",
            SampleMaterialType::BloodSerum => "blood-serum",
            // SampleMaterialType::Ascites => "ascites",
            SampleMaterialType::CsfLiquor => "csf-liquor",
            // SampleMaterialType::Saliva => "saliva",
            SampleMaterialType::StoolFaeces => "stool-faeces",
            SampleMaterialType::Urine => "urine",
            // SampleMaterialType::Swab => "swab",
            // SampleMaterialType::OtherLiquid => "liquid-other",
            // SampleMaterialType::TissueFfpe => "tissue-ffpe",
            SampleMaterialType::TumorTissueFfpe => "tumor-tissue-ffpe",
            SampleMaterialType::NormalTissueFfpe => "normal-tissue-ffpe",
            // SampleMaterialType::OtherTissueFfpe => "other-tissue-ffpe",
            // SampleMaterialType::TissueFrozen => "tissue-frozen",
            SampleMaterialType::TumorTissueFrozen => "tumor-tissue-frozen",
            SampleMaterialType::NormalTissueFrozen => "normal-tissue-frozen",
            // SampleMaterialType::OtherTissueFrozen => "other-tissue-frozen",
            // SampleMaterialType::OtherTissueStorage => "tissue-other",
            SampleMaterialType::Dna => "dna",
            // SampleMaterialType::CfDna => "cf-dna",
            // SampleMaterialType::GDna => "g-dna",
            SampleMaterialType::Rna => "rna",
            // SampleMaterialType::OtherDerivative => "derivative-other",
        }
    }
}

impl CodeSystemAdapter for SampleMaterialType {
    fn get_name() -> String {
        "SampleMaterialTypeCS".to_string()
    }

    fn get_title() -> String {
        "Sample Material Type CS".to_string()
    }

    fn get_description() -> String {
        "SampleMaterialType".to_string()
    }
    
    fn get_html_description() -> String {
        "SampleMaterialType CodeSystem".to_string()
    }

    fn get_url() -> String {
        Self::get_code_system_url(Self::get_name().as_str())
    }
}

impl CriteriaConverter for SampleMaterialType {
    
    fn get_criteria() -> Vec<Criteria> {
        let whole_blood = Criteria::new_with_description(
            SampleMaterialType::WholeBlood.as_str(),
            "Whole Blood",
            "Whole Blood",
        );
        let bone_marrow = Criteria::new_with_description(
            SampleMaterialType::BoneMarrow.as_str(),
            "Bone Marrow",
            "Bone Marrow",
        );
        let plasma = Criteria::new_with_description(
            SampleMaterialType::BloodPlasma.as_str(),
            "Plasma",
            "Plasma",
        );
        let serum = Criteria::new_with_description(
            SampleMaterialType::BloodSerum.as_str(),
            "Serum",
            "Serum",
        );
        let csf_liquor = Criteria::new_with_description(
            SampleMaterialType::CsfLiquor.as_str(),
            "Liquor/CSF",
            "Liquor/CSF",
        );
        let stool_faeces = Criteria::new_with_description(
            SampleMaterialType::StoolFaeces.as_str(),
            "Stool/Faeces",
            "Stool/Faeces",
        );
        let urine = Criteria::new_with_description(
            SampleMaterialType::Urine.as_str(),
            SampleMaterialType::Urine.to_string().as_str(),
            "Urine",
        );
        let tumor_tissue_ffpe = Criteria::new_with_description(
            SampleMaterialType::TumorTissueFfpe.as_str(),
            "Tumor Tissue (FFPE)",
            "Tumor Tissue (FFPE)",
        );
        let normal_tissue_ffpe = Criteria::new_with_description(
            SampleMaterialType::NormalTissueFfpe.as_str(),
            SampleMaterialType::NormalTissueFfpe.to_string().as_str(),
            "Normal Tissue (FFPE)",
        );
        let tumor_tissue_frozen = Criteria::new_with_description(
            SampleMaterialType::TumorTissueFrozen.as_str(),
            "Tumor Tissue (Frozen)",
            "Tumor Tissue (Frozen)",
        );
        let normal_tissue_frozen = Criteria::new_with_description(
            SampleMaterialType::NormalTissueFrozen.as_str(),
            "Normal Tissue (Frozen)",
            "Normal Tissue (Frozen)",
        );
        let dna = Criteria::new_with_description(SampleMaterialType::Dna.as_str(), "DNA", "DNA");
        let rna = Criteria::new_with_description(SampleMaterialType::Rna.as_str(), "RNA", "RNA");

        vec![
            tumor_tissue_ffpe,
            tumor_tissue_frozen,
            normal_tissue_ffpe,
            normal_tissue_frozen,
            whole_blood,
            serum,
            plasma,
            urine,
            csf_liquor,
            stool_faeces,
            bone_marrow,
            dna,
            rna,
        ]
    }
}
