use super::*;

pub(crate) struct Node;

impl Rule for Node {
  fn id(&self) -> &'static str {
    "node"
  }

  fn name(&self) -> &'static str {
    "Node"
  }

  fn applies(&self, context: &Context) -> bool {
    context.files.contains(&PathBuf::from("package.json"))
  }

  fn actions(&self) -> &[Action] {
    &[
      Action::Remove {
        pattern: "node_modules",
        reason: "Node dependencies",
      },
      Action::Remove {
        pattern: ".angular",
        reason: "Angular cache",
      },
    ]
  }
}
