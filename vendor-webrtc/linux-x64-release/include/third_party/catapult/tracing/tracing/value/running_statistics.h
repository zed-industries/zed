// Copyright 2019 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <cmath>
#include <limits>

namespace catapult {

class RunningStatistics {
 public:
  RunningStatistics()
      : count_(0), mean_(0.0),
        max_(std::numeric_limits<double>::min()),
        min_(std::numeric_limits<double>::max()),
        sum_(0.0), variance_(0.0), meanlogs_(0.0), meanlogs_valid_(true) {}

  void Add(double value);

  int count() const { return count_;}
  double mean() const { return mean_;}
  double max() const { return max_;}
  double min() const { return min_;}
  double sum() const { return sum_;}
  double variance() const;
  double meanlogs() const;
  bool meanlogs_valid() const { return meanlogs_valid_; }

 private:
  int count_;
  double mean_;
  double max_;
  double min_;
  double sum_;
  double variance_;
  double meanlogs_;  // Mean of logarithms of samples.
  bool meanlogs_valid_;
};

}  // namespace catapult
