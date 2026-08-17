// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_BACKGROUND_TRACING_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_BACKGROUND_TRACING_HELPER_H_

#include <cstdint>
#include <string>
#include <string_view>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExecutionContext;
class PerformanceMark;

// The following class is a helper for implementing the background-tracing
// performance.mark integration. See crbug.com/1181774 for details, and refer
// to integration with performance.cc.

// This is CORE_EXPORT for component builds of blink_unittests.
class CORE_EXPORT BackgroundTracingHelper final
    : public GarbageCollected<BackgroundTracingHelper> {
 public:
  using SiteHashSet = HashSet<uint32_t>;

  explicit BackgroundTracingHelper(ExecutionContext* context);
  ~BackgroundTracingHelper();

  void MaybeEmitBackgroundTracingPerformanceMarkEvent(
      const PerformanceMark& mark);

  // Implements GarbageCollected:
  void Trace(Visitor*) const;

  // Parses a comma separated `allow_list` composed of `target_site_hash`
  // (`MD5Hash32()` of eTLD+1 represented in ASCII) and returns a set. This uses
  // std::string because it is interacting with Finch code, which doesn't use
  // WTF primitives.
  static SiteHashSet ParsePerformanceMarkSiteHashes(
      std::string_view allow_list);

 protected:
  friend class BackgroundTracingHelperTest;

  // Returns a reference to a thread-safe static singleton `SiteHashSet`,
  // with all of the configured allow-listed sites and marks. This object is
  // populated with parsed data upon first access.
  static const SiteHashSet& GetSiteHashSet();

  // Splits a string and an optional numeric suffix preceded by an
  // underscore. Returns the location of the underscore if a split is to
  // occur, otherwise returns 0.
  static size_t GetIdSuffixPos(StringView string);

  // Generates a 32-bit MD5 hash of the given string piece. This will return a
  // value that is equivalent to the first 8 bytes of a full MD5 hash. In bash
  // parlance, the returned 32-bit integer expressed in hex format:
  //
  //   printf "%08x" <hashed_value>
  //
  // will have the same value as
  //
  //   echo -n <string_value> | md5sum | cut -b 1-8
  //
  // This will return the same result as MD5Hash32Constexpr as defined in
  // base/hash/md5_constexpr.h. This uses std::string_view because it is
  // interacting with Finch code, which doesn't use WTF primitives.
  static uint32_t MD5Hash32(std::string_view string);

  // Given a mark name with an optional numeric suffix, parses out the base name
  // and suffix.
  static std::pair<StringView, std::optional<uint32_t>> SplitMarkNameAndId(
      StringView mark_name);

 private:
  std::string site_;
  uint32_t site_hash_ = 0;
  uint32_t execution_context_id_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_BACKGROUND_TRACING_HELPER_H_
