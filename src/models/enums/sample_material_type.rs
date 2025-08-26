use std::vec;

use fake::Dummy;
use strum::Display;

use crate::models::lens::{catalogue::Criteria, traits::CriteriaConverter};

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

impl CriteriaConverter for SampleMaterialType {

    // TODO: Names should be with spaces
    fn get_criteria() -> Vec<crate::models::lens::catalogue::Criteria> {
        let whole_blood = Criteria::new_with_description(
            SampleMaterialType::WholeBlood.as_str(),
            SampleMaterialType::WholeBlood.to_string().as_str(),
            "Whole Blood",
        );
        let bone_marrow = Criteria::new_with_description(
            SampleMaterialType::BoneMarrow.as_str(),
            SampleMaterialType::BoneMarrow.to_string().as_str(),
            "Bone Marrow",
        );
        let plasma = Criteria::new_with_description(
            SampleMaterialType::BloodPlasma.as_str(),
            SampleMaterialType::BloodPlasma.to_string().as_str(),
            "Plasma",
        );
        let serum = Criteria::new_with_description(
            SampleMaterialType::BloodSerum.as_str(),
            SampleMaterialType::BloodSerum.to_string().as_str(),
            "Serum",
        );
        let csf_liquor = Criteria::new_with_description(
            SampleMaterialType::CsfLiquor.as_str(),
            SampleMaterialType::CsfLiquor.to_string().as_str(),
            "Liquor/CSF",
        );
        let stool_faeces = Criteria::new_with_description(
            SampleMaterialType::StoolFaeces.as_str(),
            SampleMaterialType::StoolFaeces.to_string().as_str(),
            "Stool/Faeces",
        );
        let urine = Criteria::new_with_description(
            SampleMaterialType::Urine.as_str(),
            SampleMaterialType::Urine.to_string().as_str(),
            "Urine",
        );
        let tumor_tissue_ffpe = Criteria::new_with_description(
            SampleMaterialType::TumorTissueFfpe.as_str(),
            SampleMaterialType::TumorTissueFfpe.to_string().as_str(),
            "Tumor Tissue (FFPE)",
        );
        let normal_tissue_ffpe = Criteria::new_with_description(
            SampleMaterialType::NormalTissueFfpe.as_str(),
            SampleMaterialType::NormalTissueFfpe.to_string().as_str(),
            "Normal Tissue (FFPE)",
        );
        let tumor_tissue_frozen = Criteria::new_with_description(
            SampleMaterialType::TumorTissueFrozen.as_str(),
            SampleMaterialType::TumorTissueFrozen.to_string().as_str(),
            "Tumor Tissue (Frozen)",
        );
        let normal_tissue_frozen = Criteria::new_with_description(
            SampleMaterialType::NormalTissueFrozen.as_str(),
            SampleMaterialType::NormalTissueFrozen.to_string().as_str(),
            "Normal Tissue (Frozen)",
        );
        let dna = Criteria::new_with_description(
            SampleMaterialType::Dna.as_str(),
            SampleMaterialType::Dna.to_string().as_str(),
            "DNA",
        );
        let rna = Criteria::new_with_description(
            SampleMaterialType::Rna.as_str(),
            SampleMaterialType::Rna.to_string().as_str(),
            "RNA",
        );

        vec![
            whole_blood,
            bone_marrow,
            plasma,
            serum,
            csf_liquor,
            stool_faeces,
            urine,
            tumor_tissue_ffpe,
            normal_tissue_ffpe,
            tumor_tissue_frozen,
            normal_tissue_frozen,
            dna,
            rna,
        ]
    }
}
