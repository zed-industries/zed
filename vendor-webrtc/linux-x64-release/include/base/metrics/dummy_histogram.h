// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_DUMMY_HISTOGRAM_H_
#define BASE_METRICS_DUMMY_HISTOGRAM_H_

#include <stdint.h>

#include <memory>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/metrics/histogram_base.h"
#include "base/no_destructor.h"
#include "base/values.h"

namespace base {

// DummyHistogram is used for mocking histogram objects for histograms that
// shouldn't be recorded. It doesn't do any actual processing.
class BASE_EXPORT DummyHistogram : public HistogramBase {
 public:
  static DummyHistogram* GetInstance();

  DummyHistogram(const DummyHistogram&) = delete;
  DummyHistogram& operator=(const DummyHistogram&) = delete;

  // HistogramBase
  void CheckName(std::string_view name) const override {}
  uint64_t name_hash() const override;
  HistogramType GetHistogramType() const override;
  bool HasConstructionArguments(Sample32 expected_minimum,
                                Sample32 expected_maximum,
                                size_t expected_bucket_count) const override;
  void Add(Sample32 value) override {}
  void AddCount(Sample32 value, int count) override {}
  bool AddSamples(const HistogramSamples& samples) override;
  bool AddSamplesFromPickle(PickleIterator* iter) override;
  std::unique_ptr<HistogramSamples> SnapshotSamples() const override;
  std::unique_ptr<HistogramSamples> SnapshotUnloggedSamples() const override;
  void MarkSamplesAsLogged(const HistogramSamples& samples) override {}
  std::unique_ptr<HistogramSamples> SnapshotDelta() override;
  std::unique_ptr<HistogramSamples> SnapshotFinalDelta() const override;
  void WriteAscii(std::string* output) const override {}
  Value::Dict ToGraphDict() const override;

 protected:
  // HistogramBase:
  void SerializeInfoImpl(Pickle* pickle) const override {}
  Value::Dict GetParameters() const override;

 private:
  friend class NoDestructor<DummyHistogram>;

  DummyHistogram() : HistogramBase(DurableStringView("dummy_histogram")) {}
  ~DummyHistogram() override = default;
};

}  // namespace base

#endif  // BASE_METRICS_DUMMY_HISTOGRAM_H_
