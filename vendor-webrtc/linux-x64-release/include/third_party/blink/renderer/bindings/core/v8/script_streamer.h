// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_STREAMER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_STREAMER_H_

#include <memory>
#include <tuple>

#include "base/check_op.h"
#include "base/synchronization/waitable_event.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "third_party/blink/public/mojom/script/script_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_decoder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/parser/text_resource_decoder.h"
#include "third_party/blink/renderer/core/script/script_scheduling_type.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/background_response_processor.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "v8/include/v8.h"

namespace mojo {
class SimpleWatcher;
}

namespace blink {

namespace v8_compile_hints {
class CompileHintsForStreaming;
class V8LocalCompileHintsConsumer;
}  // namespace v8_compile_hints

class ScriptResource;
class SourceStream;
class ResponseBodyLoaderClient;

// Base class for streaming scripts. Subclasses should expose streamed script
// data by overriding the Source() method.
class CORE_EXPORT ScriptStreamer : public GarbageCollected<ScriptStreamer> {
 public:
  // For tracking why some scripts are not streamed. Not streaming is part of
  // normal operation (e.g., script already loaded, script too small) and
  // doesn't necessarily indicate a failure.
  enum class NotStreamingReason {
    kAlreadyLoaded,  // DEPRECATED
    kNotHTTP,
    kRevalidate,
    kContextNotValid,
    kEncodingNotSupported,
    kThreadBusy,  // DEPRECATED
    kV8CannotStream,
    kScriptTooSmall,
    kNoResourceBuffer,
    kHasCodeCache,
    kStreamerNotReadyOnGetSource,  // DEPRECATED
    kInlineScript,
    kDidntTryToStartStreaming,  // DEPRECATED
    kErrorOccurred,
    kStreamingDisabled,
    kSecondScriptResourceUse,
    kWorkerTopLevelScript,
    kModuleScript,
    kNoDataPipe,
    kLoadingCancelled,
    kNonJavascriptModule,
    kDisabledByFeatureList,
    kErrorScriptTypeMismatch,
    kBackgroundResponseProcessorWillBeUsed,
    // Following codes are used by BackgroundResourceScriptStreamer
    kNonJavascriptModuleBackground,
    kHasCodeCacheBackground,
    kScriptTooSmallBackground,
    kErrorOccurredBackground,
    kEncodingNotSupportedBackground,

    // Pseudo values that should never be seen in reported metrics
    kMaxValue = kEncodingNotSupportedBackground,
    kInvalid = -1,
  };

  virtual ~ScriptStreamer() = default;

  virtual v8::ScriptCompiler::StreamedSource* Source(
      v8::ScriptType expected_type) = 0;
  virtual bool IsStreamingSuppressed() const = 0;
  virtual NotStreamingReason StreamingSuppressedReason() const = 0;
  virtual v8::ScriptType GetScriptType() const = 0;
  virtual void Trace(Visitor*) const {}

  static void RecordStreamingHistogram(ScriptSchedulingType type,
                                       bool can_use_streamer,
                                       ScriptStreamer::NotStreamingReason);

  // Returns false if we cannot stream the given encoding.
  static bool ConvertEncoding(const AtomicString& encoding_name,
                              v8::ScriptCompiler::StreamedSource::Encoding*);

  // Get a successful ScriptStreamer for the given ScriptResource.
  // If
  // - there was no streamer,
  // - or streaming was suppressed,
  // - or the expected_type does not match the one with which the ScriptStreamer
  //    was started,
  // nullptr instead of a valid ScriptStreamer is returned.
  static std::tuple<ScriptStreamer*, NotStreamingReason> TakeFrom(
      ScriptResource* resource,
      mojom::blink::ScriptType expected_type);
};

// ResourceScriptStreamer streams incomplete script data to V8 so that it can be
// parsed while it's loaded. ScriptResource holds a reference to
// ResourceScriptStreamer. If the Document and the ClassicPendingScript are
// destroyed while the streaming is in progress, and ScriptStreamer handles it
// gracefully.
class CORE_EXPORT ResourceScriptStreamer final : public ScriptStreamer {
  USING_PRE_FINALIZER(ResourceScriptStreamer, Prefinalize);

