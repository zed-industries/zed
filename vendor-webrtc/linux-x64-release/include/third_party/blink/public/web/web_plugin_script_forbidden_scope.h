// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_SCRIPT_FORBIDDEN_SCOPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_SCRIPT_FORBIDDEN_SCOPE_H_

#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class BLINK_EXPORT WebPluginScriptForbiddenScope {
 public:
  WebPluginScriptForbiddenScope() = delete;
  static bool IsForbidden();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_SCRIPT_FORBIDDEN_SCOPE_H_
