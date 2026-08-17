// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_DATA_RESOURCE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_DATA_RESOURCE_HELPER_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// Helper functions providing access to ui::ResourceBundle in Blink.

// Uncompresses a gzipped resource and returns it as a string. The resource
// is specified by the resource id from Grit.
PLATFORM_EXPORT String UncompressResourceAsString(int resource_id);

// Uncompresses a gzipped resource and returns it as an ascii string. The
// resource is specified by the resource id from Grit.
PLATFORM_EXPORT String UncompressResourceAsASCIIString(int resource_id);

// Uncompresses a gzipped resource and returns it as a vector of characters.
// The resource is specified by the resource id from Grit.
PLATFORM_EXPORT Vector<char> UncompressResourceAsBinary(int resource_id);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_DATA_RESOURCE_HELPER_H_
