use super::*;

define_rule! {
  Elixir {
    id: "elixir",
    name: "Elixir",
    actions: [
      Action::Remove("_build"),
      Action::Remove(".elixir-tools"),
      Action::Remove(".elixir_ls"),
      Action::Remove(".lexical"),
    ],
    detection: Detection::Pattern("mix.exs")
  }
}
