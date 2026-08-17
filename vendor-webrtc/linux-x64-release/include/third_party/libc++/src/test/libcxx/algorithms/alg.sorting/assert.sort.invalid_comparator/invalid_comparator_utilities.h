//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_LIBCXX_ALGORITHMS_ALG_SORTING_ASSERT_SORT_INVALID_COMPARATOR_INVALID_COMPARATOR_UTILITIES_H
#define TEST_LIBCXX_ALGORITHMS_ALG_SORTING_ASSERT_SORT_INVALID_COMPARATOR_INVALID_COMPARATOR_UTILITIES_H

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <map>
#include <ranges>
#include <set>
#include <string>
#include <string_view>
#include <vector>

class ComparisonResults {
public:
  explicit ComparisonResults(std::string_view data) {
    for (auto line :
         std::views::split(data, '\n') | std::views::filter([](auto const& line) { return !line.empty(); })) {
      auto values                     = std::views::split(line, ' ');
      auto it                         = values.begin();
      std::size_t left                = std::stol(std::string((*it).data(), (*it).size()));
      it                              = std::next(it);
      std::size_t right               = std::stol(std::string((*it).data(), (*it).size()));
      it                              = std::next(it);
      bool result                     = static_cast<bool>(std::stol(std::string((*it).data(), (*it).size())));
      comparison_results[left][right] = result;
    }
  }

  bool compare(size_t* left, size_t* right) const {
    assert(left != nullptr && right != nullptr && "something is wrong with the test");
    assert(comparison_results.contains(*left) && comparison_results.at(*left).contains(*right) &&
           "malformed input data?");
    return comparison_results.at(*left).at(*right);
  }

  size_t size() const { return comparison_results.size(); }

private:
  std::map<std::size_t, std::map<std::size_t, bool>>
      comparison_results; // terrible for performance, but really convenient
};

class SortingFixture {
public:
  explicit SortingFixture(std::string_view data) : comparison_results_(data) {
    for (std::size_t i = 0; i != comparison_results_.size(); ++i) {
      elements_.push_back(std::make_unique<std::size_t>(i));
      valid_ptrs_.insert(elements_.back().get());
    }
  }

  std::vector<std::size_t*> create_elements() {
    std::vector<std::size_t*> copy;
    for (auto const& e : elements_)
      copy.push_back(e.get());
    return copy;
  }

  auto checked_predicate() {
    return [this](size_t* left, size_t* right) {
      // If the pointers passed to the comparator are not in the set of pointers we
      // set up above, then we're being passed garbage values from the algorithm
      // because we're reading OOB.
      assert(valid_ptrs_.contains(left));
      assert(valid_ptrs_.contains(right));
      return comparison_results_.compare(left, right);
    };
  }

private:
  ComparisonResults comparison_results_;
  std::vector<std::unique_ptr<std::size_t>> elements_;
  std::set<std::size_t*> valid_ptrs_;
};

#endif // TEST_LIBCXX_ALGORITHMS_ALG_SORTING_ASSERT_SORT_INVALID_COMPARATOR_INVALID_COMPARATOR_UTILITIES_H
