#[derive(Debug)]
pub(crate) struct Plan {
    actions: Vec<Action>,
}

impl Plan {
    pub(crate) fn new() -> Self {
        Self { actions: vec![] }
    }

    pub(crate) fn exec(
        &mut self,
        exe: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.actions.push(Action::Script {
            exe: exe.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
        });
    }

    pub(crate) fn write_file(&mut self, path: impl Into<String>, contents: impl Into<String>) {
        self.actions.push(Action::WriteFile {
            path: path.into(),
            contents: contents.into(),
        })
    }

    pub(crate) fn explain(self) {
        for script in self.actions {
            script.explain();
            println!();
        }
    }

    pub(crate) fn run(self) -> miette::Result<()> {
        for script in self.actions {
            script.run()?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Action {
    Script { exe: String, args: Vec<String> },
    WriteFile { path: String, contents: String },
}

impl Action {
    fn explain(self) {
        match self {
            Self::Script { exe, args } => println!("{} {}", exe, args.join(" ")),
            Self::WriteFile { path, contents } => println!("Writing to {path}:\n{contents}"),
        }
    }

    fn run(self) -> miette::Result<()> {
        match self {
            Self::Script { exe, args } => todo!("{} {}", exe, args.join(" ")),
            Self::WriteFile { path, contents } => todo!("Writing to {path}:\n{contents}"),
        }
    }
}
