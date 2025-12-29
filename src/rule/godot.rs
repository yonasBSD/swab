use super::*;

define_rule! {
  Godot {
    id: "godot",
    name: "Godot 4",
    actions: [
      Action::Remove(".godot"),
    ],
    detection: Detection::Pattern("project.godot")
  }
}
