/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef AUDIO_DSP_NELDER_MEAD_SEARCHER_H_
#define AUDIO_DSP_NELDER_MEAD_SEARCHER_H_
// This file contains an implementation of the Nelder-Mead derivative-free
// optimization algorithm using Eigen. See: Nelder, John A.; R. Mead (1965).
// "A simplex method for function minimization". Computer Journal. 7: 308–313.
//
// A nice graphical illustration can be found at:
// http://www.scholarpedia.org/article/Nelder-Mead_algorithm
//
// Example:
//
// The N-dimensional Rosenbrock function is a standard test case for non-linear
// optimization and is defined as
//   f(x1,x2,...,xN) = \sum_{i=1}({N-1} 100 * (x_{i+1} - x_i^2)^2 - (1 - x_i)^2
//
// double Rosenbrock(Eigen::Ref<const Eigen::ArrayXd>& x) {
//   const int n = v.size();
//   auto v0 = v.head(n-1);
//   auto v1 = v.tail(n-1);
//   return (100 * (v1 - v0.abs2()).abs2() + (1 - v0).abs2()).sum();
// }
//
// const int kDims = 2;
// const int kMaxEvaluations = 200;
// const double kStdDevTolerance = 0.0001;
// NelderMeadSearcher<double> searcher(kDims);
// searcher.SetSimplexFromStartingGuess(Eigen::ArrayXd::Zero(kDim), 1);
// searcher.SetObjectiveFunction(Rosenbrock);
// searcher.MinimizeObjective(kMaxEvaluations, kStdDevTolerance);
// double smallest_value_found = searcher.GetMin();
// Eigen::ArrayXf location_of_smallest_value = searcher.GetArgMin();

#include <functional>
#include <limits>

#include "glog/logging.h"
#include "absl/types/span.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

template <typename Scalar>
class NelderMeadSearcher {
 public:
  using Matrix = Eigen::Array<Scalar, Eigen::Dynamic, Eigen::Dynamic>;
  using Vector = Eigen::Array<Scalar, Eigen::Dynamic, 1>;
  using RealScalar = typename Eigen::NumTraits<Scalar>::Real;
  using ObjectiveFunction =
      std::function<RealScalar(const Eigen::Ref<const Vector>&)>;

  // Constucts a NelderMead searcher for the given number of dimensions.
  explicit NelderMeadSearcher(int dimensions);

  // Sets the simplex to |starting_guess| plus the set of points displaced
  // around it by |offset| along each canonical coordinate axis. Resets
  // num_evaluations_ to zero.
  void SetSimplexFromStartingGuess(const Vector& starting_guess, Scalar offset);
  void SetSimplexFromStartingGuessSpan(absl::Span<const Scalar> starting_guess,
                                       Scalar offset);

  // Sets the objective function to minimize. The objective must take a Vector
  // of size |dimensions_| and return a RealScalar. Resets num_evaluations_ to
  // zero.
  void SetObjectiveFunction(const ObjectiveFunction& objective_function);

  // Runs the Nelder-Mead search algorithm with the current |simplex_| and
  // |objective_function_|. The search terminates when the number of function
  // evaluations exceeds |max_evaluations|, or the standard deviation of the
  // objective values at the points of the simplex is less than
  // |standard_deviation_tolerance|, or the smallest objective value is less or
  // equal to |objective_goal|. |standard_deviation_tolerance| is ignored if
  // less or equal to zero. |objective_goal| is ignored if equal to
  // -std::numeric_limits<Scalar>::infinity().
  // REQUIRES: SetObjectiveFunction(), and SetSimplexFromStartingGuess*() or
  // SetSimplex() must be called before.
  void MinimizeObjective(int max_evaluations,
                         RealScalar standard_deviation_tolerance,
                         RealScalar objective_goal);

  // Returns the lowest value of the objective function seen so far.
  // REQUIRES: MinimizeObjective() must be called before.
  RealScalar GetMin() const;

  // Returns the simplex point corresponding to the lowest value of the
  // objective function returned by GetMin().
  // REQUIRES: MinimizeObjective() must be called before.
  Vector GetArgMin() const;

  // Returns the standard deviations of the objective function values at the
  // points of the simplex.
  // REQUIRES: MinimizeObjective() must be called before.
  RealScalar StandardDeviation() const;

  // Sets lower bounds to impose on coordinates. A lower bound of -infinity may
  // be used to specify that a coordinate is unconstrained.
  void SetLowerBounds(absl::Span<const Scalar> lower_bounds);

  // Sets upper bounds to impose on coordinates. An upper bound of +infinity may
  // be used to specify that a coordinate is unconstrained.
  void SetUpperBounds(absl::Span<const Scalar> upper_bounds);

