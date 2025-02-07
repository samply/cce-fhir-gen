use fhirbolt::model::r4b::{resources::BundleEntryRequest, types::{Code, Uri}};

pub fn get_full_url(id: &str) -> Uri {
    Uri::from(format!("https://www.cancercoreeurope.eu/fhir-xml/examples/{}", id))
}

pub fn get_bundle_entry_request(method: &str, url: &str) -> BundleEntryRequest {
    BundleEntryRequest {
        method: Code {
            value: Some(method.to_string()),
            ..Default::default()
        },
        url: Uri::from(url),
        ..Default::default()
    }
}
