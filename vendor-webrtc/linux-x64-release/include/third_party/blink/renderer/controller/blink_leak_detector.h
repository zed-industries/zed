// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_BLINK_LEAK_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_BLINK_LEAK_DETECTOR_H_

#include "base/task/single_thread_task_runner.h"
#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/leak_detector/leak_detector.mojom-blink.h"
#include "third_party/blink/renderer/controller/controller_export.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

// Implementation of Leak Detector.
class CONTROLLER_EXPORT BlinkLeakDetector : public mojom::blink::LeakDetector {
 public:
  static void Bind(scoped_refptr<base::SingleThreadTaskRunner> task_runner,
                   mojo::PendingReceiver<mojom::blink::LeakDetector>);

  explicit BlinkLeakDetector(
      base::PassKey<BlinkLeakDetector> pass_key,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  BlinkLeakDetector(const BlinkLeakDetector&) = delete;
  BlinkLeakDetector& operator=(const BlinkLeakDetector&) = delete;

  ~BlinkLeakDetector() override;

 private:
  // mojom::blink::LeakDetector implementation.
  void PerformLeakDetection(PerformLeakDetectionCallback) override;

  void TimerFiredGC(TimerBase*);
  void ReportResult();
  void ReportInvalidResult();

  TaskRunnerTimer<BlinkLeakDetector> delayed_gc_timer_;
  int number_of_gc_needed_ = 0;
  PerformLeakDetectionCallback callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_BLINK_LEAK_DETECTOR_H_
