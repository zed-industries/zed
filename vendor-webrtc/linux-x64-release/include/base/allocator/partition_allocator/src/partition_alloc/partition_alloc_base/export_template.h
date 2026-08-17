// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_EXPORT_TEMPLATE_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_EXPORT_TEMPLATE_H_

// Synopsis
//
// This header provides macros for using
// PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) macros with explicit template
// instantiation declarations and definitions. Generally, the
// PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) macros are used at declarations,
// and GCC requires them to be used at explicit instantiation declarations, but
// MSVC requires __declspec(dllexport) to be used at the explicit instantiation
// definitions instead.

// Usage
//
// In a header file, write:
//
//   extern template class
//   PA_EXPORT_TEMPLATE_DECLARE(PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE))
//   foo<bar>;
//
// In a source file, write:
//
//   template class
//   PA_EXPORT_TEMPLATE_DEFINE(PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE))
//   foo<bar>;

// Implementation notes
//
// On Windows, when building when PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
// expands to __declspec(dllexport)), we want the two lines to expand to:
//
//     extern template class foo<bar>;
//     template class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) foo<bar>;
//
// In all other cases (non-Windows, and Windows when
// PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) expands to
// __declspec(dllimport)), we want:
//
//     extern template class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) foo<bar>;
//     template class foo<bar>;
//
// The implementation of this header uses some subtle macro semantics to
// detect what the provided PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) value was
// defined as and then to dispatch to appropriate macro definitions.
// Unfortunately, MSVC's C preprocessor is rather non-compliant and requires
// special care to make it work.
//
// Issue 1.
//
//   #define F(x)
//   F()
//
// MSVC emits warning C4003 ("not enough actual parameters for macro
// 'F'), even though it's a valid macro invocation.  This affects the
// macros below that take just an "export" parameter, because export
// may be empty.
//
// As a workaround, we can add a dummy parameter and arguments:
//
//   #define F(x,_)
//   F(,)
//
// Issue 2.
//
//   #define F(x) G##x
//   #define Gj() ok
//   F(j())
//
// The correct replacement for "F(j())" is "ok", but MSVC replaces it
// with "Gj()".  As a workaround, we can pass the result to an
// identity macro to force MSVC to look for replacements again.  (This
// is why PA_EXPORT_TEMPLATE_STYLE_3 exists.)

#define PA_EXPORT_TEMPLATE_DECLARE(export)                               \
  PA_EXPORT_TEMPLATE_INVOKE(DECLARE, PA_EXPORT_TEMPLATE_STYLE(export, ), \
                            export)  // NOLINT
#define PA_EXPORT_TEMPLATE_DEFINE(export)                               \
  PA_EXPORT_TEMPLATE_INVOKE(DEFINE, PA_EXPORT_TEMPLATE_STYLE(export, ), \
                            export)  // NOLINT

// INVOKE is an internal helper macro to perform parameter replacements
// and token pasting to chain invoke another macro.  E.g.,
//     PA_EXPORT_TEMPLATE_INVOKE(DECLARE, DEFAULT, PA_EXPORT)
// will export to call
//     PA_EXPORT_TEMPLATE_DECLARE_DEFAULT(PA_EXPORT, )
// (but with PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) expanded too).
#define PA_EXPORT_TEMPLATE_INVOKE(which, style, export) \
  PA_EXPORT_TEMPLATE_INVOKE_2(which, style, export)
#define PA_EXPORT_TEMPLATE_INVOKE_2(which, style, export) \
  PA_EXPORT_TEMPLATE_##which##_##style(export, )

// Default style is to apply the PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) macro
// at declaration sites.
#define PA_EXPORT_TEMPLATE_DECLARE_DEFAULT(export, _) export
#define PA_EXPORT_TEMPLATE_DEFINE_DEFAULT(export, _)

// The "MSVC hack" style is used when PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
// is defined as __declspec(dllexport), which MSVC requires to be used at
// definition sites instead.
#define PA_EXPORT_TEMPLATE_DECLARE_EXPORT_DLLEXPORT(export, _)
#define PA_EXPORT_TEMPLATE_DEFINE_EXPORT_DLLEXPORT(export, _) export

