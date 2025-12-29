use super::*;

define_rule! {
  Cargo {
    id: "cargo",
    name: "Cargo",
    detection: Detection::Pattern("Cargo.toml"),
    actions: [
      Action::Remove("**/target"),
    ],
  }
}
