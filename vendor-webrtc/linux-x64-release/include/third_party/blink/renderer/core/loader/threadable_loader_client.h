/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_THREADABLE_LOADER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_THREADABLE_LOADER_CLIENT_H_

#include "base/containers/span.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class BytesConsumer;
class KURL;
class ResourceError;
class ResourceResponse;

class CORE_EXPORT ThreadableLoaderClient : public GarbageCollectedMixin {
 public:
  virtual void DidSendData(uint64_t /*bytesSent*/,
                           uint64_t /*totalBytesToBeSent*/) {}
  // Note that redirects for redirect modes kError and kManual are still
  // notified here. A client must return false in such cases.
  virtual bool WillFollowRedirect(uint64_t /*identifier*/,
                                  const KURL& new_url,
                                  const ResourceResponse&) {
    return true;
  }
  virtual void DidReceiveResponse(uint64_t /*identifier*/,
                                  const ResourceResponse&) {}
  virtual void DidStartLoadingResponseBody(BytesConsumer&) {}
  virtual void DidReceiveData(base::span<const char>) {}
  virtual void DidReceiveCachedMetadata(mojo_base::BigBuffer) {}
  virtual void DidFinishLoading(uint64_t /*identifier*/) {}
  virtual void DidFail(uint64_t /*identifier*/, const ResourceError&) {}
  virtual void DidFailRedirectCheck(uint64_t /*identifier*/) {}

  virtual void DidDownloadData(uint64_t /*dataLength*/) {}
  // Called for requests that had DownloadToBlob set to true. Can be called with
  // null if creating the blob failed for some reason (but the download itself
  // otherwise succeeded). Could also not be called at all if the downloaded
  // resource ended up being zero bytes.
  virtual void DidDownloadToBlob(scoped_refptr<BlobDataHandle>) {}

  ThreadableLoaderClient(const ThreadableLoaderClient&) = delete;
  ThreadableLoaderClient& operator=(const ThreadableLoaderClient&) = delete;
  virtual ~ThreadableLoaderClient() = default;

 protected:
  ThreadableLoaderClient() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_THREADABLE_LOADER_CLIENT_H_
