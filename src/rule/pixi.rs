use super::*;

define_rule! {
  Pixi {
    id: "pixi",
    name: "Pixi",
    actions: [
      Action::Remove(".pixi"),
    ],
    detection: Pattern("pixi.toml")
  }
}
