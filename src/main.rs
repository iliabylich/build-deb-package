use args::Args;
use clap::Parser;
use config::Config;
use miette::Result;
use strategist::Strategist;
use templates::Templates;

mod args;
mod config;
mod plan;
mod strategist;
mod templates;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from_env()?;

    match args {
        Args::Parse => {
            println!("{:#?}", config);
        }

        Args::Explain => {
            let plan = Strategist::make_plan(config)?;
            plan.explain();
        }

        Args::Run => {
            let plan = Strategist::make_plan(config)?;
            plan.run()?;
        }

        Args::PrintGitUrl => {
            let git_url = config.source.git_url().unwrap_or("none");
            println!("{git_url}");
        }

        Args::PrintGitTagOrBranch => {
            let git_branch = config.source.git_branch_or_tag().unwrap_or("none");
            println!("{git_branch}");
        }
    }

    Ok(())
}
