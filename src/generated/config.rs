use serde::Deserialize;
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(alias = "endHeaderName")]
    pub end_header_name: String,
    #[serde(alias = "startHeaderName")]
    pub start_header_name: String,
}
