// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_DATA_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_DATA_LOADER_H_

#include "base/notreached.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class BytesConsumer;
class FormData;
class TextResourceDecoderOptions;

// FetchDataLoader subclasses
// 1. take a BytesConsumer,
// 2. read all data, and
// 3. call either DidFetchDataLoaded...() on success or
//    DidFetchDataLoadFailed() otherwise
//    on the thread where FetchDataLoader is created.
//
// - Client's methods can be called synchronously in Start().
// - If FetchDataLoader::Cancel() is called, Client's methods will not be
//   called anymore.
class CORE_EXPORT FetchDataLoader : public GarbageCollected<FetchDataLoader> {
 public:
  class CORE_EXPORT Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() {}

    // The method corresponding to createLoaderAs... is called on success.
    virtual void DidFetchDataLoadedBlobHandle(scoped_refptr<BlobDataHandle>) {
      NOTREACHED();
    }
    virtual void DidFetchDataLoadedArrayBuffer(DOMArrayBuffer*) {
      NOTREACHED();
    }
    virtual void DidFetchDataLoadedFormData(FormData*) { NOTREACHED(); }
    virtual void DidFetchDataLoadedString(const String&) { NOTREACHED(); }
    // This is called synchronously from FetchDataLoader::Start() to provide
    // the target data pipe.  This may be a pipe extracted from the consumer
    // or a new pipe that data will be copied into.
    virtual void DidFetchDataStartedDataPipe(
        mojo::ScopedDataPipeConsumerHandle handle) {
      NOTREACHED();
    }
    // This is called after all data are read from |handle| and written
    // to |out_data_pipe|, and |out_data_pipe| is closed or aborted.
    // This may be called synchronously from FetchDataLoader::Start() or
    // delayed to a later task.
    virtual void DidFetchDataLoadedDataPipe() { NOTREACHED(); }

    // This function is called when a "custom" FetchDataLoader (none of the
    // ones listed above) finishes loading.
    virtual void DidFetchDataLoadedCustomFormat() { NOTREACHED(); }

    virtual void DidFetchDataLoadFailed() = 0;

    // This function is called when an abort has been signalled.
    virtual void Abort() = 0;

    void Trace(Visitor* visitor) const override {}
  };

  // The task runner is used to post tasks necessary for creating a blob
  // from certain kinds of consumers.
  static FetchDataLoader* CreateLoaderAsBlobHandle(
      const String& mime_type,
      scoped_refptr<base::SingleThreadTaskRunner>);
  static FetchDataLoader* CreateLoaderAsArrayBuffer();
  static FetchDataLoader* CreateLoaderAsFailure();
  static FetchDataLoader* CreateLoaderAsFormData(
      const String& multipart_boundary);
  // The text resource decoder options should be created either by
  // TextResourceDecoderOptions::CreateUTF8Decode() or
  // TextResourceDecoderOptions::CreateUTF8DecodeWithoutBOM().
  static FetchDataLoader* CreateLoaderAsString(
      const TextResourceDecoderOptions&);
  // The mojo::DataPipe consumer handle is provided via the
  // Client::DidFetchStartedDataPipe() callback.
  static FetchDataLoader* CreateLoaderAsDataPipe(
      scoped_refptr<base::SingleThreadTaskRunner>);

  virtual ~FetchDataLoader() {}

  // |consumer| must not have a client when called.
  virtual void Start(BytesConsumer* /* consumer */, Client*) = 0;

  virtual void Cancel() = 0;

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_DATA_LOADER_H_
