// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_OBSERVATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_OBSERVATION_H_

#include "third_party/blink/public/mojom/file_system_access/file_system_access_observer.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"

namespace blink {

class FileSystemObserver;

class FileSystemObservation final
    : public GarbageCollected<FileSystemObservation>,
      public mojom::blink::FileSystemAccessObserver {
 public:
  FileSystemObservation(
      ExecutionContext* context,
      FileSystemObserver* observer,
      mojo::PendingReceiver<mojom::blink::FileSystemAccessObserver>
          observation_receiver);
  ~FileSystemObservation() override = default;
  FileSystemObservation(const FileSystemObservation&) = delete;
  FileSystemObservation& operator=(const FileSystemObservation&) = delete;

  void Trace(Visitor* visitor) const;
  void DisconnectReceiver();

  // blink::mojom::blink::FileSystemAccessObserver
  void OnFileChanges(WTF::Vector<mojom::blink::FileSystemAccessChangePtr>
                         mojo_changes) override;

 private:
  void OnRemoteDisconnected();

  // The observer that set up this observation.
  Member<FileSystemObserver> observer_;
  Member<ExecutionContext> execution_context_;

  // Corresponds to a file system watch set up by calling `observe()`.
  HeapMojoReceiver<mojom::blink::FileSystemAccessObserver,
                   FileSystemObservation>
      receiver_{this, nullptr};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_OBSERVATION_H_
