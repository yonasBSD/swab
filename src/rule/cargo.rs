use super::*;

define_rule! {
  Cargo {
    id: "cargo",
    name: "Cargo",
    actions: [
      Action::Remove {
        pattern: "**/target",
        reason: "Cargo build artifacts",
      },
    ],
    applies(context) {
      context.files.contains(&PathBuf::from("Cargo.toml"))
    }
  }
}
