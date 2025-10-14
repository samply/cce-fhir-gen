use fake::Dummy;
use strum::Display;

use crate::{
    fhir::traits::CodeSystemAdapter,
    lens::{catalogue::Criteria, traits::CriteriaConverter},
};

#[derive(Debug, Display, Dummy)]
pub enum UiccStage {
    Zero,
    ZeroA,
    ZeroIs,
    // I,
    IA,
    IA1,
    IA2,
    IB,
    IB1,
    IB2,
    IC,
    II,
    IIA,
    IIA1,
    IIA2,
    IIB,
    IIC,
    III,
    IIIA,
    IIIB,
    IIIC,
    IIIC1,
    IIIC2,
    IV,
    IVA,
    IVB,
    IVC,
    IS,
}

impl UiccStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            UiccStage::Zero => "0",
            UiccStage::ZeroA => "0a",
            UiccStage::ZeroIs => "0is",
            // UiccStage::I => "I",
            UiccStage::IA => "IA",
            UiccStage::IA1 => "IA1",
            UiccStage::IA2 => "IA2",
            UiccStage::IB => "IB",
            UiccStage::IB1 => "IB1",
            UiccStage::IB2 => "IB2",
            UiccStage::IC => "IC",
            UiccStage::II => "II",
            UiccStage::IIA => "IIA",
            UiccStage::IIA1 => "IIA1",
            UiccStage::IIA2 => "IIA2",
            UiccStage::IIB => "IIB",
            UiccStage::IIC => "IIC",
            UiccStage::III => "III",
            UiccStage::IIIA => "IIIA",
            UiccStage::IIIB => "IIIB",
            UiccStage::IIIC => "IIIC",
            UiccStage::IIIC1 => "IIIC1",
            UiccStage::IIIC2 => "IIIC2",
            UiccStage::IV => "IV",
            UiccStage::IVA => "IVA",
            UiccStage::IVB => "IVB",
            UiccStage::IVC => "IVC",
            UiccStage::IS => "IS",
        }
    }
}

impl CodeSystemAdapter for UiccStage {
    fn get_name() -> String {
        "UICCStageCS".to_string()
    }

    fn get_title() -> String {
        "UICC Stage CS".to_string()
    }

    fn get_description() -> String {
        "UICC Stage".to_string()
    }

    fn get_html_description() -> String {
        "UICC Stage CodeSystem".to_string()
    }

    fn get_url() -> String {
        Self::get_code_system_url(Self::get_name().as_str())
    }
}

impl CriteriaConverter for UiccStage {
    fn get_criteria() -> Vec<Criteria> {
        let ois = Criteria::new(
            UiccStage::ZeroIs.as_str(),
            UiccStage::ZeroIs.to_string().as_str(),
        );
        let oa = Criteria::new(
            UiccStage::ZeroA.as_str(),
            UiccStage::ZeroA.to_string().as_str(),
        );
        let o = Criteria::new(
            UiccStage::Zero.as_str(),
            UiccStage::Zero.to_string().as_str(),
        );
        let ia2 = Criteria::new(UiccStage::IA2.as_str(), UiccStage::IA2.to_string().as_str());
        let ia1 = Criteria::new(UiccStage::IA1.as_str(), UiccStage::IA1.to_string().as_str());
        let ia = Criteria::new(UiccStage::IA.as_str(), UiccStage::IA.to_string().as_str());
        // let i = Criteria::new(UiccStage::I.as_str(), UiccStage::I.to_string().as_str());
        let ib2 = Criteria::new(UiccStage::IB2.as_str(), UiccStage::IB2.to_string().as_str());
        let ib1 = Criteria::new(UiccStage::IB1.as_str(), UiccStage::IB1.to_string().as_str());
        let ib = Criteria::new(UiccStage::IB.as_str(), UiccStage::IB.to_string().as_str());
        let iic = Criteria::new(UiccStage::IIC.as_str(), UiccStage::IIC.to_string().as_str());
        let iib = Criteria::new(UiccStage::IIB.as_str(), UiccStage::IIB.to_string().as_str());
        let iia2 = Criteria::new(
            UiccStage::IIA2.as_str(),
            UiccStage::IIA2.to_string().as_str(),
        );
        let iia1 = Criteria::new(
            UiccStage::IIA1.as_str(),
            UiccStage::IIA1.to_string().as_str(),
        );
        let iia = Criteria::new(UiccStage::IIA.as_str(), UiccStage::IIA.to_string().as_str());
        let ii = Criteria::new(UiccStage::II.as_str(), UiccStage::II.to_string().as_str());
        let iiic2 = Criteria::new(
            UiccStage::IIIC2.as_str(),
            UiccStage::IIIC2.to_string().as_str(),
        );
        let iiic1 = Criteria::new(
            UiccStage::IIIC1.as_str(),
            UiccStage::IIIC1.to_string().as_str(),
        );
        let iiic = Criteria::new(
            UiccStage::IIIC.as_str(),
            UiccStage::IIIC.to_string().as_str(),
        );
        let iiib = Criteria::new(
            UiccStage::IIIB.as_str(),
            UiccStage::IIIB.to_string().as_str(),
        );
        let iiia = Criteria::new(
            UiccStage::IIIA.as_str(),
            UiccStage::IIIA.to_string().as_str(),
        );
        let iii = Criteria::new(UiccStage::III.as_str(), UiccStage::III.to_string().as_str());
        let ivc = Criteria::new(UiccStage::IVC.as_str(), UiccStage::IVC.to_string().as_str());
        let ivb = Criteria::new(UiccStage::IVB.as_str(), UiccStage::IVB.to_string().as_str());
        let iva = Criteria::new(UiccStage::IVA.as_str(), UiccStage::IVA.to_string().as_str());
        let iv = Criteria::new(UiccStage::IV.as_str(), UiccStage::IV.to_string().as_str());
        let is = Criteria::new(UiccStage::IS.as_str(), UiccStage::IS.to_string().as_str());
        vec![
            ois, oa, o, ia2, ia1, ia, ib2, ib1, ib, iic, iib, iia2, iia1, iia, ii, iiic2, iiic1,
            iiic, iiib, iiia, iii, ivc, ivb, iva, iv, is,
        ]
    }
}
