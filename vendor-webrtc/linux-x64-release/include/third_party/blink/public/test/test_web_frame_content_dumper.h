// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_TEST_TEST_WEB_FRAME_CONTENT_DUMPER_H_
#define THIRD_PARTY_BLINK_PUBLIC_TEST_TEST_WEB_FRAME_CONTENT_DUMPER_H_

#include <stddef.h>
#include <stdint.h>

namespace blink {

class WebLocalFrame;
class WebView;
class WebString;

// Functions in this class may only be used in tests.
class TestWebFrameContentDumper {
 public:
  // Dumps the contents of of a WebView as text, starting from the main frame
  // and recursively appending every subframe, separated by an empty line.
  static WebString DumpWebViewAsText(WebView*, size_t max_chars);

  // Returns HTML text for the contents of this frame, generated from the DOM.
  static WebString DumpAsMarkup(WebLocalFrame*);

  // Control of |DumpLayoutTreeAsText| output.
  enum LayoutAsTextControl {
    kLayoutAsTextNormal = 0,
    kLayoutAsTextDebug = 1 << 0,
    kLayoutAsTextPrinting = 1 << 1,
    kLayoutAsTextWithLineTrees = 1 << 2
  };
  using LayoutAsTextControls = uint8_t;

  // Returns a text representation of the render tree. This method is used to
  // support web tests.
  static WebString DumpLayoutTreeAsText(
      WebLocalFrame*,
      LayoutAsTextControls to_show = kLayoutAsTextNormal);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_TEST_TEST_WEB_FRAME_CONTENT_DUMPER_H_