 public:
  ResourceScriptStreamer(
      ScriptResource* resource,
      mojo::ScopedDataPipeConsumerHandle data_pipe,
      ResponseBodyLoaderClient* response_body_loader_client,
      std::unique_ptr<TextResourceDecoder> decoder,
      scoped_refptr<base::SingleThreadTaskRunner> loading_task_runner);

  ResourceScriptStreamer(const ResourceScriptStreamer&) = delete;
  ResourceScriptStreamer& operator=(const ResourceScriptStreamer&) = delete;

  ~ResourceScriptStreamer() override;
  void Trace(Visitor*) const override;

  bool IsStreamingStarted() const;     // Have we actually started streaming?
  bool CanStartStreaming() const;      // Can we still start streaming later?
  bool IsLoaded() const;               // Has loading finished?
  bool IsFinished() const;             // Has loading & streaming finished?

  // ScriptStreamer implementation:
  v8::ScriptCompiler::StreamedSource* Source(
      v8::ScriptType expected_type) override {
    DCHECK_EQ(expected_type, script_type_);
    DCHECK(IsFinished());
    DCHECK(!IsStreamingSuppressed());
    return source_.get();
  }
  bool IsStreamingSuppressed() const override;
  NotStreamingReason StreamingSuppressedReason() const override {
    CheckState();
    return suppressed_reason_;
  }
  v8::ScriptType GetScriptType() const override;

  // Called when the script is not needed any more (e.g., loading was
  // cancelled). After calling cancel, ClassicPendingScript can drop its
  // reference to ScriptStreamer, and ScriptStreamer takes care of eventually
  // deleting itself (after the V8 side has finished too).
  void Cancel();

  const String& ScriptURLString() const { return script_url_string_; }
  uint64_t ScriptResourceIdentifier() const {
    return script_resource_identifier_;
  }

  v8_compile_hints::V8LocalCompileHintsConsumer*
  GetV8LocalCompileHintsConsumerForTest() const;

 private:
  friend class SourceStream;

  // Valid loading state transitions:
  //
  //               kLoading
  //          .--------|---------.
  //          |        |         |
  //          v        v         v
  //      kLoaded   kFailed  kCancelled
  enum class LoadingState { kLoading, kLoaded, kFailed, kCancelled };

  static const char* str(LoadingState state) {
    switch (state) {
      case LoadingState::kLoading:
        return "Loading";
      case LoadingState::kLoaded:
        return "Loaded";
      case LoadingState::kFailed:
        return "Failed";
      case LoadingState::kCancelled:
        return "Cancelled";
    }
  }

  // Maximum size of the BOM marker. Scripts whose first data chunk is smaller
  // than this constant won't be streamed.
  static constexpr size_t kMaximumLengthOfBOM = 4;

  static void RunScriptStreamingTask(
      std::unique_ptr<v8::ScriptCompiler::ScriptStreamingTask> task,
      ResourceScriptStreamer* streamer,
      SourceStream* stream);

  void OnDataPipeReadable(MojoResult result,
                          const mojo::HandleSignalsState& state);

  // Given the data we have collected already, try to start an actual V8
  // streaming task. Returns true if the task was started.
  bool TryStartStreamingTask();

  void Prefinalize();

  // When the streaming is suppressed, the data is not given to V8, but
  // ScriptStreamer still watches the resource load and notifies the upper
  // layers when loading is finished. It is used in situations when we have
  // started streaming but then we detect we don't want to stream (e.g., when
  // we have the code cache for the script) and we still want to parse and
  // execute it when it has finished loading.
  void SuppressStreaming(NotStreamingReason reason);

