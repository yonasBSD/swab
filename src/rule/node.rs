use super::*;

define_rule! {
  Node {
    id: "node",
    name: "Node",
    actions: [
      Action::Remove("**/node_modules"),
      Action::Remove(".angular"),
    ],
    applies(context) {
      context.contains("package.json")
    }
  }
}
