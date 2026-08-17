/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_BASE_RTCMACROS_H_
#define SDK_OBJC_BASE_RTCMACROS_H_

#ifdef WEBRTC_ENABLE_OBJC_SYMBOL_EXPORT

#if defined(WEBRTC_LIBRARY_IMPL)
#define RTC_OBJC_EXPORT __attribute__((visibility("default")))
#endif

#endif  // WEBRTC_ENABLE_OBJC_SYMBOL_EXPORT

#ifndef RTC_OBJC_EXPORT
#define RTC_OBJC_EXPORT
#endif

// Macro used to mark a function as deprecated.
#define RTC_OBJC_DEPRECATED(msg) __attribute__((deprecated(msg)))

// Internal macros used to correctly concatenate symbols.
#define RTC_SYMBOL_CONCAT_HELPER(a, b) a##b
#define RTC_SYMBOL_CONCAT(a, b) RTC_SYMBOL_CONCAT_HELPER(a, b)

// RTC_OBJC_TYPE_PREFIX
//
// Macro used to prepend a prefix to the API types that are exported with
// RTC_OBJC_EXPORT.
//
// Clients can patch the definition of this macro locally and build
// WebRTC.framework with their own prefix in case symbol clashing is a
// problem.
//
// This macro must be defined uniformily across all the translation units.
#ifndef RTC_OBJC_TYPE_PREFIX
#define RTC_OBJC_TYPE_PREFIX
#endif

#ifndef RTC_CONSTANT_TYPE_PREFIX
#define RTC_CONSTANT_TYPE_PREFIX k
#endif

// RCT_OBJC_TYPE
//
// Macro used internally to declare API types. Declaring an API type without
// using this macro will not include the declared type in the set of types
// that will be affected by the configurable RTC_OBJC_TYPE_PREFIX.
#define RTC_OBJC_TYPE(type_name) \
  RTC_SYMBOL_CONCAT(RTC_OBJC_TYPE_PREFIX, type_name)

#define RTC_CONSTANT_TYPE(type_name) RTC_SYMBOL_CONCAT(RTC_CONSTANT_TYPE_PREFIX, type_name)

#if defined(__cplusplus)
#define RTC_EXTERN extern "C" RTC_OBJC_EXPORT
#else
#define RTC_EXTERN extern RTC_OBJC_EXPORT
#endif

#ifdef __OBJC__
#define RTC_FWD_DECL_OBJC_CLASS(classname) @class classname
#else
#define RTC_FWD_DECL_OBJC_CLASS(classname) typedef struct objc_object classname
#endif

#endif  // SDK_OBJC_BASE_RTCMACROS_H_
