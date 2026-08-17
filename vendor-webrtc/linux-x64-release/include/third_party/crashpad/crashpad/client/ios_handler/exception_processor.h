// Copyright 2020 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_UTIL_IOS_EXCEPTION_PROCESSOR_H_
#define CRASHPAD_UTIL_IOS_EXCEPTION_PROCESSOR_H_

#include "base/files/file_path.h"
#include "util/misc/capture_context.h"

namespace crashpad {

//! \brief An interface for notifying the CrashpadClient of NSExceptions.
class ObjcExceptionDelegate {
 public:
  //! \brief The exception processor detected an exception as it was thrown and
  //!     captured the cpu context.
  //!
  //! \param context The cpu context of the thread throwing an exception.
  virtual void HandleUncaughtNSExceptionWithContext(
      NativeCPUContext* context) = 0;

  //! \brief The exception processor did not detect the exception as it was
  //!     thrown, and instead caught the exception via the
  //!     NSUncaughtExceptionHandler.
  //!
  //! \param frames An array of call stack frame addresses.
  //! \param num_frames The number of frames in |frames|.
  virtual void HandleUncaughtNSException(const uint64_t* frames,
                                         const size_t num_frames) = 0;

  //! \brief Generate an intermediate dump from an NSException caught with its
  //!     associated CPU context. Because the method for intercepting
  //!     exceptions is imperfect, write the the intermediate dump to a
  //!     temporary location specified by \a path. If the NSException matches
  //!     the one used in the UncaughtExceptionHandler, call
  //!     MoveIntermediateDumpAtPathToPending to move to the proper Crashpad
  //!     database pending location.
  //!
  //! \param[in] context The cpu context of the thread throwing an exception.
  //! \param[in] path Path to write the intermediate dump.
  virtual void HandleUncaughtNSExceptionWithContextAtPath(
      NativeCPUContext* context,
      const base::FilePath& path) = 0;

  //! \brief Moves an intermediate dump to the pending directory. This is meant
  //!     to be used by the UncaughtExceptionHandler, when the NSException
  //!     caught by the preprocessor matches the UncaughtExceptionHandler.
  //!
  //! \param[in] path Path to the specific intermediate dump.
  virtual bool MoveIntermediateDumpAtPathToPending(
      const base::FilePath& path) = 0;

 protected:
  ~ObjcExceptionDelegate() {}
};

//! \brief Installs the Objective-C exception preprocessor.
//!
//! When code raises an Objective-C exception, unwind the stack looking for
//! any exception handlers. If an exception handler is encountered, test to
//! see if it is a function known to be a catch-and-rethrow 'sinkhole' exception
//! handler. Various routines in UIKit do this, and they obscure the
//! crashing stack, since the original throw location is no longer present
//! on the stack (just the re-throw) when Crashpad captures the crash
//! report. In the case of sinkholes, trigger an immediate exception to
//! capture the original stack.
//!
//! This should be installed at the same time the CrashpadClient installs the
//! signal handler. It should only be installed once.
void InstallObjcExceptionPreprocessor(ObjcExceptionDelegate* delegate);

//! \brief Uninstalls the Objective-C exception preprocessor. Expected to be
//!     used by tests only.
void UninstallObjcExceptionPreprocessor();

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_EXCEPTION_PROCESSOR_H_
