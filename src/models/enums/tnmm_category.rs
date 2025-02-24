use fake::Dummy;

#[derive(Debug, Dummy)]
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
