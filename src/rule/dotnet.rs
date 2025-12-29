use super::*;

define_rule! {
  Dotnet {
    id: "dotnet",
    name: ".NET",
    actions: [
      Action::Remove("bin"),
      Action::Remove("obj"),
    ],
    detection: All(
      Box::new(Any(
        Box::new(Pattern("**/*.csproj")),
        Box::new(Pattern("**/*.fsproj")),
      )),
      Box::new(All(
        Box::new(Not(Box::new(Pattern("Assembly-CSharp.csproj")))),
        Box::new(Not(Box::new(Pattern("project.godot")))),
      )),
    )
  }
}
