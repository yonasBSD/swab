use super::*;

pub(crate) struct Rust;

impl Rule for Rust {
  fn id(&self) -> &'static str {
    "rust"
  }

  fn name(&self) -> &'static str {
    "Rust"
  }

  fn applies(&self, context: &Context) -> bool {
    context.files.contains(&PathBuf::from("Cargo.toml"))
  }

  fn actions(&self) -> &[Action] {
    &[Action {
      pattern: "target",
      reason: "Rust build artifacts",
    }]
  }
}
