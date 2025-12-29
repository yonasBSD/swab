use super::*;

define_rule! {
  Swift {
    id: "swift",
    name: "Swift",
    actions: [
      Action::Remove(".build"),
      Action::Remove(".swiftpm"),
    ],
    detection: Pattern("Package.swift")
  }
}
