// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header defines macros to export component's symbols.
// See "platform/platform_export.h" for details.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CORE_EXPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CORE_EXPORT_H_

#include "build/build_config.h"

//
// BLINK_CORE_IMPLEMENTATION
//
#if !defined(BLINK_CORE_IMPLEMENTATION)
#define BLINK_CORE_IMPLEMENTATION 0
#endif

//
// CORE_EXPORT
//
#if !defined(COMPONENT_BUILD)
#define CORE_EXPORT  // No need of export
#else

#if defined(COMPILER_MSVC)
#if BLINK_CORE_IMPLEMENTATION
#define CORE_EXPORT __declspec(dllexport)
#else
#define CORE_EXPORT __declspec(dllimport)
#endif
#endif  // defined(COMPILER_MSVC)

#if defined(COMPILER_GCC)
#if BLINK_CORE_IMPLEMENTATION
#define CORE_EXPORT __attribute__((visibility("default")))
#else
#define CORE_EXPORT
#endif
#endif  // defined(COMPILER_GCC)

#endif  // !defined(COMPONENT_BUILD)

//
// CORE_EXTERN_TEMPLATE_EXPORT
// CORE_TEMPLATE_EXPORT
//
#if BLINK_CORE_IMPLEMENTATION

#if defined(COMPILER_MSVC)
#define CORE_EXTERN_TEMPLATE_EXPORT
#define CORE_TEMPLATE_EXPORT CORE_EXPORT
#endif

#if defined(COMPILER_GCC)
#define CORE_EXTERN_TEMPLATE_EXPORT CORE_EXPORT
#define CORE_TEMPLATE_EXPORT
#endif

#else  // BLINK_CORE_IMPLEMENTATION

#define CORE_EXTERN_TEMPLATE_EXPORT CORE_EXPORT
#define CORE_TEMPLATE_EXPORT

#endif  // BLINK_CORE_IMPLEMENTATION

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CORE_EXPORT_H_
