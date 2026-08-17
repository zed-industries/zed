// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_PERF_CONFIDENCE_RATIO_BOOTSTRAP_ESTIMATOR_H_
#define TESTING_PERF_CONFIDENCE_RATIO_BOOTSTRAP_ESTIMATOR_H_

// This class provides construction of ratios between two measured stochastic
// quantities, including confidence intervals. The intended use is performance
// benchmarking (so that one can say with reasonable certainty that “binary A
// is 5% faster then binary B”), but it could in theory be used for other things
// as well.
//
// This is not a simple problem. The naïve way is to assume normality
// and use the standard rule-of-thumb that 95% CI is +/- 1.96 sigma,
// but benchmarks are frequently very non-normal (and we frequently
// do not have enough samples for the central limit theorem to come
// into play); in particular, measurements tend to have a long tail
// into slowness. The division makes it even worse, of course (dividing
// two normal distributions by each other does not become another normal
// distribution, even in the limit). Pinpoint tries to sidestep this using
// the Mann-Whitney U test, which has hardly any assumptions at all
// (its inputs do not even need to be numbers, just anything that
// can be ordered) but it has very low power, cannot produce confidence
// intervals and fundamentally measures something different from
// what people typically expect (the question “which one would most often
// win a race” is distinct from “which one has the lowest average”,
// giving a potentially problematic advantage to distributions with
// longer tails).
//
// This class uses the nonparametric bootstrap, a randomized method.
// A full introduction to the bootstrap is out of scope, but the basic
// idea is that instead of just doing r = sum(A) / sum(B), we resample the
// input data lots of times, and computing r over those resamples.
// The distribution of r then becomes more amenable to normal statistical
// methods. (This feels like cheating by just drawing more numbers out of
// thin air from the data we have, but it has a sound statistical basis.)
// We use the “bias-corrected and accelerated” (BCa) method for computing
// the confidence intervals, which is the generally recommended method
// these days; it automatically corrects for second-order bias and skew
// effects, with only fairly reasonable assumptions about the underlying
// distribution. We follow the exposition in this 2020 paper (which is
// basically an explanation of the original 1987 paper):
//
//   Efron, Narasimhan: “The automatic construction of bootstrap confidence
//   intervals”
//
// Even though the bootstrap is a very flexible method, it still makes
// some assumptions, in particular that the samples are independent and
// identically distributed. In practice, this may be violated because e.g.:
//
//   a) Something happened during one or more of the runs that impacted
//      their times (not identically distributed), e.g. a heavy cron job
//      or thermal throttling after the first N samples. This can cause
//      outliers, which bootstrapping is not inherently immune against.
//      If you have an uncontrolled measurement environment, consider
//      filtering outliers or warm-up runs to reduce the impact.
//   b) One or both of the sides had very lucky or unlucky code or data
//      placement, unrelated to the patch itself, causing a bias
//      (samples are not independent). This is generally hard to do something
//      about, but at least one thing to keep in mind is to always run with
//      binary names of the same length (i.e., don't call one of your binaries
//      _old, since argv[0] is put on the stack and this can perturb the
//      layout if you are unlucky).
//
// It also does not solve the problem of multiple comparisons (you could
// consider e.g. Bonferroni correction), and it does not do well with extremely
// high confidence levels unless you have lots of samples (e.g., CI=99.9999%
// with 100 samples will quickly hit the edge of the empirical distribution,
// which is not correct).
//
// I've spot-checked the results against the bcajack R package,
// which is the authors' own implementation of the methods in the paper;
// of course, in a system based on randomness, it's impossible to get
// exactly the same numbers. We don't the optimization of block-based
// jackknife for computing the acceleration (which takes it down from
// O(n²) to O(n)), as we generally only have a couple hundred samples.
// We also don't implement the calculation of error bars stemming from
// the resampling randomness (so-called internal error).
//
// This class is generally written to be independent of Blink, for slightly
// more general reuse (though the test is part of blink_unittests). It is
// not intended to be part of the main Chromium build, only used in
// auxiliary utilities.

#include <stdint.h>

#include <random>
#include <vector>

#include "third_party/blink/renderer/core/core_export.h"

class RatioBootstrapEstimator {
 public:
  explicit RatioBootstrapEstimator(uint64_t seed) : gen_(seed) {}

  struct Sample {
    // These are generally assumed to be time spent,
    // but if you have a higher-is-better score, you can just
    // invert the returned ratios below to get the improvement.
    double before, after;
  };
  struct Estimate {
    // The most likely single value (just sum(before) / sum(after),
    // without any further statistical computation). E.g., if the
    // new code is 5% faster, this will return 1.05 (which means
    // it is capable of computing 1.05 as many items in a given time;
    // 5% higher throughput).
    double point_estimate;

    // The xx% confidence interval.
    double lower, upper;
  };

  // NOTE: The slowest part of this code is generally drawing the random
  // numbers. To counteract this, we allow computing multiple series at the
  // same time in parallel, reusing the randomness (the computations are
  // independent). However, they should be the same length, possibly +/- 1;
  // if not, the resampling will use the shortest one for all values, which
  // could return wider confidence intervals than would be ideal if there is
  // a large discrepancy.
  //
  // We've generally found num_resamples = 2000 to give reasonable accuracy;
  // generally enough that the percentage values fluctuate only in the
  // first digit after the decimal point.
  //
  // Confidence level is typically a number like 0.95 (95% CI) or 0.99.
  //
  // If compute_geometric_mean is set to true, you will get an extra
  // estimate at the end, estimating the geometric mean between all the
  // other ratios.
  std::vector<Estimate> ComputeRatioEstimates(
      const std::vector<std::vector<Sample>>& data,
      unsigned num_resamples,
      double confidence_level,
      bool compute_geometric_mean);

  // Pulled out for unit testing only.
  static double InverseNormalCDF(double p);

 private:
  static double EstimateRatioExcept(const std::vector<Sample>& x,
                                    int skip_index);
  static double EstimateGeometricMeanExcept(
      const std::vector<std::vector<Sample>>& x,
      int skip_index);

  // mt19937 isn't a great PRNG (for one, it has huge state), but
  //
  //   a) It makes it easier to port this code to a non-Chromium context, and
  //   b) We need the determinism for unit testing purposes (otherwise,
  //      we are almost certain to make a test that is flaky to some degree),
  //      and e.g. base::RandomBitGenerator does not support seeding.
  std::mt19937 gen_;
};

#endif  // TESTING_PERF_CONFIDENCE_RATIO_BOOTSTRAP_ESTIMATOR_H_
