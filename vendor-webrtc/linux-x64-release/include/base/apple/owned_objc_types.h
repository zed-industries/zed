// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file intentionally does not have header guards; it's included inside
// macros to generate classes. The following lines silence a presubmit and
// Tricium warning that would otherwise be triggered by this:
//
// no-include-guard-because-multiply-included
// NOLINT(build/header_guard)

#if !defined(BASE_APPLE_OWNED_OBJC_H_)
#error Please #include "base/apple/owned_objc.h" instead of this file
#endif

#if BUILDFLAG(IS_MAC)
GENERATE_STRONG_OBJC_PROTOCOL(NSAccessibility)
GENERATE_STRONG_OBJC_TYPE(NSCursor)
GENERATE_STRONG_OBJC_TYPE(NSEvent)
#elif BUILDFLAG(IS_IOS)
// UIAccessibility is an informal protocol on NSObject, so create an owning type
// for NSObject specifically for use in accessibility. Do not use this type for
// general NSObject containment purposes; see
// https://chromium.googlesource.com/chromium/src/+/main/docs/mac/mixing_cpp_and_objc.md
// for advice on how to mix C++ and Objective-C in the Chromium project.
GENERATE_STRONG_OBJC_TYPE(NSObject)
GENERATE_STRONG_OBJC_TYPE(UIEvent)
#if BUILDFLAG(USE_BLINK)
GENERATE_STRONG_OBJC_TYPE(BEKeyEntry)
#endif  // BUILDFLAG(USE_BLINK)
#endif

#if BUILDFLAG(IS_MAC)
GENERATE_WEAK_OBJC_TYPE(NSView)
GENERATE_WEAK_OBJC_TYPE(NSWindow)
#elif BUILDFLAG(IS_IOS)
GENERATE_WEAK_OBJC_TYPE(UIView)
GENERATE_WEAK_OBJC_TYPE(UIWindow)
#endif
