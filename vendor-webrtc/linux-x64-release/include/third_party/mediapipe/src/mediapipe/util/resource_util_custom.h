#ifndef MEDIAPIPE_UTIL_RESOURCE_UTIL_CUSTOM_H_
#define MEDIAPIPE_UTIL_RESOURCE_UTIL_CUSTOM_H_

#include <string>

#include "mediapipe/framework/port/status.h"

namespace mediapipe {

typedef std::function<absl::Status(const std::string&, std::string*)>
    ResourceProviderFn;

// Returns true if files are provided via a custom resource provider.
bool HasCustomGlobalResourceProvider();

// Overrides the behavior of GetResourceContents.
void SetCustomGlobalResourceProvider(ResourceProviderFn fn);

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_RESOURCE_UTIL_CUSTOM_H_
