use super::*;

#[derive(Debug)]
pub(crate) struct Context {
  pub(crate) directories: HashSet<PathBuf>,
  pub(crate) files: HashSet<PathBuf>,
  pub(crate) root: PathBuf,
}

impl TryFrom<PathBuf> for Context {
  type Error = Error;

  fn try_from(value: PathBuf) -> Result<Self> {
    let mut directories = HashSet::new();
    let mut files = HashSet::new();

    for entry in WalkDir::new(&value).follow_links(false) {
      let entry = entry?;

      if entry.depth() == 0 {
        continue;
      }

      let relative = entry
        .path()
        .strip_prefix(&value)
        .unwrap_or(entry.path())
        .to_path_buf();

      if entry.file_type().is_dir() {
        directories.insert(relative);
      } else {
        files.insert(relative);
      }
    }

    Ok(Self {
      directories,
      files,
      root: value,
    })
  }
}
