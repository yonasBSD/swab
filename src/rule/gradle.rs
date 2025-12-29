use super::*;

define_rule! {
  Gradle {
    id: "gradle",
    name: "Gradle",
    actions: [
      Action::Remove("build"),
      Action::Remove(".gradle"),
    ],
    detection: Detection::Any(
      Box::new(Detection::Pattern("build.gradle")),
      Box::new(Detection::Pattern("build.gradle.kts")),
    )
  }
}
