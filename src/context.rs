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
    let (mut directories, mut files) = (HashSet::new(), HashSet::new());

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

impl Context {
  pub(crate) fn matches(&self, rule: &dyn Rule) -> Result<Vec<PathBuf>> {
    let matchers = rule
      .actions()
      .iter()
      .filter_map(|action| match action {
        Action::Remove { pattern, .. } => Some(pattern),
        Action::Command(_) => None,
      })
      .map(|pattern| Ok(Glob::new(pattern)?.compile_matcher()))
      .collect::<Result<Vec<_>>>()?;

    let matches = matchers
      .into_iter()
      .flat_map(|matcher| {
        self
          .directories
          .iter()
          .chain(self.files.iter())
          .filter(move |path| matcher.is_match(path))
          .cloned()
      })
      .collect::<HashSet<_>>();

    let mut matched = matches.into_iter().collect::<Vec<PathBuf>>();
    matched.sort_unstable();

    let (pruned, _) = matched.into_iter().fold(
      (Vec::new(), Vec::new()),
      |(mut pruned, mut kept_directories), relative_path| {
        let full_path = self.root.join(&relative_path);

        if !full_path.exists() {
          return (pruned, kept_directories);
        }

        if kept_directories
          .iter()
          .any(|dir| relative_path.starts_with(dir))
        {
          return (pruned, kept_directories);
        }

        if full_path.is_dir() {
          kept_directories.push(relative_path.clone());
        }

        pruned.push(relative_path);

        (pruned, kept_directories)
      },
    );

    Ok(pruned)
  }

  pub(crate) fn modified_time(&self) -> Result<SystemTime> {
    Ok(fs::metadata(&self.root)?.modified()?)
  }
}

#[cfg(test)]
mod tests {
  use {super::*, temptree::temptree};

  struct TestRule {
    actions: &'static [Action],
  }

  impl Rule for TestRule {
    fn actions(&self) -> &[Action] {
      self.actions
    }

    fn applies(&self, _context: &Context) -> bool {
      true
    }

    fn id(&self) -> &'static str {
      "test"
    }

    fn name(&self) -> &'static str {
      "test"
    }
  }

  #[test]
  fn matches_returns_empty_when_no_patterns_match() {
    let tree = temptree! {
      "README.md": "hello",
    };

    let context = Context::try_from(tree.path().to_path_buf()).unwrap();

    let rule = TestRule {
      actions: &[Action::Remove {
        pattern: "nope/**",
        reason: "test",
      }],
    };

    assert!(context.matches(&rule).unwrap().is_empty());
  }

  #[test]
  fn matches_only_files() {
    let tree = temptree! {
      "b.log": "b",
      "a.log": "a",
    };

    let context = Context::try_from(tree.path().to_path_buf()).unwrap();

    let rule = TestRule {
      actions: &[Action::Remove {
        pattern: "*.log",
        reason: "test",
      }],
    };

    assert_eq!(
      context.matches(&rule).unwrap(),
      vec![PathBuf::from("a.log"), PathBuf::from("b.log")],
    );
  }

  #[test]
  fn matches_skips_deleted_paths() {
    let tree = temptree! {
      "stale.log": "x",
    };

    let root = tree.path();

    let context = Context::try_from(root.to_path_buf()).unwrap();

    fs::remove_file(root.join("stale.log")).unwrap();

    let rule = TestRule {
      actions: &[Action::Remove {
        pattern: "*.log",
        reason: "test",
      }],
    };

    assert!(context.matches(&rule).unwrap().is_empty());
  }

  #[test]
  fn matches_prunes_nested_paths() {
    let tree = temptree! {
      "node_modules": {
        "left-pad": {
          "index.js": "x",
        },
      },
      "target": {
        "debug": {
          "app": "x",
        },
      },
      "README.md": "hello",
    };

    let context = Context::try_from(tree.path().to_path_buf()).unwrap();

    let rule = TestRule {
      actions: &[
        Action::Remove {
          pattern: "node_modules",
          reason: "test",
        },
        Action::Remove {
          pattern: "node_modules/**",
          reason: "test",
        },
        Action::Remove {
          pattern: "target",
          reason: "test",
        },
        Action::Remove {
          pattern: "target/**",
          reason: "test",
        },
        Action::Remove {
          pattern: "*.md",
          reason: "test",
        },
        Action::Command("echo ignored"),
      ],
    };

    assert_eq!(
      context.matches(&rule).unwrap(),
      vec![
        PathBuf::from("README.md"),
        PathBuf::from("node_modules"),
        PathBuf::from("target"),
      ],
    );
  }
}
