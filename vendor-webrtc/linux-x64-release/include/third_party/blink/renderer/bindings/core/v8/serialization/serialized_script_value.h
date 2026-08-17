/*
 * Copyright (C) 2009, 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZED_SCRIPT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZED_SCRIPT_VALUE_H_

#include <algorithm>
#include <memory>

#include "base/containers/heap_array.h"
#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "base/functional/callback_forward.h"
#include "base/types/optional_util.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/common/messaging/message_port_channel.h"
#include "third_party/blink/public/common/messaging/message_port_descriptor.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_transfer_token.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/native_value_traits.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/transferables.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/mojo/mojo_handle.h"
#include "third_party/blink/renderer/core/streams/readable_stream_transferring_optimizer.h"
#include "third_party/blink/renderer/core/streams/writable_stream_transferring_optimizer.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer/array_buffer_contents.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partitions.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "v8/include/v8.h"

namespace blink {

class BlobDataHandle;
class DOMSharedArrayBuffer;
class ExceptionState;
class ExecutionContext;
class MessagePort;
class ScriptObject;
class StaticBitmapImage;
class Transferables;
class UnpackedSerializedScriptValue;
class WebBlobInfo;

typedef HashMap<String, scoped_refptr<BlobDataHandle>> BlobDataHandleMap;
typedef Vector<mojo::ScopedHandle> MojoScopedHandleArray;
typedef Vector<WebBlobInfo> WebBlobInfoArray;
typedef HeapVector<Member<DOMSharedArrayBuffer>> SharedArrayBufferArray;

class CORE_EXPORT SerializedScriptValue
    : public ThreadSafeRefCounted<SerializedScriptValue> {
  USING_FAST_MALLOC(SerializedScriptValue);

 public:
  class Stream final {
    DISALLOW_NEW();

   public:
    explicit Stream(MessagePortDescriptor descriptor)
        : channel(std::move(descriptor)) {}
    Stream(MessagePortDescriptor descriptor,
           std::unique_ptr<ReadableStreamTransferringOptimizer> optimizer)
        : channel(std::move(descriptor)),
          readable_optimizer(std::move(optimizer)) {}
    Stream(MessagePortDescriptor descriptor,
           std::unique_ptr<WritableStreamTransferringOptimizer> optimizer)
        : channel(std::move(descriptor)),
          writable_optimizer(std::move(optimizer)) {}

    MessagePortChannel channel;
    std::unique_ptr<ReadableStreamTransferringOptimizer> readable_optimizer;
    std::unique_ptr<WritableStreamTransferringOptimizer> writable_optimizer;
  };

  using ArrayBufferContentsArray = Vector<ArrayBufferContents, 1>;
  using SharedArrayBufferContentsArray = Vector<ArrayBufferContents, 1>;
  using ImageBitmapContentsArray = Vector<scoped_refptr<StaticBitmapImage>, 1>;
  using TransferredWasmModulesArray = WTF::Vector<v8::CompiledWasmModule>;
  using MessagePortChannelArray = Vector<MessagePortChannel>;
  using StreamArray = Vector<Stream>;
  using FileSystemAccessTokensArray =
      Vector<mojo::PendingRemote<mojom::blink::FileSystemAccessTransferToken>>;

  // Increment this for each incompatible change to the wire format.
  // Version 2: Added StringUCharTag for UChar v8 strings.
  // Version 3: Switched to using uuids as blob data identifiers.
  // Version 4: Extended File serialization to be complete.
  // Version 5: Added CryptoKeyTag for Key objects.
  // Version 6: Added indexed serialization for File, Blob, and FileList.
  // Version 7: Extended File serialization with user visibility.
  // Version 8: File.lastModified in milliseconds (seconds-based in earlier
  //            versions.)
  // Version 9: Added Map and Set support.
  // [versions skipped]
  // Version 16: Separate versioning between V8 and Blink.
  // Version 17: Remove unnecessary byte swapping.
  // Version 18: Add a list of key-value pairs for ImageBitmap and ImageData to
  //             support color space information, compression, etc.
  // Version 19: Add DetectedBarcode, DetectedFace, and DetectedText support.
  // Version 20: Remove DetectedBarcode, DetectedFace, and DetectedText support.
  // Version 21: Add support for trailer data which marks required exposed
  //             interfaces.
  //
  // The following versions cannot be used, in order to be able to
  // deserialize version 0 SSVs. The class implementation has details.
  // DO NOT USE: 35, 64, 68, 73, 78, 82, 83, 85, 91, 98, 102, 108, 123.
  //
  // WARNING: Increasing this value is a change which cannot safely be rolled
  // back without breaking compatibility with data stored on disk. It is
  // strongly recommended that you do not make such changes near a release
  // milestone branch point.
  //
  // Recent changes are routinely reverted in preparation for branch, and this
  // has been the cause of at least one bug in the past.
  //
  // WARNING: if you're changing this from version 21, your change will interact
  // with the fix to https://crbug.com/1341844. Consult bug owner before
  // proceeding. (After that is settled, remove this paragraph.)
  static constexpr uint32_t kWireFormatVersion = 21;

  // This enumeration specifies whether we're serializing a value for storage;
  // e.g. when writing to IndexedDB. This corresponds to the forStorage flag of
  // the HTML spec:
  // https://html.spec.whatwg.org/C/#safe-passing-of-structured-data
  enum StoragePolicy {
    // Not persisted; used only during the execution of the browser.
    kNotForStorage,
    // May be written to disk and read during a subsequent execution of the
    // browser.
    kForStorage,
  };

  struct SerializeOptions {
    STACK_ALLOCATED();

   public:
    enum WasmSerializationPolicy {
      kUnspecified,  // Invalid value, used as default initializer.
      kTransfer,     // In-memory transfer without (necessarily) serializing.
      kSerialize,    // Serialize to a byte stream.
      kBlockedInNonSecureContext  // Block transfer or serialization.
    };

    SerializeOptions() = default;
    explicit SerializeOptions(StoragePolicy for_storage)
        : for_storage(for_storage) {}

    Transferables* transferables = nullptr;
    WebBlobInfoArray* blob_info = nullptr;
    WasmSerializationPolicy wasm_policy = kTransfer;
    StoragePolicy for_storage = kNotForStorage;
  };
  static scoped_refptr<SerializedScriptValue> Serialize(v8::Isolate*,
                                                        v8::Local<v8::Value>,
                                                        const SerializeOptions&,
                                                        ExceptionState&);
  static scoped_refptr<SerializedScriptValue> SerializeAndSwallowExceptions(
      v8::Isolate*,
      v8::Local<v8::Value>);

  static scoped_refptr<SerializedScriptValue> Create();
  static scoped_refptr<SerializedScriptValue> Create(const String&);
  static scoped_refptr<SerializedScriptValue> Create(base::span<const uint8_t>);

  ~SerializedScriptValue();

  static scoped_refptr<SerializedScriptValue> NullValue();
  static scoped_refptr<SerializedScriptValue> UndefinedValue();

  String ToWireString() const;

  base::span<const uint8_t> GetWireData() const {
    return data_buffer_.as_span();
  }

  // Deserializes the value (in the current context). Returns a null value in
  // case of failure.
  struct DeserializeOptions {
    STACK_ALLOCATED();

   public:
    MessagePortArray* message_ports = nullptr;
    const WebBlobInfoArray* blob_info = nullptr;
  };
  v8::Local<v8::Value> Deserialize(v8::Isolate* isolate) {
    return Deserialize(isolate, DeserializeOptions());
  }
  v8::Local<v8::Value> Deserialize(v8::Isolate*, const DeserializeOptions&);

  // Takes ownership of a reference and creates an "unpacked" version of this
  // value, where the transferred contents have been turned into complete
  // objects local to this thread. A SerializedScriptValue can only be unpacked
  // once, and the result is bound to a thread.
  // See UnpackedSerializedScriptValue.h for more details.
  static UnpackedSerializedScriptValue* Unpack(
      scoped_refptr<SerializedScriptValue>);

  // Used for debugging. Returns true if there are "packed" transferred contents
  // which would require this value to be unpacked before deserialization.
  // See UnpackedSerializedScriptValue.h for more details.
  bool HasPackedContents() const;

  // Helper function which pulls the values out of a JS sequence and into a
  // MessagePortArray.  Also validates the elements per sections 4.1.13 and
  // 4.1.15 of the WebIDL spec and section 8.3.3 of the HTML5 spec and generates
  // exceptions as appropriate.
  // Returns true if the array was filled, or false if the passed value was not
  // of an appropriate type.
  static bool ExtractTransferables(v8::Isolate*,
                                   const HeapVector<ScriptObject>&,
                                   Transferables&,
                                   ExceptionState&);

  static ArrayBufferArray ExtractNonSharedArrayBuffers(Transferables&);

  // Helper function which pulls ArrayBufferContents out of an ArrayBufferArray
  // and neuters the ArrayBufferArray.  Returns nullptr if there is an
  // exception.
  static ArrayBufferContentsArray TransferArrayBufferContents(
      v8::Isolate*,
      const ArrayBufferArray&,
      ExceptionState&);

  static ImageBitmapContentsArray TransferImageBitmapContents(
      v8::Isolate*,
      const ImageBitmapArray&,
      ExceptionState&);

  // Informs V8 about external memory allocated and owned by this object.
  // Large values should contribute to GC counters to eventually trigger a GC,
  // otherwise flood of postMessage() can cause OOM.
  // Ok to invoke multiple times (only adds memory once).
  // The memory registration is revoked automatically in destructor.
  void RegisterMemoryAllocatedWithCurrentScriptContext();

  // The dual, unregistering / subtracting the external memory allocation costs
  // of this SerializedScriptValue with the current context. This includes
  // discounting the cost of the transferables.
  //
  // The value is updated and marked as having no allocations registered,
  // hence subsequent calls will be no-ops.
  void UnregisterMemoryAllocatedWithCurrentScriptContext();

  const uint8_t* Data() const { return data_buffer_.data(); }
  size_t DataLengthInBytes() const { return data_buffer_.size(); }

  TransferredWasmModulesArray& WasmModules() { return wasm_modules_; }
  SharedArrayBufferContentsArray& SharedArrayBuffersContents() {
    return shared_array_buffers_contents_;
  }
  BlobDataHandleMap& BlobDataHandles() { return blob_data_handles_; }
  FileSystemAccessTokensArray& FileSystemAccessTokens() {
    return file_system_access_tokens_;
  }
  MojoScopedHandleArray& MojoHandles() { return mojo_handles_; }
  ArrayBufferContentsArray& GetArrayBufferContentsArray() {
    return array_buffer_contents_array_;
  }
  void SetArrayBufferContentsArray(ArrayBufferContentsArray contents) {
    array_buffer_contents_array_ = std::move(contents);
  }
  ImageBitmapContentsArray& GetImageBitmapContentsArray() {
    return image_bitmap_contents_array_;
  }
  void SetImageBitmapContentsArray(ImageBitmapContentsArray contents);

  StreamArray& GetStreams() { return streams_; }

  const v8::SharedValueConveyor* MaybeGetSharedValueConveyor() const {
    return base::OptionalToPtr(shared_value_conveyor_);
  }

  bool IsLockedToAgentCluster() const;

  // Returns true after serializing script values that remote origins cannot
  // access.
  bool IsOriginCheckRequired() const;

  // Returns true if it is expected to be possible to deserialize this value in
  // the provided context. It might not be if, for instance, the value contains
  // interfaces not exposed in all realms.
  bool CanDeserializeIn(ExecutionContext*);

  // Testing hook to allow overriding whether a value can be deserialized in a
  // particular execution context. Callers are responsible for assuring thread
  // safety, and resetting this after the test.
  using CanDeserializeInCallback =
      base::RepeatingCallback<bool(const SerializedScriptValue&,
                                   ExecutionContext*,
                                   bool can_deserialize)>;
  static void OverrideCanDeserializeInForTesting(CanDeserializeInCallback);
  struct ScopedOverrideCanDeserializeInForTesting {
    explicit ScopedOverrideCanDeserializeInForTesting(
        CanDeserializeInCallback callback) {
      OverrideCanDeserializeInForTesting(std::move(callback));
    }
    ~ScopedOverrideCanDeserializeInForTesting() {
      OverrideCanDeserializeInForTesting({});
    }
  };

  // Derive from Attachments to define collections of objects to serialize in
  // modules. They can be registered using GetOrCreateAttachment().
  class Attachment {
   public:
    virtual ~Attachment() = default;
    virtual bool IsLockedToAgentCluster() const = 0;
  };

  template <typename T>
  T* GetOrCreateAttachment() {
    auto result = attachments_.insert(&T::kAttachmentKey, std::unique_ptr<T>());
    if (!result.stored_value->value)
      result.stored_value->value = std::make_unique<T>();
    return static_cast<T*>(result.stored_value->value.get());
  }

  template <typename T>
  const T* GetAttachmentIfExists() const {
    auto it = attachments_.find(&T::kAttachmentKey);
    if (it == attachments_.end())
      return nullptr;
    return static_cast<T*>(it->value.get());
  }

  struct BufferDeleter {
    void operator()(uint8_t* buffer) { WTF::Partitions::BufferFree(buffer); }
  };
  using DataBufferPtr = base::HeapArray<uint8_t, BufferDeleter>;

  // Takes ownership rather than copying.
  static scoped_refptr<SerializedScriptValue> Create(
      DataBufferPtr&& data_buffer);

  static DataBufferPtr AllocateBuffer(size_t);

  // Called to take ownership of `data_buffer_` and destroy `this`.
  // This enforces that there are no other references to `this`.
  DataBufferPtr ConsumeAndTakeBuffer() &&;

 private:
  friend class ScriptValueSerializer;
  friend class V8ScriptValueSerializer;
  friend class UnpackedSerializedScriptValue;

  SerializedScriptValue();
  explicit SerializedScriptValue(DataBufferPtr);

  void SetData(DataBufferPtr data) { data_buffer_ = std::move(data); }

  void TransferArrayBuffers(v8::Isolate*,
                            const ArrayBufferArray&,
                            ExceptionState&);
  void TransferImageBitmaps(v8::Isolate*,
                            const ImageBitmapArray&,
                            ExceptionState&);
  void TransferOffscreenCanvas(v8::Isolate*,
                               const OffscreenCanvasArray&,
                               ExceptionState&);
  void TransferReadableStreams(ScriptState*,
                               const ReadableStreamArray&,
                               ExceptionState&);
  void TransferReadableStream(ScriptState* script_state,
                              ExecutionContext* execution_context,
                              ReadableStream* readable_streams,
                              ExceptionState& exception_state);
  void TransferWritableStreams(ScriptState*,
                               const WritableStreamArray&,
                               ExceptionState&);
  void TransferWritableStream(ScriptState* script_state,
                              ExecutionContext* execution_context,
                              WritableStream* writable_streams,
                              ExceptionState& exception_state);
  void TransferTransformStreams(ScriptState*,
                                const TransformStreamArray&,
                                ExceptionState&);
  MessagePort* AddStreamChannel(ExecutionContext*);

  void CloneSharedArrayBuffers(SharedArrayBufferArray&);

  DataBufferPtr data_buffer_;
  size_t data_buffer_size_ = 0;

  // These two have one-use transferred contents, and are stored in
  // UnpackedSerializedScriptValue thereafter.
  ArrayBufferContentsArray array_buffer_contents_array_;
  ImageBitmapContentsArray image_bitmap_contents_array_;

  // |streams_| is also single-use but is special-cased because it works
  // with ServiceWorkers.
  StreamArray streams_;

  // These do not have one-use transferred contents, like the above.
  TransferredWasmModulesArray wasm_modules_;
  BlobDataHandleMap blob_data_handles_;
  MojoScopedHandleArray mojo_handles_;
  SharedArrayBufferContentsArray shared_array_buffers_contents_;
  FileSystemAccessTokensArray file_system_access_tokens_;
  HashMap<const void* const*, std::unique_ptr<Attachment>> attachments_;

  std::optional<v8::SharedValueConveyor> shared_value_conveyor_;
  raw_ptr<v8::Isolate> isolate_;
  bool has_registered_external_allocation_;
#if DCHECK_IS_ON()
  bool was_unpacked_ = false;
#endif
  NO_UNIQUE_ADDRESS V8ExternalMemoryAccounterBase external_memory_accounter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZED_SCRIPT_VALUE_H_
