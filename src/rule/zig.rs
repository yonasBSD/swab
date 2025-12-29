use super::*;

define_rule! {
  Zig {
    id: "zig",
    name: "Zig",
    actions: [
      Action::Remove {
        pattern: "zig-cache",
        reason: "Zig cache",
      },
      Action::Remove {
        pattern: ".zig-cache",
        reason: "Zig cache",
      },
      Action::Remove {
        pattern: "zig-out",
        reason: "Zig build output",
      },
    ],
    applies(context) {
      context.files.contains(&PathBuf::from("build.zig"))
    }
  }
}
