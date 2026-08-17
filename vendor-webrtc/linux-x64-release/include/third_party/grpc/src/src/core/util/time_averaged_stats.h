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

#ifndef GRPC_SRC_CORE_UTIL_TIME_AVERAGED_STATS_H
#define GRPC_SRC_CORE_UTIL_TIME_AVERAGED_STATS_H

namespace grpc_core {

// This tracks a time-decaying weighted average.  It works by collecting
// batches of samples and then mixing their average into a time-decaying
// weighted mean.  It is designed for batch operations where we do many adds
// before updating the average.

class TimeAveragedStats {
 public:
  TimeAveragedStats(double init_avg, double regress_weight,
                    double persistence_factor);

  // Add a sample to the current batch.
  void AddSample(double value);
  // Complete a batch and compute the new estimate of the average sample
  // value.
  double UpdateAverage();

  double aggregate_weighted_avg() const { return aggregate_weighted_avg_; }
  double aggregate_total_weight() const { return aggregate_total_weight_; }

 private:
  // The initial average value.  This is the reported average until the first
  // grpc_time_averaged_stats_update_average call.  If a positive regress_weight
  // is used, we also regress towards this value on each update.
  const double init_avg_;
  // The sample weight of "init_avg" that is mixed in with each call to
  // grpc_time_averaged_stats_update_average.  If the calls to
  // grpc_time_averaged_stats_add_sample stop, this will cause the average to
  // regress back to the mean.  This should be non-negative.  Set it to 0 to
  // disable the bias.  A value of 1 has the effect of adding in 1 bonus sample
  // with value init_avg to each sample period.
  const double regress_weight_;
  // This determines the rate of decay of the time-averaging from one period
  // to the next by scaling the aggregate_total_weight of samples from prior
  // periods when combining with the latest period.  It should be in the range
  // [0,1].  A higher value adapts more slowly.  With a value of 0.5, if the
  // batches each have k samples, the samples_in_avg_ will grow to 2 k, so the
  // weighting of the time average will eventually be 1/3 new batch and 2/3
  // old average.
  const double persistence_factor_;

  // The total value of samples since the last UpdateAverage().
  double batch_total_value_ = 0;
  // The number of samples since the last UpdateAverage().
  double batch_num_samples_ = 0;
  // The time-decayed sum of batch_num_samples_ over previous batches.  This is
  // the "weight" of the old aggregate_weighted_avg_ when updating the
  // average.
  double aggregate_total_weight_ = 0;
  // A time-decayed average of the (batch_total_value_ / batch_num_samples_),
  // computed by decaying the samples_in_avg_ weight in the weighted average.
  double aggregate_weighted_avg_ = init_avg_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_TIME_AVERAGED_STATS_H
