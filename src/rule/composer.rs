use super::*;

define_rule! {
  Composer {
    id: "composer",
    name: "Composer (PHP)",
    actions: [
      Action::Remove("vendor"),
    ],
    detection: Detection::Pattern("composer.json")
  }
}
