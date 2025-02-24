use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum TnmrSymbol {
    R,
    Nine,
}

const R_TUPLE: (&str, &str) = ("r", "Classification was used to assess a recurrence");
const NINE_TUPLE: (&str, &str) = ("9", "Native classification before a recurrence");

impl TnmrSymbol {
    pub fn code(&self) -> &'static str {
        match self {
            TnmrSymbol::R => R_TUPLE.0,
            TnmrSymbol::Nine => NINE_TUPLE.0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TnmrSymbol::R => R_TUPLE.1,
            TnmrSymbol::Nine => NINE_TUPLE.1,
        }
    }
}
