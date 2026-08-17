/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL GOOGLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_DOCUMENT_LOAD_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_DOCUMENT_LOAD_TIMING_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/public/mojom/confidence_level.mojom-blink.h"
#include "third_party/blink/public/mojom/navigation/system_entropy.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value.h"

namespace base {
class Clock;
class TickClock;
}  // namespace base

namespace blink {

class DocumentLoader;
class KURL;
class LocalFrame;

using RandomizedConfidenceValue =
    std::pair<double, mojom::blink::ConfidenceLevel>;

// The values need to outlive DocumentLoadTiming for metrics reporting.
struct DocumentLoadTimingValues final
    : public GarbageCollected<DocumentLoadTimingValues> {
  base::TimeTicks unload_event_start;
  base::TimeTicks unload_event_end;
  base::TimeTicks redirect_start;
  base::TimeTicks redirect_end;
  base::TimeTicks fetch_start;
  base::TimeTicks response_end;
  base::TimeTicks load_event_start;
  base::TimeTicks load_event_end;
  base::TimeTicks activation_start;
  base::TimeTicks critical_ch_restart;

  uint16_t redirect_count = 0;
  bool has_cross_origin_redirect = false;
  bool can_request_from_previous_document = false;

  mojom::blink::SystemEntropy system_entropy_at_navigation_start =
      mojom::blink::SystemEntropy::kNormal;
  std::optional<RandomizedConfidenceValue> randomized_confidence;

  void Trace(Visitor*) const {}
};

class CORE_EXPORT DocumentLoadTiming final {
  DISALLOW_NEW();

 public:
  explicit DocumentLoadTiming(DocumentLoader&);

  base::TimeDelta MonotonicTimeToZeroBasedDocumentTime(base::TimeTicks) const;
  int64_t ZeroBasedDocumentTimeToMonotonicTime(double dom_event_time) const;
  base::TimeDelta MonotonicTimeToPseudoWallTime(base::TimeTicks) const;

  void MarkNavigationStart();
  void SetNavigationStart(base::TimeTicks);
  void SetBackForwardCacheRestoreNavigationStart(base::TimeTicks);
  void MarkCommitNavigationEnd();

  void SetInputStart(base::TimeTicks);

  void SetUserTimingMarkFullyLoaded(base::TimeDelta);
  void SetUserTimingMarkFullyVisible(base::TimeDelta);
  void SetUserTimingMarkInteractive(base::TimeDelta);

  // Sets `custom_user_timing_mark` and notifies timing changed immediately.
  // Clear `custom_timing_mark` once it's notified to avoid duplicated mark
  // entries are notified.
  void NotifyCustomUserTimingMarkAdded(const AtomicString& mark_name,
                                       const base::TimeDelta& start_time);

  void AddRedirect(const KURL& redirecting_url, const KURL& redirected_url);
  void SetRedirectStart(base::TimeTicks);
  void SetRedirectEnd(base::TimeTicks);
  void SetRedirectCount(uint16_t value) {
    document_load_timing_values_->redirect_count = value;
  }
  void SetHasCrossOriginRedirect(bool value) {
    document_load_timing_values_->has_cross_origin_redirect = value;
  }

  void SetUnloadEventStart(base::TimeTicks);
  void SetUnloadEventEnd(base::TimeTicks);

  void MarkFetchStart();
  void SetFetchStart(base::TimeTicks);

  void SetResponseEnd(base::TimeTicks);

  void MarkLoadEventStart();
  void MarkLoadEventEnd();

  void SetActivationStart(base::TimeTicks);

  void SetCanRequestFromPreviousDocument(bool value) {
    document_load_timing_values_->can_request_from_previous_document = value;
  }

  void SetSystemEntropyAtNavigationStart(mojom::blink::SystemEntropy value) {
    document_load_timing_values_->system_entropy_at_navigation_start = value;
  }

  void SetRandomizedConfidence(
      const std::optional<RandomizedConfidenceValue>& value);

  void SetCriticalCHRestart(base::TimeTicks critical_ch_restart);

  DocumentLoadTimingValues* GetDocumentLoadTimingValues() const {
    return document_load_timing_values_.Get();
  }

