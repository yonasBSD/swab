#[derive(Debug)]
pub(crate) enum Action {
  #[allow(unused)]
  Command(&'static str),
  Remove(&'static str),
}
