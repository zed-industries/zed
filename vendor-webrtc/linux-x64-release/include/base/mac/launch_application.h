// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_LAUNCH_APPLICATION_H_
#define BASE_MAC_LAUNCH_APPLICATION_H_

#import <AppKit/AppKit.h>

#include <string>
#include <variant>
#include <vector>

#include "base/base_export.h"
#include "base/command_line.h"
#include "base/files/file_path.h"
#include "base/functional/callback_forward.h"

// Launches an application.
//
// What makes this different from `LaunchProcess()` in /base/process/launch.h?
// That code creates a sub-process, which is useful for utility processes and
// the like, but inappropriate for independent applications.
// `LaunchApplication()` below, on the other hand, launches an app in the way
// that the Finder or Dock would launch an app.

namespace base::mac {

struct LaunchApplicationOptions {
  bool activate = true;
  bool create_new_instance = false;
  bool prompt_user_if_needed = false;

  // When this option is set to true, a private SPI is used to launch the app
  // "invisibly". Apps launched this way do not show up as running.
  // Note that opening URLs in an already running hidden-in-background app
  // appears to always cause the app to transition to foreground, even if we've
  // requested a background launch.
  bool hidden_in_background = false;
};

using LaunchApplicationCallback =
    base::OnceCallback<void(NSRunningApplication*, NSError*)>;

using CommandLineArgs =
    std::variant<std::monostate, CommandLine, std::vector<std::string>>;

// Launches the specified application.
//   - `app_bundle_path`: the location of the application to launch
//   - `command_line_args`: the command line arguments to pass to the
//      application if the app isn't already running (the default-constructed
//      monostate alternative means no arguments)
//      - Note: The application to launch is specified as `app_bundle_path`, so
//        if `base::CommandLine` is used to provide command line arguments, its
//        first argument will be ignored
//   - `url_specs`: the URLs for the application to open (an empty vector is OK)
//   - `options`: options to modify the launch
//   - `callback`: the result callback
//
// When the launch is complete, `callback` is called on the main thread. If the
// launch succeeded, it will be called with an `NSRunningApplication*` and the
// `NSError*` will be nil. If the launch failed, it will be called with an
// `NSError*`, and the `NSRunningApplication*` will be nil.
BASE_EXPORT void LaunchApplication(const FilePath& app_bundle_path,
                                   const CommandLineArgs& command_line_args,
                                   const std::vector<std::string>& url_specs,
                                   LaunchApplicationOptions options,
                                   LaunchApplicationCallback callback);

}  // namespace base::mac

#endif  // BASE_MAC_LAUNCH_APPLICATION_H_
