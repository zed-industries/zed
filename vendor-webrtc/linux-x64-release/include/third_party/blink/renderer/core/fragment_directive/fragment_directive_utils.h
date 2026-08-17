// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_FRAGMENT_DIRECTIVE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_FRAGMENT_DIRECTIVE_UTILS_H_

namespace blink {

class LocalFrame;

class FragmentDirectiveUtils {
 public:
  // Updates the URL by removing the text fragment selector.
  static void RemoveSelectorsFromUrl(LocalFrame* frame);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_FRAGMENT_DIRECTIVE_UTILS_H_
