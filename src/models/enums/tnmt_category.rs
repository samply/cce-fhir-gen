use fake::Dummy;
use strum::Display;

use crate::lens::{catalogue::Criteria, traits::CriteriaConverter};

#[derive(Debug, Display, Dummy)]
pub enum TnmtCategory {
    Zero,
    One,
    OneA,
    OneA1,
    OneA2,
    OneB,
    OneB1,
    OneB2,
    OneC,
    OneC1,
    OneC2,
    OneC3,
    OneD,
    OneMi,
    Two,
    TwoA,
    TwoA1,
    TwoA2,
    TwoB,
    TwoC,
    TwoD,
    Three,
    ThreeA,
    ThreeB,
    ThreeC,
    ThreeD,
    Four,
    FourA,
    FourB,
    FourC,
    FourD,
    FourE,
    A,
    Is,
    IsDcis,
    IsLcis,
    IsPaget,
    IsPd,
    IsPu,
    X,
}

impl TnmtCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmtCategory::Zero => "0",
            TnmtCategory::One => "1",
            TnmtCategory::OneA => "1a",
            TnmtCategory::OneA1 => "1a1",
            TnmtCategory::OneA2 => "1a2",
            TnmtCategory::OneB => "1b",
            TnmtCategory::OneB1 => "1b1",
            TnmtCategory::OneB2 => "1b2",
            TnmtCategory::OneC => "1c",
            TnmtCategory::OneC1 => "1c1",
            TnmtCategory::OneC2 => "1c2",
            TnmtCategory::OneC3 => "1c3",
            TnmtCategory::OneD => "1d",
            TnmtCategory::OneMi => "1mi",
            TnmtCategory::Two => "2",
            TnmtCategory::TwoA => "2a",
            TnmtCategory::TwoA1 => "2a1",
            TnmtCategory::TwoA2 => "2a2",
            TnmtCategory::TwoB => "2b",
            TnmtCategory::TwoC => "2c",
            TnmtCategory::TwoD => "2d",
            TnmtCategory::Three => "3",
            TnmtCategory::ThreeA => "3a",
            TnmtCategory::ThreeB => "3b",
            TnmtCategory::ThreeC => "3c",
            TnmtCategory::ThreeD => "3d",
            TnmtCategory::Four => "4",
            TnmtCategory::FourA => "4a",
            TnmtCategory::FourB => "4b",
            TnmtCategory::FourC => "4c",
            TnmtCategory::FourD => "4d",
            TnmtCategory::FourE => "4e",
            TnmtCategory::A => "a",
            TnmtCategory::Is => "is",
            TnmtCategory::IsDcis => "is(DCIS)",
            TnmtCategory::IsLcis => "is(LCIS)",
            TnmtCategory::IsPaget => "is(Paget)",
            TnmtCategory::IsPd => "is(pd)",
            TnmtCategory::IsPu => "is(pu)",
            TnmtCategory::X => "X",
        }
    }
}