  // Clears all bounds previously set.
  void ClearBounds();

  // Sets the simplex to the columns in |simplex|. |simplex| must have
  // dimensions_ rows and dimensions_ + 1 columns. If any bounds are
  // active they will be applied to simplex. Resets num_evaluations_ to zero.
  void SetSimplex(const Matrix& simplex);

  // Returns a reference the current simplex. The NelderMeadSearcher
  // must outlive any use of the return value.
  const Matrix& GetSimplex() const;

  // Number of function evaluations, i.e. calls to objective_function_, since
  // the last call to one of MinimizeObjective().
  int num_evaluations() const { return num_evaluations_; }

  // Resets num_evalations to zero.
  void reset_evaluations() { num_evaluations_ = 0; }

 private:
  // Evaluates the objective at the given point.
  template <typename VectorType>
  inline RealScalar EvaluateObjective(const VectorType& point);

  // Evaluates the objective at all points in the simplex and updates
  // argmin_index_ and argmax_index_.
  void EvaluateObjectiveAtAllPoints();

  // Clamps values in all columns of simplex_;
  void EnforceBoundsOnAllPoints();

  // Computes centroid of simplex points except simplex_.col(argmax_index_).
  inline Vector Centroid() const;

  // Returns the simplex point with the highest value of the objective function.
  inline Vector GetArgMax() const;

  // Returns the highest value of the objective function at the simplex points.
  inline RealScalar GetMax() const;

  // Returns the second highest value of the objective function at the simplex
  // points.
  inline RealScalar GetSecondHighest() const;

  // Replaces the simplex point with the highest function value with the new
  // point, and updates argmax_index_.
  void ReplaceArgMax(const Vector& new_point, RealScalar new_function_value);

  // Replace all points p_i by (p_i + p_min) / 2.
  void ShrinkSimplex();

  // Returns true if |point| is feasible.
  bool IsFeasible(const Vector& point) const;

  const int dimensions_;
  // TODO: Add a constructor or setter for the following algorithm
  // constants. By default we use the standard (1, 1/2, 2) strategy found to
  // work well in the original paper.
  const RealScalar reflection_coefficient_;
  const RealScalar expansion_coefficient_;
  const RealScalar contraction_coefficient_;
  const RealScalar shrinkage_coefficient_;
  ObjectiveFunction objective_function_;
  Matrix simplex_;
  Vector function_values_;
  Vector lower_bounds_;
  Vector upper_bounds_;
  int argmin_index_;
  int argmax_index_;
  int num_evaluations_;

  NelderMeadSearcher(const NelderMeadSearcher&) = delete;
  NelderMeadSearcher& operator=(const NelderMeadSearcher&) = delete;
};

// Implementation below. Avert your eyes.

