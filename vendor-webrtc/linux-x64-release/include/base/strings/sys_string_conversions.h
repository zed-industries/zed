// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_SYS_STRING_CONVERSIONS_H_
#define BASE_STRINGS_SYS_STRING_CONVERSIONS_H_

// Provides system-dependent string type conversions for cases where it's
// necessary to not use ICU. Generally, you should not need this in Chrome,
// but it is used in some shared code. Dependencies should be minimal.

#include <stdint.h>

#include <string>
#include <string_view>

#include "base/base_export.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_APPLE)
#include <CoreFoundation/CoreFoundation.h>

#include "base/apple/scoped_cftyperef.h"

#ifdef __OBJC__
@class NSString;
#endif
#endif  // BUILDFLAG(IS_APPLE)

namespace base {

// Converts between wide and UTF-8 representations of a string. On error, the
// result is system-dependent.
[[nodiscard]] BASE_EXPORT std::string SysWideToUTF8(const std::wstring& wide);
[[nodiscard]] BASE_EXPORT std::wstring SysUTF8ToWide(std::string_view utf8);

// Converts between wide and the system multi-byte representations of a string.
// DANGER: This will lose information and can change (on Windows, this can
// change between reboots).
[[nodiscard]] BASE_EXPORT std::string SysWideToNativeMB(
    const std::wstring& wide);
[[nodiscard]] BASE_EXPORT std::wstring SysNativeMBToWide(
    std::string_view native_mb);

// Windows-specific ------------------------------------------------------------

#if BUILDFLAG(IS_WIN)

// Converts between 8-bit and wide strings, using the given code page. The
// code page identifier is one accepted by the Windows function
// MultiByteToWideChar().
[[nodiscard]] BASE_EXPORT std::wstring SysMultiByteToWide(std::string_view mb,
                                                          uint32_t code_page);
[[nodiscard]] BASE_EXPORT std::string SysWideToMultiByte(
    const std::wstring& wide,
    uint32_t code_page);

#endif  // BUILDFLAG(IS_WIN)

// Mac-specific ----------------------------------------------------------------

#if BUILDFLAG(IS_APPLE)

// Converts between strings and CFStringRefs/NSStrings.

// Converts a string to a CFStringRef. Returns null on failure.
[[nodiscard]] BASE_EXPORT apple::ScopedCFTypeRef<CFStringRef>
SysUTF8ToCFStringRef(std::string_view utf8);
[[nodiscard]] BASE_EXPORT apple::ScopedCFTypeRef<CFStringRef>
SysUTF16ToCFStringRef(std::u16string_view utf16);

// Converts a CFStringRef to a string. Returns an empty string on failure. It is
// not valid to call these with a null `ref`.
[[nodiscard]] BASE_EXPORT std::string SysCFStringRefToUTF8(CFStringRef ref);
[[nodiscard]] BASE_EXPORT std::u16string SysCFStringRefToUTF16(CFStringRef ref);

#ifdef __OBJC__

// Converts a string to an autoreleased NSString. Returns nil on failure.
[[nodiscard]] BASE_EXPORT NSString* SysUTF8ToNSString(std::string_view utf8);
[[nodiscard]] BASE_EXPORT NSString* SysUTF16ToNSString(
    std::u16string_view utf16);

// Converts an NSString to a string. Returns an empty string on failure or if
// `ref` is nil.
[[nodiscard]] BASE_EXPORT std::string SysNSStringToUTF8(NSString* ref);
[[nodiscard]] BASE_EXPORT std::u16string SysNSStringToUTF16(NSString* ref);

#endif  // __OBJC__

#endif  // BUILDFLAG(IS_APPLE)

}  // namespace base

#endif  // BASE_STRINGS_SYS_STRING_CONVERSIONS_H_
