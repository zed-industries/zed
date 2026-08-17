// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_LOAD_TYPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_LOAD_TYPE_H_

namespace blink {

// The type of load for a navigation.
// TODO(clamy, toyoshim): Currently WebFrameLoadType represents multiple
// concepts that should be orthogonal and could be represented by multiple enum
// classes. We should consider what WebFrameLoadType should represent and what
// shouldn't.
// See https://crbug.com/707715 for further discussion.
//
// kStandard:
//   Follows network and cache protocols, e.g. using cached entries unless
//   they are expired. Used in usual navigations.
// kBackForward:
//   Uses cached entries even if the entries are stale. Used in history back and
//   forward navigations.
// kReload:
//   Revalidates a cached entry for the main resource if one exists, but follows
//   protocols for other subresources. Blink internally uses this for the same
//   page navigation. Also used in optimized reload for mobiles in a field
//   trial.
// kReplaceCurrentItem:
//   Same as Standard, but replaces the current navigation entry in the history.
//   In terms of cache policy, it should work in the same manner as kStandard.
// kReloadBypassingCache:
//   Bypasses any caches, memory and disk cache in the browser, and caches in
//   proxy servers, to fetch fresh contents directly from the end server.
//   Used in Shift-Reload.
// kRestore:
//   Used in  session restore (e.g., restoring all the tabs in a previous
//   session) and tab restore (e.g., re-opening a closed tab, or duplicating a
//   tab). This behaves the same as kBackForward.
enum class WebFrameLoadType {
  kStandard,
  kBackForward,
  kReload,
  kReplaceCurrentItem,
  kReloadBypassingCache,
  kRestore,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_LOAD_TYPE_H_
