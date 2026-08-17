// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_security_origin.h"
#include "third_party/blink/public/platform/web_string.h"
#include "v8/include/v8-forward.h"

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ISOLATED_WORLD_INFO_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ISOLATED_WORLD_INFO_H_

namespace blink {

struct WebIsolatedWorldInfo {
  // Associates an isolated world with a security origin. XMLHttpRequest
  // instances used in that world will be considered to come from that origin,
  // not the frame's.
  //
  // Currently the origin shouldn't be aliased, because IsolatedCopy() is
  // taken before associating it to an isolated world and aliased relationship,
  // if any, is broken. crbug.com/779730
  // Note: If this is null, the security origin for the isolated world is
  // cleared.
  WebSecurityOrigin security_origin;

  // Associates a content security policy with an isolated world. This policy
  // should be used when evaluating script in the isolated world.
  //
  // Note: If this is null, the content security policy for the isolated world
  // is cleared. Else if this is specified, |security_origin| must also be
  // specified.
  WebString content_security_policy;

  // Associates an isolated world with human-readable name which is useful for
  // extension debugging.
  WebString human_readable_name;

  // Associates an isolated world with an optional tag that does not vary
  // between browser sessions or between renderers, unlike the world ID which
  // can be randomly assigned. The exact meaning will depend on the embedder
  // and the type of isolated world. For example Chrome extensions use the
  // host ID, as per extensions::ScriptInjection::GetHostIdForIsolatedWorld.
  // Some types of isolated world will not have a suitable tag so will leave
  // this empty.
  WebString stable_id;
};

// The ID of the "main" execution world for a document.
static constexpr int32_t kMainDOMWorldId = 0;

// Sets up an isolated world by associating a |world_id| with |info|.
// worldID must be > 0 (as 0 represents the main world).
// worldID must be < kEmbedderWorldIdLimit, high number used internally.
BLINK_EXPORT void SetIsolatedWorldInfo(int32_t world_id,
                                       const WebIsolatedWorldInfo& info);

// Checks if |world_id| is equal or exceeds kEmbedderWorldIdLimit value.
BLINK_EXPORT bool IsEqualOrExceedEmbedderWorldIdLimit(int32_t world_id);

// Returns the stable ID that was set with SetIsolatedWorldInfo.
BLINK_EXPORT WebString GetIsolatedWorldStableId(v8::Local<v8::Context>);

// Returns the human readable name that was set with SetIsolatedWorldInfo.
BLINK_EXPORT WebString
    GetIsolatedWorldHumanReadableName(v8::Local<v8::Context>);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ISOLATED_WORLD_INFO_H_
