use super::*;

define_rule! {
  Cmake {
    id: "cmake",
    name: "CMake",
    detection: Detection::Pattern("CMakeLists.txt"),
    actions: [
      Action::Remove("build"),
      Action::Remove("cmake-build-debug"),
      Action::Remove("cmake-build-release"),
    ],
  }
}
