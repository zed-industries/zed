// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header defines macros to export component's symbols.
// See "platform/PlatformExport.h" for details.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MODULES_EXPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MODULES_EXPORT_H_

#include "build/build_config.h"

//
// BLINK_MODULES_IMPLEMENTATION
//
#if !defined(BLINK_MODULES_IMPLEMENTATION)
#define BLINK_MODULES_IMPLEMENTATION 0
#endif

//
// MODULES_EXPORT
//
#if !defined(COMPONENT_BUILD)
#define MODULES_EXPORT  // No need of export
#else

#if defined(COMPILER_MSVC)
#if BLINK_MODULES_IMPLEMENTATION
#define MODULES_EXPORT __declspec(dllexport)
#else
#define MODULES_EXPORT __declspec(dllimport)
#endif
#endif  // defined(COMPILER_MSVC)

#if defined(COMPILER_GCC)
#if BLINK_MODULES_IMPLEMENTATION
#define MODULES_EXPORT __attribute__((visibility("default")))
#else
#define MODULES_EXPORT
#endif
#endif  // defined(COMPILER_GCC)

#endif  // !defined(COMPONENT_BUILD)

//
// MODULES_EXTERN_TEMPLATE_EXPORT
// MODULES_TEMPLATE_EXPORT
//
#if BLINK_MODULES_IMPLEMENTATION

#if defined(COMPILER_MSVC)
#define MODULES_EXTERN_TEMPLATE_EXPORT
#define MODULES_TEMPLATE_EXPORT MODULES_EXPORT
#endif

#if defined(COMPILER_GCC)
#define MODULES_EXTERN_TEMPLATE_EXPORT MODULES_EXPORT
#define MODULES_TEMPLATE_EXPORT
#endif

#else  // BLINK_MODULES_IMPLEMENTATION

#define MODULES_EXTERN_TEMPLATE_EXPORT MODULES_EXPORT
#define MODULES_TEMPLATE_EXPORT

#endif  // BLINK_MODULES_IMPLEMENTATION

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MODULES_EXPORT_H_
