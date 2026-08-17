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

// Utilities for rounding and other number-related operations.
//
// NOTE: These functions do not handle over/underflow.

#ifndef AUDIO_DSP_NUMBER_UTIL_H_
#define AUDIO_DSP_NUMBER_UTIL_H_

#include <iterator>
#include <utility>
#include <vector>


#include "glog/logging.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Computes value - floor(value / modulus) * modulus.  This is similar to the
// standard remainder operator %, but the result is always in {0, ...,
// modulus - 1}. The modulus must be positive.
int Modulo(int value, int modulus);

// Round down to nearest integer multiple of a positive factor, equivalent to
// floor(value / factor) * factor.
int RoundDownToMultiple(int value, int factor);

// Round up to nearest integer multiple of a positive factor, equivalent to
// ceil(value / factor) * factor.
int RoundUpToMultiple(int value, int factor);

// Compute the greatest common divisor (GCD) using Euclid's algorithm. This
// 10-line function is essentially a copy of MathUtil::GCD() to avoid an
// otherwise unneeded dependency on mobile platforms.
int GreatestCommonDivisor(int a, int b);
int GreatestCommonDivisor(const std::vector<int>& a);

// Returns true if value is an integer power of 2 or zero.
inline bool IsPowerOfTwoOrZero(unsigned value) {
  return (value & (value - 1)) == 0;
}

// Compute floor(log2(value)). Returns -1 if value == 0.
int Log2Floor(unsigned value);

// Compute ceil(log2(value)).
// TODO: Replace with Bits::Log2Ceiling.
int Log2Ceiling(unsigned value);

// Find the minimum power of 2 >= value.
unsigned NextPowerOfTwo(unsigned value);

// ArithmeticSequence represents double-valued arithmetic sequences like
// {5.0, 5.1, 5.2, ..., 6.0} having the form (base + step * i). Sequence
// elements are generated on the fly rather than storing them. To get the
// elements as a vector, use the CopyTo method.
//
// The semantics are the same as Matlab's "base:step:limit" syntax, with the
// limit included as the last element of the sequence if it is a whole number
// of steps away from base:
//   ArithmeticSequence(1.2, 0.1, 1.5)   => {1.2, 1.3, 1.4, 1.5},
//   ArithmeticSequence(1.2, 0.1, 1.59)  => {1.2, 1.3, 1.4, 1.5},
//   ArithmeticSequence(1.5, -0.1, 1.2)  => {1.5, 1.4, 1.3, 1.2},
//   ArithmeticSequence(1.2, -0.1, 1.5)  => {} (empty).
//
// Care is taken with respect to numerical accuracy so that the size of the
// sequence is reliably computed. If base, step, or limit are within 4 epsilon
// relative difference of some base', step', limit' for which (limit' - base') /
// step' is integer, we compute the size to be that integer. The size satisfies
//   base + step * (size - 1) <= limit + tol < base + step * size
// for step > 0 (similarly for step < 0) where
//   tol = min(epsilon*(5*(|base| + |limit|) + 7*|limit - base|, |step|/3).
// and epsilon is machine double epsilon (about 2.22e-16). Beware that results
// are unreliable if |step|/3 is the smaller quantity in the above min.
//
// Examples:
//   // ArithmeticSequence has vector-like accessors.
//   ArithmeticSequence seq(5.0, 0.1, 6.0);
//   for (int i = 0; i < seq.size(); ++i) {
//     double value = seq[i];
//   }
//
//   // Ranged for loop iteration is supported too.
//   for (double value : seq) {
//     // Successive iterates are value = 5.0, 5.1, 5.2, ..., 6.0.
//   }
class ArithmeticSequence {
 public:
  class Iterator;
  typedef double value_type;
  typedef Iterator iterator;
  typedef Iterator const_iterator;
  typedef int size_type;
  typedef int difference_type;

  // Default constructor makes an empty sequence.
  ArithmeticSequence(): base_(0.0), step_(1.0), limit_(0.0), size_(0) {}

  // Construct the sequence base + step * i. The size of the sequence is such
  // that it ends on or just before limit.
  // NOTE: All arguments must be finite and step must be nonzero.
  ArithmeticSequence(double base, double step, double limit);

  double base() const { return base_; }
  double step() const { return step_; }
  double limit() const { return limit_; }
  int size() const { return size_; }
  bool empty() const { return size_ <= 0; }

  double operator[](int i) const {
    ABSL_DCHECK(0 <= i && i < size_) << "Out of bounds index " << i
        << " in ArithmeticSequence of size " << size_ << '.';
    return (i < size_ - 1) ? base_ + step_ * i : limit_;
  }

