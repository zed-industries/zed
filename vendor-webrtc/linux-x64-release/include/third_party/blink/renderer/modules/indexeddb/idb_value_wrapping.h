// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_VALUE_WRAPPING_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_VALUE_WRAPPING_H_

#include <memory>
#include <utility>

#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "base/feature_list.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/platform/web_blob_info.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

class BlobDataHandle;
class ExceptionState;
class IDBValue;
class ScriptState;
class ScriptValue;
class SerializedScriptValue;

// Logic for serializing V8 values for storage in IndexedDB.
//
// An IDBValueWrapper instance drives the serialization of a single V8 value to
// IndexedDB. An instance's lifecycle goes through the following stages:
// 1) Cloning - Right after an instance is constructed, its internal
//    representation is optimized for structured cloning via the Clone() method.
//    This may be necessary when extracting the primary key and/or index keys
//    for the serialized value.
// 2) Wrapping - DoneCloning() transitions the instance to an internal
//    representation optimized IPC and disk storage. See below for details.
// 3) Reading results - After any desired wrapping is performed, the Take*()
//    methods yield the serialized value components passed to the backing store.
//    To avoid unnecessary copies, the Take*() methods move out parts of the
//    internal representation, so each Take*() method can be called at most
//    once.
//
// Example usage:
//     IDBValueWrapper wrapper;
//     wrapper.Clone(...);  // Structured clone used to extract keys.
//     wrapper.DoneCloning();
//     std::unique_ptr<IDBValue> value = std::move(wrapper).Build();
//
// V8 values are first serialized via SerializedScriptValue (SSV), which is
// essentially a byte array plus an array of attached Blobs. The SSV output's
// byte array is then further compressed via Snappy. If the compressed array is
// not too large, it will be stored directly in IndexedDB's backing store,
// together with references to the attached Blobs.
//
// Values that are still "large" after compression are converted into a Blob
// (additional to those already attached). Specifically, the byte array in the
// SSV output is replaced with a "wrapped value" marker, and stored inside a
// Blob that is tacked to the end of the SSV's Blob array. IndexedDB's backing
// store receives the "wrapped value" marker and the references to the Blobs,
// while the large byte array in the SSV output is handled by the Blob storage
// system.
//
// In summary:
// "normal" v8::Value -> SSV + Snappy -> IDBValue (stores SSV output) -> LevelDB
// "large" v8::Value -> SSV + Snappy -> IDBValue (stores SSV output) ->
//     Blob (stores SSV output) + IDBValue (stores Blob reference) -> LevelDB
//
// Full picture that accounts for Blob attachments:
// "normal" v8::Value -> SSV (byte array, Blob attachments) -> Snappy ->
//     IDBValue (bytes: compressed SSV byte array, blobs: SSV Blob attachments)
//     -> LevelDB
// "large" v8::Value -> SSV (byte array, Blob attachments) -> Snappy ->
//     IDBValue (bytes: "wrapped value" marker,
//               blobs: SSV Blob attachments +
//                      [wrapper Blob(compressed SSV byte array)] ->
//     LevelDB
class MODULES_EXPORT IDBValueWrapper {
  DISALLOW_NEW();

 public:
  // Wrapper for an IndexedDB value.
  //
  // The serialization process can throw an exception. The caller is responsible
  // for checking exception_state.
  //
  // The wrapper's internal representation is optimized for cloning the
  // serialized value. DoneCloning() must be called to transition to an internal
  // representation optimized for writing.
  IDBValueWrapper(
      v8::Isolate*,
      v8::Local<v8::Value>,
      SerializedScriptValue::SerializeOptions::WasmSerializationPolicy,
      ExceptionState&);
  ~IDBValueWrapper();

  // Creates a clone of the serialized value.
  //
  // This method is used to fulfill the IndexedDB specification requirement that
  // a value's key and index keys are extracted from a structured clone of the
  // value, which avoids the issue of side-effects in custom getters.
  //
  // This method cannot be called after DoneCloning().
  void Clone(ScriptState*, ScriptValue* clone);

  // Optimizes the serialized value's internal representation for writing to
  // disk.
  //
  // This must be called before Take*() methods can be called. After this method
  // is called, Clone() cannot be called anymore.
  void DoneCloning();

  // Transfers ownership of the serialized byte array, blob infos, and FSA
  // tokens to the returned `IDBValue`.
  //
  // This method must be called at most once, and must be called after
  // WrapIfBiggerThan().
  std::unique_ptr<IDBValue> Build() &&;

  size_t DataLengthBeforeWrapInBytes() { return original_data_length_; }

  // MIME type used for Blobs that wrap IDBValues.
  static constexpr const char* kWrapMimeType =
      "application/vnd.blink-idb-value-wrapper";

  // Used to serialize the wrapped value. Exposed for testing.
  static void WriteVarInt(unsigned value, Vector<char>& output);

  void set_wrapping_threshold_for_test(unsigned threshold) {
    wrapping_threshold_override_ = threshold;
  }

  void set_compression_threshold_for_test(size_t threshold) {
    compression_threshold_override_ = threshold;
  }

