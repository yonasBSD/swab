#[derive(Debug)]
#[allow(unused)]
pub(crate) enum Action {
  Command(&'static str),
  Remove(&'static str),
}
