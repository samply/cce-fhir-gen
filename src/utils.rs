use fhirbolt::{
    model::r4b::{
        resources::BundleEntryRequest,
        types::{Code, Uri},
    },
    serde::SerializeResource,
    xml,
};

use crate::{extensions::option_ext::OptionExt, models::enums::id_type::IdType};

const CCE_URL: &str = "https://www.cancercoreeurope.eu";
const LOINC_URL: &str = "https://loinc.org";

const UICC_STAGE_CS: &str = "UICCStageCS";
const SITE_LOCATION_CS: &str = "SitelocationCS";
const SYST_THERAPY_TYPE_CS: &str = "SYSTTherapyTypeCS";
const VITAL_STATUS_CS: &str = "VitalStatusCS";
const SAMPLE_MATERIAL_TYPE_CS: &str = "SampleMaterialType";

const TNMMCS_CS: &str = "TNMMCS";
const TNMNCS_CS: &str = "TNMNCS";
const TNMTCS_CS: &str = "TNMTCS";
const TNMR_SYMBOL_CS: &str = "TNMrSymbolCS";
const TNMY_SYMBOL_CS: &str = "TNMySymbolCS";

pub const OBSERVATION_STATUS: &str = "final";

pub fn get_fhir_url() -> String {
    format!("{CCE_URL}/fhir/core")
}

pub fn get_loinc_url() -> Uri {
    Uri::from(LOINC_URL)
}

fn get_code_system_url(name: &str) -> String {
    format!("{}/CodeSystem/{}", get_fhir_url(), name)
}

pub fn get_uicc_stage_url() -> Uri {
    Uri::from(get_code_system_url(UICC_STAGE_CS))
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

pub fn get_tnmm_url() -> Uri {
    Uri::from(get_code_system_url(TNMMCS_CS))
}

pub fn get_tnmn_url() -> Uri {
    Uri::from(get_code_system_url(TNMNCS_CS))
}

pub fn get_tnmt_url() -> Uri {
    Uri::from(get_code_system_url(TNMTCS_CS))
}

pub fn get_tnmr_symbol_url() -> Uri {
    Uri::from(get_code_system_url(TNMR_SYMBOL_CS))
}

pub fn get_tnmy_symbol_url() -> Uri {
    Uri::from(get_code_system_url(TNMY_SYMBOL_CS))
}

pub fn get_body_site_url() -> Uri {
    Uri::from("urn:oid:1.3.6.1.4.1.19376.1.3.11.36")
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

pub fn get_ids(res_group: Option<&str>, id_type: IdType, res_type: &str, i: i8) -> (String, String) {
    let id_type_str = match id_type {
        IdType::Id => id_type.as_str().to_string(),
        IdType::Identifier => format!("src-{}", id_type.as_str()),
    };
    let id = format!("{res_type}-{}-{}", id_type_str, i);

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
    fn test_get_ids_with_id_type_id() {
        let (bundle_id, bundle_ref_id) = get_ids(None, IdType::Id, "Bundle", 1);
        assert_eq!(bundle_id, "Bundle-id-1", "id does not match");
        assert_eq!(bundle_ref_id, "Bundle/Bundle-id-1", "ref id does not match");
    }

    #[test]
    fn test_get_ids_with_id_type_identifier() {
        let (bundle_id, bundle_ref_id) = get_ids(None, IdType::Identifier, "Bundle", 1);
        assert_eq!(bundle_id, "Bundle-src-identifier-1", "id does not match");
        assert_eq!(bundle_ref_id, "Bundle/Bundle-src-identifier-1", "ref id does not match");
    }

    #[test]
    fn test_get_sample_mat_type_url() {
        let smt_url = get_sample_mat_type_url();
        let expected = Uri::from("https://www.cancercoreeurope.eu/fhir/core/CodeSystem/SampleMaterialType");
        assert_eq!(smt_url, expected, "urls do not match");
    }
}
