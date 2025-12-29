use super::*;

define_rule! {
  Cargo {
    id: "cargo",
    name: "Cargo",
    actions: [
      Action::Remove("**/target"),
    ],
    detection: Pattern("Cargo.toml")
  }
}
