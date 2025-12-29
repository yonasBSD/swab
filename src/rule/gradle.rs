use super::*;

define_rule! {
  Gradle {
    id: "gradle",
    name: "Gradle",
    detection: Detection::Any(
      Box::new(Detection::Pattern("build.gradle")),
      Box::new(Detection::Pattern("build.gradle.kts")),
    ),
    actions: [
      Action::Remove("build"),
      Action::Remove(".gradle"),
    ],
  }
}
