use super::*;

define_rule! {
  Stack {
    id: "stack",
    name: "Stack (Haskell)",
    actions: [
      Action::Remove(".stack-work"),
    ],
    applies(context) {
      context.contains("stack.yaml")
    }
  }
}
