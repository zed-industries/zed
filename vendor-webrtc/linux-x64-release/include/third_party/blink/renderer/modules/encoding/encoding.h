// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ENCODING_ENCODING_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ENCODING_ENCODING_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace blink {

namespace encoding {

// The Encoding Standard has a definition of whitespace that differs from
// WTF::isWhiteSpace() (it excludes vertical tab).
bool IsASCIIWhiteSpace(UChar);

}  // namespace encoding

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ENCODING_ENCODING_H_
