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
        package_name: &str,
        version: &str,
        arch: &str,
    ) {
        plan.exec("mkdir", ["-p", "debian"]);

        if self.changelog {
            write_changelog(plan, package_name, version);
        }

        if let Some(compat) = self.compat {
            compat.write(plan);
        }

        if let Some(control) = self.control {
            control.write(plan, package_name, arch);
        }

        if let Some(rules) = self.rules {
            write_rules(plan, rules);
        }
    }
}

fn write_changelog(plan: &mut Plan, package_name: &str, version: &str) {
    plan.write_file(
        "debian/changelog",
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
    fn write(self, plan: &mut Plan) {
        plan.write_file("debian/compat", format!("{}", self.version));
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Control {
    dependencies: Option<Vec<String>>,
    description: String,
}

impl Control {
    fn write(self, plan: &mut Plan, package_name: &str, arch: &str) {
        let Self {
            dependencies,
            description,
        } = self;
        let dependencies = dependencies.unwrap_or_default().join(", ");

        plan.write_file(
            "debian/control",
            format!(
                "Source: {package_name}
Section: utils
Priority: extra
Maintainer: John Doe <john@doe.org>
Standards-Version: 4.6.2

Package: {package_name}
Section: utils
Priority: extra
Architecture: {arch}
Depends: {dependencies}
Description: {description}
"
            ),
        );
    }
}

fn write_rules(plan: &mut Plan, mut map: HashMap<String, Vec<String>>) {
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
        "debian/rules",
        format!(
            "#!/usr/bin/make -f
export DH_VERBOSE = 1

{rules}
"
        ),
    );

    plan.exec("chmod", ["+x", "debian/rules"]);
}
