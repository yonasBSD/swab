use super::*;

define_rule! {
  Turborepo {
    id: "turborepo",
    name: "Turborepo",
    actions: [
      Action::Remove(".turbo"),
    ],
    detection: Detection::Pattern("turbo.json")
  }
}