  // Called by ScriptStreamingTask when it has streamed all data to V8 and V8
  // has processed it.
  void StreamingCompleteOnBackgroundThread(LoadingState loading_state);

  // The four methods below should not be called synchronously, as they can
  // trigger script resource client callbacks.

  // Streaming completed with loading in the given `state`.
  void StreamingComplete(LoadingState loading_state);
  // Loading completed in the given state, without ever starting streaming.
  void LoadCompleteWithoutStreaming(LoadingState loading_state,
                                    NotStreamingReason no_streaming_reason);
  // Helper for the above methods to notify the client that loading has
  // completed in the given state. Streaming is guaranteed to either have
  // completed or be suppressed.
  void SendClientLoadFinishedCallback();

  // Has the script streamer been detached from its client. If true, then we can
  // safely abort loading and not output any more data.
  bool IsClientDetached() const;

  void AdvanceLoadingState(LoadingState new_state);
  void CheckState() const;

  LoadingState loading_state_ = LoadingState::kLoading;

  Member<ScriptResource> script_resource_;
  Member<ResponseBodyLoaderClient> response_body_loader_client_;

  ScriptDecoderWithClientPtr script_decoder_;

  // Fields active during asynchronous (non-streaming) reads.
  mojo::ScopedDataPipeConsumerHandle data_pipe_;
  std::unique_ptr<mojo::SimpleWatcher> watcher_;

  // Fields active during streaming.
  SourceStream* stream_ = nullptr;
  std::unique_ptr<v8::ScriptCompiler::StreamedSource> source_;

  // The reason that streaming is disabled
  NotStreamingReason suppressed_reason_ = NotStreamingReason::kInvalid;

  // Keep the script URL string for event tracing.
  const String script_url_string_;

  // Keep the script resource dentifier for event tracing.
  const uint64_t script_resource_identifier_;

  // Encoding of the streamed script. Saved for sanity checking purposes.
  v8::ScriptCompiler::StreamedSource::Encoding encoding_;

  const v8::ScriptType script_type_;

  // For transmitting crowdsourced or local compile hints to V8 while streaming.
  std::unique_ptr<v8_compile_hints::CompileHintsForStreaming> compile_hints_;
};

// BackgroundInlineScriptStreamer allows parsing and compiling inline scripts in
// the background before they have been parsed by the HTML parser. Use
// InlineScriptStreamer::From() to create a ScriptStreamer from this class.
class CORE_EXPORT BackgroundInlineScriptStreamer final
    : public WTF::ThreadSafeRefCounted<BackgroundInlineScriptStreamer> {
 public:
  BackgroundInlineScriptStreamer(
      v8::Isolate* isolate,
      const String& text,
      v8::ScriptCompiler::CompileOptions compile_options);

  void Run();
  bool IsStarted() const { return started_.IsSet(); }
  void Cancel() { cancelled_.Set(); }

  // This may return false if V8 failed to create a background streaming task.
  bool CanStream() const { return task_.get(); }

  v8::ScriptCompiler::StreamedSource* Source(v8::ScriptType expected_type);

 private:
  friend class WTF::ThreadSafeRefCounted<BackgroundInlineScriptStreamer>;
  ~BackgroundInlineScriptStreamer() = default;

  std::unique_ptr<v8::ScriptCompiler::StreamedSource> source_;
  std::unique_ptr<v8::ScriptCompiler::ScriptStreamingTask> task_;
  base::WaitableEvent event_;
  base::AtomicFlag started_;
  base::AtomicFlag cancelled_;
};

// ScriptStreamer is garbage collected so must be created on the main thread.
// This class wraps a BackgroundInlineScriptStreamer to be used on the main
// thread.
class CORE_EXPORT InlineScriptStreamer final : public ScriptStreamer {
 public:
  static InlineScriptStreamer* From(
      scoped_refptr<BackgroundInlineScriptStreamer> streamer);

  explicit InlineScriptStreamer(
      scoped_refptr<BackgroundInlineScriptStreamer> streamer)
      : streamer_(std::move(streamer)) {}

