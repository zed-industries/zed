// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SHELL_ENCRYPTION_BASE_SHELL_ENCRYPTION_EXPORT_TEMPLATE_H_
#define SHELL_ENCRYPTION_BASE_SHELL_ENCRYPTION_EXPORT_TEMPLATE_H_

// This was borrowed from Chromium's base/export_template.h.
//
// TODO(crbug.com/1271458): Use shell-encryption template macro's name
// instead of generic Chromium macro names.

// Synopsis
//
// This header provides macros for using FOO_EXPORT macros with explicit
// template instantiation declarations and definitions.
// Generally, the FOO_EXPORT macros are used at declarations,
// and GCC requires them to be used at explicit instantiation declarations,
// but MSVC requires __declspec(dllexport) to be used at the explicit
// instantiation definitions instead.

// Usage
//
// In a header file, write:
//
//   extern template class EXPORT_TEMPLATE_DECLARE(FOO_EXPORT) foo<bar>;
//
// In a source file, write:
//
//   template class EXPORT_TEMPLATE_DEFINE(FOO_EXPORT) foo<bar>;

// Implementation notes
//
// On Windows, when building the FOO library (that is, when FOO_EXPORT expands
// to __declspec(dllexport)), we want the two lines to expand to:
//
//     extern template class foo<bar>;
//     template class FOO_EXPORT foo<bar>;
//
// In all other cases (non-Windows, and Windows when using the FOO library (that
// is when FOO_EXPORT expands to __declspec(dllimport)), we want:
//
//     extern template class FOO_EXPORT foo<bar>;
//     template class foo<bar>;
//
// The implementation of this header uses some subtle macro semantics to
// detect what the provided FOO_EXPORT value was defined as and then
// to dispatch to appropriate macro definitions.

#define EXPORT_TEMPLATE_DECLARE(foo_export) \
  EXPORT_TEMPLATE_INVOKE(DECLARE, EXPORT_TEMPLATE_STYLE(foo_export), foo_export)
#define EXPORT_TEMPLATE_DEFINE(foo_export) \
  EXPORT_TEMPLATE_INVOKE(DEFINE, EXPORT_TEMPLATE_STYLE(foo_export), foo_export)

// INVOKE is an internal helper macro to perform parameter replacements
// and token pasting to chain invoke another macro.  E.g.,
//     EXPORT_TEMPLATE_INVOKE(DECLARE, DEFAULT, FOO_EXPORT)
// will expand to call
//     EXPORT_TEMPLATE_DECLARE_DEFAULT(FOO_EXPORT)
// (but with FOO_EXPORT expanded too).
#define EXPORT_TEMPLATE_INVOKE(which, style, foo_export) \
  EXPORT_TEMPLATE_INVOKE_2(which, style, foo_export)
#define EXPORT_TEMPLATE_INVOKE_2(which, style, foo_export) \
  EXPORT_TEMPLATE_##which##_##style(foo_export)

// Default style is to apply the FOO_EXPORT macro at declaration sites.
#define EXPORT_TEMPLATE_DECLARE_DEFAULT(foo_export) foo_export
#define EXPORT_TEMPLATE_DEFINE_DEFAULT(foo_export)

// The "declspec" style is used when FOO_EXPORT is defined
// as __declspec(dllexport), which MSVC requires to be used at
// definition sites instead.
#define EXPORT_TEMPLATE_DECLARE_EXPORT_DLLEXPORT(foo_export)
#define EXPORT_TEMPLATE_DEFINE_EXPORT_DLLEXPORT(foo_export) foo_export

