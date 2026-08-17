// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_HISTOGRAM_ACCUMULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_HISTOGRAM_ACCUMULATOR_H_

#include <memory>
#include <mutex>
#include <string>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class HistogramBase;
}

namespace blink {

// Computes cumulative values of V8 metrics (e.g., sum of all lazy compile
// times) accumulated over the whole renderer process.
class PLATFORM_EXPORT V8HistogramAccumulator {
 public:
  static V8HistogramAccumulator* GetInstance();

  void* RegisterHistogram(base::HistogramBase* histogram,
                          const std::string& name);
  void AddSample(void* raw_histogram, int sample);
  void GenerateDataInteractive();

  V8HistogramAccumulator(const V8HistogramAccumulator&) = delete;
  V8HistogramAccumulator& operator=(const V8HistogramAccumulator&) = delete;

 private:
  V8HistogramAccumulator();
  ~V8HistogramAccumulator() = default;

  struct HistogramAndSum {
    explicit HistogramAndSum(base::HistogramBase* histogram)
        : original_histogram(histogram) {}
    HistogramAndSum(base::HistogramBase* histogram,
                    std::atomic<int>* sum_microseconds)
        : original_histogram(histogram), sum_microseconds(sum_microseconds) {}
    raw_ptr<base::HistogramBase> original_histogram;
    raw_ptr<std::atomic<int>> sum_microseconds = nullptr;
  };

  void* RegisterHistogramImpl(base::HistogramBase* histogram,
                              const std::string& name);
  void AddSampleImpl(void* raw_histogram, int sample);
  void GenerateDataInteractiveImpl();

 private:
  struct AccumulatingHistograms {
    raw_ptr<base::HistogramBase> interactive_histogram;
    // TODO(329408826): Add more accumulating points;
  };

  AccumulatingHistograms compile_foreground_;
  AccumulatingHistograms compile_background_;
  AccumulatingHistograms execute_;

  std::atomic<int> compile_foreground_sum_microseconds_ = 0;
  std::atomic<int> compile_background_sum_microseconds_ = 0;
  std::atomic<int> execute_sum_microseconds_ = 0;

  WTF::Vector<std::unique_ptr<HistogramAndSum>> histogram_and_sums_;

  // Protects histogram_and_sums_.
  std::mutex histogram_and_sums_mutex_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_HISTOGRAM_ACCUMULATOR_H_
