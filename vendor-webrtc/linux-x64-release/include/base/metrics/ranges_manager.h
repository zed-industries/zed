// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_RANGES_MANAGER_H_
#define BASE_METRICS_RANGES_MANAGER_H_

#include <unordered_set>
#include <vector>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/metrics/bucket_ranges.h"
#include "base/synchronization/lock.h"

namespace base {

// Manages BucketRanges and their lifetime. When registering a BucketRanges
// to a RangesManager instance, if an equivalent one already exists (one with
// the exact same ranges), the passed BucketRanges is deleted. This is useful to
// prevent duplicate instances of equivalent BucketRanges. Upon the destruction
// of a RangesManager instance, all BucketRanges managed by it are destroyed. A
// BucketRanges instance should not be registered to multiple RangesManagers.
class BASE_EXPORT RangesManager {
 public:
  RangesManager();

  RangesManager(const RangesManager&) = delete;
  RangesManager& operator=(const RangesManager&) = delete;

  virtual ~RangesManager();

  // Gets the canonical BucketRanges object corresponding to `ranges`. If one
  // does not exist, then `ranges` will be registered with this object, which
  // will take ownership of it. Returns a pointer to the canonical ranges
  // object. If it's different than `ranges`, the caller is responsible for
  // deleting `ranges`.
  virtual const BucketRanges* GetOrRegisterCanonicalRanges(
      const BucketRanges* ranges);

  // Gets all registered BucketRanges. The order of returned BucketRanges is not
  // guaranteed.
  virtual std::vector<const BucketRanges*> GetBucketRanges() const;

  // Some tests may instantiate temporary StatisticsRecorders, each having their
  // own RangesManager. During the tests, ranges may get registered with a
  // recorder that later gets released, which would release the ranges as well.
  // Calling this method prevents this, as the tests may not expect them to be
  // deleted.
  void DoNotReleaseRangesOnDestroyForTesting();

 protected:
  // Used to get the hash of a BucketRanges, which is simply its checksum.
  struct BucketRangesHash {
    size_t operator()(const BucketRanges* a) const;
  };

  // Comparator for BucketRanges. See `BucketRanges::Equals()`.
  struct BucketRangesEqual {
    bool operator()(const BucketRanges* a, const BucketRanges* b) const;
  };

  // Type for a set of unique RangesBucket, with their hash and equivalence
  // defined by `BucketRangesHash` and `BucketRangesEqual`.
  typedef std::unordered_set<raw_ptr<const BucketRanges, CtnExperimental>,
                             BucketRangesHash,
                             BucketRangesEqual>
      RangesMap;

  // Removes all registered BucketRanges and destroys them. This is called in
  // the destructor.
  virtual void ReleaseBucketRanges();

  virtual RangesMap& GetRanges();
  virtual const RangesMap& GetRanges() const;

 private:
  // The set of unique BucketRanges registered to the RangesManager.
  RangesMap ranges_;

  // Whether or not to release the registered BucketRanges when this
  // RangesManager is destroyed. See `DoNotReleaseRangesOnDestroyForTesting()`.
  bool do_not_release_ranges_on_destroy_for_testing_ = false;
};

class BASE_EXPORT ThreadSafeRangesManager final : public RangesManager {
 public:
  ThreadSafeRangesManager();

  ThreadSafeRangesManager(const RangesManager&) = delete;
  ThreadSafeRangesManager& operator=(const ThreadSafeRangesManager&) = delete;

  ~ThreadSafeRangesManager() override;

  // RangesManager:
  const BucketRanges* GetOrRegisterCanonicalRanges(
      const BucketRanges* ranges) override;
  std::vector<const BucketRanges*> GetBucketRanges() const override;

 protected:
  // RangesManager:
  void ReleaseBucketRanges() override;
  RangesMap& GetRanges() override;
  const RangesMap& GetRanges() const override;

 private:
  // Used to protect access to |ranges_|.
  mutable base::Lock lock_;
};

}  // namespace base

#endif  // BASE_METRICS_RANGES_MANAGER_H_
