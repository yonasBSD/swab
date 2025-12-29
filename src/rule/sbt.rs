use super::*;

define_rule! {
  Sbt {
    id: "sbt",
    name: "SBT (Scala)",
    detection: Detection::Pattern("build.sbt"),
    actions: [
      Action::Remove("target"),
      Action::Remove("project/target"),
    ],
  }
}
