use super::*;

#[macro_export]
macro_rules! define_rule {
  (
    $(#[$doc:meta])*
    $name:ident {
      id: $id:literal,
      name: $rule_name:literal,
      actions: [$($action:expr),* $(,)?],
      applies($context:ident) $body:block $(,)?
    }
  ) => {
    $(#[$doc])*
    pub(crate) struct $name;

    impl $crate::rule::Rule for $name {
      fn actions(&self) -> &[$crate::action::Action] {
        &[$($action),*]
      }

      fn applies(&self, $context: &$crate::context::Context) -> bool {
        $body
      }

      fn id(&self) -> &'static str {
        $id
      }

      fn name(&self) -> &'static str {
        $rule_name
      }
    }
  };
}

pub(crate) use {cargo::Cargo, node::Node, zig::Zig};

mod cargo;
mod node;
mod zig;

pub(crate) trait Rule: Sync {
  /// A description of what the rule does.
  fn actions(&self) -> &[Action];

  /// Determines if the rule applies to the given context.
  fn applies(&self, context: &Context) -> bool;

  /// A unique identifier for the rule.
  #[allow(unused)]
  fn id(&self) -> &'static str;

  /// A human-readable name for the rule.
  fn name(&self) -> &'static str;
}
