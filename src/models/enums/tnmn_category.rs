use fake::Dummy;

#[derive(Debug, Dummy)]
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
