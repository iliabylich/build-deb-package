use std::{
    io::{BufRead, BufReader},
    process::Stdio,
};

use miette::{Context, IntoDiagnostic};

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

    pub(crate) fn cwd(&mut self, path: impl Into<String>) {
        self.actions
            .push(Action::ChangeWorkingDir { path: path.into() })
    }

    pub(crate) fn explain(self) {
        for script in self.actions {
            script.explain();
            println!();
        }
    }

    pub(crate) fn run(self) -> miette::Result<()> {
        for script in self.actions {
            println!("::group::{}", script.header());
            let result = script.run();
            println!("::endgroup::");
            if result.is_err() {
                return result;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Action {
    ChangeWorkingDir { path: String },
    Script { exe: String, args: Vec<String> },
    WriteFile { path: String, contents: String },
}

const RESET: anstyle::Reset = anstyle::Reset;
const GREEN: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green)));
const YELLOW: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow)));

impl Action {
    fn explain(&self) {
        println!("{}", self.explanation());
    }

    fn explanation(&self) -> String {
        match self {
            Self::ChangeWorkingDir { path } => format!("{GREEN}cwd {path}{RESET}"),
            Self::Script { exe, args } => format!("{GREEN}{} {}{RESET}", exe, args.join(" ")),
            Self::WriteFile { path, contents } => {
                format!("{YELLOW}Writing to {path}:{RESET}\n{contents}")
            }
        }
    }

    fn header(&self) -> String {
        match self {
            Action::ChangeWorkingDir { path } => format!("cwd to {path}"),
            Action::Script { exe, args } => format!("running {} {}", exe, args.join(" ")),
            Action::WriteFile { path, .. } => format!("writing to {path}"),
        }
    }

    fn run(self) -> miette::Result<()> {
        self.explain();

        match self {
            Self::ChangeWorkingDir { path } => cwd(path),
            Self::Script { exe, args } => spawn_and_forward_stdout_and_stderr(exe, args),
            Self::WriteFile { path, contents } => write_file(path, contents),
        }
    }
}

fn cwd(path: String) -> miette::Result<()> {
    std::env::set_current_dir(&path)
        .into_diagnostic()
        .with_context(|| format!("failed to change working directory to {path}"))?;

    Ok(())
}

fn spawn_and_forward_stdout_and_stderr(exe: String, args: Vec<String>) -> miette::Result<()> {
    let mut child = std::process::Command::new(exe.clone())
        .args(args.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdout = child
        .stdout
        .take()
        .with_context(|| format!("failed to get child's stdout of {} {:?}", exe, args))?;

    let child_stderr = child
        .stderr
        .take()
        .with_context(|| format!("failed to get child's stderr of {} {:?}", exe, args))?;

    let (stdout_tx, _stdout_rx) = std::sync::mpsc::channel();
    let (stderr_tx, _stderr_rx) = std::sync::mpsc::channel();

    let stdout_thread = std::thread::spawn(move || {
        let stdout_lines = BufReader::new(child_stdout).lines();
        for line in stdout_lines {
            let line = line.unwrap();
            println!("{}", line);
            stdout_tx.send(line).unwrap();
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        let stderr_lines = BufReader::new(child_stderr).lines();
        for line in stderr_lines {
            let line = line.unwrap();
            eprintln!("{}", line);
            stderr_tx.send(line).unwrap();
        }
    });

    let status = child
        .wait()
        .into_diagnostic()
        .context("failed to wait on child")?;

    if !status.success() {
        miette::bail!(
            "failed to execute {} {:?}\nstatus code: {:?}",
            exe,
            args,
            status.code()
        );
    }

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    Ok(())
}

fn write_file(path: String, contents: String) -> miette::Result<()> {
    std::fs::write(&path, contents)
        .into_diagnostic()
        .with_context(|| format!("Failed to write to {path}"))
}
