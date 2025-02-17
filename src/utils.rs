use fhirbolt::{
    model::r4b::{
        resources::BundleEntryRequest,
        types::{Code, Uri},
    },
    serde::SerializeResource,
    xml,
};

use crate::extensions::option_ext::OptionExt;

const CCE_URL: &str = "https://www.cancercoreeurope.eu";

const SITE_LOCATION_CS: &str = "SitelocationCS";
const SYST_THERAPY_TYPE_CS: &str = "SYSTTherapyTypeCS";
const VITAL_STATUS_CS: &str = "VitalStatusCS";
const SAMPLE_MATERIAL_TYPE_CS: &str = "SampleMaterialType";

pub fn get_fhir_url() -> String {
    format!("{CCE_URL}/fhir/core")
}

fn get_code_system_url(name: &str) -> String {
    format!("{}/CodeSystem/{}", get_fhir_url(), name)
}

pub fn get_site_location_url() -> Uri {
    Uri::from(get_code_system_url(SITE_LOCATION_CS))
}

pub fn get_sample_mat_type_url() -> Uri {
    Uri::from(get_code_system_url(SAMPLE_MATERIAL_TYPE_CS))
}

pub fn get_syst_therapy_type_url() -> Uri {
    Uri::from(get_code_system_url(SYST_THERAPY_TYPE_CS))
}

pub fn get_vital_status_url() -> Uri {
    Uri::from(get_code_system_url(VITAL_STATUS_CS))
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ids() {
        let (bundle_id, bundle_ref_id) = get_ids(None, "Bundle", 1);
        assert_eq!(bundle_id, "Bundle-identifier-1", "id does not match");
        assert_eq!(bundle_ref_id, "Bundle/Bundle-identifier-1", "ref id does not match");
    }

    #[test]
    fn test_get_sample_mat_type_url() {
        let smt_url = get_sample_mat_type_url();
        let expected = Uri::from("https://www.cancercoreeurope.eu/fhir/core/CodeSystem/SampleMaterialType");
        assert_eq!(smt_url, expected, "urls do not match");
    }
}
