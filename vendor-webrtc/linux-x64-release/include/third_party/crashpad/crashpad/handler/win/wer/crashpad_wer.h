// Copyright 2022 The Crashpad Authors
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

#ifndef CRASHPAD_HANDLER_WIN_WER_CRASHPAD_WER_H_
#define CRASHPAD_HANDLER_WIN_WER_CRASHPAD_WER_H_

#include <Windows.h>
#include <werapi.h>

namespace crashpad::wer {
//! \brief Embedder calls this from OutOfProcessExceptionEventCallback().
//!
//! In the embedder's WER runtime exception helper, call this during
//! OutOfProcessExceptionEventCallback().
//!
//! \param[in] handled_exceptions is an array of exception codes that the helper
//!     should pass on to crashpad handler (if possible). Pass nullptr and set
//!     num_handled_exceptions to 0 to pass every exception on to the crashpad
//!     handler.
//! \param[in] num_handled_exceptions is the number of elements in the array
//!     passed to handled_exceptions.
//! \param[in] pContext is the context provided by WerFault to the helper.
//! \param[in] pExceptionInformation is the exception information provided by
//!     WerFault to the helper DLL.
//!
//! \return `true` if the target process was dumped by the crashpad handler then
//! terminated, or `false` otherwise.
bool ExceptionEvent(
    const DWORD* handled_exceptions,
    size_t num_handled_exceptions,
    const PVOID pContext,
    const PWER_RUNTIME_EXCEPTION_INFORMATION pExceptionInformation);

}  // namespace crashpad::wer

#endif  // CRASHPAD_HANDLER_WIN_WER_CRASHPAD_WER_H_
