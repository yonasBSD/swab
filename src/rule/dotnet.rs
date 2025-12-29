use super::*;

define_rule! {
  Dotnet {
    id: "dotnet",
    name: ".NET",
    actions: [
      Action::Remove("bin"),
      Action::Remove("obj"),
    ],
    applies(context) {
      (context.contains("**/*.csproj")
        || context.contains("**/*.fsproj"))
        && !context.contains("Assembly-CSharp.csproj")
        && !context.contains("project.godot")
    }
  }
}
