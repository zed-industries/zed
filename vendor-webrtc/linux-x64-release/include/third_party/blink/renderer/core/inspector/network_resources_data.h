/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * 1. Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. AND ITS CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL GOOGLE INC.
 * OR ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_NETWORK_RESOURCES_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_NETWORK_RESOURCES_DATA_H_

#include "net/cert/x509_certificate.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/html/parser/text_resource_decoder.h"
#include "third_party/blink/renderer/core/inspector/inspector_page_agent.h"
#include "third_party/blink/renderer/core/loader/resource/font_resource.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/network/http_header_map.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class EncodedFormData;
class ExecutionContext;
class ResourceResponse;
class TextResourceDecoder;

class XHRReplayData final : public GarbageCollected<XHRReplayData> {
 public:
  XHRReplayData(ExecutionContext*,
                const AtomicString& method,
                const KURL&,
                bool async,
                bool include_credentials);

  void AddHeader(const AtomicString& key, const AtomicString& value);

  ExecutionContext* GetExecutionContext() const {
    return execution_context_.Get();
  }
  const AtomicString& Method() const { return method_; }
  const KURL& Url() const { return url_; }
  bool Async() const { return async_; }
  const HTTPHeaderMap& Headers() const { return headers_; }
  bool IncludeCredentials() const { return include_credentials_; }

  void Trace(Visitor* visitor) const { visitor->Trace(execution_context_); }

 private:
  WeakMember<ExecutionContext> execution_context_;
  AtomicString method_;
  KURL url_;
  bool async_;
  HTTPHeaderMap headers_;
  bool include_credentials_;
};

