use super::*;

define_rule! {
  Cargo {
    id: "cargo",
    name: "Cargo",
    actions: [
      Action::Remove("**/target"),
    ],
    detection: Detection::Pattern("Cargo.toml")
  }
}
