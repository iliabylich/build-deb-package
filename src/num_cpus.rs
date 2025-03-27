use crate::colors::{GREEN, RESET};
use miette::{Context as _, IntoDiagnostic as _, Result};

pub(crate) fn num_cpus() -> Result<u8> {
    let stdout = std::process::Command::new("nproc")
        .output()
        .into_diagnostic()
        .context("failed to call nproc")?
        .stdout;
    let stdout = String::from_utf8(stdout)
        .into_diagnostic()
        .context("non-utf-8 output of nproc")?;

    let num_cpus = stdout
        .trim()
        .parse()
        .into_diagnostic()
        .context("non-numeric output of nproc")?;

    println!("{GREEN}Number of CPUs: {num_cpus}{RESET}");

    Ok(num_cpus)
}
