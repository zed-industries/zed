/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_DOM_FILE_SYSTEM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_DOM_FILE_SYSTEM_H_

#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/filesystem/dom_file_system_base.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DirectoryEntry;
class ExecutionContext;
class FileEntry;

class MODULES_EXPORT DOMFileSystem final
    : public DOMFileSystemBase,
      public ActiveScriptWrappable<DOMFileSystem>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Creates a new isolated file system for the given filesystemId.
  static DOMFileSystem* CreateIsolatedFileSystem(ExecutionContext*,
                                                 const String& filesystem_id);

  DOMFileSystem(ExecutionContext*,
                const String& name,
                mojom::blink::FileSystemType,
                const KURL& root_url);

  DirectoryEntry* root() const;

  // DOMFileSystemBase overrides.
  void AddPendingCallbacks() override;
  void RemovePendingCallbacks() override;
  void ReportError(ErrorCallback, base::File::Error error) override;

  static void ReportError(ExecutionContext*,
                          ErrorCallback,
                          base::File::Error error);

  // ScriptWrappable overrides.
  bool HasPendingActivity() const final;

  void CreateWriter(const FileEntry*,
                    FileWriterCallbacks::SuccessCallback,
                    FileWriterCallbacks::ErrorCallback);
  void CreateFile(const FileEntry*,
                  SnapshotFileCallback::SuccessCallback,
                  SnapshotFileCallback::ErrorCallback);

  // Schedule a callback. This should not cross threads (should be called on the
  // same context thread).
  static void ScheduleCallback(ExecutionContext* execution_context,
                               base::OnceClosure task);

  void Trace(Visitor*) const override;

 private:
  static String TaskNameForInstrumentation() { return "FileSystem"; }

  int number_of_pending_callbacks_;
  Member<DirectoryEntry> root_entry_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_DOM_FILE_SYSTEM_H_
