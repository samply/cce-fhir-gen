pub struct BiLingualDisplay {
    pub de: String,
    pub en: String,
}

impl BiLingualDisplay {
    pub fn new(de: &str, en: &str) -> Self {
        Self { de: de.to_string(), en: en.to_string() }
    }
}

pub struct BiLingualKey {
    pub key: String,
    pub de: String,
    pub en: String,
}

impl BiLingualKey {
    pub fn new(key: &str, de: &str, en: &str) -> Self {
        Self { key: key.to_string(), de: de.to_string(), en: en.to_string() }
    }
}
