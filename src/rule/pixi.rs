use super::*;

define_rule! {
  Pixi {
    id: "pixi",
    name: "Pixi",
    actions: [
      Action::Remove(".pixi"),
    ],
    applies(context) {
      context.contains("pixi.toml")
    }
  }
}
