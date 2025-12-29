use super::*;

define_rule! {
  Python {
    id: "python",
    name: "Python",
    actions: [
      Action::Remove(".mypy_cache"),
      Action::Remove(".nox"),
      Action::Remove(".pytest_cache"),
      Action::Remove(".ruff_cache"),
      Action::Remove(".tox"),
      Action::Remove(".venv"),
      Action::Remove("__pycache__"),
      Action::Remove("__pypackages__"),
    ],
    detection: Pattern("pyproject.toml")
  }
}
