// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_REQUEST_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_REQUEST_LOADER_H_

#include <cstdint>
#include <memory>

#include "base/dcheck_is_on.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ref.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/fileapi/file_reader_client.h"
#include "third_party/blink/renderer/core/fileapi/file_reader_loader.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class IDBValue;

// Loads IndexedDB values that have been wrapped in Blobs by IDBValueWrapper.
//
// An IDBRequestLoader unwraps the result of a single IDBRequest. While most
// IndexedDB requests result in a single value, getAll() in IDBObjectStore and
// IDBIndex results in an array of values. In the interest of simplicity,
// IDBRequestLoader only knows how to unwrap an array of values, even though
// most of the time the array will consist of a single element. This design
// assumes that the overhead of creating and destroying a Vector is much smaller
// than the IPC overhead required to load the Blob data into the renderer.
class IDBRequestLoader : public GarbageCollected<IDBRequestLoader>,
                         public FileReaderClient {
 public:
  // |values| contains the unwrapped values; null |error| indicates success.
  using LoadCompleteCallback =
      base::OnceCallback<void(Vector<std::unique_ptr<IDBValue>>&& values,
                              DOMException* error)>;

  // Creates a loader that will unwrap IDBValues.
  //
  // Unwrapping of |values| is done in-place on |execution_context| and is
  // stopped if the context is destroyed. The unwrapped values are returned to
  // the caller through |load_complete_callback| or when `Cancel()` is called.
  IDBRequestLoader(Vector<std::unique_ptr<IDBValue>>&& values,
                   ExecutionContext* execution_context,
                   LoadCompleteCallback&& load_complete_callback);

  ~IDBRequestLoader() override;

  // Start unwrapping values.
  //
  // When the unwrapping completes, the loader will call
  // load_complete_callback_.
  void Start();
  // Halt the process of unwrapping values, if possible, and return the values
  // that were passed in the constructor.
  Vector<std::unique_ptr<IDBValue>>&& Cancel();

  // FileReaderClient implementation.
  FileErrorCode DidStartLoading(uint64_t) override;
  FileErrorCode DidReceiveData(base::span<const uint8_t> data) override;
  void DidFinishLoading() override;
  void DidFail(FileErrorCode error_code) override;
  void Trace(Visitor* visitor) const override {
    FileReaderClient::Trace(visitor);
    visitor->Trace(loader_);
    visitor->Trace(execution_context_);
  }

 private:
  // Starts unwrapping the next wrapped IDBValue.
  //
  // If no more wrapped IDBValues are found, this calls OnLoadComplete() which
  // calls load_complete_callback_.
  void StartNextValue();

  // Called when unwrapping of all values is complete.
  void OnLoadComplete(FileErrorCode error_code);

  Member<FileReaderLoader> loader_;

  // All the values to be unwrapped. These are moved back to the caller when
  // `load_complete_callback_` is run or when the load is canceled.
  Vector<std::unique_ptr<IDBValue>> values_;

  // The execution context on which blob data is read by `loader_`.
  WeakMember<ExecutionContext> execution_context_;

  // The callback to run when unwrapping is complete (successfully or not).
  LoadCompleteCallback load_complete_callback_;

  // Buffer used to unwrap an IDBValue.
  Vector<char> wrapped_data_;

  // The value being currently unwrapped.
  Vector<std::unique_ptr<IDBValue>>::iterator current_value_;

#if DCHECK_IS_ON()
  // True after Start() is called.
  bool started_ = false;

  // True after Cancel() is called.
  bool canceled_ = false;

  // True between a call to FileReaderLoader::Start() and the FileReaderLoader's
  // call to DidFinishLoading() or to DidFail().
  bool file_reader_loading_ = false;
#endif  // DCHECK_IS_ON()

  // The last time that this object started loading a wrapped blob.
  base::TimeTicks start_loading_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_REQUEST_LOADER_H_
