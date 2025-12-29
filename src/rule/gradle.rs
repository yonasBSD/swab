use super::*;

define_rule! {
  Gradle {
    id: "gradle",
    name: "Gradle",
    actions: [
      Remove("build"),
      Remove(".gradle"),
    ],
    detection: Any(
      Box::new(Pattern("build.gradle")),
      Box::new(Pattern("build.gradle.kts")),
    )
  }
}
