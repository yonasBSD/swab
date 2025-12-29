use super::*;

define_rule! {
  Turborepo {
    id: "turborepo",
    name: "Turborepo",
    detection: Detection::Pattern("turbo.json"),
    actions: [
      Action::Remove(".turbo"),
    ],
  }
}
