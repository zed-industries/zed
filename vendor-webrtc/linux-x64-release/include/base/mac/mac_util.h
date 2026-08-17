// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_MAC_UTIL_H_
#define BASE_MAC_MAC_UTIL_H_

#include <AvailabilityMacros.h>
#import <CoreGraphics/CoreGraphics.h>
#include <stdint.h>

#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"

namespace base {
class FilePath;
}

namespace base::mac {

// Returns an sRGB color space.  The return value is a static value; do not
// release it!
BASE_EXPORT CGColorSpaceRef GetSRGBColorSpace();

// Adds the specified application to the set of Login Items with specified
// "hide" flag. This has the same effect as adding/removing the application in
// SystemPreferences->Accounts->LoginItems or marking Application in the Dock
// as "Options->Open on Login".
// Does nothing if the application is already set up as Login Item with
// specified hide flag.
BASE_EXPORT void AddToLoginItems(const FilePath& app_bundle_file_path,
                                 bool hide_on_startup);

// Removes the specified application from the list of Login Items.
BASE_EXPORT void RemoveFromLoginItems(const FilePath& app_bundle_file_path);

// Returns true if the current process was automatically launched as a
// 'Login Item' or via Lion's Resume. Used to suppress opening windows.
BASE_EXPORT bool WasLaunchedAsLoginOrResumeItem();

// Returns true if the current process was automatically launched as a
// 'Login Item' or via Resume, and the 'Reopen windows when logging back in'
// checkbox was selected by the user.  This indicates that the previous
// session should be restored.
BASE_EXPORT bool WasLaunchedAsLoginItemRestoreState();

// Returns true if the current process was automatically launched as a
// 'Login Item' with 'hide on startup' flag. Used to suppress opening windows.
BASE_EXPORT bool WasLaunchedAsHiddenLoginItem();

// Remove the quarantine xattr from the given file. Returns false if there was
// an error, or true otherwise.
BASE_EXPORT bool RemoveQuarantineAttribute(const FilePath& file_path);

// Sets the tags on a given file or folder.
BASE_EXPORT void SetFileTags(const FilePath& file_path,
                             const std::vector<std::string>& file_tags);

// The following two functions return the version of the macOS currently
// running. MacOSVersion() returns the full trio of version numbers, packed into
// one int (e.g. macOS 12.6.5 returns 12'06'05), and MacOSMajorVersion() returns
// only the major version number (e.g. macOS 12.6.5 returns 12). Use for runtime
// OS version checking. Prefer to use @available in Objective-C files. Note that
// this does not include any Rapid Security Response (RSR) suffixes (the "(a)"
// at the end of version numbers.)
BASE_EXPORT __attribute__((const)) int MacOSVersion();
inline __attribute__((const)) int MacOSMajorVersion() {
  return MacOSVersion() / 1'00'00;
}

enum class CPUType {
  kIntel,
  kTranslatedIntel,  // Rosetta
  kArm,
};

// Returns the type of CPU this is being executed on.
BASE_EXPORT CPUType GetCPUType();

// Returns an OS name + version string. e.g.:
//
//   "macOS Version 10.14.3 (Build 18D109)"
//
// Parts of this string change based on OS locale, so it's only useful for
// displaying to the user.
BASE_EXPORT std::string GetOSDisplayName();

// Returns the serial number of the macOS device.
BASE_EXPORT std::string GetPlatformSerialNumber();

// System Settings (née System Preferences) pane or subpanes to open via
// `OpenSystemSettingsPane()`, below. The naming is based on the naming in the
// System Settings app in the latest macOS release, macOS 13 Ventura.
enum class SystemSettingsPane {
  // Accessibility > Captions
  kAccessibility_Captions,

  // Date & Time
  kDateTime,

  // Network > Proxies
  kNetwork_Proxies,

  // Notifications; optionally pass a bundle identifier as `id_param` to
  // directly open the notification settings page for the given app.
  kNotifications,

  // Printers & Scanners
  kPrintersScanners,

  // Privacy & Security
  kPrivacySecurity,

  // Privacy & Security > Accessibility
  kPrivacySecurity_Accessibility,

  // Privacy & Security > Bluetooth
  kPrivacySecurity_Bluetooth,

  // Privacy & Security > Camera
  kPrivacySecurity_Camera,

  // Privacy & Security > Extensions > Sharing
  kPrivacySecurity_Extensions_Sharing,

  // Privacy & Security > Location Services
  kPrivacySecurity_LocationServices,

  // Privacy & Security > Microphone
  kPrivacySecurity_Microphone,

  // Privacy & Security > Screen Recording
  kPrivacySecurity_ScreenRecording,

  // Trackpad
  kTrackpad,
};

// Opens the specified System Settings pane. If the specified subpane does not
// exist on the release of macOS that is running, the parent pane will open
// instead. For some panes, `id_param` can be used to specify a subpane. See the
// various SystemSettingsPane values for details.
BASE_EXPORT void OpenSystemSettingsPane(SystemSettingsPane pane,
                                        const std::string& id_param = "");

// ------- For testing --------

// An implementation detail of `MacOSVersion()` above, exposed for testing.
BASE_EXPORT int ParseOSProductVersionForTesting(
    const std::string_view& version);

}  // namespace base::mac

#endif  // BASE_MAC_MAC_UTIL_H_
