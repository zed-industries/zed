// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_THIRD_PARTY_SCRIPT_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_THIRD_PARTY_SCRIPT_DETECTOR_H_

#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/re2/src/re2/re2.h"

namespace blink {

class LocalDOMWindow;

class ThirdPartyScriptDetector final
    : public GarbageCollected<ThirdPartyScriptDetector>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  static ThirdPartyScriptDetector& From(LocalDOMWindow&);

  explicit ThirdPartyScriptDetector(LocalDOMWindow&);
  ThirdPartyScriptDetector(const ThirdPartyScriptDetector&) = delete;
  ThirdPartyScriptDetector& operator=(const ThirdPartyScriptDetector&) = delete;

  void Trace(Visitor*) const override;

  // Enumerated list of top 3 third party technologies based on traffic, from
  // httparchive July 2023 data.
  enum class Technology {
    kNone = 0,
    kWordPress = 1 << 0,
    kGoogleAnalytics = 1 << 1,
    kGoogleFontApi = 1 << 2,
    kGoogleTagManager = 1 << 3,
    kGoogleMaps = 1 << 4,
    kMetaPixel = 1 << 5,
    kYouTube = 1 << 6,
    kAdobeAnalytics = 1 << 7,
    kTiktokPixel = 1 << 8,
    kHotjar = 1 << 9,
    kGoogleAdSense = 1 << 10,
    kGooglePublisherTag = 1 << 11,
    kGoogleAdsLibraries = 1 << 12,
    kFundingChoices = 1 << 13,
    kElementor = 1 << 14,
    kSliderRevolution = 1 << 15,
    kLast = kSliderRevolution
    // If adding new technologies, add above kLast and shift kLast accordingly.
    // Keep in sync with `ThirdPartyTechnology` in
    // base/tracing/protos/chrome_track_event.proto.
    // The enum value (n) here is converted to the proto value (m) by
    // m = bit_width(n) + 1.
    // Max value allowed: 1 << 63. Limited by UKM bitfield.
  };

  Technology Detect(const WTF::String url);

 private:
  RE2 precompiled_detection_regex__;
  HashMap<WTF::String, Technology> url_to_technology_cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_THIRD_PARTY_SCRIPT_DETECTOR_H_
