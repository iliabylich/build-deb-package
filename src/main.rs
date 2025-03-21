use args::Args;
use clap::Parser;
use config::Config;
use input::Input;
use list::List;
use miette::{Result, bail};
use strategist::Strategist;
use templates::Templates;

mod args;
mod config;
mod input;
mod list;
mod plan;
mod strategist;
mod templates;

fn main() -> Result<()> {
    let input = Input::from_env()?;

    match (Args::parse(), input) {
        (Args::Parse, Input::Singular(config)) => {
            println!("{:#?}", config);
        }

        (Args::Explain, Input::Singular(config)) => {
            let plan = Strategist::make_plan(config)?;
            plan.explain();
        }

        (Args::Run, input) => {
            let configs = match input {
                Input::Plural(list) => list.configs()?,
                Input::Singular(config) => vec![config],
            };
            for config in configs {
                let plan = Strategist::make_plan(config)?;
                plan.run()?;
            }
        }

        (Args::PrintGitUrl, Input::Singular(config)) => {
            let git_url = config.source.git_url().unwrap_or("none");
            println!("{git_url}");
        }

        (Args::PrintGitTagOrBranch, Input::Singular(config)) => {
            let git_branch = config.source.git_branch_or_tag().unwrap_or("none");
            println!("{git_branch}");
        }

        (args, input) => {
            bail!("can't run {args:?} on '{input}'")
        }
    }

    Ok(())
}
