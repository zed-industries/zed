// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_BUNDLE_LOCATIONS_H_
#define BASE_APPLE_BUNDLE_LOCATIONS_H_

#include "base/base_export.h"
#include "base/files/file_path.h"

#if defined(__OBJC__)
#import <Foundation/Foundation.h>
#endif  // __OBJC__

namespace base {
class FilePath;
}

// NSBundle isn't thread-safe; all functions in this file must be called on the
// main thread.

namespace base::apple {

// This file provides several functions to explicitly request the various
// component bundles of Chrome.  Please use these methods rather than calling
// `+[NSBundle mainBundle]` or `CFBundleGetMainBundle()`.
//
// Terminology
//  - "Outer Bundle" - This is the main bundle for Chrome; it's what
//  `+[NSBundle mainBundle]` returns when Chrome is launched normally.
//
//  - "Main Bundle" - This is the bundle from which Chrome was launched.
//  This will be the same as the outer bundle except when Chrome is launched
//  via an app shortcut, in which case this will return the app shortcut's
//  bundle rather than the main Chrome bundle.
//
//  - "Framework Bundle" - This is the bundle corresponding to the Chrome
//  framework.
//
// Guidelines for use:
//  - To access a resource, the Framework bundle should be used.
//  - If the choice is between the Outer or Main bundles then please choose
//  carefully.  Most often the Outer bundle will be the right choice, but for
//  cases such as adding an app to the "launch on startup" list, the Main
//  bundle is probably the one to use.

// Methods for retrieving the various bundles.
BASE_EXPORT FilePath MainBundlePath();
BASE_EXPORT FilePath OuterBundlePath();
BASE_EXPORT FilePath FrameworkBundlePath();
BASE_EXPORT std::string MainBundleIdentifier();
#if defined(__OBJC__)
BASE_EXPORT NSBundle* MainBundle();
BASE_EXPORT NSURL* MainBundleURL();
BASE_EXPORT NSBundle* OuterBundle();
BASE_EXPORT NSURL* OuterBundleURL();
BASE_EXPORT NSBundle* FrameworkBundle();
#endif  // __OBJC__

// Set the bundle that the preceding functions will return, overriding the
// default values. Restore the default by passing in `nil` or an empty
// `FilePath`.
BASE_EXPORT void SetOverrideOuterBundlePath(const FilePath& file_path);
BASE_EXPORT void SetOverrideFrameworkBundlePath(const FilePath& file_path);
#if defined(__OBJC__)
BASE_EXPORT void SetOverrideOuterBundle(NSBundle* bundle);
BASE_EXPORT void SetOverrideFrameworkBundle(NSBundle* bundle);
#endif  // __OBJC__

}  // namespace base::apple

#endif  // BASE_APPLE_BUNDLE_LOCATIONS_H_
