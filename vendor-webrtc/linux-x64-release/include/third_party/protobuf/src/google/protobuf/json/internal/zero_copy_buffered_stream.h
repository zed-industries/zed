// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_JSON_INTERNAL_ZERO_COPY_BUFFERED_STREAM_H__
#define GOOGLE_PROTOBUF_JSON_INTERNAL_ZERO_COPY_BUFFERED_STREAM_H__

#include <algorithm>
#include <cstdint>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/io/zero_copy_stream.h"
#include "google/protobuf/stubs/status_macros.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

// Utilities for parsing contiguous buffers out of ZeroCopyInputStreams.

namespace google {
namespace protobuf {
namespace json_internal {
// Forward decl. for use by helper types below.
class ZeroCopyBufferedStream;

// An RAII type that represents holding a reference into the backing buffer
// of a ZeroCopyBufferedStream. This allows for automatic management of the
// backing buffer.
class BufferingGuard {
 public:
  explicit BufferingGuard(ZeroCopyBufferedStream* owner = nullptr);
  ~BufferingGuard();

  BufferingGuard(const BufferingGuard& other) : BufferingGuard(other.owner_) {}
  BufferingGuard& operator=(const BufferingGuard& other) {
    this->~BufferingGuard();
    new (this) BufferingGuard(other);
    return *this;
  }

 private:
  friend class Mark;
  ZeroCopyBufferedStream* owner_ = nullptr;
};

// A string that may own its contents, or live inside of a buffer owned by
// a ZeroCopyBufferedStream.
//
// Note that this type holds onto a reference to the owning
// ZeroCopyBufferedStream; this allows it to be durable against strings being
// moved around for buffering puroses.
class MaybeOwnedString {
 public:
  explicit MaybeOwnedString(std::string value) : data_(std::move(value)) {}
  MaybeOwnedString(ZeroCopyBufferedStream* stream, size_t start, size_t len,
                   BufferingGuard token)
      : data_(StreamOwned{stream, start, len}), token_(token) {}

  // Returns the string as a view, regardless of whether it is owned or not.
  absl::string_view AsView() const {
    if (auto* unowned = absl::get_if<StreamOwned>(&data_)) {
      return unowned->AsView();
    }

    return absl::get<std::string>(data_);
  }

  operator absl::string_view() const { return AsView(); }  // NOLINT

  // Returns a reference to an owned string; if the wrapped string is not
  // owned, this function will perform a copy and make it owned.
  std::string& ToString() {
    if (auto* unowned = absl::get_if<StreamOwned>(&data_)) {
      data_ = std::string(unowned->AsView());
      token_ = BufferingGuard{};
    }

    return absl::get<std::string>(data_);
  }

  template <typename String>
  friend bool operator==(const MaybeOwnedString& lhs, const String& rhs) {
    return lhs.AsView() == rhs;
  }
  template <typename String>
  friend bool operator!=(const MaybeOwnedString& lhs, const String& rhs) {
    return !(lhs == rhs);
  }

 private:
  struct StreamOwned {
    ZeroCopyBufferedStream* stream;
    size_t start, len;
    absl::string_view AsView() const;
  };
  absl::variant<std::string, StreamOwned> data_;
  BufferingGuard token_;
};

// A mark in a stream. See ZeroCopyBufferedStream::Mark().
class Mark {
 public:
  // Returns a maybe-owned string up to the unread bytes boundary, except for
  // the last `clip` bytes.
  MaybeOwnedString UpToUnread(size_t clip = 0) const;

  // Discards this mark and its hold on the buffer.
  void Discard() && { guard_ = BufferingGuard(); }

 private:
  friend ZeroCopyBufferedStream;
  Mark(size_t offset, BufferingGuard guard) : offset_(offset), guard_(guard) {}

  size_t offset_;
  BufferingGuard guard_;
};

// A wrapper over a ZeroCopyInputStream that allows doing as-needed buffer for
// obtaining contiguous chunks larger than those the underlying stream might
// provide, while minimizing the amount of actual copying.
class ZeroCopyBufferedStream {
 public:
  explicit ZeroCopyBufferedStream(io::ZeroCopyInputStream* stream)
      : stream_(stream) {}

  // Returns whether the stream is currently at eof.
  //
  // This function will buffer at least one character to verify whether it
  // actually *is* at EOF.
  bool AtEof() {
    (void)BufferAtLeast(1);
    return eof_;
  }

  // Takes exactly n characters from a string.
  absl::StatusOr<MaybeOwnedString> Take(size_t len) {
    auto buffering = BufferAtLeast(len);
    RETURN_IF_ERROR(buffering.status());

    size_t start = cursor_;
    RETURN_IF_ERROR(Advance(len));
    return MaybeOwnedString(this, start, len, *buffering);
  }

  // Takes characters to form a string, according to the given predicate. Stops
  // early if an EOF is hit.
  //
  // The predicate must have type `(int, char) -> bool`; the first argument
  // is the index of the character.
  template <typename Pred>
  absl::StatusOr<MaybeOwnedString> TakeWhile(Pred p);

  // Places a mark in the stream, ensuring that all characters consumed after
  // the mark are buffered. This can be used to parse some characters and then
  // recover everything that follows as a contiguous string_view so that it may
  // be processed a second time.
  //
  // The returned value is an RAII type that ensure the buffer sticks around
  // long enough.
  Mark BeginMark() { return Mark(cursor_, BufferingGuard(this)); }

  // Peeks the next character in the stream.
  //
  // This function will not enable buffering on its own, and will read past the
  // end of the buffer if at EOF; BufferAtLeast() should be called before
  // calling this function.
  char PeekChar() {
    ABSL_DCHECK(!Unread().empty());
    return Unread()[0];
  }

