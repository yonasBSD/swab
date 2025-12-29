use super::*;

define_rule! {
  Turborepo {
    id: "turborepo",
    name: "Turborepo",
    actions: [
      Action::Remove(".turbo"),
    ],
    applies(context) {
      context.contains("turbo.json")
    }
  }
}
