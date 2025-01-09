use crate::config::Config;
use clap::Parser;
use miette::{Context, IntoDiagnostic};

#[derive(Parser)]
pub(crate) enum Args {
    Parse,
    Explain,
    Run,

    PrintGitUrl,
    PrintGitTagOrBranch,
}

pub(crate) fn parse() -> Args {
    Args::parse()
}

impl Args {
    pub(crate) fn run(self) -> miette::Result<()> {
        let base_configs_dir = std::env::var("BASE_CONFIGS_DIR")
            .into_diagnostic()
            .context("BASE_CONFIGS_DIR is not set")?;

        let configs = std::env::var("PACKAGES")
            .into_diagnostic()
            .context("PACKAGES is not set")?;

        for config in configs.split(",") {
            let path = format!("{base_configs_dir}/{config}.toml");
            self.run_one(&path)?;
        }

        Ok(())
    }

    fn run_one(&self, path: &str) -> miette::Result<()> {
        let config = Config::from_path(path)?;

        match self {
            Args::Parse => {
                println!("{:#?}", config);
            }

            Args::Explain => {
                let plan = config.into_plan();
                plan.explain();
            }

            Args::Run => {
                let plan = config.into_plan();
                plan.run()?;
            }

            Args::PrintGitUrl => {
                println!("{}", config.source.git_url().unwrap_or("none"));
            }

            Args::PrintGitTagOrBranch => {
                println!("{}", config.source.git_branch_or_tag().unwrap_or("none"));
            }
        }

        Ok(())
    }
}
