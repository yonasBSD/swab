use super::*;

define_rule! {
  Gradle {
    id: "gradle",
    name: "Gradle",
    actions: [
      Action::Remove("build"),
      Action::Remove(".gradle"),
    ],
    applies(context) {
      context.contains("build.gradle")
        || context.contains("build.gradle.kts")
    }
  }
}
