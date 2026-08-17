// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_FRAME_SERIALIZER_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_FRAME_SERIALIZER_TEST_HELPER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class WebLocalFrameImpl;

class WebFrameSerializerTestHelper {
  STATIC_ONLY(WebFrameSerializerTestHelper);

 public:
  // Returns the MHTML serialization of |frame|. Header and footer are included.
  static String GenerateMHTML(WebLocalFrameImpl*);

  // Returns the body parts of MHTML serialization of |frame|. Header and footer
  // are excluded.
  static String GenerateMHTMLParts(WebLocalFrameImpl*);

  // Same as GenerateMHTML(), except that popup overlays are removed.
  static String GenerateMHTMLWithPopupOverlayRemoved(WebLocalFrameImpl*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_FRAME_SERIALIZER_TEST_HELPER_H_
