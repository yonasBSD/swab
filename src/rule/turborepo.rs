use super::*;

define_rule! {
  Turborepo {
    id: "turborepo",
    name: "Turborepo",
    actions: [
      Action::Remove(".turbo"),
    ],
    detection: Pattern("turbo.json")
  }
}
