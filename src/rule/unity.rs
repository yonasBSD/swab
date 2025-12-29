use super::*;

define_rule! {
  Unity {
    id: "unity",
    name: "Unity",
    actions: [
      Action::Remove("Library"),
      Action::Remove("Temp"),
      Action::Remove("Obj"),
      Action::Remove("Logs"),
      Action::Remove("MemoryCaptures"),
      Action::Remove("Build"),
      Action::Remove("Builds"),
    ],
    detection: Detection::Pattern("Assembly-CSharp.csproj")
  }
}
