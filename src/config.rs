use crate::colors::{GREEN, RESET, YELLOW};
use miette::{Context as _, IntoDiagnostic as _, Result};
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    #[serde(skip)]
    pub(crate) package_name: String,
    #[serde(skip)]
    pub(crate) config_dir: String,
    #[serde(skip)]
    pub(crate) config_path: String,

    pub(crate) version: Version,
    pub(crate) dependencies: Vec<String>,
    pub(crate) source: Source,
    pub(crate) debian: Debian,
    pub(crate) arch: String,

    #[serde(default)]
    pub(crate) env: HashMap<String, String>,
    #[serde(default)]
    pub(crate) path: Vec<String>,
    pub(crate) additionally_produced_packages: Option<Vec<String>>,
}

impl Config {
    pub(crate) fn new(dir: &str, path: &str) -> Result<Self> {
        let absolute_path = format!("{dir}/{path}");

        let content = std::fs::read_to_string(&absolute_path)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to read {absolute_path}"))?;

        let mut config: Config = toml::from_str(&content)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to parse {absolute_path}"))?;

        config.config_dir = dir.to_string();
        config.config_path = path.to_string();

        config.package_name = Path::new(&absolute_path)
            .with_extension("")
            .file_name()
            .wrap_err_with(|| format!("failed to get base filename from {absolute_path}"))?
            .to_str()
            .wrap_err("not a UTF-8 path")?
            .to_string();

        Ok(config)
    }

    pub(crate) fn bump_version_trailer(self) -> Result<()> {
        let dir = self.config_dir;
        let path = self.config_path;

        let absolute_path = format!("{dir}/{path}");
        let content = std::fs::read_to_string(&absolute_path)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to read {absolute_path}",))?;

        if content.contains("0-0-stamp") {
            println!("[{path}] {YELLOW}Skipping, it has monotonically incrementing version{RESET}");
            return Ok(());
        }

        const VERSION_PREFIX: &str = "version = { specific = \"";
        let (pre, post) = content
            .split_once(VERSION_PREFIX)
            .with_context(|| format!("failed to find version prefix in {absolute_path}"))?;
        let (version, post) = post
            .split_once('"')
            .with_context(|| format!("no version terminator in {path}"))?;
        let new_version = if let Some((base, trailer)) = version.split_once('-') {
            let trailer = trailer
                .parse::<u32>()
                .into_diagnostic()
                .with_context(|| format!("non-numeric version trailer in {path}"))?;
            format!("{base}-{}", trailer + 1)
        } else {
            format!("{version}-1")
        };
        println!("[{path}] {GREEN}Bumping {version} -> {new_version}{RESET}");

        let new_config = format!("{pre}{VERSION_PREFIX}{new_version}\"{post}");

        std::fs::write(absolute_path, new_config)
            .into_diagnostic()
            .with_context(|| format!("failed to update {path}"))?;

        Ok(())
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
    #[serde(default)]
    pub(crate) rules: HashMap<String, Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Control {
    pub(crate) dependencies: Vec<String>,
    pub(crate) description: String,
}
