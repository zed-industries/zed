// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_VERSION_INFO_VERSION_INFO_H_
#define BASE_VERSION_INFO_VERSION_INFO_H_

#include <string>
#include <string_view>

#include "base/sanitizer_buildflags.h"
#include "base/version_info/channel.h"
#include "base/version_info/version_info_values.h"
#include "build/branding_buildflags.h"
#include "build/build_config.h"

namespace base {
class Version;
}

namespace version_info {

// Returns the product name, e.g. "Chromium" or "Google Chrome".
constexpr std::string_view GetProductName() {
  return PRODUCT_NAME;
}

// Returns the version number, e.g. "6.0.490.1".
constexpr std::string_view GetVersionNumber() {
  return PRODUCT_VERSION;
}

// Returns the major component (aka the milestone) of the version as an int,
// e.g. 6 when the version is "6.0.490.1".
int GetMajorVersionNumberAsInt();

// Like GetMajorVersionNumberAsInt(), but returns a string.
std::string GetMajorVersionNumber();

// Returns the result of GetVersionNumber() as a base::Version.
const base::Version& GetVersion();

// Returns a version control specific identifier of this release.
constexpr std::string_view GetLastChange() {
  return LAST_CHANGE;
}

// Returns whether this is an "official" release of the current version, i.e.
// whether knowing GetVersionNumber() is enough to completely determine what
// GetLastChange() is.
constexpr bool IsOfficialBuild() {
  return IS_OFFICIAL_BUILD;
}

// Returns the OS type, e.g. "Windows", "Linux", "FreeBSD", ...
constexpr std::string_view GetOSType() {
#if BUILDFLAG(IS_WIN)
  return "Windows";
#elif BUILDFLAG(IS_IOS)
  return "iOS";
#elif BUILDFLAG(IS_MAC)
  return "Mac OS X";
#elif BUILDFLAG(IS_CHROMEOS)
#if BUILDFLAG(GOOGLE_CHROME_BRANDING)
  return "ChromeOS";
#else
  return "ChromiumOS";
#endif
#elif BUILDFLAG(IS_ANDROID)
  return "Android";
#elif BUILDFLAG(IS_LINUX)
  return "Linux";
#elif BUILDFLAG(IS_FREEBSD)
  return "FreeBSD";
#elif BUILDFLAG(IS_OPENBSD)
  return "OpenBSD";
#elif BUILDFLAG(IS_SOLARIS)
  return "Solaris";
#elif BUILDFLAG(IS_FUCHSIA)
  return "Fuchsia";
#else
  return "Unknown";
#endif
}

// Returns a list of sanitizers enabled in this build.
constexpr std::string_view GetSanitizerList() {
  return ""
#if defined(ADDRESS_SANITIZER)
         "address "
#endif
#if BUILDFLAG(IS_HWASAN)
         "hwaddress "
#endif
#if defined(LEAK_SANITIZER)
         "leak "
#endif
#if defined(MEMORY_SANITIZER)
         "memory "
#endif
#if defined(THREAD_SANITIZER)
         "thread "
#endif
#if defined(UNDEFINED_SANITIZER)
         "undefined "
#endif
      ;
}

}  // namespace version_info

#endif  // BASE_VERSION_INFO_VERSION_INFO_H_
