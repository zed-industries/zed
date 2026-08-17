// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_IMAGECAPTURE_IMAGE_CAPTURE_FRAME_GRABBER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_IMAGECAPTURE_IMAGE_CAPTURE_FRAME_GRABBER_H_

#include <memory>

#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/public/platform/web_callbacks.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_sink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/skia/include/core/SkRefCnt.h"

class SkImage;

namespace blink {

class ImageBitmap;
class MediaStreamComponent;

// A ScopedPromiseResolver is a move-only wrapper for ScriptPromiseResolver that
// assures the promise is always settled. This is particularly useful when
// you are passing callbacks binding the resolver through code which may
// silently drop them.
//
// Example usage:
//
//   void RespondWithSuccess(ScopedPromiseResolver<Foo> resolver) {
//     resolver.TakeResolver()->Resolve(Foo("everything is great"));
//   }
//
//   void OnCallbacksDropped(Persistent<PromiseResolver<Foo>> resolver) {
//     // Ownership of the promise resolver is passed to this function if
//     // ScopedPromiseResolver::TakeResolver isn't called before the
//     // ScopedPromiseResolver is destroyed.
//     resolver->Reject(FooError("everything is terrible"));
//   }
//
//   // Blink client implementation
//   void FooClientImpl::doMagic(PromiseResolver<Foo>* resolver) {
//     ScopedPromiseResolver scoped_resolver(resolver,
//         WTF::BindOnce(&OnCallbacksDropped));
//
//     // Call to some lower-level service which may never run the callback we
//     // give it.
//     foo_service_->DoMagic(WTF::BindOnce(&RespondWithSuccess,
//                                          std::move(scoped_resolver)));
//   }
//
// If the bound RespondWithSuccess callback actually runs, TakeResolver() will
// relinquish ownership of the resolver.
//
// If the bound RespondWithSuccess callback is instead destroyed first,
// the ScopedPromiseResolver destructor will invoke OnCallbacksDropped,
// executing our desired default behavior.
template <typename ResolveType>
class ScopedPromiseResolver {
 public:
  using DestructionCallback = base::OnceCallback<void(
      Persistent<ScriptPromiseResolver<ResolveType>> resolver)>;

  ScopedPromiseResolver(ScriptPromiseResolver<ResolveType>* resolver,
                        DestructionCallback destruction_callback)
      : resolver_(resolver),
        destruction_callback_(std::move(destruction_callback)) {}

  ~ScopedPromiseResolver() {
    if (destruction_callback_ && resolver_) {
      std::move(destruction_callback_).Run(resolver_);
    }
  }

  ScopedPromiseResolver(ScopedPromiseResolver&& other) = default;
  ScopedPromiseResolver(const ScopedPromiseResolver& other) = delete;

  ScopedPromiseResolver& operator=(ScopedPromiseResolver&& other) = default;
  ScopedPromiseResolver& operator=(const ScopedPromiseResolver& other) = delete;

  ScriptPromiseResolver<ResolveType>* TakeResolver() {
    ScriptPromiseResolver<ResolveType>* result = resolver_;
    resolver_ = nullptr;
    return result;
  }

 private:
  Persistent<ScriptPromiseResolver<ResolveType>> resolver_;
  DestructionCallback destruction_callback_;
};

// This class grabs Video Frames from a given Media Stream Video Track, binding
// a method of an ephemeral SingleShotFrameHandler every time grabFrame() is
// called. This method receives an incoming media::VideoFrame on a background
// thread and converts it into the appropriate SkBitmap which is sent back to
// OnSkBitmap(). This class is single threaded throughout.
class ImageCaptureFrameGrabber final : public MediaStreamVideoSink {
 public:
  ImageCaptureFrameGrabber() = default;

  ImageCaptureFrameGrabber(const ImageCaptureFrameGrabber&) = delete;
  ImageCaptureFrameGrabber& operator=(const ImageCaptureFrameGrabber&) = delete;

  ~ImageCaptureFrameGrabber() override;

  void GrabFrame(MediaStreamComponent* component,
                 ScriptPromiseResolver<ImageBitmap>* resolver,
                 scoped_refptr<base::SingleThreadTaskRunner> task_runner,
                 base::TimeDelta timeout);

 private:
  // Internal class to receive, convert and forward one frame.
  class SingleShotFrameHandler;

  void OnSkImage(ScopedPromiseResolver<ImageBitmap> resolver,
                 sk_sp<SkImage> image);
  void OnTimeout();

  // Flag to indicate that there is a frame grabbing in progress.
  bool frame_grab_in_progress_ = false;
  TaskHandle timeout_task_handle_;

  THREAD_CHECKER(thread_checker_);
  base::WeakPtrFactory<ImageCaptureFrameGrabber> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_IMAGECAPTURE_IMAGE_CAPTURE_FRAME_GRABBER_H_
