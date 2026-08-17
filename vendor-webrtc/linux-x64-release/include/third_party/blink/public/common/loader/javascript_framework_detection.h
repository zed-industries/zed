#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_JAVASCRIPT_FRAMEWORK_DETECTION_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_JAVASCRIPT_FRAMEWORK_DETECTION_H_

#include <cstdint>
#include <map>

#include "third_party/blink/public/mojom/loader/javascript_framework_detection.mojom-shared.h"

namespace blink {

// The existence of a framework might be detected, without a detected version.
static constexpr int64_t kNoFrameworkVersionDetected = 0;
using mojom::JavaScriptFramework;

// A map containing versions of detected frameworks.
// Frameworks that are not detected at all would not appear in the map, while
// frameworks that are detected without a version would have a value of
// kNoFrameworkVersionDetected.
struct JavaScriptFrameworkDetectionResult {
  std::map<JavaScriptFramework, int16_t> detected_versions;
};

inline bool operator==(const JavaScriptFrameworkDetectionResult& a,
                       const JavaScriptFrameworkDetectionResult& b) {
  return a.detected_versions == b.detected_versions;
}

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_JAVASCRIPT_FRAMEWORK_DETECTION_H_
