// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_SINGLE_SAMPLE_METRICS_H_
#define BASE_METRICS_SINGLE_SAMPLE_METRICS_H_

#include <memory>
#include <string>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/metrics/histogram_base.h"

namespace base {

// See base/metrics/histograms.h for parameter definitions. Must only be used
// and destroyed from the same thread as construction.
class BASE_EXPORT SingleSampleMetric {
 public:
  virtual ~SingleSampleMetric() = default;

  virtual void SetSample(HistogramBase::Sample32 sample) = 0;
};

// Factory for creating single sample metrics. A single sample metric only
// reports its sample once at destruction time. The sample may be changed prior
// to destruction using the SetSample() method as many times as desired.
//
// The metric creation methods are safe to call from any thread, however the
// returned class must only be used and destroyed from the same thread as
// construction.
//
// See base/metrics/histogram_macros.h for usage recommendations and
// base/metrics/histogram.h for full parameter definitions.
class BASE_EXPORT SingleSampleMetricsFactory {
 public:
  virtual ~SingleSampleMetricsFactory() = default;

  // Returns the factory provided by SetFactory(), or if no factory has been set
  // a default factory will be provided (future calls to SetFactory() will fail
  // if the default factory is ever vended).
  static SingleSampleMetricsFactory* Get();
  static void SetFactory(std::unique_ptr<SingleSampleMetricsFactory> factory);

  // The factory normally persists until process shutdown, but in testing we
  // should avoid leaking it since it sets a global.
  static void DeleteFactoryForTesting();

  // The methods below return a single sample metric for counts histograms; see
  // method comments for the corresponding histogram macro.

  // UMA_HISTOGRAM_CUSTOM_COUNTS()
  virtual std::unique_ptr<SingleSampleMetric> CreateCustomCountsMetric(
      const std::string& histogram_name,
      HistogramBase::Sample32 min,
      HistogramBase::Sample32 max,
      uint32_t bucket_count) = 0;
};

// Default implementation for when no factory has been provided to the process.
// Samples are only recorded within the current process in this case, so samples
// will be lost in the event of sudden process termination.
class BASE_EXPORT DefaultSingleSampleMetricsFactory
    : public SingleSampleMetricsFactory {
 public:
  DefaultSingleSampleMetricsFactory() = default;
  DefaultSingleSampleMetricsFactory(const DefaultSingleSampleMetricsFactory&) =
      delete;
  DefaultSingleSampleMetricsFactory& operator=(
      const DefaultSingleSampleMetricsFactory&) = delete;
  ~DefaultSingleSampleMetricsFactory() override = default;

  // SingleSampleMetricsFactory:
  std::unique_ptr<SingleSampleMetric> CreateCustomCountsMetric(
      const std::string& histogram_name,
      HistogramBase::Sample32 min,
      HistogramBase::Sample32 max,
      uint32_t bucket_count) override;
};

class BASE_EXPORT DefaultSingleSampleMetric : public SingleSampleMetric {
 public:
  DefaultSingleSampleMetric(const std::string& histogram_name,
                            HistogramBase::Sample32 min,
                            HistogramBase::Sample32 max,
                            uint32_t bucket_count,
                            int32_t flags);

  DefaultSingleSampleMetric(const DefaultSingleSampleMetric&) = delete;
  DefaultSingleSampleMetric& operator=(const DefaultSingleSampleMetric&) =
      delete;

  ~DefaultSingleSampleMetric() override;

  // SingleSampleMetric:
  void SetSample(HistogramBase::Sample32 sample) override;

 private:
  const raw_ptr<HistogramBase> histogram_;

  // The last sample provided to SetSample(). We use -1 as a sentinel value to
  // indicate no sample has been set.
  HistogramBase::Sample32 sample_ = -1;
};

}  // namespace base

#endif  // BASE_METRICS_SINGLE_SAMPLE_METRICS_H_
