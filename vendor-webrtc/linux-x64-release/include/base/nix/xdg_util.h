// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_NIX_XDG_UTIL_H_
#define BASE_NIX_XDG_UTIL_H_

// XDG refers to http://en.wikipedia.org/wiki/Freedesktop.org .
// This file contains utilities found across free desktop environments.

#include <optional>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/strings/cstring_view.h"

namespace base {

class CommandLine;
class Environment;
class FilePath;
struct LaunchOptions;

namespace nix {

enum DesktopEnvironment {
  DESKTOP_ENVIRONMENT_OTHER = 0,
  DESKTOP_ENVIRONMENT_CINNAMON = 1,
  DESKTOP_ENVIRONMENT_DEEPIN = 2,
  DESKTOP_ENVIRONMENT_GNOME = 3,
  // KDE{3,4,5,6} are sufficiently different that we count
  // them as different desktop environments here.
  DESKTOP_ENVIRONMENT_KDE3 = 4,
  DESKTOP_ENVIRONMENT_KDE4 = 5,
  DESKTOP_ENVIRONMENT_KDE5 = 6,
  DESKTOP_ENVIRONMENT_KDE6 = 12,
  DESKTOP_ENVIRONMENT_PANTHEON = 7,
  DESKTOP_ENVIRONMENT_UKUI = 8,
  DESKTOP_ENVIRONMENT_UNITY = 9,
  DESKTOP_ENVIRONMENT_XFCE = 10,
  DESKTOP_ENVIRONMENT_LXQT = 11,
};

// Values based on valid types indicated in:
// https://www.freedesktop.org/software/systemd/man/pam_systemd.html; though
// "Unset" and "Other" are provided by us to distinguish between the potentially
// valid "Unspecified" and other cases where we may not be able to find the
// value.
enum class SessionType {
  kUnset = 0,
  kOther = 1,
  kUnspecified = 2,
  kTty = 3,
  kX11 = 4,
  kWayland = 5,
  kMir = 6,
};

using XdgActivationTokenCallback = base::OnceCallback<void(std::string token)>;
using XdgActivationTokenCreator =
    base::RepeatingCallback<void(XdgActivationTokenCallback callback)>;
using XdgActivationLaunchOptionsCallback =
    base::OnceCallback<void(LaunchOptions)>;

// The default XDG config directory name.
inline constexpr char kDotConfigDir[] = ".config";

// The XDG config directory environment variable.
inline constexpr char kXdgConfigHomeEnvVar[] = "XDG_CONFIG_HOME";

// The XDG current desktop environment variable.
inline constexpr char kXdgCurrentDesktopEnvVar[] = "XDG_CURRENT_DESKTOP";

// The XDG session type environment variable.
inline constexpr char kXdgSessionTypeEnvVar[] = "XDG_SESSION_TYPE";

// The XDG activation token environment variable.
inline constexpr char kXdgActivationTokenEnvVar[] = "XDG_ACTIVATION_TOKEN";

// Internally used to communicate the activation token between a newly launched
// process and an existing browser process.
inline constexpr char kXdgActivationTokenSwitch[] = "xdg-activation-token";

// Utility function for getting XDG directories.
// |env_name| is the name of an environment variable that we want to use to get
// a directory path. |fallback_dir| is the directory relative to $HOME that we
// use if |env_name| cannot be found or is empty. |fallback_dir| may be NULL.
// Examples of |env_name| are XDG_CONFIG_HOME and XDG_DATA_HOME.
BASE_EXPORT FilePath GetXDGDirectory(Environment* env,
                                     cstring_view env_name,
                                     const char* fallback_dir);

// Wrapper around xdg_user_dir_lookup() from src/base/third_party/xdg-user-dirs
// This looks up "well known" user directories like the desktop and music
// folder. Examples of |dir_name| are DESKTOP and MUSIC.
BASE_EXPORT FilePath GetXDGUserDirectory(const char* dir_name,
                                         const char* fallback_dir);

// Get the path to write user-specific application data files to, as specified
// in the XDG Base Directory Specification:
// http://standards.freedesktop.org/basedir-spec/latest/
BASE_EXPORT FilePath GetXDGDataWriteLocation(Environment* env);

// Get the list of paths to search for application data files, in order of
// preference, as specified in the XDG Base Directory Specification:
// http://standards.freedesktop.org/basedir-spec/latest/
// Called on the FILE thread.
BASE_EXPORT std::vector<FilePath> GetXDGDataSearchLocations(Environment* env);

// Return an entry from the DesktopEnvironment enum with a best guess
// of which desktop environment we're using.  We use this to know when
// to attempt to use preferences from the desktop environment --
// proxy settings, password manager, etc.
BASE_EXPORT DesktopEnvironment GetDesktopEnvironment(Environment* env);

// Return a string representation of the given desktop environment.
// May return NULL in the case of DESKTOP_ENVIRONMENT_OTHER.
BASE_EXPORT const char* GetDesktopEnvironmentName(DesktopEnvironment env);
// Convenience wrapper that calls GetDesktopEnvironment() first.
BASE_EXPORT const char* GetDesktopEnvironmentName(Environment* env);

// Return an entry from the SessionType enum with a best guess
// of which session type we're using.
BASE_EXPORT SessionType GetSessionType(Environment& env);

// Sets the global activation token from the environment and returns it if it
// exists, and removes it from the environment to prevent it from leaking into
// child processes.
BASE_EXPORT std::optional<std::string> ExtractXdgActivationTokenFromEnv(
    Environment& env);

// Sets the global activation token from the command line if it exists and
// removes it from the command line.
BASE_EXPORT void ExtractXdgActivationTokenFromCmdLine(
    base::CommandLine& cmd_line);

// Sets the global activation token not received from the command line or as
// an environment variable, e.g. from notification D-BUS API.
BASE_EXPORT void SetActivationToken(std::string token);

// Transfers ownership of the currently set global activation token if set.
BASE_EXPORT std::optional<std::string> TakeXdgActivationToken();

// Sets the global token creator.
BASE_EXPORT void SetXdgActivationTokenCreator(
    XdgActivationTokenCreator token_creator);

// Tries to create an xdg-activation token and invokes the `callback` with
// `LaunchOptions` containing the token if available, or empty `LaunchOptions`.
// This must be called on UI thread.
BASE_EXPORT void CreateLaunchOptionsWithXdgActivation(
    XdgActivationLaunchOptionsCallback callback);

// Tries to create an xdg-activation token and invokes the `callback` with the
// token if available, or an empty string.
// This must be called on UI thread.
BASE_EXPORT void CreateXdgActivationToken(XdgActivationTokenCallback callback);

// Returns a request path as specified in v0.9 of xdg-desktop-portal:
// https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Request.html
BASE_EXPORT
std::string XdgDesktopPortalRequestPath(const std::string& sender,
                                        const std::string& token);

// Returns a session path as specified in v0.9 of xdg-desktop-portal:
// https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Session.html
BASE_EXPORT
std::string XdgDesktopPortalSessionPath(const std::string& sender,
                                        const std::string& token);

}  // namespace nix
}  // namespace base

#endif  // BASE_NIX_XDG_UTIL_H_
