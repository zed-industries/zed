// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MOVING_WINDOW_H_
#define BASE_MOVING_WINDOW_H_

#include <math.h>
#include <stddef.h>

#include <cmath>
#include <functional>
#include <limits>
#include <vector>

#include "base/check_op.h"
#include "base/memory/raw_ref.h"
#include "base/time/time.h"

namespace base {

// Class to efficiently calculate statistics in a sliding window.
// This class isn't thread safe.
// Supported statistics are Min/Max/Mean/Deviation.
// You can also iterate through the items in the window.
// The class is modular: required features must be specified in the template
// arguments.
// Non listed features don't consume memory or runtime cycles at all.
//
// Usage:
// base::MovingWindow<int,
//                    base::MovingWindowFeatures::Min,
//                    base::MovingWindowFeatures::Max>
//                    moving_window(window_size);
//
// Following convenience shortcuts are provided with predefined sets of
// features:
// MovingMax/MovingMin/MovingAverage/MovingAverageDeviation/MovingMinMax.
//
// Methods:
// Constructor:
//   MovingWindow(size_t window_size);
//
// Window update (available for all templates):
//   AddSample(T value) const;
//   size_t Count() const;
//   void Reset();
//
// Available for MovingWindowFeatures::Min:
//    T Min() const;
//
// Available for MovingWindowFeatures::Max:
//    T Max() const;
//
// Available for MovingWindowFeatures::Mean:
//    U Mean<U>() const;
//
// Available for MovingWindowFeatures::Deviation:
//    U Deviation<U>() const;
//
// Available for MovingWindowFeatures::Iteration. Iterating through the window:
//    iterator begin() const;
//    iterator begin() const;
//    size_t size() const;

// Features supported by the class.
struct MovingWindowFeatures {
  struct Min {
    static bool has_min;
  };

  struct Max {
    static bool has_max;
  };

  // Need to specify a type capable of holding a sum of all elements in the
  // window.
  template <typename SumType>
  struct Mean {
    static SumType has_mean;
  };

  // Need to specify a type capable of holding a sum of squares of all elements
  // in the window.
  template <typename SumType>
  struct Deviation {
    static SumType has_deviation;
  };

  struct Iteration {
    static bool has_iteration;
  };
};

// Main template.
template <typename T, typename... Features>
class MovingWindow;

// Convenience shortcuts.
template <typename T>
using MovingMax = MovingWindow<T, MovingWindowFeatures::Max>;

template <typename T>
using MovingMin = MovingWindow<T, MovingWindowFeatures::Min>;

template <typename T>
using MovingMinMax =
    MovingWindow<T, MovingWindowFeatures::Min, MovingWindowFeatures::Max>;

template <typename T, typename SumType>
using MovingAverage = MovingWindow<T, MovingWindowFeatures::Mean<SumType>>;

template <typename T>
using MovingAverageDeviation =
    MovingWindow<T,
                 MovingWindowFeatures::Mean<T>,
                 MovingWindowFeatures::Deviation<double>>;

namespace internal {

// Class responsible only for calculating maximum in the window.
// It's reused to calculate both min and max via inverting the comparator.
template <typename T, typename Comparator>
class MovingExtremumBase {
 public:
  explicit MovingExtremumBase(size_t window_size)
      : window_size_(window_size),
        values_(window_size),
        added_at_(window_size),
        last_idx_(window_size - 1),
        compare_(Comparator()) {}
  ~MovingExtremumBase() = default;

  // Add new sample to the stream.
  void AddSample(const T& value, size_t total_added) {
    // Remove old elements from the back of the window;
    while (size_ > 0 && added_at_[begin_idx_] + window_size_ <= total_added) {
      ++begin_idx_;
      if (begin_idx_ == window_size_) {
        begin_idx_ = 0;
      }
      --size_;
    }
    // Remove small elements from the front of the window because they can never
    // become the maximum in the window since the currently added element is
    // bigger than them and will leave the window later.
    while (size_ > 0 && compare_(values_[last_idx_], value)) {
      if (last_idx_ == 0) {
        last_idx_ = window_size_;
      }
      --last_idx_;
      --size_;
    }
    DCHECK_LT(size_, window_size_);
    ++last_idx_;
    if (last_idx_ == window_size_) {
      last_idx_ = 0;
    }
    values_[last_idx_] = value;
    added_at_[last_idx_] = total_added;
    ++size_;
  }