class NetworkResourcesData final
    : public GarbageCollected<NetworkResourcesData> {
 public:
  class ResourceData final : public GarbageCollected<ResourceData>,
                             public FontResourceClearDataObserver {
    friend class NetworkResourcesData;

   public:
    ResourceData(NetworkResourcesData*,
                 const String& request_id,
                 const String& loader_id,
                 const KURL&);

    String RequestId() const { return request_id_; }
    String LoaderId() const { return loader_id_; }

    String FrameId() const { return frame_id_; }
    void SetFrameId(const String& frame_id) { frame_id_ = frame_id; }

    KURL RequestedURL() const { return requested_url_; }

    // Returns the size of request and response content.
    size_t ContentSize() const;
    bool HasContent() const { return !content_.IsNull(); }
    String Content() const { return content_; }
    void SetContent(const String&, bool base64_encoded);

    bool Base64Encoded() const { return base64_encoded_; }

    bool IsContentEvicted() const { return is_content_evicted_; }
    // Evicts the post data and the respone content.
    [[nodiscard]] size_t EvictContent();

    InspectorPageAgent::ResourceType GetType() const { return type_; }
    void SetType(InspectorPageAgent::ResourceType type) { type_ = type; }

    int HttpStatusCode() const { return http_status_code_; }
    void SetHTTPStatusCode(int http_status_code) {
      http_status_code_ = http_status_code;
    }

    String MimeType() const { return mime_type_; }
    void SetMimeType(const String& mime_type) { mime_type_ = mime_type; }

    String TextEncodingName() const { return text_encoding_name_; }
    void SetTextEncodingName(const String& text_encoding_name) {
      text_encoding_name_ = text_encoding_name;
    }

    const Resource* CachedResource() const { return cached_resource_.Get(); }
    void SetResource(const Resource*);

    XHRReplayData* XhrReplayData() const { return xhr_replay_data_.Get(); }
    void SetXHRReplayData(XHRReplayData* xhr_replay_data) {
      xhr_replay_data_ = xhr_replay_data;
    }

    BlobDataHandle* DownloadedFileBlob() const {
      return downloaded_file_blob_.get();
    }
    void SetDownloadedFileBlob(scoped_refptr<BlobDataHandle> blob) {
      downloaded_file_blob_ = std::move(blob);
    }

    int64_t RawHeaderSize() const { return raw_header_size_; }
    void SetRawHeaderSize(int64_t size) { raw_header_size_ = size; }

    net::X509Certificate* Certificate() { return certificate_.get(); }
    void SetCertificate(scoped_refptr<net::X509Certificate> certificate) {
      certificate_ = std::move(certificate);
    }
    int64_t PendingEncodedDataLength() const {
      return pending_encoded_data_length_;
    }
    void ClearPendingEncodedDataLength() { pending_encoded_data_length_ = 0; }
    void AddPendingEncodedDataLength(size_t encoded_data_length) {
      pending_encoded_data_length_ += encoded_data_length;
    }
    void SetPostData(scoped_refptr<EncodedFormData> post_data) {
      post_data_ = std::move(post_data);
    }
    EncodedFormData* PostData() const { return post_data_.get(); }

    // FontResourceClearDataObserver implementation.
    void FontResourceDataWillBeCleared() override;

    void Trace(Visitor*) const override;

    const std::optional<SegmentedBuffer>& Data() const { return data_buffer_; }

   private:
    bool HasData() const { return data_buffer_.has_value(); }
    void AppendData(base::span<const char> data);
    // Removes just the response content.
    [[nodiscard]] size_t RemoveResponseContent();
    size_t DecodeDataToContent();
    void ProcessCustomWeakness(const LivenessBroker&);

    Member<NetworkResourcesData> network_resources_data_;
    String request_id_;
    String loader_id_;
    String frame_id_;
    KURL requested_url_;
    String content_;
    Member<XHRReplayData> xhr_replay_data_;
    bool base64_encoded_;
    std::optional<SegmentedBuffer> data_buffer_;
    bool is_content_evicted_;
    InspectorPageAgent::ResourceType type_;
    int http_status_code_;

    String mime_type_;
    String text_encoding_name_;
    int64_t raw_header_size_;
    int64_t pending_encoded_data_length_;

    // We use UntracedMember<> here to do custom weak processing.
    UntracedMember<const Resource> cached_resource_;

    scoped_refptr<BlobDataHandle> downloaded_file_blob_;
    scoped_refptr<net::X509Certificate> certificate_;
    scoped_refptr<EncodedFormData> post_data_;
  };

  NetworkResourcesData(size_t total_buffer_size, size_t resource_buffer_size);
  ~NetworkResourcesData();

  void ResourceCreated(const String& request_id,
                       const String& loader_id,
                       const KURL&,
                       scoped_refptr<EncodedFormData>);
  void ResponseReceived(const String& request_id,
                        const String& frame_id,
                        const ResourceResponse&);
  void BlobReceived(const String& request_id, scoped_refptr<BlobDataHandle>);
  void SetResourceType(const String& request_id,
                       InspectorPageAgent::ResourceType);
  InspectorPageAgent::ResourceType GetResourceType(const String& request_id);
  void SetResourceContent(const String& request_id,
                          const String& content,
                          bool base64_encoded = false);
  void MaybeAddResourceData(const String& request_id,
                            base::span<const char> data);
  void MaybeDecodeDataToContent(const String& request_id);
  void AddResource(const String& request_id, const Resource*);
  ResourceData const* Data(const String& request_id);
  void Clear(const String& preserved_loader_id = String());

  void SetResourcesDataSizeLimits(size_t maximum_resources_content_size,
                                  size_t maximum_single_resource_content_size);
  void SetXHRReplayData(const String& request_id, XHRReplayData*);
  XHRReplayData* XhrReplayData(const String& request_id);
  void SetCertificate(const String& request_id,
                      scoped_refptr<net::X509Certificate>);
  HeapVector<Member<ResourceData>> Resources();

  int64_t GetAndClearPendingEncodedDataLength(const String& request_id);
  void AddPendingEncodedDataLength(const String& request_id,
                                   size_t encoded_data_length);
  void Trace(Visitor*) const;

 private:
  ResourceData* ResourceDataForRequestId(const String& request_id) const;
  void EnsureNoDataForRequestId(const String& request_id);
  bool EnsureFreeSpace(uint64_t);
  ResourceData* PrepareToAddResourceData(const String& request_id,
                                         uint64_t data_length);
  void MaybeAddResourceData(const String& request_id,
                            scoped_refptr<const SharedBuffer>);

  Deque<String> request_ids_deque_;

  typedef HeapHashMap<String, Member<ResourceData>> ResourceDataMap;
  ResourceDataMap request_id_to_resource_data_map_;
  size_t content_size_;
  size_t maximum_resources_content_size_;
  size_t maximum_single_resource_content_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_NETWORK_RESOURCES_DATA_H_
