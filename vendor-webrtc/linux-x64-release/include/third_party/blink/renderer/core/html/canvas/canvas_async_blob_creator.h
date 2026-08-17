// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_ASYNC_BLOB_CREATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_ASYNC_BLOB_CREATOR_H_

#include <memory>

#include "base/location.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_blob_callback.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_image_encode_options.h"
#include "third_party/blink/renderer/core/canvas_interventions/canvas_interventions_helper.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/static_bitmap_image.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/image-encoders/image_encoder.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class ExecutionContext;
class ImageDataBuffer;

class CORE_EXPORT CanvasAsyncBlobCreator
    : public GarbageCollected<CanvasAsyncBlobCreator> {
 public:
  // This enum is used to back an UMA histogram, and should therefore be treated
  // as append-only. Idle tasks are not implemented for some image types.
  enum IdleTaskStatus {
    kIdleTaskNotStarted = 0,
    kIdleTaskStarted = 1,
    kIdleTaskCompleted = 2,
    kIdleTaskFailed = 3,
    kIdleTaskSwitchedToImmediateTask = 4,
    kIdleTaskNotSupported = 5,
    kMaxValue = kIdleTaskNotSupported,
  };
  enum ToBlobFunctionType {
    kHTMLCanvasToBlobCallback,
    kOffscreenCanvasConvertToBlobPromise
  };

  void ScheduleAsyncBlobCreation(const double& quality);

  CanvasAsyncBlobCreator(
      scoped_refptr<StaticBitmapImage>,
      const ImageEncodeOptions* options,
      ToBlobFunctionType function_type,
      base::TimeTicks start_time,
      ExecutionContext*,
      const IdentifiableToken& input_digest,
      CanvasInterventionsHelper::CanvasInterventionType intervention_type,
      ScriptPromiseResolver<Blob>*);
  CanvasAsyncBlobCreator(
      scoped_refptr<StaticBitmapImage>,
      const ImageEncodeOptions*,
      ToBlobFunctionType,
      V8BlobCallback*,
      base::TimeTicks start_time,
      ExecutionContext*,
      const IdentifiableToken& input_digest,
      CanvasInterventionsHelper::CanvasInterventionType intervention_type,
      ScriptPromiseResolver<Blob>* = nullptr);
  virtual ~CanvasAsyncBlobCreator();

  // Methods are virtual for mocking in unit tests
  virtual void SignalTaskSwitchInStartTimeoutEventForTesting() {}
  virtual void SignalTaskSwitchInCompleteTimeoutEventForTesting() {}

  virtual void Trace(Visitor*) const;

 protected:
  static ImageEncodeOptions* GetImageEncodeOptionsForMimeType(
      ImageEncodingMimeType);
  // Methods are virtual for unit testing
  virtual void ScheduleInitiateEncoding(double quality);
  virtual void IdleEncodeRows(base::TimeTicks deadline);
  virtual void PostDelayedTaskToCurrentThread(const base::Location&,
                                              base::OnceClosure,
                                              double delay_ms);
  virtual void SignalAlternativeCodePathFinishedForTesting() {}
  virtual void CreateBlobAndReturnResult(Vector<unsigned char> encoded_image);
  virtual void CreateNullAndReturnResult();

  void InitiateEncoding(double quality, base::TimeTicks deadline);

 protected:
  IdleTaskStatus idle_task_status_;
  bool fail_encoder_initialization_for_test_;
  bool enforce_idle_encoding_for_test_;

 private:
  friend class CanvasAsyncBlobCreatorTest;

  void Dispose();

  scoped_refptr<StaticBitmapImage> image_;
  Member<ExecutionContext> context_;

  // The following members are used for progressive/idle encoding,
  // see comment above the implementation of ScheduleAsyncBlobCreation.
  sk_sp<SkImage> skia_image_;
  SkPixmap src_data_;  // Holds a raw pointer owned by `skia_ìmage`.
  std::unique_ptr<ImageEncoder> encoder_;
  Vector<unsigned char> encoded_image_;
  int num_rows_completed_;

  ImageEncodingMimeType mime_type_;
  ToBlobFunctionType function_type_;

  // Chrome metrics use
  base::TimeTicks start_time_;
  base::TimeTicks schedule_idle_task_start_time_;
  bool static_bitmap_image_loaded_;
  IdentifiableToken input_digest_;
  CanvasInterventionsHelper::CanvasInterventionType intervention_type_;

  // Used when CanvasAsyncBlobCreator runs on main thread only
  scoped_refptr<base::SingleThreadTaskRunner> parent_frame_task_runner_;

  // Used for HTMLCanvasElement only
  Member<V8BlobCallback> callback_;

  // Used for OffscreenCanvas only
  Member<ScriptPromiseResolver<Blob>> script_promise_resolver_;

  static bool EncodeImage(std::unique_ptr<ImageDataBuffer>,
                          ImageEncodingMimeType,
                          const double& quality,
                          Vector<unsigned char>* encoded_image);

  // PNG, JPEG
  bool InitializeEncoder(double quality);
  void ForceEncodeRows();  // Similar to IdleEncodeRows without deadline.

  // WEBP
  static void EncodeImageOnEncoderThread(
      CrossThreadHandle<CanvasAsyncBlobCreator>,
      scoped_refptr<base::SingleThreadTaskRunner>,
      sk_sp<SkImage>,
      std::unique_ptr<ImageDataBuffer>,
      ImageEncodingMimeType,
      double quality);

  void IdleTaskStartTimeoutEvent(double quality);
  void IdleTaskCompleteTimeoutEvent();

  void RecordIdentifiabilityMetric();
  void TraceCanvasContent(Vector<unsigned char>* encoded_image);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_ASYNC_BLOB_CREATOR_H_
