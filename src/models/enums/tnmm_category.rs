use fake::Dummy;
use strum::Display;

use crate::lens::{catalogue::Criteria, traits::CriteriaConverter};

#[derive(Debug, Display, Dummy)]
pub enum TnmmCategory {
    Zero,
    One,
    OneA,
    OneB,
    OneC,
    OneD,
    OneE,
    ZeroIMinus,
    ZeroIPlus,
    ZeroMolMinus,
    ZeroMolPlus,
}

impl TnmmCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmmCategory::Zero => "0",
            TnmmCategory::One => "1",
            TnmmCategory::OneA => "1a",
            TnmmCategory::OneB => "1b",
            TnmmCategory::OneC => "1c",
            TnmmCategory::OneD => "1d",
            TnmmCategory::OneE => "1e",
            TnmmCategory::ZeroIMinus => "0(i-)",
            TnmmCategory::ZeroIPlus => "0(i+)",
            TnmmCategory::ZeroMolMinus => "0(mol-)",
            TnmmCategory::ZeroMolPlus => "0(mol+)",
        }
    }
}

impl CriteriaConverter for TnmmCategory {
    fn get_criteria() -> Vec<Criteria> {
        let zero = Criteria::new(
            TnmmCategory::Zero.as_str(),
            TnmmCategory::Zero.to_string().as_str(),
        );
        let one = Criteria::new(
            TnmmCategory::One.as_str(),
            TnmmCategory::One.to_string().as_str(),
        );
        let one_a = Criteria::new(
            TnmmCategory::OneA.as_str(),
            TnmmCategory::OneA.to_string().as_str(),
        );
        let one_b = Criteria::new(
            TnmmCategory::OneB.as_str(),
            TnmmCategory::OneB.to_string().as_str(),
        );
        let one_c = Criteria::new(
            TnmmCategory::OneC.as_str(),
            TnmmCategory::OneC.to_string().as_str(),
        );
        let one_d = Criteria::new(
            TnmmCategory::OneD.as_str(),
            TnmmCategory::OneD.to_string().as_str(),
        );
        let one_e = Criteria::new(
            TnmmCategory::OneE.as_str(),
            TnmmCategory::OneE.to_string().as_str(),
        );
        let zero_i_minus = Criteria::new(
            TnmmCategory::ZeroIMinus.as_str(),
            TnmmCategory::ZeroIMinus.to_string().as_str(),
        );
        let zero_i_plus = Criteria::new(
            TnmmCategory::ZeroIPlus.as_str(),
            TnmmCategory::ZeroIPlus.to_string().as_str(),
        );
        let zero_mol_minus = Criteria::new(
            TnmmCategory::ZeroMolMinus.as_str(),
            TnmmCategory::ZeroMolMinus.to_string().as_str(),
        );
        let zero_mol_plus = Criteria::new(
            TnmmCategory::ZeroMolPlus.as_str(),
            TnmmCategory::ZeroMolPlus.to_string().as_str(),
        );
        vec![
            zero,
            one,
            one_a,
            one_b,
            one_c,
            one_d,
            one_e,
            zero_i_minus,
            zero_i_plus,
            zero_mol_minus,
            zero_mol_plus,
        ]
    }
}
