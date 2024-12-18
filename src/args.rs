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
        let path = std::env::var("CONFIG_PATH")
            .into_diagnostic()
            .context("CONFIG_PATH is not set")?;
        let config = Config::from_path(&path)?;

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
