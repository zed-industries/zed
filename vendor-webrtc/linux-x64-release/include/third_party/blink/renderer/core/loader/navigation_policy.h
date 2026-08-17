/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_NAVIGATION_POLICY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_NAVIGATION_POLICY_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class Event;
struct WebWindowFeatures;

enum NavigationPolicy {
  kNavigationPolicyDownload,
  kNavigationPolicyCurrentTab,
  kNavigationPolicyNewBackgroundTab,
  kNavigationPolicyNewForegroundTab,
  kNavigationPolicyNewWindow,
  kNavigationPolicyNewPopup,
  kNavigationPolicyPictureInPicture,
  kNavigationPolicyLinkPreview,
};

// Returns a NavigationPolicy to use for starting a navigation
// based on the Event. This function takes care of some security checks,
// ensuring that synthesized events cannot trigger arbitrary downloads
// or new tabs without user intention coming from a real input event.
CORE_EXPORT NavigationPolicy NavigationPolicyFromEvent(const Event*);

// Returns a NavigationPolicy to use for navigating a new window.
// This function respects user intention coming from a real input event,
// and ensures that we don't perform a download instead of navigation.
CORE_EXPORT NavigationPolicy
NavigationPolicyForCreateWindow(const WebWindowFeatures&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_NAVIGATION_POLICY_H_
