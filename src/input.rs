use crate::{Config, List};
use miette::{Context as _, IntoDiagnostic as _, Result, bail};

#[expect(clippy::large_enum_variant)]
#[derive(Debug)]
pub(crate) enum Input {
    Plural(List),
    Singular(Config),
}

impl Input {
    pub(crate) fn from_env() -> Result<Self> {
        let dir = std::env::var("BASE_CONFIGS_DIR")
            .into_diagnostic()
            .context("BASE_CONFIGS_DIR is not set")?;

        let path = std::env::var("CONFIG_PATH")
            .into_diagnostic()
            .context("CONFIG_PATH is not set")?;

        if path.ends_with(".list") {
            let list = List::new(&dir, &path)?;
            Ok(Self::Plural(list))
        } else if path.ends_with(".toml") {
            let config = Config::new(&dir, &path)?;
            Ok(Self::Singular(config))
        } else {
            bail!("given config ({path}) is neither .list nor .toml")
        }
    }

    pub(crate) fn expand_into_config_list(self) -> Result<Vec<Config>> {
        match self {
            Input::Plural(list) => list.configs(),
            Input::Singular(config) => Ok(vec![config]),
        }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plural(_) => ".list file",
                Self::Singular(_) => ".toml file",
            }
        )
    }
}
