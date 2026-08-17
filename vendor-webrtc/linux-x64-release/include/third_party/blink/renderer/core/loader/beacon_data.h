// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_BEACON_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_BEACON_DATA_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Blob;
class DOMArrayBufferView;
class DOMArrayBuffer;
class EncodedFormData;
class FormData;
class ResourceRequest;
class URLSearchParams;

// BeaconData handles beacon data serialization.
class BeaconData {
  STACK_ALLOCATED();

 public:
  virtual void Serialize(ResourceRequest&) const = 0;

 protected:
  virtual uint64_t size() const = 0;
  virtual const AtomicString GetContentType() const = 0;
  virtual scoped_refptr<EncodedFormData> GetEncodedFormData() const = 0;
};

class BeaconString final : public BeaconData {
 public:
  explicit BeaconString(const String& data);
  void Serialize(ResourceRequest& request) const override;

 protected:
  uint64_t size() const override;
  const AtomicString GetContentType() const override { return content_type_; }
  scoped_refptr<EncodedFormData> GetEncodedFormData() const override;

 private:
  const String data_;
  AtomicString content_type_;
};

class BeaconBlob final : public BeaconData {
 public:
  explicit BeaconBlob(Blob* data);
  void Serialize(ResourceRequest& request) const override;

 protected:
  uint64_t size() const override;
  const AtomicString GetContentType() const override { return content_type_; }
  scoped_refptr<EncodedFormData> GetEncodedFormData() const override;

 private:
  Blob* const data_;
  AtomicString content_type_;
};

class BeaconDOMArrayBufferView final : public BeaconData {
 public:
  explicit BeaconDOMArrayBufferView(DOMArrayBufferView* data);
  void Serialize(ResourceRequest& request) const override;

 protected:
  uint64_t size() const override;
  const AtomicString GetContentType() const override { return g_null_atom; }
  scoped_refptr<EncodedFormData> GetEncodedFormData() const override;

 private:
  DOMArrayBufferView* const data_;
};

class BeaconDOMArrayBuffer final : public BeaconData {
 public:
  explicit BeaconDOMArrayBuffer(DOMArrayBuffer* data);
  void Serialize(ResourceRequest& request) const override;

 protected:
  uint64_t size() const override;
  const AtomicString GetContentType() const override { return g_null_atom; }
  scoped_refptr<EncodedFormData> GetEncodedFormData() const override;

 private:
  DOMArrayBuffer* const data_;
};

class BeaconURLSearchParams final : public BeaconData {
 public:
  explicit BeaconURLSearchParams(URLSearchParams* data);
  void Serialize(ResourceRequest& request) const override;

 protected:
  uint64_t size() const override;
  const AtomicString GetContentType() const override { return content_type_; }
  scoped_refptr<EncodedFormData> GetEncodedFormData() const override;

 private:
  URLSearchParams* const data_;
  AtomicString content_type_;
};

class BeaconFormData final : public BeaconData {
 public:
  explicit BeaconFormData(FormData* data);
  void Serialize(ResourceRequest& request) const override;

 protected:
  uint64_t size() const override;
  const AtomicString GetContentType() const override { return content_type_; }
  scoped_refptr<EncodedFormData> GetEncodedFormData() const override;

 private:
  FormData* const data_;
  scoped_refptr<EncodedFormData> entity_body_;
  AtomicString content_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_BEACON_DATA_H_
