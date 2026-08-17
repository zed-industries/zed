//===- record_section_tracker.h -- for fixed-sized record sects -*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// RecordSectionsTracker: Responsible for managing sections of metadata records
// with fixed sizes.
//
//===----------------------------------------------------------------------===//

#ifndef ORC_RT_RECORD_SECTION_TRACKER_H
#define ORC_RT_RECORD_SECTION_TRACKER_H

#include "error.h"
#include "executor_address.h"
#include <algorithm>
#include <vector>

namespace orc_rt {

/// Used to manage sections of fixed-sized metadata records (e.g. pointer
/// sections, selector refs, etc.)
template <typename RecordElement> class RecordSectionsTracker {
public:
  /// Add a section to the "new" list.
  void add(span<RecordElement> Sec) { New.push_back(std::move(Sec)); }

  /// Returns true if there are new sections to process.
  bool hasNewSections() const { return !New.empty(); }

  /// Returns the number of new sections to process.
  size_t numNewSections() const { return New.size(); }

  /// Process all new sections.
  template <typename ProcessSectionFunc>
  std::enable_if_t<std::is_void_v<
      std::invoke_result_t<ProcessSectionFunc, span<RecordElement>>>>
  processNewSections(ProcessSectionFunc &&ProcessSection) {
    for (auto &Sec : New)
      ProcessSection(Sec);
    moveNewToProcessed();
  }

  /// Proces all new sections with a fallible handler.
  ///
  /// Successfully handled sections will be moved to the Processed
  /// list.
  template <typename ProcessSectionFunc>
  std::enable_if_t<
      std::is_same_v<
          Error, std::invoke_result_t<ProcessSectionFunc, span<RecordElement>>>,
      Error>
  processNewSections(ProcessSectionFunc &&ProcessSection) {
    for (size_t I = 0; I != New.size(); ++I) {
      if (auto Err = ProcessSection(New[I])) {
        for (size_t J = 0; J != I; ++J)
          Processed.push_back(New[J]);
        New.erase(New.begin(), New.begin() + I);
        return Err;
      }
    }
    moveNewToProcessed();
    return Error::success();
  }

  /// Move all sections back to New for reprocessing.
  void reset() {
    moveNewToProcessed();
    New = std::move(Processed);
  }

  /// Remove the section with the given range.
  bool removeIfPresent(ExecutorAddrRange R) {
    if (removeIfPresent(New, R))
      return true;
    return removeIfPresent(Processed, R);
  }

private:
  void moveNewToProcessed() {
    if (Processed.empty())
      Processed = std::move(New);
    else {
      Processed.reserve(Processed.size() + New.size());
      std::copy(New.begin(), New.end(), std::back_inserter(Processed));
      New.clear();
    }
  }

  bool removeIfPresent(std::vector<span<RecordElement>> &V,
                       ExecutorAddrRange R) {
    auto RI = std::find_if(
        V.rbegin(), V.rend(),
        [RS = R.toSpan<RecordElement>()](const span<RecordElement> &E) {
          return E.data() == RS.data();
        });
    if (RI != V.rend()) {
      V.erase(std::next(RI).base());
      return true;
    }
    return false;
  }

  std::vector<span<RecordElement>> Processed;
  std::vector<span<RecordElement>> New;
};

} // namespace orc_rt

#endif // ORC_RT_RECORD_SECTION_TRACKER_H
