// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_TRANSFERABLES_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_TRANSFERABLES_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DOMArrayBufferBase;
class ExceptionState;
class ImageBitmap;
class OffscreenCanvas;
class MessagePort;
class MojoHandle;
class ReadableStream;
class WritableStream;
class TransformStream;
class MediaStreamTrack;

using ArrayBufferArray = HeapVector<Member<DOMArrayBufferBase>>;
using ImageBitmapArray = HeapVector<Member<ImageBitmap>>;
using OffscreenCanvasArray = HeapVector<Member<OffscreenCanvas>>;
using MessagePortArray = HeapVector<Member<MessagePort>>;
using GCedMessagePortArray = GCedHeapVector<Member<MessagePort>>;
using MojoHandleArray = HeapVector<Member<blink::MojoHandle>>;
using ReadableStreamArray = HeapVector<Member<ReadableStream>>;
using WritableStreamArray = HeapVector<Member<WritableStream>>;
using TransformStreamArray = HeapVector<Member<TransformStream>>;
using MediaStreamTrackArray = HeapVector<Member<MediaStreamTrack>>;

class CORE_EXPORT Transferables final {
  STACK_ALLOCATED();

 public:
  Transferables() = default;

  Transferables(const Transferables&) = delete;
  Transferables& operator=(const Transferables&) = delete;

  ~Transferables();

  ArrayBufferArray array_buffers;
  ImageBitmapArray image_bitmaps;
  OffscreenCanvasArray offscreen_canvases;
  MessagePortArray message_ports;
  MojoHandleArray mojo_handles;
  ReadableStreamArray readable_streams;
  WritableStreamArray writable_streams;
  TransformStreamArray transform_streams;
  MediaStreamTrackArray media_stream_tracks;

  class CORE_EXPORT TransferList : public GarbageCollectedMixin {
   public:
    virtual ~TransferList() = default;
    virtual void FinalizeTransfer(ExceptionState&) {}
  };

  HeapHashMap<const void* const*, Member<TransferList>> transfer_lists;

  template <typename T>
  T* GetOrCreateTransferList() {
    auto result = transfer_lists.insert(&T::kTransferListKey, nullptr);
    if (!result.stored_value->value)
      result.stored_value->value = MakeGarbageCollected<T>();
    return static_cast<T*>(result.stored_value->value.Get());
  }

  template <typename T>
  const T* GetTransferListIfExists() const {
    auto it = transfer_lists.find(&T::kTransferListKey);
    if (it == transfer_lists.end())
      return nullptr;
    return static_cast<T*>(it->value.Get());
  }
};

// Along with extending |Transferables| to hold a new kind of transferable
// objects, serialization handling code changes are required:
//   - extend ScriptValueSerializer::copyTransferables()
//   - alter SerializedScriptValue(Factory) to do the actual transfer.

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_TRANSFERABLES_H_
