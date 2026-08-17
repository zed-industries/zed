// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_ACCESS_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_ACCESS_MANAGER_H_

#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_manager.mojom-blink.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_observer_host.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExecutionContext;

// This class maintains a connection to the FileSystemAccessManager remote.
class MODULES_EXPORT FileSystemAccessManager final
    : public GarbageCollected<FileSystemAccessManager>,
      public Supplement<ExecutionContext>,
      public ExecutionContextClient {
 public:
  static const char kSupplementName[];

  static FileSystemAccessManager& From(ExecutionContext*);

  explicit FileSystemAccessManager(ExecutionContext*);

  FileSystemAccessManager(const FileSystemAccessManager&) = delete;
  FileSystemAccessManager& operator=(const FileSystemAccessManager&) = delete;

  HeapMojoRemote<mojom::blink::FileSystemAccessManager>::Proxy* operator->() {
    return remote_.get();
  }

  // GarbageCollected
  void Trace(Visitor*) const override;

 private:
  // TODO(https://crbug.com/1241174): Extend ExecutionContextLifecycleObserver
  // and override ContextEnteredBackForwardCache() to respond gracefully when a
  // page enters the BFCache (e.g. by releasing locks).

  void EnsureConnection();

  HeapMojoRemote<mojom::blink::FileSystemAccessManager> remote_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_ACCESS_MANAGER_H_
