use super::*;

define_rule! {
  Jupyter {
    id: "jupyter",
    name: "Jupyter",
    actions: [
      Action::Remove("**/.ipynb_checkpoints"),
    ],
    detection: Pattern("**/*.ipynb")
  }
}
