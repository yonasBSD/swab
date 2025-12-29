use super::*;

define_rule! {
  Sbt {
    id: "sbt",
    name: "SBT (Scala)",
    actions: [
      Action::Remove("target"),
      Action::Remove("project/target"),
    ],
    detection: Detection::Pattern("build.sbt")
  }
}
