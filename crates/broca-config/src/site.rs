pub use camino::Utf8Path;
pub use camino::Utf8PathBuf;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum SiteLocation {
    Root(Utf8PathBuf),
    Config(Utf8PathBuf),
}

impl SiteLocation {
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
