// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BODY_STREAM_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BODY_STREAM_BUFFER_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/network/public/mojom/chunked_data_pipe_getter.mojom-blink.h"
#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/fetch/bytes_uploader.h"
#include "third_party/blink/renderer/core/fetch/fetch_data_loader.h"
#include "third_party/blink/renderer/core/streams/underlying_byte_source_base.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"

namespace blink {

class EncodedFormData;
class ExceptionState;
class ReadableStream;
class ScriptState;
class CachedMetadataHandler;

class CORE_EXPORT BodyStreamBuffer final
    : public UnderlyingByteSourceBase,
      public ExecutionContextLifecycleObserver,
      public BytesConsumer::Client {
 public:
  using PassKey = base::PassKey<BodyStreamBuffer>;

  // Create a BodyStreamBuffer for |consumer|.
  // |consumer| must not have a client.
  // This function must be called after entering an appropriate V8 context.
  // |signal| should be non-null when this BodyStreamBuffer is associated with a
  // Response that was created by fetch().
  static BodyStreamBuffer* Create(
      ScriptState*,
      BytesConsumer* consumer,
      AbortSignal* signal,
      CachedMetadataHandler* cached_metadata_handler,
      scoped_refptr<BlobDataHandle> side_data_blob = nullptr);

  // Create() should be used instead of calling this constructor directly.
  BodyStreamBuffer(PassKey,
                   ScriptState*,
                   BytesConsumer* consumer,
                   AbortSignal* signal,
                   CachedMetadataHandler* cached_metadata_handler,
                   scoped_refptr<BlobDataHandle> side_data_blob);

  BodyStreamBuffer(ScriptState*,
                   ReadableStream* stream,
                   CachedMetadataHandler* cached_metadata_handler,
                   scoped_refptr<BlobDataHandle> side_data_blob = nullptr);

  BodyStreamBuffer(const BodyStreamBuffer&) = delete;
  BodyStreamBuffer& operator=(const BodyStreamBuffer&) = delete;

  ReadableStream* Stream() { return stream_.Get(); }

  // Callable only when neither locked nor disturbed.
  scoped_refptr<BlobDataHandle> DrainAsBlobDataHandle(
      BytesConsumer::BlobSizePolicy,
      ExceptionState&);
  scoped_refptr<EncodedFormData> DrainAsFormData(ExceptionState&);
  void DrainAsChunkedDataPipeGetter(
      ScriptState*,
      mojo::PendingReceiver<network::mojom::blink::ChunkedDataPipeGetter>,
      BytesUploader::Client* client);
  // While loading is in progress, a SelfKeepAlive is used to prevent this
  // object from being garbage collected. If the context is destroyed, the
  // SelfKeepAlive is cleared. See https://crbug.com/1292744 for details.
  void StartLoading(FetchDataLoader*,
                    FetchDataLoader::Client* /* client */,
                    ExceptionState&);
  void Tee(BodyStreamBuffer**, BodyStreamBuffer**, ExceptionState&);

  // UnderlyingByteSourceBase
  ScriptPromise<IDLUndefined> Pull(ReadableByteStreamController* controller,
                                   ExceptionState&) override;
  ScriptPromise<IDLUndefined> Cancel() override;
  ScriptPromise<IDLUndefined> Cancel(v8::Local<v8::Value> reason) override;
  ScriptState* GetScriptState() override;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  // BytesConsumer::Client
  void OnStateChange() override;
  String DebugName() const override { return "BodyStreamBuffer"; }

  bool IsStreamReadable() const;
  bool IsStreamClosed() const;
  bool IsStreamErrored() const;
  bool IsStreamLocked() const;
  bool IsStreamDisturbed() const;

  // Closes the stream if necessary, and then locks and disturbs it. Should not
  // be called if |stream_broken_| is true.
  void CloseAndLockAndDisturb(ExceptionState&);

  bool IsAborted();

  // Returns the CachedMetadataHandler associated with the contents of this
  // stream. This can return nullptr. Streams' ownership model applies, so this
  // function is expected to be called by the owner of this stream.
  CachedMetadataHandler* GetCachedMetadataHandler() {
    DCHECK(!IsStreamLocked());
    DCHECK(!IsStreamDisturbed());
    return cached_metadata_handler_.Get();
  }

  // Take the blob representing any side data associated with this body
  // stream.  This must be called before the body is drained or begins
  // loading.
  scoped_refptr<BlobDataHandle> TakeSideDataBlob();
  scoped_refptr<BlobDataHandle> GetSideDataBlobForTest() const {
    return side_data_blob_;
  }

  bool IsMadeFromReadableStream() const { return made_from_readable_stream_; }

  void Trace(Visitor*) const override;

 private:
  friend class BodyStreamBufferUnderlyingByteSource;
  friend class BodyStreamBufferUnderlyingSource;

  class LoaderClient;

  // This method exists to avoid re-entrancy inside the BodyStreamBuffer
  // constructor. It is called by Create(). It should not be called after
  // using the ReadableStream* constructor.
  void Init();

  BytesConsumer* ReleaseHandle(ExceptionState&);
  void Abort();
  void Close(ExceptionState&);
  void GetError();
  void RaiseOOMError();
  void CancelConsumer();
  void ProcessData(ExceptionState&);
  void EndLoading();
  void StopLoading();

  Member<ScriptState> script_state_;
  Member<ReadableStream> stream_;
  Member<BytesUploader> stream_uploader_;
  Member<BytesConsumer> consumer_;
  // We need this member to keep it alive while loading.
  Member<FetchDataLoader> loader_;
  // We need this to ensure that we detect that abort has been signalled
  // correctly.
  Member<AbortSignal> signal_;
  // We need to keep the abort algorithms alive for the duration of the load.
  Member<AbortSignal::AlgorithmHandle> stream_buffer_abort_handle_;
  Member<AbortSignal::AlgorithmHandle> loader_client_abort_handle_;
  // CachedMetadata handler used for loading compiled WASM code.
  Member<CachedMetadataHandler> cached_metadata_handler_;
  // Additional side data associated with this body stream.  It should only be
  // retained until the body is drained or starts loading.  Client code, such
  // as service workers, can call TakeSideDataBlob() prior to consumption.
  scoped_refptr<BlobDataHandle> side_data_blob_;
  WebScopedVirtualTimePauser virtual_time_pauser_;
  bool stream_needs_more_ = false;
  bool made_from_readable_stream_;
  bool in_process_data_ = false;

  // TODO(ricea): Remove remaining uses of |stream_broken_|.
  bool stream_broken_ = false;

  // Used to remain alive when there's a loader_.
  SelfKeepAlive<BodyStreamBuffer> keep_alive_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BODY_STREAM_BUFFER_H_
