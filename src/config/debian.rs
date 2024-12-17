use crate::plan::Plan;
use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Debian {
    pub(crate) changelog: bool,
    pub(crate) compat: Option<Compat>,
    pub(crate) control: Option<Control>,
    pub(crate) rules: Option<HashMap<String, Vec<String>>>,
}

impl Debian {
    pub(crate) fn write_files(
        self,
        plan: &mut Plan,
        build_dir: &str,
        package_name: &str,
        version: &str,
    ) {
        let debian_dir = format!("{build_dir}/debian");
        plan.exec("mkdir", ["-p", &debian_dir]);

        if self.changelog {
            write_changelog(plan, &debian_dir, package_name, version);
        }

        if let Some(compat) = self.compat {
            compat.write(plan, &debian_dir);
        }

        if let Some(control) = self.control {
            control.write(plan, &debian_dir, package_name);
        }

        if let Some(rules) = self.rules {
            write_rules(plan, &debian_dir, rules);
        }
    }
}

fn write_changelog(plan: &mut Plan, debian_dir: &str, package_name: &str, version: &str) {
    plan.write_file(
        format!("{debian_dir}/changelog"),
        format!(
            "{package_name} ({version}) unstable; urgency=low

  * Release

 -- John Doe <john@doe.org>  Wed, 22 May 2024 17:54:24 +0000
"
        ),
    );
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Compat {
    version: u8,
}

impl Compat {
    fn write(self, plan: &mut Plan, debian_dir: &str) {
        plan.write_file(format!("{debian_dir}/compat"), format!("{}", self.version));
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Control {
    dependencies: Option<Vec<String>>,
    description: String,
}

impl Control {
    fn write(self, plan: &mut Plan, debian_dir: &str, package_name: &str) {
        let Self {
            dependencies,
            description,
        } = self;
        let dependencies = dependencies.unwrap_or_default().join(" ");

        plan.write_file(
            format!("{debian_dir}/control"),
            format!(
                "Source: {package_name}
Section: utils
Priority: extra
Maintainer: John Doe <john@doe.org>
Standards-Version: 4.6.2

Package: {package_name}
Section: utils
Priority: extra
Architecture: amd64
Depends: {dependencies}
Description: {description}
"
            ),
        );
    }
}

fn write_rules(plan: &mut Plan, debian_dir: &str, mut map: HashMap<String, Vec<String>>) {
    let mut rules = String::new();

    fn append_rule(out: &mut String, rule: String, lines: Vec<String>) {
        use std::fmt::Write;

        writeln!(out, "{rule}:").unwrap();
        for line in lines {
            writeln!(out, "\t{line}").unwrap();
        }
        writeln!(out).unwrap();
    }

    if let Some((rule, lines)) = map.remove_entry("%") {
        append_rule(&mut rules, rule, lines);
    }
    for (rule, lines) in map {
        append_rule(&mut rules, rule, lines);
    }

    plan.write_file(
        format!("{debian_dir}/rules"),
        format!(
            "#!/usr/bin/make -f
export DH_VERBOSE = 1

{rules}
"
        ),
    )
}