 private:
  // Evaluates if the specified uncompressed length merits a compression
  // attempt.
  bool ShouldCompress(size_t uncompressed_length) const;
  // Tries to compress `wire_bytes_` via Snappy, storing the output in
  // `wire_data_buffer_`. If the compression effect is small, the compression
  // will be discarded and an uncompressed value will be stored in
  // `wire_data_buffer_` (mainly to avoid an extra memory allocation when later
  // reading the value).
  void MaybeCompress();

  // Stores `wire_bytes_` in a Blob if it is over the size threshold.
  void MaybeStoreInBlob();

  // V8 value serialization state.
  scoped_refptr<SerializedScriptValue> serialized_value_;
  Vector<WebBlobInfo> blob_info_;

  // Buffer for wire data that is not stored in SerializedScriptValue.
  //
  // This buffer ends up storing metadata generated by wrapping operations.
  Vector<char> wire_data_buffer_;

  // Points into SerializedScriptValue's data buffer, or into wire_data_buffer_.
  // TODO(367764863) Rewrite to base::raw_span.
  RAW_PTR_EXCLUSION base::span<const uint8_t> wire_data_;

  size_t original_data_length_ = 0;

  std::optional<unsigned> wrapping_threshold_override_;
  std::optional<size_t> compression_threshold_override_;

#if DCHECK_IS_ON()
  // Accounting for lifecycle stages.
  bool had_exception_ = false;
  bool done_cloning_ = false;
  bool owns_wire_bytes_ = true;
#endif  // DCHECK_IS_ON()
};

// State and logic for unwrapping large IndexedDB values from Blobs.
//
// See IDBValueWrapper for an explanation of the wrapping concept.
//
// Once created, an IDBValueUnwrapper instance can be used to unwrap multiple
// Blobs. For each Blob to be unwrapped, the caller should first call Parse().
// If the method succeeds, the IDBValueUnwrapper will store the parse state,
// which can be obtained using WrapperBlobSize() and WrapperBlobHandle().
class MODULES_EXPORT IDBValueUnwrapper {
  STACK_ALLOCATED();

 public:
  IDBValueUnwrapper();

  // True if the IDBValue's data was wrapped in a Blob.
  static bool IsWrapped(IDBValue*);

  // True if at least one of the IDBValues' data was wrapped in a Blob.
  static bool IsWrapped(const Vector<std::unique_ptr<IDBValue>>&);

  // Unwraps an IDBValue that has wrapped Blob data, placing the result in
  // `wrapped_value`.
  static void Unwrap(Vector<char>&& wrapper_blob_content,
                     IDBValue& wrapped_value);

  // Decompresses the value in `buffer` and stores in one of the two provided
  // buffers (exactly one must be provided). Returns true on success.
  static bool Decompress(
      base::span<const uint8_t> buffer,
      Vector<char>* out_buffer,
      SerializedScriptValue::DataBufferPtr* out_buffer_in_place);

  // Parses the wrapper Blob information from a wrapped IDBValue.
  //
  // Returns true for success, and false for failure. Failure can mean that the
  // given value was not a wrapped IDBValue, or that the value bytes were
  // corrupted.
  bool Parse(IDBValue*);

  // Returns the size of the Blob obtained by the last Unwrap() call.
  //
  // Should only be called after a successful result from Unwrap().
  inline unsigned WrapperBlobSize() const { return blob_size_; }

  // Returns a handle to the Blob obtained by the last Unwrap() call.
  //
  // Should only be called exactly once after a successful result from Unwrap().
  scoped_refptr<BlobDataHandle> WrapperBlobHandle();

 private:
  friend class IDBValueUnwrapperReadTestHelper;

  // Used to deserialize the wrapped value.
  bool ReadVarInt(unsigned&);
  bool ReadBytes(Vector<uint8_t>&);

  // Resets the parsing state.
  bool Reset();

  // Deserialization cursor in the `data_` of the IDBValue being unwrapped.
  base::span<const uint8_t> parse_span_;

  // The size of the Blob holding the data for the last unwrapped IDBValue.
  unsigned blob_size_;

  // Handle to the Blob holding the data for the last unwrapped IDBValue.
  scoped_refptr<BlobDataHandle> blob_handle_;
};

// This flag controls behavior that decompresses
// `IDBValue::data_` directly into a buffer that's passed by ownership to
// `SerializedScriptValue`.
//
//  * For values that are not compressed, this flag has no effect: `data_` is
//  always copied on conversion to a script value.
//  * For values that are compressed,
//    * Normally `data_` will be decompressed the first time it's serialized,
//    overwriting `data_`, and *copied* into `SerializedScriptValue` the first
//    time and every subsequent time one is created.
//    * When this flag is enabled, `data_` will be decompressed *directly into*
//    a buffer that's passed off to `SerializedScriptValue`, which avoids a copy
//    and the memory overhead that entails. However this will happen every time
//    the value is deserialized, so if that happens more than once, the
//    decompression routine must run more than once.
MODULES_EXPORT BASE_DECLARE_FEATURE(kIdbDecompressValuesInPlace);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_VALUE_WRAPPING_H_
