use fake::Dummy;
use strum::Display;

use crate::lens::{catalogue::Criteria, traits::CriteriaConverter};

#[derive(Debug, Display, Dummy)]
pub enum TumorSiteLocation {
    L,
    R,
    B,
    C,
    N,
    U,
}

impl TumorSiteLocation {
    pub fn as_str(&self) -> &'static str {
        match self {
            TumorSiteLocation::L => "Left",
            TumorSiteLocation::R => "Right",
            TumorSiteLocation::B => "Bilateral",
            TumorSiteLocation::C => "Centered",
            TumorSiteLocation::N => "Not applicable",
            TumorSiteLocation::U => "Unknown",
        }
    }
}

impl CriteriaConverter for TumorSiteLocation {
    fn get_criteria() -> Vec<Criteria> {
        let left = Criteria::new(
            TumorSiteLocation::L.as_str(),
            TumorSiteLocation::L.to_string().as_str(),
        );
        let right = Criteria::new(
            TumorSiteLocation::R.as_str(),
            TumorSiteLocation::R.to_string().as_str(),
        );
        let bilateral = Criteria::new(
            TumorSiteLocation::B.as_str(),
            TumorSiteLocation::B.to_string().as_str(),
        );
        let centered = Criteria::new(
            TumorSiteLocation::C.as_str(),
            TumorSiteLocation::C.to_string().as_str(),
        );
        let not_applicable = Criteria::new(
            TumorSiteLocation::N.as_str(),
            TumorSiteLocation::N.to_string().as_str(),
        );
        let unknown = Criteria::new(
            TumorSiteLocation::U.as_str(),
            TumorSiteLocation::U.to_string().as_str(),
        );
        vec![left, right, bilateral, centered, unknown, not_applicable]
    }
}
