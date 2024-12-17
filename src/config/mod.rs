use crate::plan::Plan;
use miette::{Context, IntoDiagnostic};
use serde::Deserialize;
use std::path::Path;

mod debian;
mod source;
mod version;

use debian::Debian;
use source::Source;
use version::Version;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) name: String,

    pub(crate) version: Version,
    pub(crate) dependencies: Vec<String>,
    pub(crate) source: Source,
    pub(crate) debian: Debian,
}

impl Config {
    pub(crate) fn from_path(path: &str) -> miette::Result<Self> {
        let content = std::fs::read_to_string(path)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to open {:?}", path))?;

        let mut config: Config = toml::from_str(&content)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to parse {:?}", path))?;

        if config.name.is_empty() {
            config.name = base_file_name(path)?;
        }

        Ok(config)
    }

    pub(crate) fn into_plan(self) -> Plan {
        let package_name = self.name;
        let version = self.version.resolve();
        let build_dir = format!("/build/{package_name}-{version}");

        let mut plan = Plan::new();

        plan.exec("mkdir", ["/build"]);
        self.source.fetch(&mut plan, &build_dir);

        install_dependencies(&mut plan, self.dependencies);

        self.debian
            .write_files(&mut plan, &build_dir, &package_name, &version);

        plan
    }
}

fn base_file_name(path: &str) -> miette::Result<String> {
    let path = Path::new(path).with_extension("");
    Ok(path
        .file_name()
        .wrap_err_with(|| format!("failed to get base filename from {:?}", path))?
        .to_str()
        .wrap_err("not a UTF-8 path")?
        .to_string())
}

fn install_dependencies(plan: &mut Plan, dependencies: Vec<String>) {
    plan.exec("apt", ["update"]);
    plan.exec(
        "apt",
        std::iter::once("install".to_string()).chain(dependencies),
    );
}
