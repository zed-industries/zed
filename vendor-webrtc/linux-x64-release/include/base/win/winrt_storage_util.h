// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_WINRT_STORAGE_UTIL_H_
#define BASE_WIN_WINRT_STORAGE_UTIL_H_

#include <stdint.h>
#include <windows.storage.streams.h>
#include <wrl/client.h>

#include "base/base_export.h"

namespace base {
namespace win {

// Gets an array of bytes in the |buffer|, |out| represents a array of
// bytes used by byte stream read and write.
BASE_EXPORT HRESULT
GetPointerToBufferData(ABI::Windows::Storage::Streams::IBuffer* buffer,
                       uint8_t** out,
                       UINT32* length);

// Creates stream |buffer| from |data| that represents a array of bytes
// and the |length| of bytes.
BASE_EXPORT HRESULT CreateIBufferFromData(
    const uint8_t* data,
    UINT32 length,
    Microsoft::WRL::ComPtr<ABI::Windows::Storage::Streams::IBuffer>* buffer);

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_WINRT_STORAGE_UTIL_H_
