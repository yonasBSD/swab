use super::*;

pub(crate) trait PathExt {
  fn directories(&self) -> Result<Vec<PathBuf>>;
  fn size(&self) -> Result<u64>;
}

impl PathExt for Path {
  fn directories(&self) -> Result<Vec<PathBuf>> {
    let mut directories = Vec::new();

    for entry in fs::read_dir(self)? {
      let entry = entry?;

      let path = entry.path();

      if path.is_dir() {
        directories.push(path);
      }
    }

    directories.sort_unstable();

    Ok(directories)
  }

  fn size(&self) -> Result<u64> {
    let metadata = fs::metadata(self)?;

    if metadata.is_file() {
      return Ok(metadata.len());
    }

    if !metadata.is_dir() {
      return Ok(0);
    }

    let mut total = 0;

    for entry in WalkDir::new(self).follow_links(false) {
      let entry = entry?;

      if entry.file_type().is_file() {
        total += entry.metadata()?.len();
      }
    }

    Ok(total)
  }
}

#[cfg(test)]
mod tests {
  use {super::*, tempfile::TempDir};

  struct Test {
    expected_size: Option<u64>,
    target: Option<PathBuf>,
    tempdir: TempDir,
  }

  impl Test {
    fn dir(self, path: &str) -> Self {
      fs::create_dir_all(self.tempdir.path().join(path)).unwrap();

      self
    }

    fn error(self) {
      assert!(
        self
          .target
          .expect("target must be set for error test")
          .size()
          .is_err()
      );
    }

    fn file(self, path: &str, content: &str) -> Self {
      let file_path = self.tempdir.path().join(path);

      if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
      }

      fs::write(&file_path, content).unwrap();

      self
    }

    fn new() -> Self {
      Self {
        expected_size: None,
        target: None,
        tempdir: TempDir::new().unwrap(),
      }
    }

    fn run(self) {
      assert_eq!(
        self
          .target
          .unwrap_or_else(|| self.tempdir.path().to_path_buf())
          .size()
          .unwrap(),
        self.expected_size.expect("expected size must be set")
      );
    }

    fn size(self, expected: u64) -> Self {
      Self {
        expected_size: Some(expected),
        ..self
      }
    }

    fn target(self, path: &str) -> Self {
      Self {
        target: Some(self.tempdir.path().join(path)),
        ..self
      }
    }
  }

  #[test]
  fn size_of_file() {
    Test::new()
      .file("test.txt", "hello")
      .target("test.txt")
      .size(5)
      .run();
  }

  #[test]
  fn size_of_empty_file() {
    Test::new()
      .file("empty.txt", "")
      .target("empty.txt")
      .size(0)
      .run();
  }

  #[test]
  fn size_of_empty_directory() {
    Test::new().size(0).run();
  }

  #[test]
  fn size_of_directory_with_files() {
    Test::new()
      .file("a.txt", "aaa")
      .file("b.txt", "bbbbb")
      .size(8)
      .run();
  }

  #[test]
  fn size_of_nested_directory() {
    Test::new()
      .file("root.txt", "root")
      .file("subdir/nested.txt", "nested")
      .size(10)
      .run();
  }

  #[test]
  fn size_of_deeply_nested_directory() {
    Test::new()
      .file("a.txt", "a")
      .file("level1/b.txt", "bb")
      .file("level1/level2/c.txt", "ccc")
      .size(6)
      .run();
  }

  #[test]
  fn size_of_subdirectory() {
    Test::new()
      .file("root.txt", "root")
      .file("subdir/nested.txt", "nested")
      .target("subdir")
      .size(6)
      .run();
  }

  #[test]
  fn size_of_empty_subdirectory() {
    Test::new().dir("subdir").target("subdir").size(0).run();
  }

  #[test]
  fn size_of_nonexistent_path_returns_error() {
    Test::new().target("does_not_exist").error();
  }
}
