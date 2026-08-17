// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_CONTENT_DUMPER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_CONTENT_DUMPER_H_

#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class WebLocalFrame;
class WebString;

// This class exposes an API to dump the text of a WebFrame and all subframes to
// a string, subject to the following limitations:
// * OOPIFs are not supported. This class will only return text from direct
// descendant frames that live in the same process. To get text from other
// processes, use this class there.
// * No guarantees are made regarding accuracy, the amount of page text that is
// captured, or when a result is generated.
// * No specification is given for how text from one frame will be delimited
// from the next.
// * The dumped text may contain text that is not visible to the user, whether
// due to scrolling, display:none, unexpanded divs, etc.
// * The ordering of the dumped text does not reflect the ordering of text on
// the page as seen by the user. Each element's text may appear in a totally
// different position in the dump from its position on the page.
//
// For the above reasons, this should not be used for any purpose that is
// consumable by a human. For example: "Select All", "Text to Speech", "Find in
// page", and other user-visible surfaces must not use this API.
//
// Also note that this utility is resource-intensive, consuming significant
// memory and CPU during a text capture.
class BLINK_EXPORT WebFrameContentDumper {
 public:
  // Returns the contents of this frame's local subtree as a string.  If the
  // text is longer than |max_chars|, it will be clipped to that length.
  static WebString DumpFrameTreeAsText(WebLocalFrame* frame, size_t max_chars);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_CONTENT_DUMPER_H_
