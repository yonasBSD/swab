use super::*;

define_rule! {
  Python {
    id: "python",
    name: "Python",
    detection: Detection::Pattern("pyproject.toml"),
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
  }
}
