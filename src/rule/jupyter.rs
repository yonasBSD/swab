use super::*;

define_rule! {
  Jupyter {
    id: "jupyter",
    name: "Jupyter",
    actions: [
      Action::Remove("**/.ipynb_checkpoints"),
    ],
    applies(context) {
      context.has_file_with_extension("ipynb")
    }
  }
}
