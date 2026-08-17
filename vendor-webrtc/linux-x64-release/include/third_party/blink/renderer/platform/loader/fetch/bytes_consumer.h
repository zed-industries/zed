// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BYTES_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BYTES_CONSUMER_H_

#include <ostream>

#include "base/memory/scoped_refptr.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/network/encoded_form_data.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// BytesConsumer represents the "consumer" side of a data pipe. A user
// can read data from it.
//
// A BytesConsumer is bound to the thread on which it is created.
// BytesConsumer has four states: waiting, readable, closed and errored. Once
// the state becomes closed or errored, it will never change. |readable| means
// that the BytesConsumer is ready to read non-empty bytes synchronously.
class PLATFORM_EXPORT BytesConsumer : public GarbageCollected<BytesConsumer> {
 public:
  enum class Result {
    kOk,
    kShouldWait,
    kDone,
    kError,
  };
  // Readable and Waiting are indistinguishable from BytesConsumer users.
  enum class PublicState {
    kReadableOrWaiting,
    kClosed,
    kErrored,
  };
  enum class BlobSizePolicy {
    // The returned blob must have a valid size (i.e. != kuint64max).
    kDisallowBlobWithInvalidSize,
    // The returned blob can have an invalid size.
    kAllowBlobWithInvalidSize
  };
  class PLATFORM_EXPORT Error {
   public:
    Error() {}
    explicit Error(const String& message) : message_(message) {}
    const String& Message() const { return message_; }
    bool operator==(const Error& e) const { return e.message_ == message_; }

   private:
    String message_;
  };
  // Client gets notification from the associated ByteConsumer.
  class PLATFORM_EXPORT Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() {}

    // This function is called when the state changes (e.g., readable =>
    // errored). This function can be called more than needed, i.e., it can
    // be called even when the state is not actually changed, but it is
    // guaranteed that this function cannot be called after the state
    // becomes closed or errored.
    //
    // This function is not called when the state change is trigerred by
    // public methods called by a user. For example, when a user reads
    // data by |read| and the state changes from waiting to readable, this
    // function will not be called.
    virtual void OnStateChange() = 0;

    // Each implementation should return a string that represents the
    // implementation for debug purpose.
    virtual String DebugName() const = 0;
  };

  virtual ~BytesConsumer() {}

  // Begins a two-phase read. On success, the function stores a buffer
  // that contains the read data of length `buffer`.
  // Returns Ok when readable.
  // Returns ShouldWait when it's waiting.
  // Returns Done when it's closed.
  // Returns Error when errored.
  // When not readable, the caller doesn't have to (and must not) call
  // EndRead, because the read session implicitly ends in that case.
  //
  // `buffer` will become invalid when this object becomes unreachable,
  // even if EndRead is not called.
  //
  // `buffer` will be set to empty if not readable.
  [[nodiscard]] virtual Result BeginRead(base::span<const char>& buffer) = 0;

  // Ends a two-phase read.
  // This function can modify this BytesConsumer's state.
  // Returns Ok when the consumer stays readable or waiting.
  // Returns Done when it's closed.
  // Returns Error when it's errored.
  [[nodiscard]] virtual Result EndRead(size_t read_size) = 0;

  // Drains the data as a BlobDataHandle.
  // When this function returns a non-null value, the returned blob handle
  // contains bytes that would be read through the BeginRead and
  // EndRead functions without calling this function. In such a case, this
  // object becomes closed.
  // When this function returns null value, this function does nothing.
  // When |policy| is DisallowBlobWithInvalidSize, this function doesn't
  // return a non-null blob handle with unspecified size.
  // The type of the returned blob handle may not be meaningful.
  virtual scoped_refptr<BlobDataHandle> DrainAsBlobDataHandle(
      BlobSizePolicy = BlobSizePolicy::kDisallowBlobWithInvalidSize) {
    return nullptr;
  }

  // Drains the data as an EncodedFormData.
  // When this function returns a non-null value, the returned form data
  // contains bytes that would be read through the BeginRead and
  // EndRead functions without calling this function. In such a case, this
  // object becomes closed.
  // When this function returns null value, this function does nothing.
  // This function returns a non-null form data when the handle is made
  // from an EncodedFormData-convertible value.
  virtual scoped_refptr<EncodedFormData> DrainAsFormData() { return nullptr; }

  // Drains the data as a ScopedDataPipeConsumerHandle.
  // When this function returns a valid handle, the returned pipe handle
  // contains bytes that would be read through the BeginRead and
  // EndRead functions without calling this function. The consumer may
  // become closed or remain in the open state depending on if it has
  // received an explicit completion signal.  If the consumer becomes
  // closed OnstateChange() will *not* be called.  Instead manually
  // call GetPublicState() to check if draining closed the consumer.
  //
  // When this function returns an invalid handle, this function does nothing.
  virtual mojo::ScopedDataPipeConsumerHandle DrainAsDataPipe() {
    return mojo::ScopedDataPipeConsumerHandle();
  }

  // Sets a client. This can be called only when no client is set. When
  // this object is already closed or errored, this function does nothing.
  virtual void SetClient(Client*) = 0;
  // Clears the set client.
  // A client will be implicitly cleared when this object gets closed or
  // errored (after the state change itself is notified).
  virtual void ClearClient() = 0;

  // Cancels this ByteConsumer. This function does nothing when |this| is
  // already closed or errored. Otherwise, this object becomes closed.
  // This function cannot be called in a two-phase read.
  virtual void Cancel() = 0;

  // Returns the current state.
  virtual PublicState GetPublicState() const = 0;

  // Returns the associated error of this object. This function can be called
  // only when errored.
  virtual Error GetError() const = 0;

  // Each implementation should return a string that represents the
  // implementation for debug purpose.
  virtual String DebugName() const = 0;

  // Returns a BytesConsumer whose state is Closed.
  static BytesConsumer* CreateClosed();

  // Returns a BytesConsumer whose state is Errored.
  static BytesConsumer* CreateErrored(const Error&);

  virtual void Trace(Visitor* visitor) const {}

 protected:
  // This InternalState directly corresponds to the states in the class
  // comments. This enum is defined here for subclasses.
  enum class InternalState {
    kWaiting,
    kClosed,
    kErrored,
  };

  static PublicState GetPublicStateFromInternalState(InternalState state) {
    switch (state) {
      case InternalState::kWaiting:
        return PublicState::kReadableOrWaiting;
      case InternalState::kClosed:
        return PublicState::kClosed;
      case InternalState::kErrored:
        return PublicState::kErrored;
    }
    NOTREACHED();
  }
};

PLATFORM_EXPORT std::ostream& operator<<(
    std::ostream& out,
    const BytesConsumer::PublicState& state);
PLATFORM_EXPORT std::ostream& operator<<(std::ostream& out,
                                         const BytesConsumer::Result& result);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BYTES_CONSUMER_H_
