use super::*;

define_rule! {
  Composer {
    id: "composer",
    name: "Composer (PHP)",
    detection: Detection::Pattern("composer.json"),
    actions: [
      Action::Remove("vendor"),
    ],
  }
}
