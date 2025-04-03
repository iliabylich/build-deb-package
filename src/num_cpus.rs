use crate::colors::{GREEN, RESET};
use anyhow::{Context as _, Result};

pub(crate) fn num_cpus() -> Result<u8> {
    let stdout = std::process::Command::new("nproc")
        .output()
        .context("failed to call nproc")?
        .stdout;
    let stdout = String::from_utf8(stdout).context("non-utf-8 output of nproc")?;

    let num_cpus = stdout
        .trim()
        .parse()
        .context("non-numeric output of nproc")?;

    println!("{GREEN}Number of CPUs: {num_cpus}{RESET}");

    Ok(num_cpus)
}
