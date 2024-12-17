use crate::plan::Plan;

#[derive(serde::Deserialize, Debug)]
pub(crate) enum Source {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "git-clone")]
    GitClone { url: String, branch_or_tag: String },
}

impl Source {
    pub(crate) fn fetch(self, plan: &mut Plan, build_dir: &str) {
        match self {
            Source::None => {
                plan.exec("mkdir", [build_dir]);
            }
            Source::GitClone { url, branch_or_tag } => {
                plan.exec(
                    "git",
                    [
                        "clone",
                        &url,
                        "--filter=blob:none",
                        "--recursive",
                        "--shallow-submodules",
                        "--depth=1",
                        "-q",
                        &format!("--branch={}", branch_or_tag),
                        build_dir,
                    ],
                );
            }
        }
    }
}
