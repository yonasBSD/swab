use super::*;

define_rule! {
  Cargo {
    id: "cargo",
    name: "Cargo",
    actions: [
      Action::Remove("**/target"),
    ],
    applies(context) {
      context.files.contains(&PathBuf::from("Cargo.toml"))
    }
  }
}
