#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_REPORTER_STATISTIC_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_REPORTER_STATISTIC_H_

#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/calculator_profile.pb.h"

namespace mediapipe {
namespace reporter {

// Allows the user to push data and maintains a counter, mean, and stddev of
// that data.
class Statistic {
 public:
  Statistic() : counter_(0) {}

  // Clears the current statistic.
  void Clear() { counter_ = 0; }

  // Pushes a single value into the statistic, updating mean and stddev.
  void Push(double x);

  // Returns the number of data points used to calculate the mean and stddev.
  int data_count() const;

  // Returns the mean of the data pushed into this statistic.
  double mean() const;

  // Returns the variance of the data pushed into this statistic.
  double variance() const;

  // Returns the standard deviation of the data pushed into this statistic.
  double stddev() const;

  // Returns the sum of values of this statistic.
  double total() const;

 private:
  int counter_;
  double total_impl_;

  // Welford's algorithm allows us to keep a running standard deviation. We need
  // to hang onto the mean and sum of squared differences in between calls to
  // push().
  double mean_;
  double ssd_;
};

}  // namespace reporter
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_REPORTER_STATISTIC_H_
