use super::*;

define_rule! {
  Cabal {
    id: "cabal",
    name: "Cabal (Haskell)",
    actions: [
      Remove("dist-newstyle"),
    ],
    detection: Pattern("cabal.project")
  }
}
