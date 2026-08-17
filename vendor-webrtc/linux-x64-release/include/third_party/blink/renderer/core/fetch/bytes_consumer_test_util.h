// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BYTES_CONSUMER_TEST_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BYTES_CONSUMER_TEST_UTIL_H_

#include "mojo/public/cpp/system/data_pipe.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/core/fetch/fetch_data_loader.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class BytesConsumerTestUtil {
  STATIC_ONLY(BytesConsumerTestUtil);

 public:
  class MockBytesConsumer : public BytesConsumer {
   public:
    MOCK_METHOD1(BeginRead, Result(base::span<const char>&));
    MOCK_METHOD1(EndRead, Result(size_t));
    MOCK_METHOD1(DrainAsBlobDataHandle,
                 scoped_refptr<BlobDataHandle>(BlobSizePolicy));
    MOCK_METHOD0(DrainAsDataPipe, mojo::ScopedDataPipeConsumerHandle());
    MOCK_METHOD0(DrainAsFormData, scoped_refptr<EncodedFormData>());
    MOCK_METHOD1(SetClient, void(Client*));
    MOCK_METHOD0(ClearClient, void());
    MOCK_METHOD0(Cancel, void());
    MOCK_CONST_METHOD0(GetPublicState, PublicState());
    MOCK_CONST_METHOD0(GetError, Error());

    MockBytesConsumer();

    String DebugName() const override { return "MockBytesConsumer"; }
  };

  class MockFetchDataLoaderClient
      : public GarbageCollected<MockFetchDataLoaderClient>,
        public FetchDataLoader::Client {
   public:
    void Trace(Visitor* visitor) const override {
      FetchDataLoader::Client::Trace(visitor);
    }

    MOCK_METHOD1(DidFetchDataLoadedBlobHandleMock,
                 void(scoped_refptr<BlobDataHandle>));
    MOCK_METHOD1(DidFetchDataLoadedArrayBufferMock, void(DOMArrayBuffer*));
    MOCK_METHOD1(DidFetchDataLoadedFormDataMock, void(FormData*));
    MOCK_METHOD1(DidFetchDataLoadedString, void(const String&));
    MOCK_METHOD0(DidFetchDataLoadStream, void());
    MOCK_METHOD0(DidFetchDataLoadFailed, void());
    MOCK_METHOD0(Abort, void());

    void DidFetchDataLoadedArrayBuffer(DOMArrayBuffer* array_buffer) override {
      DidFetchDataLoadedArrayBufferMock(array_buffer);
    }
    // TODO(yhirano): Remove DidFetchDataLoadedBlobHandleMock.
    void DidFetchDataLoadedBlobHandle(
        scoped_refptr<BlobDataHandle> blob_data_handle) override {
      DidFetchDataLoadedBlobHandleMock(std::move(blob_data_handle));
    }
    void DidFetchDataLoadedFormData(FormData* FormData) override {
      DidFetchDataLoadedFormDataMock(FormData);
    }
  };
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BYTES_CONSUMER_TEST_UTIL_H_
