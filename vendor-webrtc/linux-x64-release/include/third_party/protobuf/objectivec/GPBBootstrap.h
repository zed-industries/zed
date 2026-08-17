// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// For int32_t in GPB_ENUM/GPB_ENUM_FWD_DECLARE below.
#import <stdint.h>

/**
 * The Objective C runtime has complete enough info that most protos don’t end
 * up using this, so leaving it on is no cost or very little cost.  If you
 * happen to see it causing bloat, this is the way to disable it. If you do
 * need to disable it, try only disabling it for Release builds as having
 * full TextFormat can be useful for debugging.
 **/
#ifndef GPBOBJC_SKIP_MESSAGE_TEXTFORMAT_EXTRAS
#define GPBOBJC_SKIP_MESSAGE_TEXTFORMAT_EXTRAS 0
#endif

// Used in the generated code to give sizes to enums. int32_t was chosen based
// on the fact that Protocol Buffers enums are limited to this range.
#if !__has_feature(objc_fixed_enum)
#error All supported Xcode versions should support objc_fixed_enum.
#endif

// If the headers are imported into Objective-C++, we can run into an issue
// where the definition of NS_ENUM (really CF_ENUM) changes based on the C++
// standard that is in effect.  If it isn't C++11 or higher, the definition
// doesn't allow us to forward declare. We work around this one case by
// providing a local definition. The default case has to use NS_ENUM for the
// magic that is Swift bridging of enums.
#if (defined(__cplusplus) && __cplusplus && __cplusplus < 201103L)
#define GPB_ENUM(X)   \
  enum X : int32_t X; \
  enum X : int32_t
#else
#define GPB_ENUM(X) NS_ENUM(int32_t, X)
#endif

/**
 * GPB_ENUM_FWD_DECLARE is used for forward declaring enums, for example:
 *
 * ```
 * GPB_ENUM_FWD_DECLARE(Foo_Enum)
 *
 * @interface BarClass : NSObject
 * @property (nonatomic) enum Foo_Enum value;
 * - (void)bazMethod:(enum Foo_Enum)value;
 * @end
 * ```
 **/
#define GPB_ENUM_FWD_DECLARE(X) enum X : int32_t

/**
 * Based upon CF_INLINE. Forces inlining in non DEBUG builds.
 **/
#if !defined(DEBUG)
#define GPB_INLINE static __inline__ __attribute__((always_inline))
#else
#define GPB_INLINE static __inline__
#endif

/**
 * For use in public headers that might need to deal with ARC.
 **/
#ifndef GPB_UNSAFE_UNRETAINED
#if __has_feature(objc_arc)
#define GPB_UNSAFE_UNRETAINED __unsafe_unretained
#else
#define GPB_UNSAFE_UNRETAINED
#endif
#endif

/**
 * Attribute used for Objective-C proto interface deprecations without messages.
 **/
#ifndef GPB_DEPRECATED
#define GPB_DEPRECATED __attribute__((deprecated))
#endif

/**
 * Attribute used for Objective-C proto interface deprecations with messages.
 **/
#ifndef GPB_DEPRECATED_MSG
#if __has_extension(attribute_deprecated_with_message)
#define GPB_DEPRECATED_MSG(msg) __attribute__((deprecated(msg)))
#else
#define GPB_DEPRECATED_MSG(msg) __attribute__((deprecated))
#endif
#endif

// If property name starts with init we need to annotate it to get past ARC.
// http://stackoverflow.com/questions/18723226/how-do-i-annotate-an-objective-c-property-with-an-objc-method-family/18723227#18723227
//
// Meant to be used internally by generated code.
#define GPB_METHOD_FAMILY_NONE __attribute__((objc_method_family(none)))

// Prevent subclassing of generated proto classes.
#ifndef GPB_FINAL
#define GPB_FINAL __attribute__((objc_subclassing_restricted))
#endif  // GPB_FINAL

// ----------------------------------------------------------------------------
// These version numbers are all internal to the ObjC Protobuf runtime; they
// are used to ensure compatibility between the generated sources and the
// headers being compiled against and/or the version of sources being run
// against.
//
// They are all #defines so the values are captured into every .o file they
// are used in and to allow comparisons in the preprocessor.

// Current library runtime version.
// - Gets bumped when the runtime makes changes to the interfaces between the
//   generated code and runtime (things added/removed, etc).
#define GOOGLE_PROTOBUF_OBJC_VERSION 30007

// Minimum runtime version supported for compiling/running against.
// - Gets changed when support for the older generated code is dropped.
#define GOOGLE_PROTOBUF_OBJC_MIN_SUPPORTED_VERSION 30007
