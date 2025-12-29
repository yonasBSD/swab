use super::*;

define_rule! {
  Dotnet {
    id: "dotnet",
    name: ".NET",
    actions: [
      Action::Remove("bin"),
      Action::Remove("obj"),
    ],
    detection: Detection::All(
      Box::new(Detection::Any(
        Box::new(Detection::Pattern("**/*.csproj")),
        Box::new(Detection::Pattern("**/*.fsproj")),
      )),
      Box::new(Detection::All(
        Box::new(Detection::Not(Box::new(Detection::Pattern("Assembly-CSharp.csproj")))),
        Box::new(Detection::Not(Box::new(Detection::Pattern("project.godot")))),
      )),
    )
  }
}
