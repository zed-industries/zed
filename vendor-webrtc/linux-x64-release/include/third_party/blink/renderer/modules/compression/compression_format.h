// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_COMPRESSION_FORMAT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_COMPRESSION_FORMAT_H_

#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ExceptionState;

// This enum is used in UMA. Do not delete or re-order entries. New entries
// should only be added at the end. Please keep in sync with
// "CompressionStreamsFormat" in //tools/metrics/histograms/enums.xml.
enum class CompressionFormat {
  kGzip = 0,
  kDeflate = 1,
  kDeflateRaw = 2,
  kMaxValue = kDeflateRaw,
};

// Converts the JavaScript name |format| to the equivalent enum value. If the
// string does not correspond to any of the supported formats, an exception is
// thrown.
CompressionFormat LookupCompressionFormat(const AtomicString& format,
                                          ExceptionState&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_COMPRESSION_FORMAT_H_
