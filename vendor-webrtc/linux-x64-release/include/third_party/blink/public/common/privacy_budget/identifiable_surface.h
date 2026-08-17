// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_SURFACE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_SURFACE_H_

#include <stdint.h>

#include <algorithm>
#include <compare>
#include <cstddef>
#include <functional>
#include <tuple>

#include "services/metrics/public/cpp/ukm_builders.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"

namespace blink {

// An identifiable surface.
//
// See also: ../../../../../docs/privacy_budget/good_identifiable_surface.md
//
// This class intends to be a lightweight wrapper over a simple 64-bit integer.
// It exhibits the following characteristics:
//
//   * All methods are constexpr.
//   * Immutable.
//   * Efficient enough to pass by value.
//
// Internally, an identifiable surface is represented as a 64-bit unsigned
// integer that can be used as the metric hash for reporting metrics via UKM.
//
// The least-significant |kTypeBits| of the value is used to store
// a IdentifiableSurface::Type value. The remainder stores the 56
// least-significant bits of an `IdentifiableToken` as illustrated below:
//              ✂
//    ┌─────────┊────────────────────────────────────────┐ ┌──────────┐
//    │(discard)✂           IdentifiableToken            │ │   Type   │
//    └─────────┊───────────────────┬────────────────────┘ └────┬─────┘
// Bit 64       ┊55                 ┊                   0   7   ┊    0
//              ✂                   ↓                           ↓
//              ┌────────────────────────────────────────┬──────────┐
//              │                                        │          │
//              └────────────────────────────────────────┴──────────┘
//           Bit 64                                     8 7        0
//              │←────────────── IdentifiableSurface ──────────────→│
//
// Only the lower 56 bits of `IdentifiableToken` contribute to an
// `IdentifiableSurface`.
//
// See descriptions for the `Type` enum values for details on how the
// `IdentifiableToken` is generated for each type. The descriptions use the
// following notation to indicate how the value is recorded:
//
//     IdentifiableSurface = { IdentifiableToken value, Type value }
//     Value = [description of how the value is constructed]
class IdentifiableSurface {
 public:
  // Number of bits used by Type.
  static constexpr int kTypeBits = 8;

  // Bitmask for extracting Type value from a surface hash.
  static constexpr uint64_t kTypeMask = (1 << kTypeBits) - 1;

  // Indicator for an uninitialized IdentifiableSurface. Maps to
  // {Type::kReservedInternal, 0} which is not possible for a valid surface.
  static constexpr uint64_t kInvalidHash = 0;

  // Type of identifiable surface.
  //
  // Even though the data type is uint64_t, we can only use 8 bits due to how we
  // pack the surface type and a digest of the input into a 64 bits.
  //
  // These values are used for aggregation across versions. Entries should not
  // be renumbered and numeric values should never be reused.
  enum class Type : uint64_t {
    // This type is reserved for internal use and should not be used for
    // reporting any identifiability metrics.
    //
    // All metrics defined under the Identifiability event in
    // tools/metrics/ukm.xml fall into this type. Hence using
    // `ukm::builders::Identifiability` results in metrics with this type.
    kReservedInternal = 0,

    // Represents a web feature whose output directly contributes to
    // identifiability.
    //
    // These APIs are annotated with the `[HighEntropy=Direct]` extended WebIDL
    // attribute in their respective IDL file. Each such API also has an
    // associated `UseCounter` value specified directly via the
    // `[MeasureAs=??]` attribute or indirectly via the `[Measure]` attribute.
    // This `UseCounter` value is the key for recording the output of the API.
    // `web_feature.mojom`[1] defines all the `UseCounter` values and is
    // available as mojom::WebFeature.
    //
    //     IdentifiableSurface = { mojom::WebFeature, kWebFeature }
    //     Value = IdentifiableToken( $(output of the attribute or method) )
    //
    // [1]: //blink/public/mojom/use_counter/metrics/web_feature.mojom
    kWebFeature = 1,

    // Reserved 2.

