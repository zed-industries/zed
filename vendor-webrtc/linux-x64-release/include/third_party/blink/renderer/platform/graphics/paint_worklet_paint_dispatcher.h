// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_WORKLET_PAINT_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_WORKLET_PAINT_DISPATCHER_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/graphics/paint_worklet_painter.h"
#include "third_party/blink/renderer/platform/graphics/platform_paint_worklet_layer_painter.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace blink {

// PaintWorkletPaintDispatcher is responsible for mediating between the raster
// threads and the PaintWorklet thread(s). It receives requests from raster
// threads to paint a paint class instance represented by a PaintWorkletInput,
// dispatches the input to the appropriate PaintWorklet, synchronously receives
// the result, and passes it back to the raster thread.
//
// Each PaintWorklet (there is one per frame, either same-origin or
// same-process-cross-origin) has a backing thread, which may be shared between
// worklets, and a scheduler, which is not shared. All PaintWorklets for a
// single renderer process share one PaintWorkletPaintDispatcher on the
// compositor side.
class PLATFORM_EXPORT PaintWorkletPaintDispatcher {
  USING_FAST_MALLOC(PaintWorkletPaintDispatcher);

 public:
  static std::unique_ptr<PlatformPaintWorkletLayerPainter>
  CreateCompositorThreadPainter(
      base::WeakPtr<PaintWorkletPaintDispatcher>* paintee);

  PaintWorkletPaintDispatcher();
  PaintWorkletPaintDispatcher(const PaintWorkletPaintDispatcher&) = delete;
  PaintWorkletPaintDispatcher& operator=(const PaintWorkletPaintDispatcher&) =
      delete;
  virtual ~PaintWorkletPaintDispatcher() = default;

  // Dispatches a set of paint class instances - each represented by a
  // PaintWorkletInput - to the appropriate PaintWorklet threads, asynchronously
  // returning the results on the calling thread via the passed callback.
  //
  // Only one dispatch may be going on at a given time; the caller must wait for
  // the passed callback to be called before calling DispatchWorklets again.
  void DispatchWorklets(cc::PaintWorkletJobMap,
                        PlatformPaintWorkletLayerPainter::DoneCallback);

  // Reports whether or not there is an ongoing dispatch (e.g. a set of
  // PaintWorklet instances have been dispatched to the worklet, but the results
  // have not yet been received.)
  bool HasOngoingDispatch() const;

  // Register and unregister a PaintWorklet (represented in this context by a
  // PaintWorkletPainter). A given PaintWorklet is registered once all its
  // global scopes have been created, and is usually only unregistered when the
  // associated PaintWorklet thread is being torn down.
  //
  // The passed in PaintWorkletPainter* should only be used on the given
  // base::SingleThreadTaskRunner.
  using PaintWorkletId = int;
  void RegisterPaintWorkletPainter(PaintWorkletPainter*,
                                   scoped_refptr<base::SingleThreadTaskRunner>);
  void UnregisterPaintWorkletPainter(PaintWorkletId);

  // The main thread is given a base::WeakPtr to this class to hand to the
  // PaintWorklet thread(s), so that they can register and unregister
  // PaintWorklets. See blink::WebFrameWidgetImpl for where this happens.
  base::WeakPtr<PaintWorkletPaintDispatcher> GetWeakPtr() {
    return weak_factory_.GetWeakPtr();
  }

  using PaintWorkletPainterToTaskRunnerMap =
      HashMap<PaintWorkletId,
              std::pair<CrossThreadPersistent<PaintWorkletPainter>,
                        scoped_refptr<base::SingleThreadTaskRunner>>>;
  const PaintWorkletPainterToTaskRunnerMap& PainterMapForTesting() const {
    return painter_map_;
  }

 protected:
  virtual scoped_refptr<base::SingleThreadTaskRunner> GetCompositorTaskRunner();

 private:
  // Called when results are available for the previous call to
  // |DispatchWorklets|.
  void AsyncPaintDone();

  // This class handles paint class instances for multiple PaintWorklets. These
  // are disambiguated via the PaintWorklets unique id; this map exists to do
  // that disambiguation.
  PaintWorkletPainterToTaskRunnerMap painter_map_;

  // Whilst an asynchronous paint is underway (see |DispatchWorklets|), we store
  // the input jobs and the completion callback. The jobs are shared with the
  // PaintWorklet thread(s) during the dispatch, whilst the callback only ever
  // stays on the calling thread.
  cc::PaintWorkletJobMap ongoing_jobs_;
  base::OnceCallback<void(cc::PaintWorkletJobMap)> on_async_paint_complete_;

  // Used to ensure that appropriate methods are called on the same thread.
  // Currently only used for the asynchronous dispatch path.
  SEQUENCE_CHECKER(sequence_checker_);

  base::WeakPtrFactory<PaintWorkletPaintDispatcher> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_WORKLET_PAINT_DISPATCHER_H_
