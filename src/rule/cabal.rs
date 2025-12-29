use super::*;

define_rule! {
  Cabal {
    id: "cabal",
    name: "Cabal (Haskell)",
    detection: Detection::Pattern("cabal.project"),
    actions: [
      Action::Remove("dist-newstyle"),
    ],
  }
}