  v8::ScriptCompiler::StreamedSource* Source(
      v8::ScriptType expected_type) override {
    return streamer_->Source(expected_type);
  }
  bool IsStreamingSuppressed() const override { return false; }
  NotStreamingReason StreamingSuppressedReason() const override {
    return NotStreamingReason::kInvalid;
  }
  v8::ScriptType GetScriptType() const override {
    return v8::ScriptType::kClassic;
  }

  void Trace(Visitor* visitor) const override {
    ScriptStreamer::Trace(visitor);
  }

 private:
  scoped_refptr<BackgroundInlineScriptStreamer> streamer_;
};

// BackgroundResourceScriptStreamer allows starting the script parser from the
// background thread of BackgroundURLLoader. MaybeStartProcessingResponse()
// method of `background_processor_` is called by the BackgroundURLLoader on the
// background thread, and triggers the script parser on another background
// thread.
class CORE_EXPORT BackgroundResourceScriptStreamer : public ScriptStreamer {
 public:
  // This is an utility structure to hold the decoded data and the streamed
  // source or consume code cache task which are passed from the background
  // thread to the main thread.
  class CORE_EXPORT Result {
   public:
    Result(String decoded_data,
           std::unique_ptr<ParkableStringImpl::SecureDigest> digest,
           std::unique_ptr<v8::ScriptCompiler::StreamedSource> streamed_source);
    Result(String decoded_data,
           std::unique_ptr<ParkableStringImpl::SecureDigest> digest,
           std::unique_ptr<v8::ScriptCompiler::ConsumeCodeCacheTask>
               consume_code_cache_task);
    ~Result() = default;

    Result(const Result&) = delete;
    Result& operator=(const Result&) = delete;

    Result(Result&&) = default;
    Result& operator=(Result&&) = default;

    String decoded_data;
    std::unique_ptr<ParkableStringImpl::SecureDigest> digest;
    std::unique_ptr<v8::ScriptCompiler::StreamedSource> streamed_source;
    std::unique_ptr<v8::ScriptCompiler::ConsumeCodeCacheTask>
        consume_code_cache_task;
  };

  explicit BackgroundResourceScriptStreamer(ScriptResource* script_resource);
  BackgroundResourceScriptStreamer(const BackgroundResourceScriptStreamer&) =
      delete;
  BackgroundResourceScriptStreamer& operator=(
      const BackgroundResourceScriptStreamer&) = delete;

  ~BackgroundResourceScriptStreamer() override;
  void Trace(Visitor*) const override;

  // ScriptStreamer implementation:
  v8::ScriptCompiler::StreamedSource* Source(
      v8::ScriptType expected_type) override;
  bool IsStreamingSuppressed() const override {
    return suppressed_reason_ != NotStreamingReason::kInvalid;
  }
  NotStreamingReason StreamingSuppressedReason() const override {
    return suppressed_reason_;
  }
  v8::ScriptType GetScriptType() const override;

  std::unique_ptr<BackgroundResponseProcessorFactory>
  CreateBackgroundResponseProcessorFactory();

  bool HasDecodedData() const { return !!result_; }
  bool HasConsumeCodeCacheTask() const {
    return result_ && result_->consume_code_cache_task;
  }

  ParkableString TakeDecodedData();
  std::unique_ptr<v8::ScriptCompiler::ConsumeCodeCacheTask>
  TakeConsumeCodeCacheTask();

 private:
  class BackgroundProcessor;
  class BackgroundProcessorFactory;

  void OnResult(std::unique_ptr<Result> result,
                NotStreamingReason suppressed_reason);

  Member<ScriptResource> script_resource_;
  const v8::ScriptType script_type_;

  std::unique_ptr<Result> result_;
  // The reason that streaming is disabled
  NotStreamingReason suppressed_reason_ = NotStreamingReason::kInvalid;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_STREAMER_H_