template <typename Scalar>
NelderMeadSearcher<Scalar>::NelderMeadSearcher(int dimensions)
    : dimensions_(dimensions),
      reflection_coefficient_(1),
      expansion_coefficient_(2),
      contraction_coefficient_(0.5),
      shrinkage_coefficient_(0.5),
      simplex_(dimensions, dimensions + 1),
      function_values_(dimensions + 1),
      lower_bounds_(0),
      upper_bounds_(0),
      argmin_index_(-1),
      num_evaluations_(0) {
  ABSL_CHECK_GT(dimensions, 0);
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::SetSimplexFromStartingGuess(
    const Vector& starting_guess, Scalar offset) {
  ABSL_CHECK_EQ(starting_guess.size(), simplex_.rows());
  ABSL_CHECK(IsFeasible(starting_guess))
      << "Starting guess must satisfy constraints.";
  simplex_.col(0) = starting_guess;
  for (int dim = 0; dim < dimensions_; ++dim) {
    simplex_.col(dim + 1) = starting_guess;
    simplex_(dim, dim + 1) += offset;
  }
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::SetSimplexFromStartingGuessSpan(
    absl::Span<const Scalar> starting_guess, Scalar offset) {
  Eigen::Map<const Vector> starting_guess_map(starting_guess.data(),
                                              starting_guess.size());
  SetSimplexFromStartingGuess(starting_guess_map, offset);
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::SetObjectiveFunction(
    const ObjectiveFunction& objective_function) {
  objective_function_ = objective_function;
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::MinimizeObjective(
    int max_evaluations, RealScalar standard_deviation_tolerance,
    RealScalar objective_goal) {
  ABSL_CHECK_EQ(simplex_.cols(), dimensions_ + 1)
      << "Did you forget to call "
         "SetSimplex() or "
         "SetSimplexFromStartingGuess()?";
  ABSL_CHECK(objective_function_ != nullptr)
      << "Did you forget to call SetObjectiveFunction?";
  num_evaluations_ = 0;
  EnforceBoundsOnAllPoints();
  EvaluateObjectiveAtAllPoints();

  // Invariants:
  //   1. functions_values_(i) contains the result of
  //     objective_function_(simplex_.col(i)) for i in 0...dimensions_.
  //   2. All simplex points are feasible.
  Vector centroid_point(dimensions_);
  Vector reflection_point(dimensions_);
  Vector expansion_point(dimensions_);
  Vector contraction_point(dimensions_);
  while (num_evaluations_ < max_evaluations && GetMin() > objective_goal &&
         StandardDeviation() > standard_deviation_tolerance) {
    VLOG(2) << "\nmin = " << GetMin()
            << ", num_evaluations = " << num_evaluations_
            << ", stddev = " << StandardDeviation();
    VLOG(3) << "argmin = \n" << GetArgMin();
    centroid_point = Centroid();
    reflection_point = (1 + reflection_coefficient_) * centroid_point -
                       reflection_coefficient_ * GetArgMax();
    // Try reflection.
    const RealScalar reflection_objective = EvaluateObjective(reflection_point);
    if (reflection_objective < GetMin()) {
      // The reflection is a new minimum => Try to expand simplex along
      // the line (max, centroid_point)
      expansion_point = expansion_coefficient_ * reflection_point +
                        (1 - expansion_coefficient_) * centroid_point;
      const RealScalar expansion_objective = EvaluateObjective(expansion_point);
      // Accept the better of the reflection and expansion points and go to the
      // next iteration.
      if (expansion_objective < reflection_objective) {
        VLOG(2) << "Expansion";
        ReplaceArgMax(expansion_point, expansion_objective);
      } else {
        VLOG(2) << "Good Reflection";
        ReplaceArgMax(reflection_point, reflection_objective);
      }
      continue;
    }

    const float second_highest = GetSecondHighest();
    if (reflection_objective < second_highest) {
      // Accept reflection point and go to the next iteration.
      VLOG(2) << "Reflection";
      ReplaceArgMax(reflection_point, reflection_objective);
      continue;
    }

    const float highest = GetMax();
    bool shrink = true;
    if (reflection_objective < highest) {
      // Attempt outside contraction.
      contraction_point =
          centroid_point +
          contraction_coefficient_ * (reflection_point - centroid_point);
      const RealScalar contraction_objective =
          EvaluateObjective(contraction_point);
      if (contraction_objective <= reflection_objective) {
        VLOG(2) << "Outside contraction";
        ReplaceArgMax(contraction_point, contraction_objective);
        shrink = false;
      }
    } else {
      // Attempt inside contraction.
      contraction_point = centroid_point + contraction_coefficient_ *
                                               (GetArgMax() - centroid_point);
      const RealScalar contraction_objective =
          EvaluateObjective(contraction_point);
      if (contraction_objective < highest) {
        VLOG(2) << "Inside contraction";
        ReplaceArgMax(contraction_point, contraction_objective);
        shrink = false;
      }
    }
    if (shrink) {
      // Contraction failed: Shrink the entire simplex towards GetArgMin().
      VLOG(2) << "Shrink";
      ShrinkSimplex();
    }
  }
}

template <typename Scalar>
typename NelderMeadSearcher<Scalar>::Vector
NelderMeadSearcher<Scalar>::GetArgMin() const {
  ABSL_CHECK_GE(argmin_index_, 0) << "Did you forget to call MinimizeObjective()?";
  return simplex_.col(argmin_index_);
}

template <typename Scalar>
typename NelderMeadSearcher<Scalar>::RealScalar
NelderMeadSearcher<Scalar>::GetMin() const {
  ABSL_CHECK_GE(argmin_index_, 0) << "Did you forget to call MinimizeObjective()?";
  return function_values_(argmin_index_);
}

template <typename Scalar>
typename NelderMeadSearcher<Scalar>::RealScalar
NelderMeadSearcher<Scalar>::StandardDeviation() const {
  ABSL_CHECK_GE(argmin_index_, 0) << "Did you forget to call MinimizeObjective()?";
  RealScalar mean = function_values_.mean();
  return std::sqrt((function_values_ - mean).square().sum());
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::SetSimplex(const Matrix& simplex) {
  ABSL_CHECK_EQ(simplex.rows(), simplex_.rows());
  ABSL_CHECK_EQ(simplex.cols(), simplex_.cols());
  simplex_ = simplex;
}

template <typename Scalar>
const typename NelderMeadSearcher<Scalar>::Matrix&
NelderMeadSearcher<Scalar>::GetSimplex() const {
  return simplex_;
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::SetLowerBounds(
    absl::Span<const Scalar> lower_bounds) {
  ABSL_CHECK_EQ(lower_bounds.size(), dimensions_);
  const Eigen::Map<const Vector> lower_bounds_map(lower_bounds.data(),
                                                  lower_bounds.size());
  lower_bounds_ = lower_bounds_map;
  if (upper_bounds_.size()) {
    ABSL_CHECK((upper_bounds_ >= lower_bounds_).all());
  }
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::SetUpperBounds(
    absl::Span<const Scalar> upper_bounds) {
  ABSL_CHECK_EQ(upper_bounds.size(), dimensions_);
  const Eigen::Map<const Vector> upper_bounds_map(upper_bounds.data(),
                                                  upper_bounds.size());
  upper_bounds_ = upper_bounds_map;
  if (lower_bounds_.size()) {
    ABSL_CHECK((upper_bounds_ >= lower_bounds_).all());
  }
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::ClearBounds() {
  lower_bounds_.resize(0);
  upper_bounds_.resize(0);
}

template <typename Scalar>
template <typename VectorType>
inline typename NelderMeadSearcher<Scalar>::RealScalar
NelderMeadSearcher<Scalar>::EvaluateObjective(const VectorType& point) {
  if (IsFeasible(point)) {
    ++num_evaluations_;
    return objective_function_(point);
  } else {
    return std::numeric_limits<RealScalar>::infinity();
  }
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::EvaluateObjectiveAtAllPoints() {
  for (int i = 0; i < simplex_.cols(); ++i) {
    function_values_(i) = EvaluateObjective(simplex_.col(i));
  }
  function_values_.maxCoeff(&argmax_index_);
  function_values_.minCoeff(&argmin_index_);
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::ReplaceArgMax(const Vector& new_point,
                                               RealScalar new_function_value) {
  simplex_.col(argmax_index_) = new_point;
  function_values_(argmax_index_) = new_function_value;
  function_values_.maxCoeff(&argmax_index_);
  function_values_.minCoeff(&argmin_index_);
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::EnforceBoundsOnAllPoints() {
  if (lower_bounds_.size() == 0 && upper_bounds_.size() == 0) {
    return;
  }
  for (int i = 0; i < simplex_.cols(); ++i) {
    if (lower_bounds_.size() != 0) {
      simplex_.col(i) = simplex_.col(i).max(lower_bounds_);
    }
    if (upper_bounds_.size() != 0) {
      simplex_.col(i) = simplex_.col(i).min(upper_bounds_);
    }
  }
}

template <typename Scalar>
inline typename NelderMeadSearcher<Scalar>::Vector
NelderMeadSearcher<Scalar>::Centroid() const {
  return (simplex_.rowwise().sum() - GetArgMax()) / RealScalar(dimensions_);
}

template <typename Scalar>
inline typename NelderMeadSearcher<Scalar>::Vector
NelderMeadSearcher<Scalar>::GetArgMax() const {
  return simplex_.col(argmax_index_);
}

template <typename Scalar>
inline typename NelderMeadSearcher<Scalar>::RealScalar
NelderMeadSearcher<Scalar>::GetMax() const {
  return function_values_(argmax_index_);
}

template <typename Scalar>
inline typename NelderMeadSearcher<Scalar>::RealScalar
NelderMeadSearcher<Scalar>::GetSecondHighest() const {
  Scalar second_highest = -std::numeric_limits<Scalar>::infinity();
  for (int i = 0; i < dimensions_; ++i) {
    if (i != argmax_index_ && function_values_(i) > second_highest)
      second_highest = function_values_(i);
  }
  return second_highest;
}

template <typename Scalar>
void NelderMeadSearcher<Scalar>::ShrinkSimplex() {
  for (int i = 0; i < simplex_.cols(); ++i) {
    if (i != argmin_index_) {
      simplex_.col(i) =
          shrinkage_coefficient_ * (simplex_.col(i) + GetArgMin());
      function_values_(i) = EvaluateObjective(simplex_.col(i));
    }
  }
  function_values_.maxCoeff(&argmax_index_);
  function_values_.minCoeff(&argmin_index_);
}

template <typename Scalar>
bool NelderMeadSearcher<Scalar>::IsFeasible(const Vector& point) const {
  bool feasible = true;
  if (upper_bounds_.size()) {
    feasible = feasible && (point <= upper_bounds_).all();
  }
  if (feasible && lower_bounds_.size()) {
    feasible = feasible && (point >= lower_bounds_).all();
  }
  return feasible;
}

}  // namespace audio_dsp

#endif  // AUDIO_DSP_NELDER_MEAD_SEARCHER_H_