  // Get the maximum of the last `window_size` elements.
  T Value() const {
    DCHECK_GT(size_, 0u);
    return values_[begin_idx_];
  }

  // Clear all samples.
  void Reset() {
    size_ = 0;
    begin_idx_ = 0;
    last_idx_ = window_size_ - 1;
  }

 private:
  const size_t window_size_;
  // Circular buffer with some values in the window.
  // Only possible candidates for maximum are stored:
  // values form a non-increasing sequence.
  std::vector<T> values_;
  // Circular buffer storing when numbers in `values_` were added.
  std::vector<size_t> added_at_;
  // Begin of the circular buffers above.
  size_t begin_idx_ = 0;
  // Last occupied position.
  size_t last_idx_;
  // How many elements are stored in the circular buffers above.
  size_t size_ = 0;
  // Template parameter comparator.
  const Comparator compare_;
};

// Null implementation of the above class to be used when feature is disabled.
template <typename T>
struct NullExtremumImpl {
  explicit NullExtremumImpl(size_t) {}
  ~NullExtremumImpl() = default;
  void AddSample(const T&, size_t) {}
  void Reset() {}
};

// Class to hold the moving window.
// It's used to calculate replaced element for Mean/Deviation calculations.
template <typename T>
class MovingWindowBase {
 public:
  explicit MovingWindowBase(size_t window_size) : values_(window_size) {}

  ~MovingWindowBase() = default;

  void AddSample(const T& sample) {
    values_[cur_idx_] = sample;
    ++cur_idx_;
    if (cur_idx_ == values_.size()) {
      cur_idx_ = 0;
    }
  }

  // Is the window filled integer amount of times.
  bool IsLastIdx() const { return cur_idx_ + 1 == values_.size(); }

  void Reset() {
    cur_idx_ = 0;
    std::fill(values_.begin(), values_.end(), T());
  }

  T GetValue() const { return values_[cur_idx_]; }

  T operator[](size_t idx) const { return values_[idx]; }

  size_t Size() const { return values_.size(); }

  // What index will be overwritten by a new element;
  size_t CurIdx() const { return cur_idx_; }

 private:
  // Circular buffer.
  std::vector<T> values_;
  // Where the buffer begins.
  size_t cur_idx_ = 0;
};

// Null implementation of the above class to be used when feature is disabled.
template <typename T>
struct NullWindowImpl {
  explicit NullWindowImpl(size_t) {}
  ~NullWindowImpl() = default;
  void AddSample(const T& sample) {}
  bool IsLastIdx() const { return false; }
  void Reset() {}
  T GetValue() const { return T(); }
};

// Performs division allowing the class to work with more types.
// General template.
template <typename SumType, typename ReturnType>
struct DivideInternal {
  static ReturnType Compute(const SumType& sum, const size_t count) {
    return static_cast<ReturnType>(sum) / static_cast<ReturnType>(count);
  }
};

// Class to calculate moving mean.
template <typename T, typename SumType, bool IsFloating>
class MovingMeanBase {
 public:
  explicit MovingMeanBase(size_t window_size) : sum_() {}

  ~MovingMeanBase() = default;

  void AddSample(const T& sample, const T& replaced_value, bool is_last_idx) {
    sum_ += sample - replaced_value;
  }

  template <typename ReturnType = SumType>
  ReturnType Mean(const size_t count) const {
    if (count == 0) {
      return ReturnType();
    }
    return DivideInternal<SumType, ReturnType>::Compute(sum_, count);
  }
  void Reset() { sum_ = SumType(); }

  SumType Sum() const { return sum_; }

 private:
  SumType sum_;
};

// Class to calculate moving mean.
// Variant for float types with running sum to avoid rounding errors
// accumulation.
template <typename T, typename SumType>
class MovingMeanBase<T, SumType, true> {
 public:
  explicit MovingMeanBase(size_t window_size) : sum_(), running_sum_() {}

  ~MovingMeanBase() = default;

  void AddSample(const T& sample, const T& replaced_value, bool is_last_idx) {
    running_sum_ += sample;
    if (is_last_idx) {
      // Replace sum with running sum to avoid rounding errors accumulation.
      sum_ = running_sum_;
      running_sum_ = SumType();
    } else {
      sum_ += sample - replaced_value;
    }
  }

