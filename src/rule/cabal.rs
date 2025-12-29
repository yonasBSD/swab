use super::*;

define_rule! {
  Cabal {
    id: "cabal",
    name: "Cabal (Haskell)",
    actions: [
      Action::Remove("dist-newstyle"),
    ],
    detection: Detection::Pattern("cabal.project")
  }
}