// PA_EXPORT_TEMPLATE_STYLE is an internal helper macro that identifies which
// export style needs to be used for the provided
// PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) macro definition.
// "", "__attribute__(...)", and "__declspec(dllimport)" are mapped
// to "DEFAULT"; while "__declspec(dllexport)" is mapped to "MSVC_HACK".
//
// It's implemented with token pasting to transform the __attribute__ and
// __declspec annotations into macro invocations.  E.g., if
// PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) is defined as
// "__declspec(dllimport)", it undergoes the following sequence of macro
// substitutions:
//     PA_EXPORT_TEMPLATE_STYLE(PA_EXPORT,)
//     PA_EXPORT_TEMPLATE_STYLE_2(__declspec(dllimport),)
//     PA_EXPORT_TEMPLATE_STYLE_3(
//         PA_EXPORT_TEMPLATE_STYLE_MATCH__declspec(dllimport))
//     PA_EXPORT_TEMPLATE_STYLE_MATCH__declspec(dllimport)
//     PA_EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_dllimport
//     DEFAULT
#define PA_EXPORT_TEMPLATE_STYLE(export, _) PA_EXPORT_TEMPLATE_STYLE_2(export, )
#define PA_EXPORT_TEMPLATE_STYLE_2(export, _) \
  PA_EXPORT_TEMPLATE_STYLE_3(                 \
      PA_EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA##export)
#define PA_EXPORT_TEMPLATE_STYLE_3(style) style

// Internal helper macros for PA_EXPORT_TEMPLATE_STYLE.
//
// XXX: C++ reserves all identifiers containing "__" for the implementation,
// but "__attribute__" and "__declspec" already contain "__" and the token-paste
// operator can only add characters; not remove them.  To minimize the risk of
// conflict with implementations, we include "foj3FJo5StF0OvIzl7oMxA" (a random
// 128-bit string, encoded in Base64) in the macro name.
#define PA_EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA DEFAULT
#define PA_EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA__attribute__( \
    ...)                                                                    \
  DEFAULT
#define PA_EXPORT_TEMPLATE_STYLE_MATCH_foj3FJo5StF0OvIzl7oMxA__declspec(arg) \
  PA_EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_##arg

// Internal helper macros for PA_EXPORT_TEMPLATE_STYLE.
#define PA_EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_dllexport EXPORT_DLLEXPORT
#define PA_EXPORT_TEMPLATE_STYLE_MATCH_DECLSPEC_dllimport DEFAULT

// Sanity checks.
//
// PA_EXPORT_TEMPLATE_TEST uses the same macro invocation pattern as
// PA_EXPORT_TEMPLATE_DECLARE and PA_EXPORT_TEMPLATE_DEFINE do to check that
// they're working correctly. When they're working correctly, the sequence of
// macro replacements should go something like:
//
//     PA_EXPORT_TEMPLATE_TEST(DEFAULT, __declspec(dllimport));
//
//     static_assert(PA_EXPORT_TEMPLATE_INVOKE(TEST_DEFAULT,
//         PA_EXPORT_TEMPLATE_STYLE(__declspec(dllimport), ),
//         __declspec(dllimport)), "__declspec(dllimport)");
//
//     static_assert(PA_EXPORT_TEMPLATE_INVOKE(TEST_DEFAULT,
//         DEFAULT, __declspec(dllimport)), "__declspec(dllimport)");
//
//     static_assert(PA_EXPORT_TEMPLATE_TEST_DEFAULT_DEFAULT(
//         __declspec(dllimport)), "__declspec(dllimport)");
//
//     static_assert(true, "__declspec(dllimport)");
//
// When they're not working correctly, a syntax error should occur instead.
#define PA_EXPORT_TEMPLATE_TEST(want, export)                                 \
  static_assert(PA_EXPORT_TEMPLATE_INVOKE(                                    \
                    TEST_##want, PA_EXPORT_TEMPLATE_STYLE(export, ), export), \
                #export)  // NOLINT
#define PA_EXPORT_TEMPLATE_TEST_DEFAULT_DEFAULT(...) true
#define PA_EXPORT_TEMPLATE_TEST_EXPORT_DLLEXPORT_EXPORT_DLLEXPORT(...) true

PA_EXPORT_TEMPLATE_TEST(DEFAULT, );  // NOLINT
PA_EXPORT_TEMPLATE_TEST(DEFAULT, __attribute__((visibility("default"))));
PA_EXPORT_TEMPLATE_TEST(EXPORT_DLLEXPORT, __declspec(dllexport));
PA_EXPORT_TEMPLATE_TEST(DEFAULT, __declspec(dllimport));

#undef PA_EXPORT_TEMPLATE_TEST
#undef PA_EXPORT_TEMPLATE_TEST_DEFAULT_DEFAULT
#undef PA_EXPORT_TEMPLATE_TEST_EXPORT_DLLEXPORT_EXPORT_DLLEXPORT

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_EXPORT_TEMPLATE_H_
