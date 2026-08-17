/*
 * Copyright (C) 2010 Google Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_H_

#include "base/timer/elapsed_timer.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/fileapi/file_error.h"
#include "third_party/blink/renderer/core/fileapi/file_read_type.h"
#include "third_party/blink/renderer/core/fileapi/file_reader_client.h"
#include "third_party/blink/renderer/core/fileapi/file_reader_loader.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class Blob;
class ExceptionState;
class ExecutionContext;
class V8UnionArrayBufferOrString;
enum class FileErrorCode;

class CORE_EXPORT FileReader final : public EventTarget,
                                     public ActiveScriptWrappable<FileReader>,
                                     public ExecutionContextLifecycleObserver,
                                     public FileReaderAccumulator {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static FileReader* Create(ExecutionContext*);

  explicit FileReader(ExecutionContext*);
  ~FileReader() override;

  enum ReadyState { kEmpty = 0, kLoading = 1, kDone = 2 };

  void readAsArrayBuffer(Blob*, ExceptionState&);
  void readAsBinaryString(Blob*, ExceptionState&);
  void readAsText(Blob*, const String& encoding, ExceptionState&);
  void readAsText(Blob*, ExceptionState&);
  void readAsDataURL(Blob*, ExceptionState&);
  void abort();

  ReadyState getReadyState() const { return state_; }
  DOMException* error() { return error_.Get(); }
  V8UnionArrayBufferOrString* result() const;
  probe::AsyncTaskContext* async_task_context() { return &async_task_context_; }

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  // ScriptWrappable
  bool HasPendingActivity() const final;

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextLifecycleObserver::GetExecutionContext();
  }

  // FileReaderClient
  FileErrorCode DidStartLoading() override;
  FileErrorCode DidReceiveData() override;
  void DidFinishLoading(FileReaderData contents) override;
  void DidFail(FileErrorCode) override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(loadstart, kLoadstart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(progress, kProgress)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(load, kLoad)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(abort, kAbort)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(loadend, kLoadend)

  void Trace(Visitor*) const override;

 private:
  class ThrottlingController;

  void Terminate();
  void ReadInternal(Blob*, FileReadType, ExceptionState&);
  void FireEvent(const AtomicString& type);

  void ExecutePendingRead();

  ReadyState state_;

  // Internal loading state, which could differ from ReadyState as it's
  // for script-visible state while this one's for internal state.
  enum LoadingState {
    kLoadingStateNone,
    kLoadingStatePending,
    kLoadingStateLoading,
    kLoadingStateAborted
  };
  LoadingState loading_state_;
  bool still_firing_events_;

  String blob_type_;
  scoped_refptr<BlobDataHandle> blob_data_handle_;
  FileReadType read_type_;
  String encoding_;
  probe::AsyncTaskContext async_task_context_;

  Member<FileReaderLoader> loader_;
  Member<V8UnionArrayBufferOrString> result_ = nullptr;
  Member<DOMException> error_;
  std::optional<base::ElapsedTimer> last_progress_notification_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_H_
