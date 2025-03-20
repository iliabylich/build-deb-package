#[derive(clap::Parser)]
pub(crate) enum Args {
    Parse,
    Explain,
    Run,

    PrintGitUrl,
    PrintGitTagOrBranch,
}
