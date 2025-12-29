use super::*;

define_rule! {
  Maven {
    id: "maven",
    name: "Maven",
    actions: [
      Action::Remove("target"),
    ],
    detection: Pattern("pom.xml")
  }
}
