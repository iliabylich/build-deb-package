use miette::{Context as _, IntoDiagnostic as _, Result};
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    #[serde(skip)]
    pub(crate) package_name: String,

    pub(crate) version: Version,
    pub(crate) dependencies: Vec<String>,
    pub(crate) source: Source,
    pub(crate) debian: Debian,
    pub(crate) arch: String,

    pub(crate) env: Option<HashMap<String, String>>,
    pub(crate) path: Option<Vec<String>>,
    pub(crate) additionally_produced_packages: Option<Vec<String>>,
}

impl Config {
    pub(crate) fn from_path(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to open {:?}", path))?;

        let mut config: Config = toml::from_str(&content)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to parse {:?}", path))?;

        config.package_name = Path::new(path)
            .with_extension("")
            .file_name()
            .wrap_err_with(|| format!("failed to get base filename from {:?}", path))?
            .to_str()
            .wrap_err("not a UTF-8 path")?
            .to_string();

        Ok(config)
    }

    pub(crate) fn from_env() -> Result<Self> {
        let base_configs_dir = std::env::var("BASE_CONFIGS_DIR")
            .into_diagnostic()
            .context("BASE_CONFIGS_DIR is not set")?;

        let config_path = std::env::var("CONFIG_PATH")
            .into_diagnostic()
            .context("CONFIG_PATH is not set")?;

        let config_path = format!("{base_configs_dir}/{config_path}");

        let config = Self::from_path(&config_path)?;
        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub(crate) enum Source {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "git-clone")]
    GitClone {
        url: String,
        branch_or_tag: String,
        post_clone_scripts: Option<Vec<String>>,
    },
}

impl Source {
    pub(crate) fn git_url(&self) -> Option<&str> {
        match self {
            Source::None => None,
            Source::GitClone { url, .. } => Some(url),
        }
    }

    pub(crate) fn git_branch_or_tag(&self) -> Option<&str> {
        match self {
            Source::None => None,
            Source::GitClone { branch_or_tag, .. } => Some(branch_or_tag),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Debian {
    pub(crate) changelog: bool,
    pub(crate) control: Option<Control>,
    pub(crate) rules: Option<HashMap<String, Vec<String>>>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Control {
    pub(crate) dependencies: Vec<String>,
    pub(crate) description: String,
}
