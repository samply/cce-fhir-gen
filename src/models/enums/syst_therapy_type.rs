use core::fmt;

use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum SystTherapyType {
    CH,
    HO,
    IM,
    BM,
    WS,
    AS,
    TS,
    SO,
    RT,
    OP,
    CI,
    CT,
    // TODO: do we need this value?
    CIT,
    IT,
    SC,
    WW,
}

impl SystTherapyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystTherapyType::CH => "Chemotherapy",
            SystTherapyType::HO => "Hormone therapy",
            SystTherapyType::IM => "Immunotherapy and antibody therapy",
            SystTherapyType::BM => "Bone marrow transplantation",
            SystTherapyType::WS => "Wait and see",
            SystTherapyType::AS => "Active Surveillance",
            SystTherapyType::TS => "Targeted substances",
            SystTherapyType::SO => "Miscellaneous",
            SystTherapyType::RT => "Radiotherapy",
            SystTherapyType::OP => "Operation",
            SystTherapyType::CI => "Chemo- + Immuno-/Antibody therapy",
            SystTherapyType::CT => "Chemotherapy + Targeted substances",
            SystTherapyType::CIT => "Chemo- + Immuno-/Antibody therapy + Targeted substances",
            SystTherapyType::IT => "Immuno-/Antibody therapy + Targeted substances",
            SystTherapyType::SC => "Stem cell transplantation (incl. bone marrow transplantation)",
            SystTherapyType::WW => "Watchful Waiting",
        }
    }
}

impl fmt::Display for SystTherapyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystTherapyType::CH => write!(f, "CH"),
            SystTherapyType::HO => write!(f, "HO"),
            SystTherapyType::IM => write!(f, "IM"),
            SystTherapyType::BM => write!(f, "BM"),
            SystTherapyType::WS => write!(f, "WS"),
            SystTherapyType::AS => write!(f, "AS"),
            SystTherapyType::TS => write!(f, "TS"),
            SystTherapyType::SO => write!(f, "SO"),
            SystTherapyType::RT => write!(f, "RT"),
            SystTherapyType::OP => write!(f, "OP"),
            SystTherapyType::CI => write!(f, "CI"),
            SystTherapyType::CT => write!(f, "CT"),
            SystTherapyType::CIT => write!(f, "CIT"),
            SystTherapyType::IT => write!(f, "IT"),
            SystTherapyType::SC => write!(f, "SC"),
            SystTherapyType::WW => write!(f, "WW"),
        }
    }
}
