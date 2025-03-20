use crate::{Config, Templates, config::Source, plan::Plan};
use miette::{Context as _, Result};

pub(crate) struct Strategist {
    config: Config,
    templates: Templates,
    version: String,
    plan: Plan,
}

impl Strategist {
    pub(crate) fn make_plan(mut config: Config) -> Result<Plan> {
        let version = config.version.resolve();

        let env = config.env.take().unwrap_or_default();
        let path = config.path.take().unwrap_or_default();
        let plan = Plan::new(env, path);

        let templates = Templates::new()?;

        let strategist = Self {
            config,
            templates,
            version,
            plan,
        };
        strategist.make_final_plan()
    }

    fn package_name(&self) -> String {
        self.config.package_name.clone()
    }

    fn build_dir(&self) -> String {
        format!("/build/{}-{}", self.package_name(), self.version)
    }

    fn arch(&self) -> &str {
        &self.config.arch
    }

    fn take_dependencies(&mut self) -> Vec<String> {
        std::mem::take(&mut self.config.dependencies)
    }

    fn take_additionally_produced_packages(&mut self) -> Vec<String> {
        self.config
            .additionally_produced_packages
            .take()
            .unwrap_or_default()
    }

    fn make_final_plan(mut self) -> Result<Plan> {
        self.plan.exec("mkdir", ["-p", "/build"]);
        self.fetch_source();
        self.plan.cwd(self.build_dir());

        self.plan.exec("ls", ["-l", "--color=always"]);

        self.plan.exec("apt", ["update"]);
        let dependencies = self.take_dependencies();
        self.plan.exec(
            "apt",
            ["install", "-y"]
                .into_iter()
                .chain(dependencies.iter().map(|e| e.as_str()))
                .collect::<Vec<_>>(),
        );

        self.plan.exec("mkdir", ["-p", "debian"]);
        self.write_changelog()
            .context("failed write plan of changelog")?;
        self.write_compat().context("failed write plan of compat")?;
        self.write_control()
            .context("failed write plan of control")?;
        self.write_rules().context("failed write plan of rules")?;

        self.plan.exec("dh", ["binary"]);
        self.plan.cwd("/build");
        self.plan.exec("ls", ["-l", "--color=always"]);

        self.copy_deb(self.package_name());

        for package_name in self.take_additionally_produced_packages() {
            self.copy_deb(package_name)
        }

        Ok(self.plan)
    }

    fn fetch_source(&mut self) {
        match &self.config.source {
            Source::None => {
                self.plan.exec("mkdir", [self.build_dir()]);
            }
            Source::GitClone {
                url,
                branch_or_tag,
                post_clone_scripts,
            } => {
                self.plan.exec(
                    "git",
                    [
                        "clone",
                        url,
                        "--filter=blob:none",
                        "--recursive",
                        "--shallow-submodules",
                        "--depth=1",
                        "-q",
                        &format!("--branch={}", branch_or_tag),
                        &self.build_dir(),
                    ],
                );

                self.plan.cwd(self.build_dir());

                if let Some(post_clone_scripts) = post_clone_scripts {
                    for script in post_clone_scripts {
                        let mut script = script.split(" ");
                        let exe = script.next().expect("script can't be empty");
                        let args = script.collect::<Vec<_>>();
                        self.plan.exec(exe, args);
                    }
                }
            }
        }
    }

    fn write_changelog(&mut self) -> Result<()> {
        if self.config.debian.changelog {
            let contents = self
                .templates
                .changelog(&self.package_name(), &self.version)?;
            self.plan.write_file("debian/changelog", contents);
        }
        Ok(())
    }
    fn write_compat(&mut self) -> Result<()> {
        let contents = self.templates.compat()?;
        self.plan.write_file("debian/compat", contents);
        Ok(())
    }
    fn write_control(&mut self) -> Result<()> {
        if let Some(mut control) = self.config.debian.control.take() {
            let dependencies = std::mem::take(&mut control.dependencies);

            let contents = self.templates.control(
                &self.package_name(),
                self.arch(),
                &dependencies,
                &control.description,
            )?;
            self.plan.write_file("debian/control", contents);
        }
        Ok(())
    }
    fn write_rules(&mut self) -> Result<()> {
        if let Some(targets) = self.config.debian.rules.take() {
            let contents = self.templates.rules(targets)?;
            self.plan.write_file("debian/rules", contents);
            self.plan.exec("chmod", ["+x", "debian/rules"]);
        }
        Ok(())
    }

    fn copy_deb(&mut self, package_name: String) {
        let filename = format!("{package_name}_{}_{}.deb", self.version, self.arch());

        let dest = format!("/shared/{filename}");
        self.plan.exec("cp", [&filename, &dest]);

        self.plan.exec("mkdir", ["-p", "/shared/deb-latest"]);
        let shared_dest = format!("/shared/deb-latest/{package_name}.deb");
        self.plan.exec("cp", [&filename, &shared_dest]);
    }
}
