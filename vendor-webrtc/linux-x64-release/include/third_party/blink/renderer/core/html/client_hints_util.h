// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CLIENT_HINTS_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CLIENT_HINTS_UTIL_H_

#include "services/network/public/cpp/client_hints.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "third_party/blink/renderer/platform/loader/fetch/client_hints_preferences.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KURL;
class LocalDOMWindow;

// This primarily calls ClientHintsPreferences::UpdateFromMetaTagAcceptCH,
// but then updates the `execution_context` with any relevant permissions
// policy changes for client hints due to the named meta tag. Ideally, this
// would be a part of ClientHintsPreferences, but we cannot access files in
// third_party/blink/renderer/core from third_party/blink/renderer/platform.
// TODO(crbug.com/1278127): Replace w/ generic HTML policy modification.
void UpdateWindowPermissionsPolicyWithDelegationSupportForClientHints(
    ClientHintsPreferences& client_hints_preferences,
    LocalDOMWindow* local_dom_window,
    const String& header_value,
    const KURL& url,
    ClientHintsPreferences::Context* context,
    network::MetaCHType type,
    bool is_doc_preloader,
    bool is_sync_parser);

// This modifies `container_policy` to reflect any changes to client hint
// permissions which may have occurred via the named accept-ch meta tag.
// The permissions policy the browser side has for the frame was set in stone
// before HTML parsing began, so any updates must be sent via the container
// policy. It's as if the meta tag content was copied into the allow attribute
// of the iframe.
// TODO(crbug.com/1278127): Replace w/ generic HTML policy modification.
void UpdateIFrameContainerPolicyWithDelegationSupportForClientHints(
    network::ParsedPermissionsPolicy& container_policy,
    LocalDOMWindow* local_dom_window);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CLIENT_HINTS_UTIL_H_
