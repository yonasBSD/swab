use super::*;

#[derive(Debug)]
pub(crate) enum Detection<'a> {
  All(Box<Detection<'a>>, Box<Detection<'a>>),
  Any(Box<Detection<'a>>, Box<Detection<'a>>),
  Not(Box<Detection<'a>>),
  Pattern(&'a str),
}

impl Detection<'_> {
  pub(crate) fn matches(&self, context: &Context) -> bool {
    match self {
      Detection::All(left, right) => {
        left.matches(context) && right.matches(context)
      }
      Detection::Any(left, right) => {
        left.matches(context) || right.matches(context)
      }
      Detection::Not(inner) => !inner.matches(context),
      Detection::Pattern(pattern) => context.contains(pattern),
    }
  }
}
