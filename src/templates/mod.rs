use miette::{Context as _, IntoDiagnostic as _, Result};
use std::collections::HashMap;

pub(crate) struct Templates {
    engine: upon::Engine<'static>,
}

impl Templates {
    pub(crate) fn new() -> Result<Self> {
        let mut engine = upon::Engine::new();

        engine
            .add_template("changelog", include_str!("./changelog"))
            .into_diagnostic()
            .context("failed to add changelog template")?;
        engine
            .add_template("compat", include_str!("./compat"))
            .into_diagnostic()
            .context("failed to add compat template")?;
        engine
            .add_template("control", include_str!("./control"))
            .into_diagnostic()
            .context("failed to add control template")?;
        engine
            .add_template("rules", include_str!("./rules"))
            .into_diagnostic()
            .context("failed to add rules template")?;

        Ok(Self { engine })
    }

    pub(crate) fn changelog(&self, package_name: &str, version: &str) -> Result<String> {
        self.engine
            .template("changelog")
            .render(upon::value! {
                package_name: package_name,
                version: version
            })
            .to_string()
            .into_diagnostic()
            .context("failed to render changelog template")
    }

    pub(crate) fn compat(&self) -> Result<String> {
        self.engine
            .template("compat")
            .render(upon::value! {})
            .to_string()
            .into_diagnostic()
            .context("failed to render compat template")
    }

    pub(crate) fn control(
        &self,
        package_name: &str,
        arch: &str,
        dependencies: &[String],
        description: &str,
    ) -> Result<String> {
        self.engine
            .template("control")
            .render(upon::value! {
                package_name: package_name,
                arch: arch,
                dependencies: dependencies.join(", "),
                description: description
            })
            .to_string()
            .into_diagnostic()
            .context("failed to render control template")
    }

    pub(crate) fn rules(&self, mut targets: HashMap<String, Vec<String>>) -> Result<String> {
        let mut out = vec![];
        if let Some((target, lines)) = targets.remove_entry("%") {
            out.push(format!("{target}:"));
            for line in lines {
                out.push(format!("\t{line}"));
            }
        }
        for (target, lines) in targets {
            out.push(format!("{target}:"));
            for line in lines {
                out.push(format!("\t{line}"));
            }
        }
        let targets = out.join("\n");
        self.engine
            .template("rules")
            .render(upon::value! { targets: targets })
            .to_string()
            .into_diagnostic()
            .context("failed to render rules templates")
    }
}
