// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_COMMON_EXPORT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_COMMON_EXPORT_H_

// This file is NOT to be included directly by code outside blink. It
// is an implementation detail of component builds.

#if defined(COMPONENT_BUILD)
#if defined(WIN32)

#if defined(BLINK_COMMON_IMPLEMENTATION)
#define BLINK_COMMON_EXPORT __declspec(dllexport)
#define BLINK_COMMON_EXPORT_PRIVATE __declspec(dllexport)
#else
#define BLINK_COMMON_EXPORT __declspec(dllimport)
#define BLINK_COMMON_EXPORT_PRIVATE __declspec(dllimport)
#endif  // defined(BLINK_COMMON_IMPLEMENTATION)

#else  // defined(WIN32)
#if defined(BLINK_COMMON_IMPLEMENTATION)
#define BLINK_COMMON_EXPORT __attribute__((visibility("default")))
#define BLINK_COMMON_EXPORT_PRIVATE __attribute__((visibility("default")))
#else
#define BLINK_COMMON_EXPORT
#define BLINK_COMMON_EXPORT_PRIVATE
#endif
#endif

#else  /// defined(COMPONENT_BUILD)
#define BLINK_COMMON_EXPORT
#define BLINK_COMMON_EXPORT_PRIVATE
#endif

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_COMMON_EXPORT_H_
