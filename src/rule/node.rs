use super::*;

define_rule! {
  Node {
    id: "node",
    name: "Node",
    actions: [
      Action::Remove {
        pattern: "node_modules",
        reason: "Node dependencies",
      },
      Action::Remove {
        pattern: ".angular",
        reason: "Angular cache",
      },
    ],
    applies(context) {
      context.files.contains(&PathBuf::from("package.json"))
    }
  }
}
