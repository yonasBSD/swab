use super::*;

define_rule! {
  Godot {
    id: "godot",
    name: "Godot 4",
    detection: Detection::Pattern("project.godot"),
    actions: [
      Action::Remove(".godot"),
    ],
  }
}