impl CriteriaConverter for TnmtCategory {
    fn get_criteria() -> Vec<Criteria> {
        let zero = Criteria::new(TnmtCategory::Zero.as_str(), TnmtCategory::Zero.as_str());
        let one = Criteria::new(TnmtCategory::One.as_str(), TnmtCategory::One.as_str());
        let one_a = Criteria::new(TnmtCategory::OneA.as_str(), TnmtCategory::OneA.as_str());
        let one_a1 = Criteria::new(TnmtCategory::OneA1.as_str(), TnmtCategory::OneA1.as_str());
        let one_a2 = Criteria::new(TnmtCategory::OneA2.as_str(), TnmtCategory::OneA2.as_str());
        let one_b = Criteria::new(TnmtCategory::OneB.as_str(), TnmtCategory::OneB.as_str());
        let one_b1 = Criteria::new(TnmtCategory::OneB1.as_str(), TnmtCategory::OneB1.as_str());
        let one_b2 = Criteria::new(TnmtCategory::OneB2.as_str(), TnmtCategory::OneB2.as_str());
        let one_c = Criteria::new(TnmtCategory::OneC.as_str(), TnmtCategory::OneC.as_str());
        let one_c1 = Criteria::new(TnmtCategory::OneC1.as_str(), TnmtCategory::OneC1.as_str());
        let one_c2 = Criteria::new(TnmtCategory::OneC2.as_str(), TnmtCategory::OneC2.as_str());
        let one_c3 = Criteria::new(TnmtCategory::OneC3.as_str(), TnmtCategory::OneC3.as_str());
        let one_d = Criteria::new(TnmtCategory::OneD.as_str(), TnmtCategory::OneD.as_str());
        let one_mi = Criteria::new(TnmtCategory::OneMi.as_str(), TnmtCategory::OneMi.as_str());
        let two = Criteria::new(TnmtCategory::Two.as_str(), TnmtCategory::Two.as_str());
        let two_a = Criteria::new(TnmtCategory::TwoA.as_str(), TnmtCategory::TwoA.as_str());
        let two_a1 = Criteria::new(TnmtCategory::TwoA1.as_str(), TnmtCategory::TwoA1.as_str());
        let two_a2 = Criteria::new(TnmtCategory::TwoA2.as_str(), TnmtCategory::TwoA2.as_str());
        let two_b = Criteria::new(TnmtCategory::TwoB.as_str(), TnmtCategory::TwoB.as_str());
        let two_c = Criteria::new(TnmtCategory::TwoC.as_str(), TnmtCategory::TwoC.as_str());
        let two_d = Criteria::new(TnmtCategory::TwoD.as_str(), TnmtCategory::TwoD.as_str());
        let three = Criteria::new(TnmtCategory::Three.as_str(), TnmtCategory::Three.as_str());
        let three_a = Criteria::new(TnmtCategory::ThreeA.as_str(), TnmtCategory::ThreeA.as_str());
        let three_b = Criteria::new(TnmtCategory::ThreeB.as_str(), TnmtCategory::ThreeB.as_str());
        let three_c = Criteria::new(TnmtCategory::ThreeC.as_str(), TnmtCategory::ThreeC.as_str());
        let three_d = Criteria::new(TnmtCategory::ThreeD.as_str(), TnmtCategory::ThreeD.as_str());
        let four = Criteria::new(TnmtCategory::Four.as_str(), TnmtCategory::Four.as_str());
        let four_a = Criteria::new(TnmtCategory::FourA.as_str(), TnmtCategory::FourA.as_str());
        let four_b = Criteria::new(TnmtCategory::FourB.as_str(), TnmtCategory::FourB.as_str());
        let four_c = Criteria::new(TnmtCategory::FourC.as_str(), TnmtCategory::FourC.as_str());
        let four_d = Criteria::new(TnmtCategory::FourD.as_str(), TnmtCategory::FourD.as_str());
        let four_e = Criteria::new(TnmtCategory::FourE.as_str(), TnmtCategory::FourE.as_str());
        let a = Criteria::new(TnmtCategory::A.as_str(), TnmtCategory::A.as_str());
        let is = Criteria::new(TnmtCategory::Is.as_str(), TnmtCategory::Is.as_str());
        let is_dcis = Criteria::new(TnmtCategory::IsDcis.as_str(), TnmtCategory::IsDcis.as_str());
        let is_lcis = Criteria::new(TnmtCategory::IsLcis.as_str(), TnmtCategory::IsLcis.as_str());
        let is_paget = Criteria::new(
            TnmtCategory::IsPaget.as_str(),
            TnmtCategory::IsPaget.as_str(),
        );
        let is_pd = Criteria::new(TnmtCategory::IsPd.as_str(), TnmtCategory::IsPd.as_str());
        let is_pu = Criteria::new(TnmtCategory::IsPu.as_str(), TnmtCategory::IsPu.as_str());
        let x = Criteria::new(TnmtCategory::X.as_str(), TnmtCategory::X.as_str());
        vec![
            zero, one, one_a, one_a1, one_a2, one_b, one_b1, one_b2, one_c, one_c1, one_c2, one_c3,
            one_d, one_mi, two, two_a, two_a1, two_a2, two_b, two_c, two_d, three, three_a,
            three_b, three_c, three_d, four, four_a, four_b, four_c, four_d, four_e, a, is,
            is_dcis, is_lcis, is_paget, is_pd, is_pu, x,
        ]
    }
}
