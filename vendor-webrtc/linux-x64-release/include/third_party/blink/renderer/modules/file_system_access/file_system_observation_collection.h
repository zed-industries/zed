// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_OBSERVATION_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_OBSERVATION_COLLECTION_H_

#include "third_party/blink/public/mojom/file_system_access/file_system_access_observer.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class FileSystemObserver;
class FileSystemObservation;

class FileSystemObservationCollection final
    : public GarbageCollected<FileSystemObservationCollection>,
      public Supplement<ExecutionContext> {
 public:
  static const char kSupplementName[];
  static FileSystemObservationCollection* From(ExecutionContext* context);
  explicit FileSystemObservationCollection(ExecutionContext& context);
  ~FileSystemObservationCollection() = default;
  FileSystemObservationCollection(const FileSystemObservationCollection&) =
      delete;
  FileSystemObservationCollection& operator=(
      const FileSystemObservationCollection&) = delete;

  void Trace(Visitor* visitor) const override;
  void AddObservation(
      FileSystemObserver* observer,
      mojo::PendingReceiver<mojom::blink::FileSystemAccessObserver>
          observer_receiver);
  void RemoveObservation(FileSystemObserver* observer,
                         FileSystemObservation* observation);
  void RemoveObserver(FileSystemObserver* observer);

 private:
  Member<ExecutionContext> execution_context_;
  // Map of observers with active observations. The observer should stick around
  // for as long as there are active observations. As such, it is a Member of
  // the HeapHashMap.
  HeapHashMap<Member<FileSystemObserver>,
              Member<GCedHeapHashSet<Member<FileSystemObservation>>>>
      observation_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_OBSERVATION_COLLECTION_H_
