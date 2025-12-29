use super::*;

define_rule! {
  Pub {
    id: "pub",
    name: "Pub (Dart/Flutter)",
    detection: Detection::Pattern("pubspec.yaml"),
    actions: [
      Action::Remove("build"),
      Action::Remove(".dart_tool"),
      Action::Remove("linux/flutter/ephemeral"),
      Action::Remove("windows/flutter/ephemeral"),
    ],
  }
}
