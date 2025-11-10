use fhirbolt::model::r4b::types::{ContactDetail, ContactPoint, Narrative};

use crate::utils::{CCE_NAME, CCE_URL, GENERATED, XHTML_NAMESPACE};

pub fn get_contact_details() -> Vec<ContactDetail> {
    let contact_point = ContactPoint {
        system: Some("other".to_string().into()),
        value: Some(CCE_URL.to_string().into()),
        ..Default::default()
    };

    let contact_detail = ContactDetail {
        name: Some(CCE_NAME.to_string().into()),
        telecom: vec![contact_point],
        ..Default::default()
    };

    vec![contact_detail]
}

pub fn get_generated_narrative(html_description: &str, description: &str) -> Narrative {
    let narrative = Narrative {
        status: GENERATED.into(),
        div: format!(
            "<div xmlns=\"{}\"><p><b>{}</b></p><p>{}</p></div>",
            XHTML_NAMESPACE, html_description, description
        )
        .into(),
        ..Default::default()
    };

    narrative
}
