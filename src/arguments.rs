use super::*;

static RULES: &[&dyn Rule] = &[&Rust];

#[derive(Debug, Parser)]
pub(crate) struct Arguments {
  directories: Vec<PathBuf>,
  #[clap(long, help = "Enable dry run mode")]
  dry_run: bool,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    for directory in self.directories {
      ensure!(
        directory.is_dir(),
        "The path '{}' is not a valid directory.",
        directory.display()
      );

      let context = Context::try_from(directory)?;

      for rule in RULES {
        if !rule.applies(&context) {
          continue;
        }

        for action in rule.actions() {
          let matcher = Glob::new(action.pattern)?.compile_matcher();

          let mut matches: Vec<PathBuf> = context
            .directories
            .iter()
            .filter(|path| matcher.is_match(path))
            .cloned()
            .collect();

          matches.extend(
            context
              .files
              .iter()
              .filter(|path| matcher.is_match(path))
              .cloned(),
          );

          for relative_path in matches {
            let full_path = context.root.join(&relative_path);

            if !full_path.exists() {
              continue;
            }

            if self.dry_run {
              println!(
                "dry-run: [{}:{}] remove '{}' ({})",
                rule.id(),
                rule.name(),
                full_path.display(),
                action.reason
              );

              continue;
            }

            if full_path.is_dir() {
              fs::remove_dir_all(&full_path)?;
            } else {
              fs::remove_file(&full_path)?;
            }

            println!(
              "removed: [{}:{}] '{}' ({})",
              rule.id(),
              rule.name(),
              full_path.display(),
              action.reason
            );
          }
        }
      }
    }

    Ok(())
  }
}
