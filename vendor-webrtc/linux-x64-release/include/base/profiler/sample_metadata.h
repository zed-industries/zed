// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_SAMPLE_METADATA_H_
#define BASE_PROFILER_SAMPLE_METADATA_H_

#include <optional>
#include <string_view>

#include "base/base_export.h"
#include "base/profiler/metadata_recorder.h"
#include "base/threading/platform_thread.h"

// -----------------------------------------------------------------------------
// Usage documentation
// -----------------------------------------------------------------------------
//
// Overview:
// These functions provide a means to control the metadata attached to samples
// collected by the stack sampling profiler. SampleMetadataScope controls the
// scope covered by the metadata (thread, process).
//
// Any samples collected by the sampling profiler will include the active
// metadata. This enables us to later analyze targeted subsets of samples
// (e.g. those collected during paint or layout).
//
// For example:
//
//   void DidStartLoad() {
//     is_loading_metadata_.Set(1);
//   }
//
//   void DidFinishLoad() {
//     is_loading_metadata_.Remove();
//   }
//
//   base::SampleMetadata is_loading_metadata_;
//
// Alternatively, ScopedSampleMetadata can be used to ensure that the metadata
// is removed correctly.
//
// For example:
//
//   void DoExpensiveWork() {
//     base::ScopedSampleMetadata metadata("xyz", 1);
//     if (...) {
//       ...
//       if (...) {
//         ...
//         return;
//       }
//     }
//     ...
//   }

namespace base {

class TimeTicks;

enum class SampleMetadataScope {
  // All threads in the current process will have the associated metadata
  // attached to their samples.
  kProcess,
  // The metadata will only be attached to samples for the current thread.
  kThread
};

class BASE_EXPORT SampleMetadata {
 public:
  // Set the metadata value associated with |name| to be recorded for |scope|.
  explicit SampleMetadata(std::string_view name, SampleMetadataScope scope);

  SampleMetadata(const SampleMetadata&) = default;
  ~SampleMetadata() = default;

  SampleMetadata& operator=(const SampleMetadata&) = delete;

  // Set the metadata value associated with |name| in the process-global stack
  // sampling profiler metadata, overwriting any previous value set for that
  // |name|.
  void Set(int64_t value);

  // Set the metadata value associated with the pair (|name|, |key|) in the
  // process-global stack sampling profiler metadata, overwriting any previous
  // value set for that (|name|, |key|) pair. This constructor allows the
  // metadata to be associated with an additional user-defined key. One might
  // supply a key based on the frame id, for example, to distinguish execution
  // in service of scrolling between different frames. Prefer the previous
  // function if no user-defined metadata is required. Note: values specified
  // for a name and key are stored separately from values specified with only a
  // name.
  void Set(int64_t key, int64_t value);

  // Removes the metadata item with the specified name from the process-global
  // stack sampling profiler metadata.
  //
  // If such an item doesn't exist, this has no effect.
  void Remove();

  // Removes the metadata item with the specified (|name|, |key|) pair from the
  // process-global stack sampling profiler metadata. This function does not
  // alter values set with the name |name| but no key.
  //
  // If such an item doesn't exist, this has no effect.
  void Remove(int64_t key);

 private:
  const uint64_t name_hash_;
  // Scope is kept as-is instead of retrieving a PlatformThreadId in case
  // Set()/Remove() is called on a thread different from where the object was
  // constructed.
  const SampleMetadataScope scope_;
};

class BASE_EXPORT ScopedSampleMetadata {
 public:
  // Set the metadata value associated with |name| for |scope|.
  ScopedSampleMetadata(std::string_view name,
                       int64_t value,
                       SampleMetadataScope scope);

  // Set the metadata value associated with the pair (|name|, |key|) for
  // |scope|. This constructor allows the metadata to be associated with an
  // additional user-defined key. One might supply a key based on the frame id,
  // for example, to distinguish execution in service of scrolling between
  // different frames. Prefer the previous constructor if no user-defined
  // metadata is required. Note: values specified for a name and key are stored
  // separately from values specified with only a name.
  ScopedSampleMetadata(std::string_view name,
                       int64_t key,
                       int64_t value,
                       SampleMetadataScope scope);

  ScopedSampleMetadata(const ScopedSampleMetadata&) = delete;
  ~ScopedSampleMetadata();

  ScopedSampleMetadata& operator=(const ScopedSampleMetadata&) = delete;

 private:
  const uint64_t name_hash_;
  std::optional<int64_t> key_;
  std::optional<PlatformThreadId> thread_id_;
};

// Applies the specified metadata to samples already recorded between
// |period_start| and |period_end| in all thread's active profiles, subject to
// the condition that the profile fully encompasses the period and the profile
// has not already completed. The condition ensures that the metadata is applied
// only if all execution during its scope was seen in the profile. This avoids
// biasng the samples towards the 'middle' of the execution seen during the
// metadata scope (i.e. because the start or end of execution was missed), at
// the cost of missing execution that are longer than the profiling period, or
// extend before or after it. |period_end| must be <= TimeTicks::Now().
BASE_EXPORT void ApplyMetadataToPastSamples(TimeTicks period_start,
                                            TimeTicks period_end,
                                            std::string_view name,
                                            int64_t value,
                                            SampleMetadataScope scope);
BASE_EXPORT void ApplyMetadataToPastSamples(TimeTicks period_start,
                                            TimeTicks period_end,
                                            std::string_view name,
                                            int64_t key,
                                            int64_t value,
                                            SampleMetadataScope scope);

// Adds metadata as metadata global to the sampling profile. Has the effect of
// applying the metadata to all samples in the profile, even ones collected
// earlier in time. This is probably not what you want for most use cases;
// prefer using SampleMetadata / ScopedSampleMetadata /
// ApplyMetadataToPastSamples instead.
BASE_EXPORT void AddProfileMetadata(std::string_view name,
                                    int64_t key,
                                    int64_t value,
                                    SampleMetadataScope scope);

// Adds metadata as metadata global to the sampling profile for another thread
// other than calling thread. Has the effect of applying the metadata to all
// samples in the profile, even ones collected earlier in time. This is probably
// not what you want for most use cases; prefer using SampleMetadata /
// ScopedSampleMetadata / ApplyMetadataToPastSamples instead.
BASE_EXPORT void AddProfileMetadataForThread(std::string_view name,
                                             int64_t value,
                                             PlatformThreadId other_thread);

// Returns the process-global metadata recorder instance used for tracking
// sampling profiler metadata.
//
// This function should not be called by non-profiler related code.
BASE_EXPORT MetadataRecorder* GetSampleMetadataRecorder();

}  // namespace base

#endif  // BASE_PROFILER_SAMPLE_METADATA_H_
