use crate::config::Config;
use clap::Parser;

#[derive(Parser)]
pub(crate) enum Args {
    Parse { path: String },

    Explain { path: String },

    Run { path: String },
}

pub(crate) fn parse() -> Args {
    Args::parse()
}

impl Args {
    pub(crate) fn run(self) -> miette::Result<()> {
        match self {
            Args::Parse { path } => {
                println!("{:#?}", Config::from_path(&path)?);
            }

            Args::Explain { path } => {
                let config = Config::from_path(&path)?;
                let plan = config.into_plan();
                plan.explain();
            }

            Args::Run { path } => {
                let config = Config::from_path(&path)?;
                let plan = config.into_plan();
                plan.run()?;
            }
        }

        Ok(())
    }
}