// EXPORT_TEMPLATE_STYLE is an internal helper macro that identifies which
// export style needs to be used for the provided FOO_EXPORT macro definition.
// "", "__attribute__(...)", and "__declspec(dllimport)" are mapped
// to "DEFAULT"; while "__declspec(dllexport)" is mapped to "EXPORT_DLLEXPORT".
// (NaCl headers define "DLLEXPORT" already, else we'd use that.
// TODO(thakis): Rename once nacl is gone.)
//
// It's implemented with token pasting to transform the __attribute__ and
// __declspec annotations into macro invocations.  E.g., if FOO_EXPORT is
// defined as "__declspec(dllimport)", it undergoes the following sequence of
// macro substitutions:
//     EXPORT_TEMPLATE_STYLE(FOO_EXPORT)
//     EXPORT_TEMPLATE_STYLE_2(__declspec(dllimport))
//     EXPORT_TEMPLATE_STYLE_MATCH__declspec(dllimport)
//     EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_dllimport
//     DEFAULT
#define EXPORT_TEMPLATE_STYLE(foo_export) EXPORT_TEMPLATE_STYLE_2(foo_export)
#define EXPORT_TEMPLATE_STYLE_2(foo_export) \
  EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA##foo_export

// Internal helper macros for EXPORT_TEMPLATE_STYLE.
//
// XXX: C++ reserves all identifiers containing "__" for the implementation,
// but "__attribute__" and "__declspec" already contain "__" and the token-paste
// operator can only add characters; not remove them.  To minimize the risk of
// conflict with implementations, we include "foj3FJo5StF0OvIzl7oMxA" (a random
// 128-bit string, encoded in Base64) in the macro name.
#define EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA DEFAULT
#define EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA__attribute__(...) \
  DEFAULT
#define EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA__declspec(arg) \
  EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_##arg

// Internal helper macros for EXPORT_TEMPLATE_STYLE.
#define EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_dllexport EXPORT_DLLEXPORT
#define EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_dllimport DEFAULT

// Sanity checks.
//
// EXPORT_TEMPLATE_TEST uses the same macro invocation pattern as
// EXPORT_TEMPLATE_DECLARE and EXPORT_TEMPLATE_DEFINE do to check that they're
// working correctly.  When they're working correctly, the sequence of macro
// replacements should go something like:
//
//     EXPORT_TEMPLATE_TEST(DEFAULT, __declspec(dllimport));
//
//     static_assert(EXPORT_TEMPLATE_INVOKE(TEST_DEFAULT,
//         EXPORT_TEMPLATE_STYLE(__declspec(dllimport)),
//         __declspec(dllimport)), "__declspec(dllimport)");
//
//     static_assert(EXPORT_TEMPLATE_INVOKE(TEST_DEFAULT,
//         DEFAULT, __declspec(dllimport)), "__declspec(dllimport)");
//
//     static_assert(EXPORT_TEMPLATE_TEST_DEFAULT_DEFAULT(
//         __declspec(dllimport)), "__declspec(dllimport)");
//
//     static_assert(true, "__declspec(dllimport)");
//
// When they're not working correctly, a syntax error should occur instead.
#define EXPORT_TEMPLATE_TEST(want, foo_export)                               \
  static_assert(                                                             \
      EXPORT_TEMPLATE_INVOKE(TEST_##want, EXPORT_TEMPLATE_STYLE(foo_export), \
                             foo_export),                                    \
      #foo_export)
#define EXPORT_TEMPLATE_TEST_DEFAULT_DEFAULT(...) true
#define EXPORT_TEMPLATE_TEST_EXPORT_DLLEXPORT_EXPORT_DLLEXPORT(...) true

EXPORT_TEMPLATE_TEST(DEFAULT, );
EXPORT_TEMPLATE_TEST(DEFAULT, __attribute__((visibility("default"))));
EXPORT_TEMPLATE_TEST(EXPORT_DLLEXPORT, __declspec(dllexport));
EXPORT_TEMPLATE_TEST(DEFAULT, __declspec(dllimport));

#undef EXPORT_TEMPLATE_TEST
#undef EXPORT_TEMPLATE_TEST_DEFAULT_DEFAULT
#undef EXPORT_TEMPLATE_TEST_EXPORT_DLLEXPORT_EXPORT_DLLEXPORT

#endif  // SHELL_ENCRYPTION_BASE_SHELL_ENCRYPTION_EXPORT_TEMPLATE_H_
