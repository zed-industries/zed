// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_DECODER_H_

#include <memory>
#include <optional>
#include <tuple>

#include "base/location.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/data_pipe_drainer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/crypto.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ResponseBodyLoaderClient;
class ScriptDecoder;
class ScriptDecoderWithClient;
class TextResourceDecoder;

struct CORE_EXPORT ScriptDecoderDeleter {
  void operator()(const ScriptDecoder* ptr);
};
using ScriptDecoderPtr = std::unique_ptr<ScriptDecoder, ScriptDecoderDeleter>;

// ScriptDecoder decodes and hashes the script source on a worker thread. The
// OnDecodeFinishedCallback will receive the raw data and the decoded data.
// This class will be used for decoding script sources without a
// ResponseBodyLoaderClient when Blink receives the response on the background
// thread (BackgroundResourceFetch feature).
// TODO(crbug.com/328297266): Consider removing this class, and use the decoded
// text by V8.
class CORE_EXPORT ScriptDecoder {
 public:
  class CORE_EXPORT Result {
   public:
    Result(SegmentedBuffer raw_data,
           String decoded_data,
           std::unique_ptr<ParkableStringImpl::SecureDigest> digest);
    ~Result() = default;

    Result(const Result&) = delete;
    Result& operator=(const Result&) = delete;

    Result(Result&&) = default;
    Result& operator=(Result&&) = default;

    SegmentedBuffer raw_data;
    String decoded_data;
    std::unique_ptr<ParkableStringImpl::SecureDigest> digest;
  };
  using OnDecodeFinishedCallback = CrossThreadOnceFunction<void(Result)>;

  // Creates a ScriptDecoder.
  static ScriptDecoderPtr Create(
      std::unique_ptr<TextResourceDecoder> decoder,
      scoped_refptr<base::SequencedTaskRunner> client_task_runner);

  ~ScriptDecoder() = default;

  ScriptDecoder(const ScriptDecoder&) = delete;
  ScriptDecoder& operator=(const ScriptDecoder&) = delete;

  void DidReceiveData(Vector<char> data);
  void FinishDecode(OnDecodeFinishedCallback on_decode_finished_callback);

 private:
  friend struct ScriptDecoderDeleter;

  ScriptDecoder(std::unique_ptr<TextResourceDecoder> decoder,
                scoped_refptr<base::SequencedTaskRunner> client_task_runner);

  void Delete() const;

  void AppendData(const String& data);

  std::unique_ptr<TextResourceDecoder> decoder_;
  Digestor digestor_{kHashAlgorithmSha256};
  scoped_refptr<base::SequencedTaskRunner> client_task_runner_;
  scoped_refptr<base::SequencedTaskRunner> decoding_task_runner_;
  StringBuilder builder_;

  SegmentedBuffer raw_data_;
};

class DataPipeScriptDecoder;
struct CORE_EXPORT DataPipeScriptDecoderDeleter {
  void operator()(const DataPipeScriptDecoder* ptr);
};

using DataPipeScriptDecoderPtr =
    std::unique_ptr<DataPipeScriptDecoder, DataPipeScriptDecoderDeleter>;

// DataPipeScriptDecoder decodes and hashes the script source of a Mojo data
// pipe on a worker thread. The OnDecodeFinishedCallback will receive the raw
// data and the decoded data.
// Currently this class is used only when BackgroundCodeCacheDecoderStart
// feature is enabled.
class CORE_EXPORT DataPipeScriptDecoder final
    : public mojo::DataPipeDrainer::Client {
 public:
  using OnDecodeFinishedCallback =
      CrossThreadOnceFunction<void(ScriptDecoder::Result)>;

  // Creates a DataPipeScriptDecoder.
  static DataPipeScriptDecoderPtr Create(
      std::unique_ptr<TextResourceDecoder> decoder,
      scoped_refptr<base::SequencedTaskRunner> client_task_runner,
      OnDecodeFinishedCallback on_decode_finished_callback);

  void Start(mojo::ScopedDataPipeConsumerHandle source);

 private:
  friend struct DataPipeScriptDecoderDeleter;

  DataPipeScriptDecoder(
      std::unique_ptr<TextResourceDecoder> decoder,
      scoped_refptr<base::SequencedTaskRunner> client_task_runner,
      OnDecodeFinishedCallback on_decode_finished_callback);
  void Delete() const;

  // implements mojo::DataPipeDrainer::Client
  void OnDataAvailable(base::span<const uint8_t> data) override;
  void OnDataComplete() override;

  void AppendData(const String& data);

  std::unique_ptr<TextResourceDecoder> decoder_;
  scoped_refptr<base::SequencedTaskRunner> client_task_runner_;
  OnDecodeFinishedCallback on_decode_finished_callback_;
  scoped_refptr<base::SequencedTaskRunner> decoding_task_runner_;

  std::unique_ptr<mojo::DataPipeDrainer> drainer_;
  Digestor digestor_{kHashAlgorithmSha256};
  DigestValue digest_value_;
  StringBuilder builder_;

  SegmentedBuffer raw_data_;
};

struct CORE_EXPORT ScriptDecoderWithClientDeleter {
  void operator()(const ScriptDecoderWithClient* ptr);
};
using ScriptDecoderWithClientPtr =
    std::unique_ptr<ScriptDecoderWithClient, ScriptDecoderWithClientDeleter>;

// ScriptDecoderWithClient decodes and hashes the script source on a worker
// thread, and then forwards the data to the client on the client thread.
// TODO(crbug.com/328297266): Consider removing this class, and use the decoded
// text by V8.
class CORE_EXPORT ScriptDecoderWithClient {
 public:
  // Creates a ScriptDecoder.
  // - The `DidReceiveData()` method `response_body_loader_client` will be
  //   called with the received data on the client thread.
  // - The `DidReceiveDecodedData()` method `response_body_loader_client` will
  //   be called with the decoded body data on the client thread.
  static ScriptDecoderWithClientPtr Create(
      ResponseBodyLoaderClient* response_body_loader_client,
      std::unique_ptr<TextResourceDecoder> decoder,
      scoped_refptr<base::SequencedTaskRunner> client_task_runner);

  ~ScriptDecoderWithClient() = default;

  ScriptDecoderWithClient(const ScriptDecoderWithClient&) = delete;
  ScriptDecoderWithClient& operator=(const ScriptDecoderWithClient&) = delete;

  void DidReceiveData(Vector<char> data, bool send_to_client);
  void FinishDecode(CrossThreadOnceClosure main_thread_continuation);

 private:
  friend struct ScriptDecoderWithClientDeleter;

  ScriptDecoderWithClient(
      ResponseBodyLoaderClient* response_body_loader_client,
      std::unique_ptr<TextResourceDecoder> decoder,
      scoped_refptr<base::SequencedTaskRunner> client_task_runner);

  void Delete() const;

  void AppendData(const String& data);

  std::unique_ptr<TextResourceDecoder> decoder_;
  Digestor digestor_{kHashAlgorithmSha256};
  scoped_refptr<base::SequencedTaskRunner> client_task_runner_;
  scoped_refptr<base::SequencedTaskRunner> decoding_task_runner_;
  StringBuilder builder_;

  CrossThreadWeakHandle<ResponseBodyLoaderClient> response_body_loader_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_DECODER_H_
