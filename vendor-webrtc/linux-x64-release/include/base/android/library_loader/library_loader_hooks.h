// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_LIBRARY_LOADER_LIBRARY_LOADER_HOOKS_H_
#define BASE_ANDROID_LIBRARY_LOADER_LIBRARY_LOADER_HOOKS_H_

#include <jni.h>

#include "base/base_export.h"
#include "base/command_line.h"
#include "base/functional/callback.h"
#include "base/metrics/field_trial.h"

namespace base {

namespace android {

// The process the shared library is loaded in.
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base.library_loader
enum LibraryProcessType {
  // The LibraryLoad has not been initialized.
  PROCESS_UNINITIALIZED = 0,
  // Shared library is running in browser process.
  PROCESS_BROWSER = 1,
  // Shared library is running in child process.
  PROCESS_CHILD = 2,
  // Shared library is running in the app that uses webview.
  PROCESS_WEBVIEW = 3,
  // Shared library is running in child process as part of webview.
  PROCESS_WEBVIEW_CHILD = 4,
  // Shared library is running in a non-embedded WebView process.
  PROCESS_WEBVIEW_NONEMBEDDED = 5,
};

// Returns the library process type this library was loaded for.
BASE_EXPORT LibraryProcessType GetLibraryProcessType();

// Whether fewer code should be prefetched, and no-readahead should be set.
// Returns true on low-end devices, where this speeds up startup, and false
// elsewhere, where it slows it down. See
// https://bugs.chromium.org/p/chromium/issues/detail?id=758566#c71 for details.
BASE_EXPORT bool IsUsingOrderfileOptimization();

typedef bool NativeInitializationHook(LibraryProcessType library_process_type);

BASE_EXPORT void SetNativeInitializationHook(
    NativeInitializationHook native_initialization_hook);

// Record any pending renderer histogram value as histograms.  Pending values
// are set by
// JNI_LibraryLoader_RegisterChromiumAndroidLinkerRendererHistogram().
BASE_EXPORT void RecordLibraryLoaderRendererHistograms();

// Typedef for hook function to be called (indirectly from Java) once the
// libraries are loaded. The hook function should register the JNI bindings
// required to start the application. It should return true for success and
// false for failure.
// Note: this can't use base::{Once, Repeating}Callback because there is no
// way of initializing the default callback without using static objects, which
// we forbid.
typedef bool LibraryLoadedHook(JNIEnv* env,
                               jclass clazz,
                               LibraryProcessType library_process_type);

// Set the hook function to be called (from Java) once the libraries are loaded.
// SetLibraryLoadedHook may only be called from JNI_OnLoad. The hook function
// should register the JNI bindings required to start the application.

BASE_EXPORT void SetLibraryLoadedHook(LibraryLoadedHook* func);

// Call on exit to delete the AtExitManager which OnLibraryLoadedOnUIThread
// created.
BASE_EXPORT void LibraryLoaderExitHook();

// Initialize AtExitManager, this must be done at the begining of loading
// shared library.
void InitAtExitManager();

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_LIBRARY_LOADER_LIBRARY_LOADER_HOOKS_H_
