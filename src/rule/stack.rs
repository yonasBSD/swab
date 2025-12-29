use super::*;

define_rule! {
  Stack {
    id: "stack",
    name: "Stack (Haskell)",
    detection: Detection::Pattern("stack.yaml"),
    actions: [
      Action::Remove(".stack-work"),
    ],
  }
}
