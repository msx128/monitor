use prometheus_client::encoding::{EncodeLabelSet, EncodeLabelValue};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
enum Methods {
    GET,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    method: Methods,
    path: String,
}

pub fn default_lable() -> Labels {
    Labels {
        method: Methods::GET,
        path: "/metrics".to_string(),
    }
}
