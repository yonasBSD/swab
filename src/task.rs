use super::*;

#[derive(Debug)]
pub(crate) enum Task {
  Command(&'static str),
  Removal { path: PathBuf, size: u64 },
}

impl Task {
  pub(crate) fn execute(&self, context: &Context) -> Result {
    match self {
      Task::Command(command) => {
        let mut parts = command.split_whitespace();

        let program = parts
          .next()
          .ok_or_else(|| anyhow!("command action cannot be empty"))?;

        let status = process::Command::new(program)
          .args(parts)
          .current_dir(context.root.clone())
          .status()?;

        ensure!(
          status.success(),
          "command `{}` failed in `{}`",
          command,
          context.root.display()
        );

        Ok(())
      }
      Task::Removal { path, .. } => {
        let full_path = context.root.join(path);

        ensure!(
          full_path.exists(),
          "the path `{}` does not exist",
          full_path.display()
        );

        if full_path.is_dir() {
          fs::remove_dir_all(&full_path)?;
        } else {
          fs::remove_file(&full_path)?;
        }

        Ok(())
      }
    }
  }
}
