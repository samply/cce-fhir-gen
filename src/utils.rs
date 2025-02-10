use fhirbolt::{
    model::r4b::{
        resources::BundleEntryRequest,
        types::{Code, Uri},
    },
    serde::SerializeResource,
    xml,
};

use crate::extensions::option_ext::OptionExt;

pub fn get_full_url(id: &str) -> Uri {
    Uri::from(format!(
        "https://www.cancercoreeurope.eu/fhir-xml/examples/{}",
        id
    ))
}

pub fn get_bundle_entry_request(method: &str, url: &str) -> BundleEntryRequest {
    BundleEntryRequest {
        method: Code {
            value: method.to_string().into_some(),
            ..Default::default()
        },
        url: Uri::from(url),
        ..Default::default()
    }
}

pub fn print_fhir_data<T>(t: T, name: &str)
where
    T: SerializeResource,
{
    let serialized_data = xml::to_string(&t, None).unwrap();
    println!("{name}: {serialized_data:#?}");
    println!("");
}

pub fn get_ids(res_group: Option<&str>, res_type: &str, i: i8) -> (String, String) {
    let id = format!("{res_type}-identifier-{}", i);

    let ref_id = if res_group.is_some() {
        format!("{}/{}", res_group.unwrap(), id)
    } else {
        format!("{res_type}/{}", id)
    };
    
    (id, ref_id)
}
