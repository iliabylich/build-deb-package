use crate::Config;
use anyhow::{Context as _, Result};

#[derive(Debug)]
pub(crate) struct List {
    dir: String,
    paths: Vec<String>,
}

impl List {
    pub(crate) fn new(dir: &str, path: &str) -> Result<Self> {
        let path = format!("{dir}/{path}");

        let content =
            std::fs::read_to_string(&path).with_context(|| format!("failed to open {:?}", path))?;

        let paths = content
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect::<Vec<_>>();

        Ok(Self {
            dir: dir.to_string(),
            paths,
        })
    }

    pub(crate) fn configs(self) -> Result<Vec<Config>> {
        let mut out = vec![];
        for path in self.paths {
            let config = Config::new(&self.dir, &path)?;
            out.push(config)
        }
        Ok(out)
    }
}
