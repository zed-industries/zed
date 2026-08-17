// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_TDIGEST_H
#define GRPC_SRC_CORE_UTIL_TDIGEST_H

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"

namespace grpc_core {

// Represents a t-digest [1].
//
// t-digest is a structure that can store accurate accumulation of quantiles
// and other rank-based statistics, over a stream of data.
//
// There are different flavors of t-digest, but here we only implement a merging
// t-digest.
//
// Users can add values to a t-digest, and also merge t-digests.
//
// [1] Ted Dunning and Otmar Ertl, "COMPUTING EXTREMELY ACCURATE QUANTILES USING
//     t-DIGESTS", go/tdigest.
//
// Note on thread-safety: This class provides no thread-safety guarantee. Access
// to the methods of this class must be synchronized externally by the user.
class TDigest final {
 public:
  TDigest(const TDigest&) = delete;
  TDigest(TDigest&&) = delete;

  TDigest& operator=(const TDigest&) = delete;
  TDigest& operator=(TDigest&&) = delete;

  // Creates a t-digest with the given compression factor (aka delta).
  //
  // The number of centroids kept in a t-digest is in O(compression).
  // A t-digest should keep less than 2*compression.
  explicit TDigest(double compression);

  void Reset(double compression);

  // Adds `count` number of `val` to t-digest.
  void Add(double val, int64_t count);

  // Adds a single value with a count of 1 to the t-digest.
  void Add(double val) { Add(val, 1); }

  // Merges `that` t-digest into `this` t-digest.
  void Merge(const TDigest& that);

  // Returns an approximate quantile of values stored in the t-digest. Inclusive
  // i.e. largest value that <= quantile.
  //
  // `quantile` can be any real value between 0 and 1. For example, 0.99 would
  // return the 99th percentile.
  double Quantile(double quantile);

  // Returns the cumulative probability corresponding to the given value.
  // Inclusive i.e. probabiliy that <= val.
  double Cdf(double val);

  // Returns the minimum of all values added to the t-digest.
  double Min() const { return min_; }

  // Returns the maximum of all values added to the t-digest.
  double Max() const { return max_; }

  // Returns the sum of all values added to the t-digest.
  double Sum() const { return sum_; }

  // Returns the count of all values added to the t-digest.
  int64_t Count() const { return count_; }

  // Returns the compression factor of the t-digest.
  double Compression() const { return compression_; }

  // Returns the string representation of this t-digest. The string format is
  // external and compatible with all implementations of this library.
  std::string ToString();

  // Restores the t-digest from the string representation.
  // Returns an error if `string` is mal-formed where the state of this t-digest
  // is undefined.
  absl::Status FromString(absl::string_view string);

  // Returns the (approximate) size in bytes of storing this t-digest in RAM.
  // Useful when a TDigest is used as the accumulator in a Flume AccumulateFn.
  size_t MemUsageBytes() const;

  void Swap(TDigest& that) {
    std::swap(compression_, that.compression_);
    std::swap(batch_size_, that.batch_size_);
    std::swap(centroids_, that.centroids_);
    std::swap(merged_, that.merged_);
    std::swap(unmerged_, that.unmerged_);
    std::swap(min_, that.min_);
    std::swap(max_, that.max_);
    std::swap(sum_, that.sum_);
    std::swap(count_, that.count_);
  }

 private:
  // Centroid the primitive construct in t-digest.
  // A centroid has a mean and a count.
  struct CentroidPod {
    CentroidPod() : CentroidPod(0, 0) {}
    CentroidPod(double mean, int64_t count) : mean(mean), count(count) {}

    double mean;
    int64_t count;

    bool operator<(const CentroidPod& that) const {
      // For centroids with the same mean, we want to have the centroids
      // with a larger mass in front of the queue.
      //
      // See http://github.com/tdunning/t-digest/issues/78 for the discussion.
      return mean < that.mean || (mean == that.mean && count > that.count);
    }
  };

  // Adds a centroid to the unmerged list, and merge the unemerged centroids
  // when we have `batch_size` of unmerged centroids.
  void AddUnmergedCentroid(const CentroidPod& centroid);

  // Merges the batch of unmerged points and centroids.
  //
  // This is an in-place implementation of the progressive merging algorithm,
  // and does work solely using the centroids_ vector.
  void DoMerge();

  // Converts a quantile to the approximate centroid index.
  //
  // This is the k(q,delta) function in the t-digest paper.
  // See Figure 1 for more details.
  double QuantileToCentroid(double quantile) const;

  // Converts a centroid index to an approximate quantile.
  //
  // This is the _inverse_ of k(q,delta) function in the t-digest paper.
  // See Figure 1 for more details.
  double CentroidToQuantile(double centroid) const;

  // Updates min, max, sum, count.
  void UpdateStats(double min, double max, double sum, int64_t count) {
    if (count <= 0) return;
    if (min < min_) min_ = min;
    if (max > max_) max_ = max;
    count_ += count;
    sum_ += sum;
  }

  // Compression factor (aka delta).
  //
  // When zero, to be determined from the first merge.
  double compression_;
  // Maximum number of unmerged elements.
  int64_t batch_size_;

  // All centroids merged and unmerged. Unmerged centroids can actually be a
  // value or a centroid.
  std::vector<CentroidPod> centroids_;
  // Number of centroids that are already merged.
  int64_t merged_;
  // Number of centroids and values that are added but not merged yet.
  int64_t unmerged_;

  // Minimum of all values and centroid means.
  double min_;
  // Maximum of all values and centroid means.
  double max_;
  // Sum of all values and centroid means added.
  double sum_;
  // Count of all values and centroids added.
  int64_t count_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_TDIGEST_H