  base::TimeTicks InputStart() const { return input_start_; }
  std::optional<base::TimeDelta> UserTimingMarkFullyLoaded() const {
    return user_timing_mark_fully_loaded_;
  }
  std::optional<base::TimeDelta> UserTimingMarkFullyVisible() const {
    return user_timing_mark_fully_visible_;
  }
  std::optional<base::TimeDelta> UserTimingMarkInteractive() const {
    return user_timing_mark_interactive_;
  }
  std::optional<std::tuple<AtomicString, base::TimeDelta>>
  CustomUserTimingMark() {
    return custom_user_timing_mark_;
  }
  base::TimeTicks NavigationStart() const { return navigation_start_; }
  const WTF::Vector<base::TimeTicks>& BackForwardCacheRestoreNavigationStarts()
      const {
    return bfcache_restore_navigation_starts_;
  }
  base::TimeTicks CommitNavigationEnd() const { return commit_navigation_end_; }
  base::TimeTicks ReferenceMonotonicTime() const {
    return reference_monotonic_time_;
  }

  base::TimeTicks UnloadEventStart() const {
    return document_load_timing_values_->unload_event_start;
  }
  base::TimeTicks UnloadEventEnd() const {
    return document_load_timing_values_->unload_event_end;
  }
  base::TimeTicks RedirectStart() const {
    return document_load_timing_values_->redirect_start;
  }
  base::TimeTicks RedirectEnd() const {
    return document_load_timing_values_->redirect_end;
  }
  base::TimeTicks FetchStart() const {
    return document_load_timing_values_->fetch_start;
  }
  base::TimeTicks ResponseEnd() const {
    return document_load_timing_values_->response_end;
  }
  base::TimeTicks LoadEventStart() const {
    return document_load_timing_values_->load_event_start;
  }
  base::TimeTicks LoadEventEnd() const {
    return document_load_timing_values_->load_event_end;
  }
  base::TimeTicks ActivationStart() const {
    return document_load_timing_values_->activation_start;
  }
  uint16_t RedirectCount() const {
    return document_load_timing_values_->redirect_count;
  }
  bool HasCrossOriginRedirect() const {
    return document_load_timing_values_->has_cross_origin_redirect;
  }
  bool CanRequestFromPreviousDocument() const {
    return document_load_timing_values_->can_request_from_previous_document;
  }
  base::TimeTicks CriticalCHRestart() const {
    return document_load_timing_values_->critical_ch_restart;
  }
  mojom::blink::SystemEntropy SystemEntropyAtNavigationStart() const {
    return document_load_timing_values_->system_entropy_at_navigation_start;
  }
  std::optional<RandomizedConfidenceValue> RandomizedConfidence() const {
    return document_load_timing_values_->randomized_confidence;
  }

  void Trace(Visitor*) const;

  void SetTickClockForTesting(const base::TickClock* tick_clock);
  void SetClockForTesting(const base::Clock* clock);

 private:
  void MarkRedirectEnd();
  void NotifyDocumentTimingChanged();
  void EnsureReferenceTimesSet();
  LocalFrame* GetFrame() const;
  void WriteNavigationStartDataIntoTracedValue(
      perfetto::TracedValue context) const;

  base::TimeTicks reference_monotonic_time_;
  base::TimeDelta reference_wall_time_;
  base::TimeTicks input_start_;
  std::optional<base::TimeDelta> user_timing_mark_fully_loaded_;
  std::optional<base::TimeDelta> user_timing_mark_fully_visible_;
  std::optional<base::TimeDelta> user_timing_mark_interactive_;
  std::optional<std::tuple<AtomicString, base::TimeDelta>>
      custom_user_timing_mark_;
  base::TimeTicks navigation_start_;
  base::TimeTicks commit_navigation_end_;
  WTF::Vector<base::TimeTicks> bfcache_restore_navigation_starts_;

  const base::Clock* clock_;
  const base::TickClock* tick_clock_;

  Member<DocumentLoader> document_loader_;
  Member<DocumentLoadTimingValues> document_load_timing_values_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_DOCUMENT_LOAD_TIMING_H_
