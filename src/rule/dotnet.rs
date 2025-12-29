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
      (context.has_file_with_extension("csproj")
        || context.has_file_with_extension("fsproj"))
        && !context.files.contains(&PathBuf::from("Assembly-CSharp.csproj"))
        && !context.files.contains(&PathBuf::from("project.godot"))
    }
  }
}
