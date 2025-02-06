use core::fmt;

use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum TumorSiteLocation {
    L,
    R,
    B,
    C,
    N,
    U
}

impl TumorSiteLocation {
    pub fn as_str(&self) -> &'static str {
        match self {
            TumorSiteLocation::L => "left",
            TumorSiteLocation::R => "right",
            TumorSiteLocation::B => "bilateral",
            TumorSiteLocation::C => "Centerline/Center",
            TumorSiteLocation::N => "Not applicable",
            TumorSiteLocation::U => "unknown",
        }
    }
}

impl fmt::Display for TumorSiteLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TumorSiteLocation::L => write!(f, "L"),
            TumorSiteLocation::R => write!(f, "R"),
            TumorSiteLocation::B => write!(f, "B"),
            TumorSiteLocation::C => write!(f, "C"),
            TumorSiteLocation::N => write!(f, "N"),
            TumorSiteLocation::U => write!(f, "U"),
        }
    }
}