  template <typename ReturnType = SumType>
  ReturnType Mean(const size_t count) const {
    if (count == 0) {
      return ReturnType();
    }
    return DivideInternal<SumType, ReturnType>::Compute(sum_, count);
  }

  void Reset() { sum_ = running_sum_ = SumType(); }

  SumType Sum() const { return sum_; }

 private:
  SumType sum_;
  SumType running_sum_;
};

// Null implementation of the above class to be used when feature is disabled.
template <typename T>
struct NullMeanImpl {
  explicit NullMeanImpl(size_t window_size) {}
  ~NullMeanImpl() = default;

  void AddSample(const T& sample, const T&, bool) {}

  void Reset() {}
};

// Computs main Deviation fromula, allowing the class to work with more types.
// Deviation is equal to mean of squared values minus squared mean value.
// General template.
template <typename SumType, typename ReturnType>
struct DeivationInternal {
  static ReturnType Compute(const SumType& sum_squares,
                            const SumType& square_of_sum,
                            const size_t count) {
    return static_cast<ReturnType>(
        std::sqrt((static_cast<double>(sum_squares) -
                   static_cast<double>(square_of_sum) / count) /
                  count));
  }
};

// Class to compute square of the number.
// General template
template <typename T, typename SquareType>
struct SquareInternal {
  static SquareType Compute(const T& sample) {
    return static_cast<SquareType>(sample) * sample;
  }
};

// Class to calculate moving deviation.
template <typename T, typename SumType, bool IsFloating>
class MovingDeviationBase {
 public:
  explicit MovingDeviationBase(size_t window_size) : sum_sq_() {}
  ~MovingDeviationBase() = default;
  void AddSample(const T& sample, const T& replaced_value, bool is_last_idx) {
    sum_sq_ += SquareInternal<T, SumType>::Compute(sample) -
               SquareInternal<T, SumType>::Compute(replaced_value);
  }

  template <typename ReturnType, typename U>
  ReturnType Deviation(const size_t count, const U& sum) const {
    if (count == 0) {
      return ReturnType();
    }
    return DeivationInternal<SumType, ReturnType>::Compute(
        sum_sq_, SquareInternal<U, SumType>::Compute(sum), count);
  }
  void Reset() { sum_sq_ = SumType(); }

 private:
  SumType sum_sq_;
};

// Class to calculate moving deviation.
// Variant for float types with running sum to avoid rounding errors
// accumulation.
template <typename T, typename SumType>
class MovingDeviationBase<T, SumType, true> {
 public:
  explicit MovingDeviationBase(size_t window_size)
      : sum_sq_(), running_sum_() {}
  ~MovingDeviationBase() = default;
  void AddSample(const T& sample, const T& replaced_value, bool is_last_idx) {
    SumType square = SquareInternal<T, SumType>::Compute(sample);
    running_sum_ += square;
    if (is_last_idx) {
      // Replace sum with running sum to avoid rounding errors accumulation.
      sum_sq_ = running_sum_;
      running_sum_ = SumType();
    } else {
      sum_sq_ += square - SquareInternal<T, SumType>::Compute(replaced_value);
    }
  }

  template <typename ReturnType, typename U>
  ReturnType Deviation(const size_t count, const U& sum) const {
    if (count == 0) {
      return ReturnType();
    }
    return DeivationInternal<SumType, ReturnType>::Compute(
        sum_sq_, SquareInternal<U, SumType>::Compute(sum), count);
  }
  void Reset() { running_sum_ = sum_sq_ = SumType(); }