    // Reserved 3.
    // Was kLocalFontLookupByUniqueOrFamilyName.

    // Reserved 4.
    // Was kGenericFontLookup.

    // Reserved 5.
    // Was kExtensionFileAccess.

    // Reserved 6.
    // Was kExtensionContentScript.

    // Represents making a measurement of one of the above surfacess. This
    // metric is retained even if filtering discards the surface.
    kMeasuredSurface = 7,

    // WebGL parameter for WebGLRenderingContext.getParameter().
    kWebGLParameter = 8,

    // Represents a call to |MediaRecorder.isTypeSupported(mimeType)|. Input is
    // the mime type supplied to the method.
    kMediaRecorder_IsTypeSupported = 9,

    // Represents a call to |MediaSource.isTypeSupported(mimeType)|. Input is
    // the mime type supplied to the method.
    kMediaSource_IsTypeSupported = 10,

    // Represents a call to |HTMLMediaElement.canPlayType(mimeType)|. Input is
    // the mime type supplied to the method.
    kHTMLMediaElement_CanPlayType = 11,

    // Reserved 12.
    // Was kLocalFontLookupByUniqueNameOnly.

    // Reserved 13.
    // Was kLocalFontLookupByFallbackCharacter.

    // Reserved 14.
    // Was kLocalFontLookupAsLastResort.

    // Reserved 15.
    // Was kExtensionCancelRequest.

    // WebGLRenderingContext.getShaderPrecisionFormat() is a high entropy API
    // that leaks entropy about the underlying GL implementation.
    // The output is keyed on two enums, but for the identifiability study we
    // will key this type on a digest of both the enums' values.
    kWebGLShaderPrecisionFormat = 16,

    // Reserved 17.
    // Was kScrollbarSize.

    // WebGL2RenderingContext.getInternal
    kWebGLInternalFormatParameter = 18,

    // Represents a call to GPU.requestAdapter. Input is the options filter.
    kGPU_RequestAdapter = 20,

    // For instrumenting HTMLCanvas.getContext() fingerprinting. Some scripts
    // will iterate through the different possible arguments and record whether
    // each type of context is supported.
    // The input should be an instance of CanvasRenderingContext::ContextType.
    kCanvasRenderingContext = 21,

    // Represents a call to MediaDevices.getUserMedia. Input is the set of
    // constraints.
    kMediaDevices_GetUserMedia = 22,

    // NavigatorUAData.getHighEntropyValues() is, shockingly, a high entropy
    // API to provide more detailed User-Agent data. The output is keyed on
    // the hint parameter.
    kNavigatorUAData_GetHighEntropyValues = 24,

    // MediaCapabilities.decodingInfo() reveals information about whether
    // media decoding will be supported, smooth and/or power efficient,
    // according to its codec, size, and other parameters. It can further reveal
    // details about encrypted decoding support according to the key system
    // configuration provided.
    kMediaCapabilities_DecodingInfo = 25,

    // Reserved 26.
    // Was kLocalFontExistenceByUniqueNameOnly.

    // Represents a call to Navigator.getUserMedia. Input is the set of
    // constraints.
    kNavigator_GetUserMedia = 27,

    // Represents a media query being tested. Input is combination of property
    // name and the target value. Output is the result --- true or false.
    kMediaQuery_DEPRECATED = 28,

    // Reserved 29.
    // Was kLocalFontLoadPostScriptName.

    // Getting supported codecs, etc. for WebRTC sender -- key is hash of kind
    // (audio or video).
    kRtcRtpSenderGetCapabilities = 31,

    // Getting supported codecs, etc. for WebRTC receiver -- key is hash of kind
    // (audio or video).
    kRtcRtpReceiverGetCapabilities = 32,

    // Reserved 33.

    // Metadata that is not reported by the client. Different from
    // kReservedInternal in that the inputs are not required to be defined in
    // `ukm.xml`.
    //
    // This surface type should not be used in the client (browser). It's meant
    // to be a reservation for additional surfaces that are determined during
    // analysis.
    kReservedMetadata = 34,

