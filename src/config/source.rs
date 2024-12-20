use crate::plan::Plan;

#[derive(serde::Deserialize, Debug)]
pub(crate) enum Source {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "git-clone")]
    GitClone {
        url: String,
        branch_or_tag: String,
        post_clone_scripts: Option<Vec<String>>,
    },
}

impl Source {
    pub(crate) fn fetch(self, plan: &mut Plan, build_dir: &str) {
        match self {
            Source::None => {
                plan.exec("mkdir", [build_dir]);
            }
            Source::GitClone {
                url,
                branch_or_tag,
                post_clone_scripts,
            } => {
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

                plan.cwd(build_dir);

                if let Some(post_clone_scripts) = post_clone_scripts {
                    for script in post_clone_scripts {
                        let mut script = script.split(" ");
                        let exe = script.next().expect("script can't be empty");
                        let args = script.collect::<Vec<_>>();
                        plan.exec(exe, args);
                    }
                }
            }
        }
    }

    pub(crate) fn git_url(&self) -> Option<&str> {
        match self {
            Source::None => None,
            Source::GitClone { url, .. } => Some(url),
        }
    }

    pub(crate) fn git_branch_or_tag(&self) -> Option<&str> {
        match self {
            Source::None => None,
            Source::GitClone { branch_or_tag, .. } => Some(branch_or_tag),
        }
    }
}
