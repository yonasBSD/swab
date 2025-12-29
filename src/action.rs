#[derive(Debug)]
#[allow(unused)]
pub(crate) enum Action {
  Command(&'static str),
  Remove {
    pattern: &'static str,
    reason: &'static str,
  },
}