    // Reserved 35 (was kCanvasReadback).

    // Represents a media feature being tested. Input is the feature name.
    // Output is the feature value
    kMediaFeature = 36,

    // Type for synthetic surfaces used for reporting data with the goal of
    // estimating the Reid score of set of surfaces. This type does not
    // correspond to any Web APIs specifically.
    kReidScoreEstimator = 37,

    // Reserved 38.
    // Was kFontFamilyAvailable.

    // Represents determining that a local font exists or does not, based on a
    // name lookup that is allowed to match either a unique name or a family
    // name. This occurs when a font-family CSS rule doesn't match any
    // @font-face rule. Input is the lookup name. Output is a bool.
    kLocalFontExistenceByUniqueOrFamilyName = 39,

    // Represents a readback of a canvas. Input is the
    // CanvasRenderingContextType.
    //
    // Was 2 before change to paint op serialization, then 33 before removing
    // paint op serialization and using only direct canvas2d instrumentation,
    // then 35 before changing color hashing functions in
    // BaseRenderingContext2D.
    kCanvasReadback = 40,

    // We can use values up to and including |kMax|.
    kMax = (1 << kTypeBits) - 1
  };

  // These are metrics names of type 0 and are always reported when the study is
  // enabled.
  enum class ReservedSurfaceMetrics : uint64_t {
    kDocumentCreated_IsCrossOriginFrame = 0,
    kDocumentCreated_IsCrossSiteFrame = 1,
    kDocumentCreated_IsMainFrame = 2,
    kDocumentCreated_NavigationSourceId = 3,
    kWorkerClientAdded_ClientSourceId = 4,
    kWorkerClientAdded_WorkerType = 5,
    kMaxValue = kWorkerClientAdded_WorkerType
  };

  enum class WorkerType : uint64_t {
    kSharedWorker = 0,
    kServiceWorker = 1,
    kMaxValue = kServiceWorker,
  };

  static_assert(
      static_cast<uint64_t>(ReservedSurfaceMetrics::kMaxValue) <
          std::min(
              ukm::builders::Identifiability::kGeneratorVersion_926NameHash,
              ukm::builders::Identifiability::kStudyGeneration_626NameHash),
      "All the ReservedSurfaceMetrics enum values should be strictly smaller "
      "than kGeneratorVersion_926NameHash and kStudyGeneration_626NameHash to "
      "avoid collisions.");

  // HTML canvas readback -- bits [0-3] of the 64-bit input are the context type
  // (Type::kCanvasReadback), bits [4-6] are skipped ops, sensitive ops, and
  // partial image ops bits, respectively. The remaining bits are for the canvas
  // operations digest. If the digest wasn't calculated (there's no digest for
  // WebGL, for instance), the digest field is 0.
  enum CanvasTaintBit : uint64_t {
    // At least one drawing operation didn't update the digest -- this is either
    // due to performance or resource consumption reasons.
    kSkipped = UINT64_C(0x10),

    // At least one drawing operation operated on a sensitive string. Sensitive
    // strings use a 16-bit hash digest.
    kSensitive = UINT64_C(0x20),

    // At least one drawing operation was only partially digested, for
    // performance reasons.
    kPartiallyDigested = UINT64_C(0x40)
  };

  // Possible inputs for Type::kMediaFeature.
  enum class MediaFeatureName : uint64_t {
    kAnyHover = 0,
    kAnyPointer = 1,
    kColorGamut = 2,
    kForcedColors = 3,
    kGrid = 4,
    kHover = 5,
    kOrientation = 6,
    kDynamicRange = 7,
    kDisplayMode = 8,
    kNavigationControls = 9,
    kPointer = 10,
    kPrefersColorScheme = 11,
    kPrefersContrast = 12,
    kPrefersReducedMotion = 13,
    kPrefersReducedData = 14,
    kTransform3d = 15,
    kScan = 16,
    kDevicePosture = 17,
    kColor = 18,
    kColorIndex = 19,
    kMonochrome = 20,
    kAspectRatio_DEPRECATED = 21,
    kResolution = 22,
    kHorizontalViewportSegments = 23,
    kVerticalViewportSegments = 24,
    kAspectRatioNormalized = 25,
    kPrefersReducedTransparency = 26,
    kInvertedColors = 27,
    kScripting = 28,
    kDisplayState = 29,
    kResizable = 30,
    // We can use enum values up to and including 63, see static_assert below.
    kMaxValue = kResizable
  };
  static_assert(static_cast<int>(MediaFeatureName::kMaxValue) < 64,
                "MediaFeatureName only allows values < 64 since we use it in "
                "a uint64_t bitfield inside document.h to track if a media "
                "feature has already been sampled");

