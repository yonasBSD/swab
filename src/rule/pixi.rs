use super::*;

define_rule! {
  Pixi {
    id: "pixi",
    name: "Pixi",
    detection: Detection::Pattern("pixi.toml"),
    actions: [
      Action::Remove(".pixi"),
    ],
  }
}
