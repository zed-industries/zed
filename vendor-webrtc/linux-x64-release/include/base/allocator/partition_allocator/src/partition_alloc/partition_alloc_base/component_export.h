// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_COMPONENT_EXPORT_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_COMPONENT_EXPORT_H_

// Used to annotate symbols which are exported by the component named
// |component|. Note that this only does the right thing if the corresponding
// component target's sources are compiled with |IS_$component_IMPL| defined
// as 1. For example:
//
//   class PA_COMPONENT_EXPORT(FOO) Bar {};
//
// If IS_FOO_IMPL=1 at compile time, then Bar will be annotated using the
// PA_COMPONENT_EXPORT_ANNOTATION macro defined below. Otherwise it will be
// annotated using the PA_COMPONENT_IMPORT_ANNOTATION macro.
#define PA_COMPONENT_EXPORT(component)                            \
  PA_COMPONENT_MACRO_CONDITIONAL_(IS_##component##_IMPL,          \
                                  PA_COMPONENT_EXPORT_ANNOTATION, \
                                  PA_COMPONENT_IMPORT_ANNOTATION)

// Indicates whether the current compilation unit is being compiled as part of
// the implementation of the component named |component|. Expands to |1| if
// |IS_$component_IMPL| is defined as |1|; expands to |0| otherwise.
//
// Note in particular that if |IS_$component_IMPL| is not defined at all, it is
// still fine to test PA_INSIDE_COMPONENT_IMPL(component), which expands to |0|
// as expected.
#define PA_INSIDE_COMPONENT_IMPL(component) \
  PA_COMPONENT_MACRO_CONDITIONAL_(IS_##component##_IMPL, 1, 0)

// Compiler-specific macros to annotate for export or import of a symbol. No-op
// in non-component builds. These should not see much if any direct use.
// Instead use the PA_COMPONENT_EXPORT macro defined above.
#if defined(COMPONENT_BUILD)
#if defined(WIN32)
#define PA_COMPONENT_EXPORT_ANNOTATION __declspec(dllexport)
#define PA_COMPONENT_IMPORT_ANNOTATION __declspec(dllimport)
#else  // defined(WIN32)
#define PA_COMPONENT_EXPORT_ANNOTATION __attribute__((visibility("default")))
#define PA_COMPONENT_IMPORT_ANNOTATION
#endif  // defined(WIN32)
#else   // defined(COMPONENT_BUILD)
#define PA_COMPONENT_EXPORT_ANNOTATION
#define PA_COMPONENT_IMPORT_ANNOTATION
#endif  // defined(COMPONENT_BUILD)

// Below this point are several internal utility macros used for the
// implementation of the above macros. Not intended for external use.

// Helper for conditional expansion to one of two token strings. If |condition|
// expands to |1| then this macro expands to |consequent|; otherwise it expands
// to |alternate|.
#define PA_COMPONENT_MACRO_CONDITIONAL_(condition, consequent, alternate) \
  PA_COMPONENT_MACRO_SELECT_THIRD_ARGUMENT_(                              \
      PA_COMPONENT_MACRO_CONDITIONAL_COMMA_(condition), consequent, alternate)

// MSVC workaround for __VA_ARGS__ expanding into one expression.
#define PA_MSVC_EXPAND_ARG(arg) arg

// Expands to a comma (,) iff its first argument expands to |1|. Used in
// conjunction with |PA_COMPONENT_MACRO_SELECT_THIRD_ARGUMENT_()|, as the
// presence or absense of an extra comma can be used to conditionally shift
// subsequent argument positions and thus influence which argument is selected.
#define PA_COMPONENT_MACRO_CONDITIONAL_COMMA_(...) \
  PA_COMPONENT_MACRO_CONDITIONAL_COMMA_IMPL_(__VA_ARGS__, )
#define PA_COMPONENT_MACRO_CONDITIONAL_COMMA_IMPL_(x, ...) \
  PA_COMPONENT_MACRO_CONDITIONAL_COMMA_##x##_
#define PA_COMPONENT_MACRO_CONDITIONAL_COMMA_1_ ,

// Helper which simply selects its third argument. Used in conjunction with
// |PA_COMPONENT_MACRO_CONDITIONAL_COMMA_()| above to implement conditional
// macro expansion.
#define PA_COMPONENT_MACRO_SELECT_THIRD_ARGUMENT_(...) \
  PA_MSVC_EXPAND_ARG(                                  \
      PA_COMPONENT_MACRO_SELECT_THIRD_ARGUMENT_IMPL_(__VA_ARGS__))
#define PA_COMPONENT_MACRO_SELECT_THIRD_ARGUMENT_IMPL_(a, b, c, ...) c

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_COMPONENT_EXPORT_H_