  // Default constructor is invalid.
  IdentifiableSurface() : IdentifiableSurface(kInvalidHash) {}

  // Construct an IdentifiableSurface based on a precalculated metric hash. Can
  // also be used as the first step in decoding an encoded metric hash.
  static constexpr IdentifiableSurface FromMetricHash(uint64_t metric_hash) {
    return IdentifiableSurface(metric_hash);
  }

  // Construct an IdentifiableSurface based on a surface type and an input
  // token.
  static constexpr IdentifiableSurface FromTypeAndToken(
      Type type,
      IdentifiableToken token) {
    return IdentifiableSurface(KeyFromSurfaceTypeAndInput(type, token.value_));
  }

  // Construct an invalid identifiable surface.
  static constexpr IdentifiableSurface Invalid() {
    return IdentifiableSurface(kInvalidHash);
  }

  // Returns the UKM metric hash corresponding to this IdentifiableSurface.
  constexpr uint64_t ToUkmMetricHash() const { return metric_hash_; }

  // Returns the type of this IdentifiableSurface.
  constexpr Type GetType() const {
    return std::get<0>(SurfaceTypeAndInputFromMetricKey(metric_hash_));
  }

  // Returns the input hash for this IdentifiableSurface.
  //
  // The value that's returned can be different from what's used for
  // constructing the IdentifiableSurface via FromTypeAndToken() if the input is
  // >= 2^56.
  constexpr uint64_t GetInputHash() const {
    return std::get<1>(SurfaceTypeAndInputFromMetricKey(metric_hash_));
  }

  constexpr bool IsValid() const { return metric_hash_ != kInvalidHash; }

  friend constexpr auto operator<=>(const IdentifiableSurface& lhs,
                                    const IdentifiableSurface& rhs) = default;
  friend constexpr bool operator==(const IdentifiableSurface& lhs,
                                   const IdentifiableSurface& rhs) = default;

 private:
  constexpr explicit IdentifiableSurface(uint64_t metric_hash)
      : metric_hash_(metric_hash) {}

  // Returns a 64-bit metric key given an IdentifiableSurfaceType and a 64 bit
  // input digest.
  //
  // The returned key can be used as the metric hash when invoking
  // UkmEntryBuilderBase::SetMetricInternal().
  static constexpr uint64_t KeyFromSurfaceTypeAndInput(Type type,
                                                       uint64_t input) {
    uint64_t type_as_int = static_cast<uint64_t>(type);
    return type_as_int | (input << kTypeBits);
  }

  // Returns the IdentifiableSurfaceType and the input hash given a metric key.
  //
  // This is approximately the inverse of MetricKeyFromSurfaceTypeAndInput().
  // See caveat in GetInputHash() about cases where the input hash can differ
  // from that used to construct this IdentifiableSurface.
  static constexpr std::tuple<Type, uint64_t> SurfaceTypeAndInputFromMetricKey(
      uint64_t metric) {
    return std::make_tuple(static_cast<Type>(metric & kTypeMask),
                           metric >> kTypeBits);
  }

  uint64_t metric_hash_;
};

// Hash function compatible with std::hash.
struct IdentifiableSurfaceHash {
  size_t operator()(const IdentifiableSurface& s) const {
    return std::hash<uint64_t>{}(s.ToUkmMetricHash());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_SURFACE_H_
