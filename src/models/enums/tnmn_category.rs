use fake::Dummy;
use strum::Display;

use crate::lens::{catalogue::Criteria, traits::CriteriaConverter};

#[derive(Debug, Display, Dummy)]
pub enum TnmnCategory {
    Zero,
    ZeroIMinus,
    ZeroIPlus,
    ZeroMolMinus,
    ZeroMolPlus,
    One,
    OneA,
    OneB,
    OneC,
    OneMi,
    Two,
    TwoA,
    TwoB,
    TwoC,
    Three,
    ThreeA,
    ThreeB,
    ThreeC,
    X,
}

impl TnmnCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmnCategory::Zero => "0",
            TnmnCategory::ZeroIMinus => "0(i-)",
            TnmnCategory::ZeroIPlus => "0(i+)",
            TnmnCategory::ZeroMolMinus => "0(mol-)",
            TnmnCategory::ZeroMolPlus => "0(mol+)",
            TnmnCategory::One => "1",
            TnmnCategory::OneA => "1a",
            TnmnCategory::OneB => "1b",
            TnmnCategory::OneC => "1c",
            TnmnCategory::OneMi => "1mi",
            TnmnCategory::Two => "2",
            TnmnCategory::TwoA => "2a",
            TnmnCategory::TwoB => "2b",
            TnmnCategory::TwoC => "2c",
            TnmnCategory::Three => "3",
            TnmnCategory::ThreeA => "3a",
            TnmnCategory::ThreeB => "3b",
            TnmnCategory::ThreeC => "3c",
            TnmnCategory::X => "X",
        }
    }
}

impl CriteriaConverter for TnmnCategory {
    fn get_criteria() -> Vec<Criteria> {
        let zero = Criteria::new(
            TnmnCategory::Zero.as_str(),
            TnmnCategory::Zero.to_string().as_str(),
        );
        let zero_i_plus = Criteria::new(
            TnmnCategory::ZeroIPlus.as_str(),
            TnmnCategory::ZeroIPlus.to_string().as_str(),
        );
        let zero_i_minus = Criteria::new(
            TnmnCategory::ZeroIMinus.as_str(),
            TnmnCategory::ZeroIMinus.to_string().as_str(),
        );
        let zero_mol_plus = Criteria::new(
            TnmnCategory::ZeroMolPlus.as_str(),
            TnmnCategory::ZeroMolPlus.to_string().as_str(),
        );
        let zero_mol_minus = Criteria::new(
            TnmnCategory::ZeroMolMinus.as_str(),
            TnmnCategory::ZeroMolMinus.to_string().as_str(),
        );
        let one = Criteria::new(
            TnmnCategory::One.as_str(),
            TnmnCategory::One.to_string().as_str(),
        );
        let one_a = Criteria::new(
            TnmnCategory::OneA.as_str(),
            TnmnCategory::OneA.to_string().as_str(),
        );
        let one_b = Criteria::new(
            TnmnCategory::OneB.as_str(),
            TnmnCategory::OneB.to_string().as_str(),
        );
        let one_c = Criteria::new(
            TnmnCategory::OneC.as_str(),
            TnmnCategory::OneC.to_string().as_str(),
        );
        let one_mi = Criteria::new(
            TnmnCategory::OneMi.as_str(),
            TnmnCategory::OneMi.to_string().as_str(),
        );
        let two = Criteria::new(
            TnmnCategory::Two.as_str(),
            TnmnCategory::Two.to_string().as_str(),
        );
        let two_a = Criteria::new(
            TnmnCategory::TwoA.as_str(),
            TnmnCategory::TwoA.to_string().as_str(),
        );
        let two_b = Criteria::new(
            TnmnCategory::TwoB.as_str(),
            TnmnCategory::TwoB.to_string().as_str(),
        );
        let two_c = Criteria::new(
            TnmnCategory::TwoC.as_str(),
            TnmnCategory::TwoC.to_string().as_str(),
        );
        let three = Criteria::new(
            TnmnCategory::Three.as_str(),
            TnmnCategory::Three.to_string().as_str(),
        );
        let three_a = Criteria::new(
            TnmnCategory::ThreeA.as_str(),
            TnmnCategory::ThreeA.to_string().as_str(),
        );
        let three_b = Criteria::new(
            TnmnCategory::ThreeB.as_str(),
            TnmnCategory::ThreeB.to_string().as_str(),
        );
        let three_c = Criteria::new(
            TnmnCategory::ThreeC.as_str(),
            TnmnCategory::ThreeC.to_string().as_str(),
        );
        let x = Criteria::new(
            TnmnCategory::X.as_str(),
            TnmnCategory::X.to_string().as_str(),
        );
        vec![
            zero,
            zero_i_minus,
            zero_i_plus,
            zero_mol_minus,
            zero_mol_plus,
            one,
            one_a,
            one_b,
            one_c,
            one_mi,
            two,
            two_a,
            two_b,
            two_c,
            three,
            three_a,
            three_b,
            three_c,
            x,
        ]
    }
}