  // Copy elements to a vector or other container having an assign() method.
  template <typename ContainerType>
  void CopyTo(ContainerType* output) const {
    // Prevent accidentally copying unreasonably large sizes.
    constexpr int kMaxSafeSize = 1e9;
    ABSL_CHECK_LE(size_, kMaxSafeSize);
    output->assign(begin(), end());
  }

  // Define an iterator yielding double. This is a "RandomAccessIterator"
  // category iterator [http://en.cppreference.com/w/cpp/iterator].
  class Iterator: public std::iterator<
      std::random_access_iterator_tag, double> {
   public:
    Iterator(const ArithmeticSequence& range, int index)
        : range_(&range), index_(index) {}

    double operator*() const { return (*range_)[index_]; }
    double operator[](int n) const { return (*range_)[index_ + n]; }

    Iterator& operator++() { return ++index_, *this; }
    Iterator operator++(int /* postincrement */);

    Iterator& operator--() { return --index_, *this; }
    Iterator operator--(int /* postdecrement */);

    Iterator& operator+=(int n) { return index_ += n, *this; }
    Iterator operator+(int n) const;

    Iterator& operator-=(int n) { return index_ -= n, *this; }
    Iterator operator-(int n) const;

    int operator-(const Iterator& rhs) const { return index_ - rhs.index_; }

    bool operator==(const Iterator& rhs) const {
      return index_ == rhs.index_ && range_ == rhs.range_;
    }
    bool operator!=(const Iterator& rhs) const { return !(*this == rhs); }

   private:
    const ArithmeticSequence* range_;  // Not owned.
    int index_;
  };

  Iterator begin() const { return Iterator(*this, 0); }
  Iterator end() const { return Iterator(*this, size_); }

 private:
  double base_;
  double step_;
  double limit_;
  int size_;
};

inline ArithmeticSequence::Iterator operator+(
    int n, const ArithmeticSequence::Iterator& it) {
  return it + n;
}

// Iterator over combinations (selection with replacement).
class CombinationsIterator {
 public:
  // Constructor for iterating over k-element combinations of n items. Each
  // combination is given as a vector of indices by GetCurrentCombination().
  CombinationsIterator(int n, int k);

  // Get the current combination as a vector of indices of size k.
  inline const std::vector<int>& GetCurrentCombination() const {
    return indices_;
  }

  // Returns true if there are no more combinations.
  inline bool Done() const {
    return is_done_;
  }

  // Advance to the next combination in lexographical order.
  void Next();

 private:
  const int n_;
  const int k_;
  std::vector<int> indices_;
  bool is_done_;
};

// Build a table of all k-element combinations from a vector of values.
template <typename ValueType>
std::vector<std::vector<ValueType>> BuildCombinationsTable(
    const std::vector<ValueType>& values, int k) {
  std::vector<std::vector<ValueType>> table;
  for (CombinationsIterator it(static_cast<int>(values.size()), k);
       !it.Done();
       it.Next()) {
    std::vector<ValueType> table_entry;
    for (int index : it.GetCurrentCombination()) {
      table_entry.push_back(values[index]);
    }
    table.push_back(table_entry);
  }
  return table;
}

// Given a vector<int> shape, iterate over the cross product
//   {0, 1, ..., shape[0] - 1} x {0, 1, ..., shape[1] - 1} x ...
//      x {0, 1, ..., shape.back() - 1}.
// Equivalently, this class implements the variable nested loop
//   for (iN = 0; iN < shape.back(); ++iN) {
//     ...
//     for (i1 = 0; i1 < shape[1]; ++i1) {
//       for (i0 = 0; i0 < shape[0]; ++i0) {
//         // Loop body.
//       }
//     }
//     ...
//   }
//
// Beware that the total number of iterations is potentially extremely large.
//
// This class is intended to be used with a ranged for loop.
//
// Example use:
//   vector<int> shape = ...
//   for (const vector<int>& indices : CrossProductRange(shape)) {
//     ...
//   }
class CrossProductRange {
 public:
  class Iterator;
  typedef std::vector<int> value_type;
  typedef Iterator iterator;
  typedef Iterator const_iterator;

  // Construct range over shape, a vector of nonnegative integers specifying the
  // dimensions of the hyperrectangle to iterate. Zero-valued dimensions are
  // allowed and imply an empty range. The shape vector may be any size,
  // including empty.
  explicit CrossProductRange(const std::vector<int>& shape);

  const std::vector<int>& shape() const { return shape_; }
  bool empty() const;

