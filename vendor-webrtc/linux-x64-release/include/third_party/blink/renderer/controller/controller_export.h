// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_CONTROLLER_EXPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_CONTROLLER_EXPORT_H_

namespace blink {

// This macro is intended to export symbols in Source/controller/ which are
// still private to Blink (for instance, because they are used in unit tests).

#if defined(COMPONENT_BUILD)
#if defined(WIN32)
#if BLINK_CONTROLLER_IMPLEMENTATION
#define CONTROLLER_EXPORT __declspec(dllexport)
#else
#define CONTROLLER_EXPORT __declspec(dllimport)
#endif  // BLINK_CONTROLLER_IMPLEMENTATION
#else   // defined(WIN32)
#define CONTROLLER_EXPORT __attribute__((visibility("default")))
#endif
#else  // defined(COMPONENT_BUILD)
#define CONTROLLER_EXPORT
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_CONTROLLER_EXPORT_H_
