// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_SERIALIZED_BLOB_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_SERIALIZED_BLOB_MOJOM_TRAITS_H_

#include "base/memory/scoped_refptr.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/string_traits_wtf.h"
#include "third_party/blink/public/mojom/blob/serialized_blob.mojom-shared.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace mojo {

template <>
struct PLATFORM_EXPORT StructTraits<blink::mojom::SerializedBlobDataView,
                                    scoped_refptr<blink::BlobDataHandle>> {
  static bool IsNull(const scoped_refptr<blink::BlobDataHandle>& input) {
    return !input;
  }

  static void SetToNull(scoped_refptr<blink::BlobDataHandle>* output) {
    *output = nullptr;
  }

  static WTF::String uuid(const scoped_refptr<blink::BlobDataHandle>& input) {
    return input->Uuid();
  }

  static WTF::String content_type(
      const scoped_refptr<blink::BlobDataHandle>& input) {
    return input->GetType().IsNull() ? g_empty_string : input->GetType();
  }

  static uint64_t size(const scoped_refptr<blink::BlobDataHandle>& input) {
    return input->size();
  }

  static mojo::PendingRemote<blink::mojom::blink::Blob> blob(
      const scoped_refptr<blink::BlobDataHandle>& input) {
    return input->CloneBlobRemote();
  }

  static bool Read(blink::mojom::SerializedBlobDataView,
                   scoped_refptr<blink::BlobDataHandle>* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_SERIALIZED_BLOB_MOJOM_TRAITS_H_
