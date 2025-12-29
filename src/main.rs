use {
  action::Action,
  anyhow::{Error, ensure},
  arguments::Arguments,
  clap::Parser,
  context::Context,
  globset::Glob,
  rule::*,
  std::{
    backtrace::BacktraceStatus, collections::HashSet, fs, path::PathBuf,
    process,
  },
  walkdir::WalkDir,
};

mod action;
mod arguments;
mod context;
mod rule;

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");

    for (i, error) in error.chain().skip(1).enumerate() {
      if i == 0 {
        eprintln!();
        eprintln!("because:");
      }

      eprintln!("- {error}");
    }

    let backtrace = error.backtrace();

    if backtrace.status() == BacktraceStatus::Captured {
      eprintln!("backtrace:");
      eprintln!("{backtrace}");
    }

    process::exit(1);
  }
}
