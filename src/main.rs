mod args;
mod config;
mod plan;

fn main() -> miette::Result<()> {
    let args = args::parse();
    args.run()?;

    Ok(())
}
