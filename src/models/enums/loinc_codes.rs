use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum TnmClassification {
    Pathologic,
    Clinical,
}

impl TnmClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmClassification::Pathologic => "21902-2",
            TnmClassification::Clinical => "21908-9",
        }
    }
}

#[derive(Debug, Dummy)]
pub enum TnmtClassification {
    Pathologic,
    Clinical,
}

impl TnmtClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmtClassification::Pathologic => "21899-0",
            TnmtClassification::Clinical => "21905-5",
        }
    }
}

#[derive(Debug, Dummy)]
pub enum TnmnClassification {
    Pathologic,
    Clinical,
}

impl TnmnClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmnClassification::Pathologic => "21900-6",
            TnmnClassification::Clinical => "21906-3",
        }
    }
}

#[derive(Debug, Dummy)]
pub enum TnmmClassification {
    Pathologic,
    Clinical,
}

impl TnmmClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            TnmmClassification::Pathologic => "21901-4",
            TnmmClassification::Clinical => "21907-1",
        }
    }
}
