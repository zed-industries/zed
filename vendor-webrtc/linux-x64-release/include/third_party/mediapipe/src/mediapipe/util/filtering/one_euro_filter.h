#ifndef MEDIAPIPE_UTIL_FILTERING_ONE_EURO_FILTER_H_
#define MEDIAPIPE_UTIL_FILTERING_ONE_EURO_FILTER_H_

#include <cstdint>
#include <memory>

#include "absl/time/time.h"
#include "mediapipe/util/filtering/low_pass_filter.h"

namespace mediapipe {

class OneEuroFilter {
 public:
  OneEuroFilter(double frequency, double min_cutoff, double beta,
                double derivate_cutoff);

  double Apply(absl::Duration timestamp, double value_scale, double value);

 private:
  double GetAlpha(double cutoff);

  void SetFrequency(double frequency);

  void SetMinCutoff(double min_cutoff);

  void SetBeta(double beta);

  void SetDerivateCutoff(double derivate_cutoff);

  double frequency_;
  double min_cutoff_;
  double beta_;
  double derivate_cutoff_;
  std::unique_ptr<LowPassFilter> x_;
  std::unique_ptr<LowPassFilter> dx_;
  int64_t last_time_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_FILTERING_ONE_EURO_FILTER_H_
