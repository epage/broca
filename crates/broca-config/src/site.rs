pub use camino::Utf8Path;
pub use camino::Utf8PathBuf;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum Root {
    Root(Utf8PathBuf),
    Config(Utf8PathBuf),
}

impl Root {
    pub fn discover() -> Result<Self, anyhow::Error> {
        let cwd = std::env::current_dir()?;
        let cwd = Utf8PathBuf::from_path_buf(cwd).map_err(|cwd| {
            anyhow::format_err!(
                "cannot discover site configuration in non-UTF8 path `{}`",
                cwd.display()
            )
        })?;
        Self::discover_from(&cwd)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn discover_from(cwd: &Utf8Path) -> Result<Self, anyhow::Error> {
        for dir in cwd.ancestors() {
            let mut path = dir.to_owned();

            for name in CONFIG_FILE_NAME {
                path.push(name);
                if path.exists() {
                    return Ok(Self::Config(path));
                }
                path.pop();
            }

            path.push(".git");
            if path.exists() {
                return Ok(Self::Root(dir.to_owned()));
            }
            path.pop();
        }

        Err(anyhow::format_err!("no config found for {cwd}"))
    }
}

const CONFIG_FILE_NAME: &[&str] = &[".broca.toml", "_broca.toml", "broca.toml"];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug)]
pub struct Site {
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _root: Option<Root>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _unused: Vec<String>,
}

impl Site {
    #[cfg(feature = "toml")]
    pub fn load() -> Result<Self, anyhow::Error> {
        let location = Root::discover()?;
        match &location {
            Root::Root(_) => Ok(Self {
                _root: Some(location),
                ..Default::default()
            }),
            Root::Config(path) => Self::load_from(path),
        }
    }

    #[cfg(feature = "toml")]
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn load_from(toml_path: &Utf8Path) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(toml_path.as_std_path())
            .map_err(|err| anyhow::format_err!("{err}: {toml_path}"))?;
        let (mut site, unused): (Self, _) = crate::toml::from_str(&content)?;
        site._root = Some(Root::Config(toml_path.to_owned()));
        site._unused = unused;
        Ok(site)
    }
}
