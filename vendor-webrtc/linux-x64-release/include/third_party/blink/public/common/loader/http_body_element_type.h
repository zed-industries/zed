// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_HTTP_BODY_ELEMENT_TYPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_HTTP_BODY_ELEMENT_TYPE_H_

namespace blink {

enum class HTTPBodyElementType {
  kTypeData,
  kTypeFile,
  kTypeBlob,
  kTypeDataPipe,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_HTTP_BODY_ELEMENT_TYPE_H_
