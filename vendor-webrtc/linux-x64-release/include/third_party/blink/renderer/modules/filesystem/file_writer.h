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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_FILE_WRITER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_FILE_WRITER_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/filesystem/file_writer_base.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"

namespace blink {

class Blob;
class DOMException;
class ExceptionState;
class ExecutionContext;
enum class FileErrorCode;

class FileWriter final : public EventTarget,
                         public FileWriterBase,
                         public ActiveScriptWrappable<FileWriter>,
                         public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(FileWriter, Dispose);

 public:
  explicit FileWriter(ExecutionContext*);
  ~FileWriter() override;

  enum ReadyState { kInit = 0, kWriting = 1, kDone = 2 };

  void write(Blob*, ExceptionState&);
  void seek(int64_t position, ExceptionState&);
  void truncate(int64_t length, ExceptionState&);
  void abort(ExceptionState&);
  ReadyState getReadyState() const { return ready_state_; }
  DOMException* error() const { return error_.Get(); }

  // FileWriterBase
  void DidWriteImpl(int64_t bytes, bool complete) override;
  void DidTruncateImpl() override;
  void DidFailImpl(base::File::Error error) override;
  void DoTruncate(const KURL& path, int64_t offset) override;
  void DoWrite(const KURL& path, const Blob& blob, int64_t offset) override;
  void DoCancel() override;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  // ScriptWrappable
  bool HasPendingActivity() const final;

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextLifecycleObserver::GetExecutionContext();
  }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(writestart, kWritestart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(progress, kProgress)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(write, kWrite)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(abort, kAbort)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(writeend, kWriteend)

  void Trace(Visitor*) const override;

 private:
  enum Operation {
    kOperationNone,
    kOperationWrite,
    kOperationTruncate,
    kOperationAbort
  };

  void CompleteAbort();

  void DoOperation(Operation);

  void SignalCompletion(base::File::Error error_code);

  void FireEvent(const AtomicString& type);

  void SetError(FileErrorCode, ExceptionState&);

  void Dispose();

  Member<DOMException> error_;
  ReadyState ready_state_;
  Operation operation_in_progress_;
  Operation queued_operation_;
  uint64_t bytes_written_;
  uint64_t bytes_to_write_;
  uint64_t truncate_length_;
  uint64_t num_aborts_;
  uint8_t recursion_depth_;
  base::TimeTicks last_progress_notification_time_;
  Member<Blob> blob_being_written_;
  int request_id_;
  probe::AsyncTaskContext async_task_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_FILE_WRITER_H_