 private:
  SumType sum_sq_;
  SumType running_sum_;
};

// Null implementation of the above class to be used when feature is disabled.
template <typename T>
struct NullDeviationImpl {
 public:
  explicit NullDeviationImpl(size_t window_size) {}
  ~NullDeviationImpl() = default;
  void AddSample(const T&, const T&, bool) {}
  void Reset() {}
};

// Template helpers.

// Gets all enabled features in one struct.
template <typename... Features>
struct EnabledFeatures : public Features... {};

template <typename T>
concept has_member_min = requires { T::has_min; };

template <typename T>
concept has_member_max = requires { T::has_max; };

template <typename T>
concept has_member_mean = requires { T::has_mean; };

template <typename T>
concept has_member_deviation = requires { T::has_deviation; };

template <typename T>
concept has_member_iteration = requires { T::has_iteration; };

// Gets the type of the member if present.
// Can't just use decltype, because the member might be absent.
template <typename T>
struct get_type_mean {
  using type = void;
};
template <typename T>
  requires has_member_mean<T>
struct get_type_mean<T> {
  using type = decltype(T::has_mean);
};
template <typename T>
using mean_t = typename get_type_mean<T>::type;

template <typename T>
struct get_type_deviation {
  using type = void;
};
template <typename T>
  requires has_member_deviation<T>
struct get_type_deviation<T> {
  using type = decltype(T::has_deviation);
};
template <typename T>
using deviation_t = typename get_type_deviation<T>::type;

// Performs division allowing the class to work with more types.
// Specific template for TimeDelta.
template <>
struct DivideInternal<TimeDelta, TimeDelta> {
  static TimeDelta Compute(const TimeDelta& sum, const size_t count) {
    return sum / count;
  }
};

// Computs main Deviation fromula, allowing the class to work with more types.
// Deviation is equal to mean of squared values minus squared mean value.
// Specific template for TimeDelta.
template <>
struct DeivationInternal<double, TimeDelta> {
  static TimeDelta Compute(const double sum_squares,
                           const double square_of_sum,
                           const size_t count) {
    return Seconds(std::sqrt((sum_squares - square_of_sum / count) / count));
  }
};

// Class to compute square of the number.
// Specific template for TimeDelta.
template <>
struct SquareInternal<TimeDelta, double> {
  static double Compute(const TimeDelta& sample) {
    return sample.InSecondsF() * sample.InSecondsF();
  }
};

}  // namespace internal

// Implementation of the main class.
template <typename T, typename... Features>
class MovingWindow {
 public:
  // List of all requested features.
  using EnabledFeatures = internal::EnabledFeatures<Features...>;

  explicit MovingWindow(size_t window_size)
      : min_impl_(window_size),
        max_impl_(window_size),
        mean_impl_(window_size),
        deviation_impl_(window_size),
        window_impl_(window_size) {}

  // Adds sample to the window.
  void AddSample(const T& sample) {
    ++total_added_;
    min_impl_.AddSample(sample, total_added_);
    max_impl_.AddSample(sample, total_added_);
    mean_impl_.AddSample(sample, window_impl_.GetValue(),
                         window_impl_.IsLastIdx());
    deviation_impl_.AddSample(sample, window_impl_.GetValue(),
                              window_impl_.IsLastIdx());
    window_impl_.AddSample(sample);
  }

  // Returns amount of elementes so far in the stream (might be bigger than the
  // window size).
  size_t Count() const { return total_added_; }

  // Calculates min in the window.
  T Min() const
    requires internal::has_member_min<EnabledFeatures>
  {
    return min_impl_.Value();
  }

  // Calculates max in the window.
  T Max() const
    requires internal::has_member_max<EnabledFeatures>
  {
    return max_impl_.Value();
  }

  // Calculates mean in the window.
  // `ReturnType` can be used to adjust the type of the calculated mean value;
  // if not specified, uses `T` by default.
  template <typename ReturnType = T>
    requires internal::has_member_mean<EnabledFeatures>
  ReturnType Mean() const {
    return mean_impl_.template Mean<ReturnType>(
        std::min(total_added_, window_impl_.Size()));
  }

  // Calculates deviation in the window.
  // `ReturnType` can be used to adjust the type of the calculated deviation
  // value; if not specified, uses `T` by default.
  template <typename ReturnType = T>
    requires internal::has_member_deviation<EnabledFeatures>
  ReturnType Deviation() const {
    const size_t count = std::min(total_added_, window_impl_.Size());
    return deviation_impl_.template Deviation<ReturnType>(count,
                                                          mean_impl_.Sum());
  }

  // Resets the state to an empty window.
  void Reset() {
    min_impl_.Reset();
    max_impl_.Reset();
    mean_impl_.Reset();
    deviation_impl_.Reset();
    window_impl_.Reset();
    total_added_ = 0;
  }

  // iterator implementation.
  class iterator {
   public:
    ~iterator() = default;

