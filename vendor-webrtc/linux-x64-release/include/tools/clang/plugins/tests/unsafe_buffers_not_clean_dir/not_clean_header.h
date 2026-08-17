// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_NOT_CLEAN_DIR_NOT_CLEAN_HEADER_H_
#define TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_NOT_CLEAN_DIR_NOT_CLEAN_HEADER_H_

inline int unchecked_bad_stuff(int* i, unsigned s) {
  return i[s]     // This is in a known-bad header, so no error is emitted.
         + i[s];  // The second one uses caching and still no error.
}

// This macro is used outside of a function, and thus causes clang to generate a
// warning inside a `<scratch buffer>` file for the macro. We test that we
// enable/disable warnings correctly, the scratch buffer itself is not blamed.
#define INSIDE_MACRO_UNCHECKED(CLASS, FIELD, TYPE) \
  inline TYPE CLASS::FIELD(int index) const {      \
    return FIELD##s_ + index;                      \
  }

struct FooUnchecked {
  int* ptr(int index) const;
  int* ptrs_;
};

// In a known-bad file, the macro use will not error.
INSIDE_MACRO_UNCHECKED(FooUnchecked, ptr, int*);

// Macro with unsafe buffers errors in it. We'll use it from places that are
// checked and places that are not. The decision to fire the warning should
// come from where it's used.
#define DO_UNSAFE_THING_FROM_UNCHECKED_HEADER(n, v, i, s)               \
  int UncheckStructThingTryToMakeScratchBufferFunction##n(int* v,       \
                                                          unsigned s) { \
    return v[s];                                                        \
  }                                                                     \
  struct UncheckStructThingTryToMakeScratchBuffer##n {                  \
    UncheckStructThingTryToMakeScratchBuffer##n() {                     \
      int i = 0;                                                        \
      for (unsigned s; s < 100u; ++s) {                                 \
        UncheckStructThingTryToMakeScratchBufferFunction##n(&i, s);     \
      }                                                                 \
    }                                                                   \
  };

#define MACRO_CALL_FUNCTION_FROM_UNCHECKED_HEADER(x) x()

#endif  // TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_NOT_CLEAN_DIR_NOT_CLEAN_HEADER_H_
