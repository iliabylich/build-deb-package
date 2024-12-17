#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Version {
    #[serde(rename = "0-0-stamp")]
    ZeroZeroTimestamp,

    #[serde(rename = "specific")]
    Specific(String),
}

impl Version {
    pub(crate) fn resolve(&self) -> String {
        match self {
            Self::ZeroZeroTimestamp => format!("0.0.{}", chrono::Utc::now().timestamp()),
            Self::Specific(version) => version.clone(),
        }
    }
}
