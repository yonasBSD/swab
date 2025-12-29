use super::*;

define_rule! {
  Zig {
    id: "zig",
    name: "Zig",
    actions: [
      Action::Remove("zig-cache"),
      Action::Remove(".zig-cache"),
      Action::Remove("zig-out"),
    ],
    applies(context) {
      context.files.contains(&PathBuf::from("build.zig"))
    }
  }
}
