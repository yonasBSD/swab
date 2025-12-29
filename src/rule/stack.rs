use super::*;

define_rule! {
  Stack {
    id: "stack",
    name: "Stack (Haskell)",
    actions: [
      Action::Remove(".stack-work"),
    ],
    detection: Detection::Pattern("stack.yaml")
  }
}
