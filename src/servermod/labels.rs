use prometheus_client::encoding::{EncodeLabelSet, EncodeLabelValue};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
pub enum Methods {
    GET,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct HttpLabel {
    pub method: Methods,
    pub path: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct DiskLabel {
    pub mountpoint: String,
}
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct NetLabel {
    pub interface: String,
}
