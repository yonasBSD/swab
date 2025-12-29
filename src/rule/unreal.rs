use super::*;

define_rule! {
  Unreal {
    id: "unreal",
    name: "Unreal Engine",
    actions: [
      Action::Remove("Binaries"),
      Action::Remove("Build"),
      Action::Remove("Saved"),
      Action::Remove("DerivedDataCache"),
      Action::Remove("Intermediate"),
    ],
    applies(context) {
      context.has_file_with_extension("uproject")
    }
  }
}
