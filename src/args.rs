#[derive(clap::Parser, Debug)]
pub(crate) enum Args {
    Parse,
    Explain,
    Run,

    PrintGitUrl,
    PrintGitTagOrBranch,
}