  // Advances the cursor by the given number of bytes.
  absl::Status Advance(size_t bytes);

  // Returns a view of the current buffer, which may be either the owned
  // `buf_` or the stream-owned `last_chunk_`.
  //
  // The returned view is unstable: calling any function may invalidate it,
  // because there will not be a `BufferingGuard` to guard it.
  absl::string_view RawBuffer(size_t start,
                              size_t len = absl::string_view::npos) const;

  // Returns a view of RawBuffer, unread bytes; this will not be the entirety
  // of the underlying stream.
  absl::string_view Unread() const { return RawBuffer(cursor_); }

  bool IsBuffering() const { return using_buf_; }

  // Buffers at least `bytes` bytes ahead of the current cursor position,
  // possibly enabling buffering.
  //
  // Returns an error if that many bytes could not be RawBuffer.
  absl::StatusOr<BufferingGuard> BufferAtLeast(size_t bytes);

 private:
  friend BufferingGuard;
  friend Mark;
  friend MaybeOwnedString;

  // Increments the buffering refcount; this will also update `buffer_start_` if
  // necessary.
  void UpRefBuffer() {
    if (outstanding_buffer_borrows_++ == 0) {
      buffer_start_ = cursor_;
    }
  }

  // Decrements the buffering refcount; calling this function if the refcount is
  // zero is undefined behavior.
  //
  // This function should not be called directly; it is called automatically
  // by the destructor of `BufferingGuard`.
  void DownRefBuffer();

  // Obtains a new chunk from the underlying stream; returns whether there is
  // still more data to read.
  bool ReadChunk();

  // The streamer implements a buffering stream on top of the given stream, by
  // the following mechanism:
  // - `cursor_` is an offset into either `last_chunk_` or `buf_`, which can
  //   be obtained via RawBuffer() and Unread():
  //   - If `using_buf_` is true, it is an offset into `buf_`.
  //   - Otherwise it is an offset into `last_chunk_`.
  // - If `outstanding_buffer_borrows_ > 0`, someone needs the buffer to stick
  //   around. MaybeUnownedString::StreamOwned is implemented such that it does
  //   not hold onto `last_chunk_` directly, so we can freely copy it into
  //   `buf_` as needed arises.
  //   - Note that we can copy only part if we update `buffer_start_`; see
  //     RawBuffer().
  // - If we would read more data and `outstanding_buffer_borrows_ > 0`, instead
  //   of trashing `last_chunk_`, we copy it into `buf_` and append to `buf_`
  //   each time we read.
  // - If `outstanding_buffer_borrows_ == 0`, we can trash `buf_` and go back to
  //   using `last_chunk_` directly. See `DownRefBuffer()`.
  io::ZeroCopyInputStream* stream_;
  absl::string_view last_chunk_;
  std::vector<char> buf_;
  bool using_buf_ = false;
  size_t cursor_ = 0;
  // Invariant: this always refers to the earliest point at which we requested
  // buffering, since the last time outstanding_buffer_borrows_ was zero.
  size_t buffer_start_ = 0;
  bool eof_ = false;
  int outstanding_buffer_borrows_ = 0;
};

// These functions all rely on the definition of ZeroCopyBufferedStream, so must
// come after it.
inline BufferingGuard::BufferingGuard(ZeroCopyBufferedStream* owner)
    : owner_(owner) {
  if (owner_ != nullptr) {
    owner_->UpRefBuffer();
  }
}

inline BufferingGuard::~BufferingGuard() {
  if (owner_ != nullptr) {
    owner_->DownRefBuffer();
    owner_ = nullptr;
  }
}

inline absl::string_view MaybeOwnedString::StreamOwned::AsView() const {
  return stream->RawBuffer(start, len);
}

inline MaybeOwnedString Mark::UpToUnread(size_t clip) const {
  return MaybeOwnedString(guard_.owner_, offset_,
                          guard_.owner_->cursor_ - offset_ - clip, guard_);
}

template <typename Pred>
absl::StatusOr<MaybeOwnedString> ZeroCopyBufferedStream::TakeWhile(Pred p) {
  size_t start = cursor_;
  BufferingGuard guard(this);
  while (true) {
    if (!BufferAtLeast(1).ok()) {
      // We treat EOF as ending the take, rather than being an error.
      break;
    }
    if (!p(cursor_ - start, PeekChar())) {
      break;
    }
    RETURN_IF_ERROR(Advance(1));
  }

  return MaybeOwnedString(this, start, cursor_ - start, guard);
}

inline absl::string_view ZeroCopyBufferedStream::RawBuffer(size_t start,
                                                           size_t len) const {
  absl::string_view view = last_chunk_;
  if (using_buf_) {
    ABSL_DCHECK_LE(buffer_start_, start);
    start -= buffer_start_;
    view = absl::string_view(buf_.data(), buf_.size());
  }
#if 0
    // This print statement is especially useful for trouble-shooting low-level
    // bugs in the buffering logic.
    ABSL_LOG(INFO) << absl::StreamFormat("%s(\"%s\")[%d:%d]/%d:%d @ %p",
                                    using_buf_ ? "buf_" : "last_chunk_",
                                    view, start, static_cast<int>(len),
                                    buffer_start_, cursor_, this);
#endif
  ABSL_DCHECK_LE(start, view.size());
  if (len == absl::string_view::npos) {
    return view.substr(start);
  }

  ABSL_DCHECK_LE(start + len, view.size());
  return view.substr(start, len);
}
}  // namespace json_internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"
#endif  // GOOGLE_PROTOBUF_JSON_INTERNAL_ZERO_COPY_BUFFERED_STREAM_H__
