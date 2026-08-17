{{#title CMake — Rust ♡ C++}}
# CMake

There is not an officially endorsed CMake setup for CXX, but a few developers
have shared one that they got working. You can try one of these as a starting
point. If you feel that you have arrived at a CMake setup that is superior to
what is available in these links, feel free to make a PR adding it to this list.

<br>

---

- **<https://github.com/XiangpengHao/cxx-cmake-example>**

  - Supports cross-language link time optimization (LTO)

---

- **<https://github.com/david-cattermole/cxx-demo-example>**

  - Includes a cbindgen component
  - Tested on Windows 10 with MSVC, and on Linux

---

- **<https://github.com/trondhe/rusty_cmake>**

  - Alias target that can be linked into a C++ project
  - Tested on Windows 10 with GNU target, and on Linux

---

- **<https://github.com/geekbrother/cxx-corrosion-cmake>**

  - Improved rusty_cmake CMake file to use modern C++
  - Rich examples of using different primitive types and Rust's Result return to C++
  - MacOS and Linux only

---

- **<https://github.com/paandahl/cpp-with-rust>**

  - Same blobstore example as the official demo, but inverted languages
  - Minimal CMake configuration
  - Tested on Linux, macOS, and Windows

---
