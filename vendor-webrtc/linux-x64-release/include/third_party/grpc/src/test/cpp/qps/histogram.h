//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_TEST_CPP_QPS_HISTOGRAM_H
#define GRPC_TEST_CPP_QPS_HISTOGRAM_H

#include "src/proto/grpc/testing/stats.pb.h"
#include "test/core/test_util/histogram.h"

namespace grpc {
namespace testing {

class Histogram {
 public:
  // TODO(unknown): look into making histogram params not hardcoded for C++
  Histogram()
      : impl_(grpc_histogram_create(default_resolution(),
                                    default_max_possible())) {}
  ~Histogram() {
    if (impl_) grpc_histogram_destroy(impl_);
  }
  void Reset() {
    if (impl_) grpc_histogram_destroy(impl_);
    impl_ = grpc_histogram_create(default_resolution(), default_max_possible());
  }

  Histogram(Histogram&& other) noexcept : impl_(other.impl_) {
    other.impl_ = nullptr;
  }

  void Merge(const Histogram& h) { grpc_histogram_merge(impl_, h.impl_); }
  void Add(double value) { grpc_histogram_add(impl_, value); }
  double Percentile(double pctile) const {
    return grpc_histogram_percentile(impl_, pctile);
  }
  double Count() const { return grpc_histogram_count(impl_); }
  void Swap(Histogram* other) { std::swap(impl_, other->impl_); }
  void FillProto(HistogramData* p) {
    size_t n;
    const auto* data = grpc_histogram_get_contents(impl_, &n);
    for (size_t i = 0; i < n; i++) {
      p->add_bucket(data[i]);
    }
    p->set_min_seen(grpc_histogram_minimum(impl_));
    p->set_max_seen(grpc_histogram_maximum(impl_));
    p->set_sum(grpc_histogram_sum(impl_));
    p->set_sum_of_squares(grpc_histogram_sum_of_squares(impl_));
    p->set_count(grpc_histogram_count(impl_));
  }
  void MergeProto(const HistogramData& p) {
    grpc_histogram_merge_contents(impl_, &*p.bucket().begin(), p.bucket_size(),
                                  p.min_seen(), p.max_seen(), p.sum(),
                                  p.sum_of_squares(), p.count());
  }

  static double default_resolution() { return 0.01; }
  static double default_max_possible() { return 60e9; }

 private:
  Histogram(const Histogram&);
  Histogram& operator=(const Histogram&);

  grpc_histogram* impl_;
};
}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_QPS_HISTOGRAM_H
