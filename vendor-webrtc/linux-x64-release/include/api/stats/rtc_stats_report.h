/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_STATS_RTC_STATS_REPORT_H_
#define API_STATS_RTC_STATS_REPORT_H_

#include <stddef.h>
#include <stdint.h>

#include <map>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "api/ref_counted_base.h"
#include "api/scoped_refptr.h"
#include "api/stats/rtc_stats.h"
#include "api/units/timestamp.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// A collection of stats.
// This is accessible as a map from `RTCStats::id` to `RTCStats`.
class RTC_EXPORT RTCStatsReport final
    : public RefCountedNonVirtual<RTCStatsReport> {
 public:
  typedef std::map<std::string, std::unique_ptr<const RTCStats>> StatsMap;

  class RTC_EXPORT ConstIterator {
   public:
    ConstIterator(ConstIterator&& other);
    ~ConstIterator();

    ConstIterator& operator++();
    ConstIterator& operator++(int);
    const RTCStats& operator*() const;
    const RTCStats* operator->() const;
    bool operator==(const ConstIterator& other) const;
    bool operator!=(const ConstIterator& other) const;

   private:
    friend class RTCStatsReport;
    ConstIterator(const scoped_refptr<const RTCStatsReport>& report,
                  StatsMap::const_iterator it);

    // Reference report to make sure it is kept alive.
    scoped_refptr<const RTCStatsReport> report_;
    StatsMap::const_iterator it_;
  };

  static scoped_refptr<RTCStatsReport> Create(Timestamp timestamp);

  explicit RTCStatsReport(Timestamp timestamp);

  RTCStatsReport(const RTCStatsReport& other) = delete;
  scoped_refptr<RTCStatsReport> Copy() const;

  Timestamp timestamp() const { return timestamp_; }
  void AddStats(std::unique_ptr<const RTCStats> stats);
  // On success, returns a non-owning pointer to `stats`. If the stats ID is not
  // unique, `stats` is not inserted and nullptr is returned.
  template <typename T>
  T* TryAddStats(std::unique_ptr<T> stats) {
    T* stats_ptr = stats.get();
    if (!stats_
             .insert(std::make_pair(std::string(stats->id()), std::move(stats)))
             .second) {
      return nullptr;
    }
    return stats_ptr;
  }
  const RTCStats* Get(const std::string& id) const;
  size_t size() const { return stats_.size(); }

  // Gets the stat object of type `T` by ID, where `T` is any class descending
  // from `RTCStats`.
  // Returns null if there is no stats object for the given ID or it is the
  // wrong type.
  template <typename T>
  const T* GetAs(const std::string& id) const {
    const RTCStats* stats = Get(id);
    if (!stats || stats->type() != T::kType) {
      return nullptr;
    }
    return &stats->cast_to<const T>();
  }

  // Removes the stats object from the report, returning ownership of it or null
  // if there is no object with `id`.
  std::unique_ptr<const RTCStats> Take(const std::string& id);
  // Takes ownership of all the stats in `other`, leaving it empty.
  void TakeMembersFrom(scoped_refptr<RTCStatsReport> other);

  // Stats iterators. Stats are ordered lexicographically on `RTCStats::id`.
  ConstIterator begin() const;
  ConstIterator end() const;

  // Gets the subset of stats that are of type `T`, where `T` is any class
  // descending from `RTCStats`.
  template <typename T>
  std::vector<const T*> GetStatsOfType() const {
    std::vector<const T*> stats_of_type;
    for (const RTCStats& stats : *this) {
      if (stats.type() == T::kType)
        stats_of_type.push_back(&stats.cast_to<const T>());
    }
    return stats_of_type;
  }

  // Creates a JSON readable string representation of the report,
  // listing all of its stats objects.
  std::string ToJson() const;

 protected:
  friend class RefCountedNonVirtual<RTCStatsReport>;
  ~RTCStatsReport() = default;

 private:
  Timestamp timestamp_;
  StatsMap stats_;
};

}  // namespace webrtc

#endif  // API_STATS_RTC_STATS_REPORT_H_