    const T operator*() {
      DCHECK_LT(idx_, window_impl_->Size());
      return (*window_impl_)[idx_];
    }

    iterator& operator++() {
      ++idx_;
      // Wrap around the circular buffer.
      if (idx_ == window_impl_->Size()) {
        idx_ = 0;
      }
      // The only way to arrive to the current element is to
      // come around after iterating through the whole window.
      if (idx_ == window_impl_->CurIdx()) {
        idx_ = kInvalidIndex;
      }
      return *this;
    }

    bool operator==(const iterator& other) const { return idx_ == other.idx_; }

   private:
    iterator(const internal::MovingWindowBase<T>& window, size_t idx)
        : window_impl_(window), idx_(idx) {}

    static const size_t kInvalidIndex = std::numeric_limits<size_t>::max();

    raw_ref<const internal::MovingWindowBase<T>> window_impl_;
    size_t idx_;

    friend class MovingWindow<T, Features...>;
  };

  // Begin iterator. Template to enable only if iteration feature is requested.
  iterator begin() const
    requires internal::has_member_iteration<EnabledFeatures>
  {
    if (total_added_ == 0) {
      return end();
    }
    // Before window is fully filled, the oldest element is at the index 0.
    size_t idx =
        (total_added_ < window_impl_.Size()) ? 0 : window_impl_.CurIdx();

    return iterator(window_impl_, idx);
  }

  // End iterator. Template to enable only if iteration feature is requested.
  iterator end() const
    requires internal::has_member_iteration<EnabledFeatures>
  {
    return iterator(window_impl_, iterator::kInvalidIndex);
  }

  // Size of the collection. Template to enable only if iteration feature is
  // requested.
  size_t size() const
    requires internal::has_member_iteration<EnabledFeatures>
  {
    return std::min(total_added_, window_impl_.Size());
  }

 private:
  // Member for calculating min.
  // Conditionally enabled on Min feature.
  std::conditional_t<internal::has_member_min<EnabledFeatures>,
                     internal::MovingExtremumBase<T, std::greater<>>,
                     internal::NullExtremumImpl<T>>
      min_impl_;

  // Member for calculating min.
  // Conditionally enabled on Min feature.
  std::conditional_t<internal::has_member_max<EnabledFeatures>,
                     internal::MovingExtremumBase<T, std::less<>>,
                     internal::NullExtremumImpl<T>>
      max_impl_;

  // Type for sum value in Mean implementation. Might need to reuse deviation
  // sum type, because enabling only deviation feature will also enable mean
  // member (because deviation calculation depends on mean calculation).
  using MeanSumType =
      std::conditional_t<internal::has_member_mean<EnabledFeatures>,
                         internal::mean_t<EnabledFeatures>,
                         internal::deviation_t<EnabledFeatures>>;
  // Member for calculating mean.
  // Conditionally enabled on Mean or Deviation feature (because deviation
  // calculation depends on mean calculation).
  std::conditional_t<
      internal::has_member_mean<EnabledFeatures> ||
          internal::has_member_deviation<EnabledFeatures>,
      internal::
          MovingMeanBase<T, MeanSumType, std::is_floating_point_v<MeanSumType>>,
      internal::NullMeanImpl<T>>
      mean_impl_;

  // Member for calculating deviation.
  // Conditionally enabled on Deviation feature.
  std::conditional_t<
      internal::has_member_deviation<EnabledFeatures>,
      internal::MovingDeviationBase<
          T,
          internal::deviation_t<EnabledFeatures>,
          std::is_floating_point_v<internal::deviation_t<EnabledFeatures>>>,
      internal::NullDeviationImpl<T>>
      deviation_impl_;

  // Member for storing the moving window.
  // Conditionally enabled on Mean, Deviation or Iteration feature since
  // they need the elements in the window.
  // Min and Max features store elements internally so they don't need this.
  std::conditional_t<internal::has_member_mean<EnabledFeatures> ||
                         internal::has_member_deviation<EnabledFeatures> ||
                         internal::has_member_iteration<EnabledFeatures>,
                     internal::MovingWindowBase<T>,
                     internal::NullWindowImpl<T>>
      window_impl_;
  // Total number of added elements.
  size_t total_added_ = 0;
};

}  // namespace base

#endif  // BASE_MOVING_WINDOW_H_
