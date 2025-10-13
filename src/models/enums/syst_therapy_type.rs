use fake::Dummy;
use strum::Display;

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