  // Define an iterator yielding vector<int>. This is a "ForwardIterator"
  // category iterator [http://en.cppreference.com/w/cpp/iterator]. This
  // iterator is sufficient for use in a ranged for loop.
  class Iterator
      : public std::iterator<std::forward_iterator_tag, std::vector<int>> {
   public:
    // Construct "begin" iterator.
    explicit Iterator(const CrossProductRange& range);
    // Construct "end" iterator.
    Iterator(): is_end_(true) {}
    // Construct iterator at a specified flat index.
    Iterator(const CrossProductRange& range, int64 flat_index);

    // Get the current iterate, a vector of indices of size shape.size(). The
    // first index changes fastest between successive iterates.
    const std::vector<int>& operator*() const { return indices_; }

    Iterator& operator++();

    Iterator operator++(int /* postincrement */);

    bool operator==(const Iterator& other) const;

    bool operator!=(const Iterator& other) const { return !(*this == other); }

   private:
    std::vector<int> shape_;
    std::vector<int> indices_;
    bool is_end_;
  };

  Iterator begin() const { return Iterator(*this); }
  Iterator end() const { return Iterator(); }

  // Get iterator at a specified flat index. FlatIndex(n) is equivalent to (but
  // more efficient than) creating a begin() iterator and incrementing n times.
  Iterator FlatIndex(int64 flat_index) const {
    return Iterator(*this, flat_index);
  }

 private:
  std::vector<int> shape_;
};

// Approximate a given real number with a rational, a ratio a/b of integers a, b
// with 0 < b <= max_denominator, i.e., a Diophantine approximation
// [https://en.wikipedia.org/wiki/Diophantine_approximation]. The returned
// rational a/b minimizes the error |value - a/b| among all rationals a'/b' with
// 0 < b' <= max_denominator, and a/b is in reduced form (a and b have no common
// factor).
//
// Rational approximation error was evaluated empirically over a logarithmic
// sweep of max_denominator from 1 to 10^6. For each value of max_denominator,
// the worst-case (max) and average approximation errors were computed over 10^7
// random values selected uniformly over [-1, 1]. The worst approximation error
// |value - a/b| is 0.5 / max_denominator, the same as the max error obtained by
// direct quantization a/b with b = max_denominator, a = round(value * b).
// However, for most values the approximation is much better with an average
// approximation error of about max_denominator^-1.88.
//
// Algorithm:
// The given value is expanded in continued fraction representation, e.g.,
// pi = [3; 7, 15, 1, 292, ...] which is concise notation for pi =
// 3 + 1/(7 + 1/(15 + 1/(1 + 1/(292 + ...)))). Rational approximations called
// convergents are made by truncating the continued fraction, e.g. pi is about
// 3 + 1/7 = 22/7. Each convergent a/b is a best rational approximation, meaning
// that a/b is closer than any other ratio with a smaller denominator,
//   |value - a/b| < |value - a'/b'|  for any a'/b' != a/b, b' <= b.
// However, the convergents are not all of the best rational approximations. To
// cover them all, we consider also intermediate fractions, "semiconvergents,"
// between the convergents.
//
// To illustrate how convergents and semiconvergents provide progressively finer
// approximations, the table lists all best rational approximations of pi with
// denominator up to 106 [http://oeis.org/A063674 and http://oeis.org/A063673].
//
//                                   a/b  |pi - a/b|
//                               -------------------
//   Convergent [3;]           =     3/1      1.4e-1
//   Semiconvergent [3; 4]     =    13/4      1.1e-1
//   Semiconvergent [3; 5]     =    16/5      5.8e-2
//   Semiconvergent [3; 6]     =    19/6      2.5e-2
//   Convergent [3; 7]         =    22/7      1.3e-3
//   Semiconvergent [3; 7, 8]  =  179/57      1.2e-3
//   Semiconvergent [3; 7, 9]  =  201/64      9.7e-4
//   Semiconvergent [3; 7, 10] =  223/71      7.5e-4
//   Semiconvergent [3; 7, 11] =  245/78      5.7e-4
//   Semiconvergent [3; 7, 12] =  267/85      4.2e-4
//   Semiconvergent [3; 7, 13] =  289/92      2.9e-4
//   Semiconvergent [3; 7, 14] =  311/99      1.8e-4
//   Convergent [3; 7, 15]     = 333/106      8.3e-5
//
// For a given max_denominator, semiconvergents are useful since they may be
// more accurate than the best convergent. E.g. with max_denominator = 100, the
// best possible approximation of pi is the semiconvergent 311/99 with error
// 1.8e-4, while the best convergent is 22/7 with error 1.3e-3.
//
// Reference:
// https://en.wikipedia.org/wiki/Continued_fraction#Best_rational_approximations
std::pair<int, int> RationalApproximation(double value, int max_denominator);

}  // namespace audio_dsp

#endif  // AUDIO_DSP_NUMBER_UTIL_H_
