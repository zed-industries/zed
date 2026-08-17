/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 * Copyright (C) 2013 Samsung Electronics. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_SYNC_CALLBACK_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_SYNC_CALLBACK_HELPER_H_

#include "third_party/blink/renderer/core/fileapi/file_error.h"
#include "third_party/blink/renderer/modules/filesystem/file_system_callbacks.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

// Helper class to support DOMFileSystemSync implementation.
template <typename CallbackArg>
class DOMFileSystemCallbacksSyncHelper final
    : public GarbageCollected<DOMFileSystemCallbacksSyncHelper<CallbackArg>> {
 public:
  DOMFileSystemCallbacksSyncHelper() = default;

  void Trace(Visitor* visitor) const { visitor->Trace(result_); }

  // Simple/new success and error callback wrappers.
  void OnSuccess(CallbackArg* arg) {
    DCHECK(arg);
    result_ = arg;
  }
  void OnError(base::File::Error error_code) { error_code_ = error_code; }

  CallbackArg* GetResultOrThrow(ExceptionState& exception_state) {
    if (error_code_ != base::File::FILE_OK) {
      file_error::ThrowDOMException(exception_state, error_code_);
      return nullptr;
    }

    return result_.Get();
  }

 private:
  Member<CallbackArg> result_;
  base::File::Error error_code_ = base::File::FILE_OK;
};

using EntryCallbacksSyncHelper = DOMFileSystemCallbacksSyncHelper<Entry>;

using FileSystemCallbacksSyncHelper = DOMFileSystemCallbacksSyncHelper<
    DOMFileSystem>;

using FileWriterCallbacksSyncHelper = DOMFileSystemCallbacksSyncHelper<
    FileWriterBase>;

using MetadataCallbacksSyncHelper = DOMFileSystemCallbacksSyncHelper<
    Metadata>;

using VoidCallbacksSyncHelper = DOMFileSystemCallbacksSyncHelper<
    ExecutionContext /* dummy_arg_for_sync_helper */>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_SYNC_CALLBACK_HELPER_H_
