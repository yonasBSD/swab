use super::*;

define_rule! {
  Zig {
    id: "zig",
    name: "Zig",
    detection: Detection::Pattern("build.zig"),
    actions: [
      Action::Remove("zig-cache"),
      Action::Remove(".zig-cache"),
      Action::Remove("zig-out"),
    ],
  }
}
