use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum TnmySymbol {
    Y,
    Nine,
}

// const Y_TUPLE: (&str, &str) = ("y", "Classification occurred during or after initial multimodal therapy");
// const NINE_TUPLE: (&str, &str) = ("9", "Native classification");

impl TnmySymbol {
    // pub fn code(&self) -> &'static str {
    //     match self {
    //         TnmySymbol::Y => Y_TUPLE.0,
    //         TnmySymbol::Nine => NINE_TUPLE.0,
    //     }
    // }

    // pub fn description(&self) -> &'static str {
    //     match self {
    //         TnmySymbol::Y => Y_TUPLE.1,
    //         TnmySymbol::Nine => NINE_TUPLE.1,
    //     }
    // }
}
