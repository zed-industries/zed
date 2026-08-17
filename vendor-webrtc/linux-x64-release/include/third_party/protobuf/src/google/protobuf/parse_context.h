// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PARSE_CONTEXT_H__
#define GOOGLE_PROTOBUF_PARSE_CONTEXT_H__

#include <algorithm>
#include <climits>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <limits>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/base/config.h"
#include "absl/base/prefetch.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/strings/cord.h"
#include "absl/strings/string_view.h"
#include "absl/types/optional.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/arenastring.h"
#include "google/protobuf/endian.h"
#include "google/protobuf/inlined_string_field.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/io/zero_copy_stream.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/metadata_lite.h"
#include "google/protobuf/port.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "google/protobuf/wire_format_lite.h"


// Must be included last.
#include "google/protobuf/port_def.inc"


namespace google {
namespace protobuf {

class UnknownFieldSet;
class DescriptorPool;
class MessageFactory;

namespace internal {

// Template code below needs to know about the existence of these functions.
PROTOBUF_EXPORT void WriteVarint(uint32_t num, uint64_t val, std::string* s);
PROTOBUF_EXPORT void WriteLengthDelimited(uint32_t num, absl::string_view val,
                                          std::string* s);
// Inline because it is just forwarding to s->WriteVarint
inline void WriteVarint(uint32_t num, uint64_t val, UnknownFieldSet* unknown);
inline void WriteLengthDelimited(uint32_t num, absl::string_view val,
                                 UnknownFieldSet* unknown);


// The basic abstraction the parser is designed for is a slight modification
// of the ZeroCopyInputStream (ZCIS) abstraction. A ZCIS presents a serialized
// stream as a series of buffers that concatenate to the full stream.
// Pictorially a ZCIS presents a stream in chunks like so
// [---------------------------------------------------------------]
// [---------------------] chunk 1
//                      [----------------------------] chunk 2
//                                          chunk 3 [--------------]
//
// Where the '-' represent the bytes which are vertically lined up with the
// bytes of the stream. The proto parser requires its input to be presented
// similarly with the extra
// property that each chunk has kSlopBytes past its end that overlaps with the
// first kSlopBytes of the next chunk, or if there is no next chunk at least its
// still valid to read those bytes. Again, pictorially, we now have
//
// [---------------------------------------------------------------]
// [-------------------....] chunk 1
//                    [------------------------....] chunk 2
//                                    chunk 3 [------------------..**]
//                                                      chunk 4 [--****]
// Here '-' mean the bytes of the stream or chunk and '.' means bytes past the
// chunk that match up with the start of the next chunk. Above each chunk has
// 4 '.' after the chunk. In the case these 'overflow' bytes represents bytes
// past the stream, indicated by '*' above, their values are unspecified. It is
// still legal to read them (ie. should not segfault). Reading past the
// end should be detected by the user and indicated as an error.
//
// The reason for this, admittedly, unconventional invariant is to ruthlessly
// optimize the protobuf parser. Having an overlap helps in two important ways.
// Firstly it alleviates having to performing bounds checks if a piece of code
// is guaranteed to not read more than kSlopBytes. Secondly, and more
// importantly, the protobuf wireformat is such that reading a key/value pair is
// always less than 16 bytes. This removes the need to change to next buffer in
// the middle of reading primitive values. Hence there is no need to store and
// load the current position.

class PROTOBUF_EXPORT EpsCopyInputStream {
 public:
  enum { kMaxCordBytesToCopy = 512 };
  explicit EpsCopyInputStream(bool enable_aliasing)
      : aliasing_(enable_aliasing ? kOnPatch : kNoAliasing) {}

  void BackUp(const char* ptr) {
    ABSL_DCHECK(ptr <= buffer_end_ + kSlopBytes);
    int count;
    if (next_chunk_ == patch_buffer_) {
      count = BytesAvailable(ptr);
    } else {
      count = size_ + static_cast<int>(buffer_end_ - ptr);
    }
    if (count > 0) StreamBackUp(count);
  }

  // In sanitizer mode we use memory poisoning to guarantee that:
  //  - We do not read an uninitialized token.
  //  - We would like to verify that this token was consumed, but unfortunately
  //    __asan_address_is_poisoned is allowed to have false negatives.
  class LimitToken {
   public:
    LimitToken() { internal::PoisonMemoryRegion(&token_, sizeof(token_)); }

    explicit LimitToken(int token) : token_(token) {
      internal::UnpoisonMemoryRegion(&token_, sizeof(token_));
    }

    LimitToken(const LimitToken&) = delete;
    LimitToken& operator=(const LimitToken&) = delete;

    LimitToken(LimitToken&& other) { *this = std::move(other); }

    LimitToken& operator=(LimitToken&& other) {
      internal::UnpoisonMemoryRegion(&token_, sizeof(token_));
      token_ = other.token_;
      internal::PoisonMemoryRegion(&other.token_, sizeof(token_));
      return *this;
    }

    ~LimitToken() { internal::UnpoisonMemoryRegion(&token_, sizeof(token_)); }

    int token() && {
      int t = token_;
      internal::PoisonMemoryRegion(&token_, sizeof(token_));
      return t;
    }

   private:
    int token_;
  };

  // If return value is negative it's an error
  [[nodiscard]] LimitToken PushLimit(const char* ptr, int limit) {
    ABSL_DCHECK(limit >= 0 && limit <= INT_MAX - kSlopBytes);
    // This add is safe due to the invariant above, because
    // ptr - buffer_end_ <= kSlopBytes.
    limit += static_cast<int>(ptr - buffer_end_);
    limit_end_ = buffer_end_ + (std::min)(0, limit);
    auto old_limit = limit_;
    limit_ = limit;
    return LimitToken(old_limit - limit);
  }

  [[nodiscard]] bool PopLimit(LimitToken delta) {
    // We must update the limit first before the early return. Otherwise, we can
    // end up with an invalid limit and it can lead to integer overflows.
    limit_ = limit_ + std::move(delta).token();
    if (ABSL_PREDICT_FALSE(!EndedAtLimit())) return false;
    // TODO We could remove this line and hoist the code to
    // DoneFallback. Study the perf/bin-size effects.
    limit_end_ = buffer_end_ + (std::min)(0, limit_);
    return true;
  }

  [[nodiscard]] const char* Skip(const char* ptr, int size) {
    if (size <= BytesAvailable(ptr)) {
      return ptr + size;
    }
    return SkipFallback(ptr, size);
  }
  [[nodiscard]] const char* ReadString(const char* ptr, int size,
                                       std::string* s) {
    if (size <= BytesAvailable(ptr)) {
      // Fundamentally we just want to do assign to the string.
      // However micro-benchmarks regress on string reading cases. So we copy
      // the same logic from the old CodedInputStream ReadString. Note: as of
      // Apr 2021, this is still a significant win over `assign()`.
      s->resize(size);
      char* z = &(*s)[0];
      memcpy(z, ptr, size);
      return ptr + size;
    }
    return ReadStringFallback(ptr, size, s);
  }
  [[nodiscard]] const char* AppendString(const char* ptr, int size,
                                         std::string* s) {
    if (size <= BytesAvailable(ptr)) {
      s->append(ptr, size);
      return ptr + size;
    }
    return AppendStringFallback(ptr, size, s);
  }
  // Implemented in arenastring.cc
  [[nodiscard]] const char* ReadArenaString(const char* ptr, ArenaStringPtr* s,
                                            Arena* arena);

  [[nodiscard]] const char* ReadCord(const char* ptr, int size,
                                     ::absl::Cord* cord) {
    if (size <= std::min<int>(BytesAvailable(ptr), kMaxCordBytesToCopy)) {
      *cord = absl::string_view(ptr, size);
      return ptr + size;
    }
    return ReadCordFallback(ptr, size, cord);
  }


  template <typename Tag, typename T>
  [[nodiscard]] const char* ReadRepeatedFixed(const char* ptr, Tag expected_tag,
                                              RepeatedField<T>* out);

  template <typename T>
  [[nodiscard]] const char* ReadPackedFixed(const char* ptr, int size,
                                            RepeatedField<T>* out);
  template <typename Add>
  [[nodiscard]] const char* ReadPackedVarint(const char* ptr, Add add) {
    return ReadPackedVarint(ptr, add, [](int) {});
  }
  template <typename Add, typename SizeCb>
  [[nodiscard]] const char* ReadPackedVarint(const char* ptr, Add add,
                                             SizeCb size_callback);

  uint32_t LastTag() const { return last_tag_minus_1_ + 1; }
  bool ConsumeEndGroup(uint32_t start_tag) {
    bool res = last_tag_minus_1_ == start_tag;
    last_tag_minus_1_ = 0;
    return res;
  }
  bool EndedAtLimit() const { return last_tag_minus_1_ == 0; }
  bool EndedAtEndOfStream() const { return last_tag_minus_1_ == 1; }
  void SetLastTag(uint32_t tag) { last_tag_minus_1_ = tag - 1; }
  void SetEndOfStream() { last_tag_minus_1_ = 1; }
  bool IsExceedingLimit(const char* ptr) {
    return ptr > limit_end_ &&
           (next_chunk_ == nullptr || ptr - buffer_end_ > limit_);
  }
  bool AliasingEnabled() const { return aliasing_ != kNoAliasing; }
  int BytesUntilLimit(const char* ptr) const {
    return limit_ + static_cast<int>(buffer_end_ - ptr);
  }
  // Maximum number of sequential bytes that can be read starting from `ptr`.
  int MaximumReadSize(const char* ptr) const {
    return static_cast<int>(limit_end_ - ptr) + kSlopBytes;
  }
  // Returns true if more data is available, if false is returned one has to
  // call Done for further checks.
  bool DataAvailable(const char* ptr) { return ptr < limit_end_; }

 protected:
  // Returns true if limit (either an explicit limit or end of stream) is
  // reached. It aligns *ptr across buffer seams.
  // If limit is exceeded, it returns true and ptr is set to null.
  bool DoneWithCheck(const char** ptr, int d) {
    ABSL_DCHECK(*ptr);
    if (ABSL_PREDICT_TRUE(*ptr < limit_end_)) return false;
    int overrun = static_cast<int>(*ptr - buffer_end_);
    ABSL_DCHECK_LE(overrun, kSlopBytes);  // Guaranteed by parse loop.
    if (overrun ==
        limit_) {  //  No need to flip buffers if we ended on a limit.
      // If we actually overrun the buffer and next_chunk_ is null, it means
      // the stream ended and we passed the stream end.
      if (overrun > 0 && next_chunk_ == nullptr) *ptr = nullptr;
      return true;
    }
    auto res = DoneFallback(overrun, d);
    *ptr = res.first;
    return res.second;
  }

  const char* InitFrom(absl::string_view flat) {
    overall_limit_ = 0;
    if (flat.size() > kSlopBytes) {
      limit_ = kSlopBytes;
      limit_end_ = buffer_end_ = flat.data() + flat.size() - kSlopBytes;
      next_chunk_ = patch_buffer_;
      if (aliasing_ == kOnPatch) aliasing_ = kNoDelta;
      return flat.data();
    } else {
      if (!flat.empty()) {
        std::memcpy(patch_buffer_, flat.data(), flat.size());
      }
      limit_ = 0;
      limit_end_ = buffer_end_ = patch_buffer_ + flat.size();
      next_chunk_ = nullptr;
      if (aliasing_ == kOnPatch) {
        aliasing_ = reinterpret_cast<std::uintptr_t>(flat.data()) -
                    reinterpret_cast<std::uintptr_t>(patch_buffer_);
      }
      return patch_buffer_;
    }
  }

  const char* InitFrom(io::ZeroCopyInputStream* zcis);

  const char* InitFrom(io::ZeroCopyInputStream* zcis, int limit) {
    if (limit == -1) return InitFrom(zcis);
    overall_limit_ = limit;
    auto res = InitFrom(zcis);
    limit_ = limit - static_cast<int>(buffer_end_ - res);
    limit_end_ = buffer_end_ + (std::min)(0, limit_);
    return res;
  }

 private:
  enum { kSlopBytes = 16, kPatchBufferSize = 32 };
  static_assert(kPatchBufferSize >= kSlopBytes * 2,
                "Patch buffer needs to be at least large enough to hold all "
                "the slop bytes from the previous buffer, plus the first "
                "kSlopBytes from the next buffer.");

  const char* limit_end_;  // buffer_end_ + min(limit_, 0)
  const char* buffer_end_;
  const char* next_chunk_;
  int size_;
  int limit_;  // relative to buffer_end_;
  io::ZeroCopyInputStream* zcis_ = nullptr;
  char patch_buffer_[kPatchBufferSize] = {};
  enum { kNoAliasing = 0, kOnPatch = 1, kNoDelta = 2 };
  std::uintptr_t aliasing_ = kNoAliasing;
  // This variable is used to communicate how the parse ended, in order to
  // completely verify the parsed data. A wire-format parse can end because of
  // one of the following conditions:
  // 1) A parse can end on a pushed limit.
  // 2) A parse can end on End Of Stream (EOS).
  // 3) A parse can end on 0 tag (only valid for toplevel message).
  // 4) A parse can end on an end-group tag.
  // This variable should always be set to 0, which indicates case 1. If the
  // parse terminated due to EOS (case 2), it's set to 1. In case the parse
  // ended due to a terminating tag (case 3 and 4) it's set to (tag - 1).
  // This var doesn't really belong in EpsCopyInputStream and should be part of
  // the ParseContext, but case 2 is most easily and optimally implemented in
  // DoneFallback.
  uint32_t last_tag_minus_1_ = 0;
  int overall_limit_ = INT_MAX;  // Overall limit independent of pushed limits.
  // Pretty random large number that seems like a safe allocation on most
  // systems. TODO do we need to set this as build flag?
  enum { kSafeStringSize = 50000000 };

  int BytesAvailable(const char* ptr) const {
    ABSL_DCHECK_NE(ptr, nullptr);
    ptrdiff_t available = buffer_end_ + kSlopBytes - ptr;
    ABSL_DCHECK_GE(available, 0);
    ABSL_DCHECK_LE(available, INT_MAX);
    return static_cast<int>(available);
  }

  // Advances to next buffer chunk returns a pointer to the same logical place
  // in the stream as set by overrun. Overrun indicates the position in the slop
  // region the parse was left (0 <= overrun <= kSlopBytes). Returns true if at
  // limit, at which point the returned pointer maybe null if there was an
  // error. The invariant of this function is that it's guaranteed that
  // kSlopBytes bytes can be accessed from the returned ptr. This function might
  // advance more buffers than one in the underlying ZeroCopyInputStream.
  std::pair<const char*, bool> DoneFallback(int overrun, int depth);
  // Advances to the next buffer, at most one call to Next() on the underlying
  // ZeroCopyInputStream is made. This function DOES NOT match the returned
  // pointer to where in the slop region the parse ends, hence no overrun
  // parameter. This is useful for string operations where you always copy
  // to the end of the buffer (including the slop region).
  const char* Next();
  // overrun is the location in the slop region the stream currently is
  // (0 <= overrun <= kSlopBytes). To prevent flipping to the next buffer of
  // the ZeroCopyInputStream in the case the parse will end in the last
  // kSlopBytes of the current buffer. depth is the current depth of nested
  // groups (or negative if the use case does not need careful tracking).
  inline const char* NextBuffer(int overrun, int depth);
  const char* SkipFallback(const char* ptr, int size);
  const char* AppendStringFallback(const char* ptr, int size, std::string* str);
  const char* ReadStringFallback(const char* ptr, int size, std::string* str);
  const char* ReadCordFallback(const char* ptr, int size, absl::Cord* cord);
  static bool ParseEndsInSlopRegion(const char* begin, int overrun, int depth);
  bool StreamNext(const void** data) {
    bool res = zcis_->Next(data, &size_);
    if (res) overall_limit_ -= size_;
    return res;
  }
  void StreamBackUp(int count) {
    zcis_->BackUp(count);
    overall_limit_ += count;
  }

  template <typename A>
  const char* AppendSize(const char* ptr, int size, const A& append) {
    int chunk_size = BytesAvailable(ptr);
    do {
      ABSL_DCHECK(size > chunk_size);
      if (next_chunk_ == nullptr) return nullptr;
      append(ptr, chunk_size);
      ptr += chunk_size;
      size -= chunk_size;
      // TODO Next calls NextBuffer which generates buffers with
      // overlap and thus incurs cost of copying the slop regions. This is not
      // necessary for reading strings. We should just call Next buffers.
      if (limit_ <= kSlopBytes) return nullptr;
      ptr = Next();
      if (ptr == nullptr) return nullptr;  // passed the limit
      ptr += kSlopBytes;
      chunk_size = BytesAvailable(ptr);
    } while (size > chunk_size);
    append(ptr, size);
    return ptr + size;
  }

  // AppendUntilEnd appends data until a limit (either a PushLimit or end of
  // stream. Normal payloads are from length delimited fields which have an
  // explicit size. Reading until limit only comes when the string takes
  // the place of a protobuf, ie RawMessage, lazy fields and implicit weak
  // messages. We keep these methods private and friend them.
  template <typename A>
  const char* AppendUntilEnd(const char* ptr, const A& append) {
    if (ptr - buffer_end_ > limit_) return nullptr;
    while (limit_ > kSlopBytes) {
      size_t chunk_size = BytesAvailable(ptr);
      append(ptr, chunk_size);
      ptr = Next();
      if (ptr == nullptr) return limit_end_;
      ptr += kSlopBytes;
    }
    auto end = buffer_end_ + limit_;
    ABSL_DCHECK(end >= ptr);
    append(ptr, end - ptr);
    return end;
  }

  [[nodiscard]] const char* AppendString(const char* ptr, std::string* str) {
    return AppendUntilEnd(
        ptr, [str](const char* p, ptrdiff_t s) { str->append(p, s); });
  }
  friend class ImplicitWeakMessage;

  // Needs access to kSlopBytes.
  friend PROTOBUF_EXPORT std::pair<const char*, int32_t> ReadSizeFallback(
      const char* p, uint32_t res);
};

using LazyEagerVerifyFnType = const char* (*)(const char* ptr,
                                              ParseContext* ctx);
using LazyEagerVerifyFnRef = std::remove_pointer<LazyEagerVerifyFnType>::type&;

// ParseContext holds all data that is global to the entire parse. Most
// importantly it contains the input stream, but also recursion depth and also
// stores the end group tag, in case a parser ended on a endgroup, to verify
// matching start/end group tags.
class PROTOBUF_EXPORT ParseContext : public EpsCopyInputStream {
 public:
  struct Data {
    const DescriptorPool* pool = nullptr;
    MessageFactory* factory = nullptr;
  };

  template <typename... T>
  ParseContext(int depth, bool aliasing, const char** start, T&&... args)
      : EpsCopyInputStream(aliasing), depth_(depth) {
    *start = InitFrom(std::forward<T>(args)...);
  }

  struct Spawn {};
  static constexpr Spawn kSpawn = {};

  // Creates a new context from a given "ctx" to inherit a few attributes to
  // emulate continued parsing. For example, recursion depth or descriptor pools
  // must be passed down to a new "spawned" context to maintain the same parse
  // context. Note that the spawned context always disables aliasing (different
  // input).
  template <typename... T>
  ParseContext(Spawn, const ParseContext& ctx, const char** start, T&&... args)
      : EpsCopyInputStream(false),
        depth_(ctx.depth_),
        data_(ctx.data_)
  {
    *start = InitFrom(std::forward<T>(args)...);
  }

  // Move constructor and assignment operator are not supported because "ptr"
  // for parsing may have pointed to an inlined buffer (patch_buffer_) which can
  // be invalid afterwards.
  ParseContext(ParseContext&&) = delete;
  ParseContext& operator=(ParseContext&&) = delete;
  ParseContext& operator=(const ParseContext&) = delete;

  void TrackCorrectEnding() { group_depth_ = 0; }

  // Done should only be called when the parsing pointer is pointing to the
  // beginning of field data - that is, at a tag.  Or if it is NULL.
  bool Done(const char** ptr) { return DoneWithCheck(ptr, group_depth_); }

  int depth() const { return depth_; }

  Data& data() { return data_; }
  const Data& data() const { return data_; }

  const char* ParseMessage(MessageLite* msg, const char* ptr);

  // Read the length prefix, push the new limit, call the func(ptr), and then
  // pop the limit. Useful for situations that don't have an actual message.
  template <typename Func>
  [[nodiscard]] const char* ParseLengthDelimitedInlined(const char*,
                                                        const Func& func);

  // Push the recursion depth, call the func(ptr), and then pop depth. Useful
  // for situations that don't have an actual message.
  template <typename Func>
  [[nodiscard]] const char* ParseGroupInlined(const char* ptr,
                                              uint32_t start_tag,
                                              const Func& func);

  // Use a template to avoid the strong dep into TcParser. All callers will have
  // the dep.
  template <typename Parser = TcParser>
  PROTOBUF_ALWAYS_INLINE const char* ParseMessage(
      MessageLite* msg, const TcParseTableBase* tc_table, const char* ptr) {
    return ParseLengthDelimitedInlined(ptr, [&](const char* ptr) {
      return Parser::ParseLoop(msg, ptr, this, tc_table);
    });
  }
  template <typename Parser = TcParser>
  PROTOBUF_ALWAYS_INLINE const char* ParseGroup(
      MessageLite* msg, const TcParseTableBase* tc_table, const char* ptr,
      uint32_t start_tag) {
    return ParseGroupInlined(ptr, start_tag, [&](const char* ptr) {
      return Parser::ParseLoop(msg, ptr, this, tc_table);
    });
  }

  [[nodiscard]] PROTOBUF_NDEBUG_INLINE const char* ParseGroup(MessageLite* msg,
                                                              const char* ptr,
                                                              uint32_t tag) {
    if (--depth_ < 0) return nullptr;
    group_depth_++;
    auto old_depth = depth_;
    auto old_group_depth = group_depth_;
    ptr = msg->_InternalParse(ptr, this);
    if (ptr != nullptr) {
      ABSL_DCHECK_EQ(old_depth, depth_);
      ABSL_DCHECK_EQ(old_group_depth, group_depth_);
    }
    group_depth_--;
    depth_++;
    if (ABSL_PREDICT_FALSE(!ConsumeEndGroup(tag))) return nullptr;
    return ptr;
  }

 private:
  // Out-of-line routine to save space in ParseContext::ParseMessage<T>
  //   LimitToken old;
  //   ptr = ReadSizeAndPushLimitAndDepth(ptr, &old)
  // is equivalent to:
  //   int size = ReadSize(&ptr);
  //   if (!ptr) return nullptr;
  //   LimitToken old = PushLimit(ptr, size);
  //   if (--depth_ < 0) return nullptr;
  [[nodiscard]] const char* ReadSizeAndPushLimitAndDepth(const char* ptr,
                                                         LimitToken* old_limit);

  // As above, but fully inlined for the cases where we care about performance
  // more than size. eg TcParser.
  [[nodiscard]] PROTOBUF_ALWAYS_INLINE const char*
  ReadSizeAndPushLimitAndDepthInlined(const char* ptr, LimitToken* old_limit);

  // The context keeps an internal stack to keep track of the recursive
  // part of the parse state.
  // Current depth of the active parser, depth counts down.
  // This is used to limit recursion depth (to prevent overflow on malicious
  // data), but is also used to index in stack_ to store the current state.
  int depth_;
  // Unfortunately necessary for the fringe case of ending on 0 or end-group tag
  // in the last kSlopBytes of a ZeroCopyInputStream chunk.
  int group_depth_ = INT_MIN;
  Data data_;
};

template <int>
struct EndianHelper;

template <>
struct EndianHelper<1> {
  static uint8_t Load(const void* p) { return *static_cast<const uint8_t*>(p); }
};

template <>
struct EndianHelper<2> {
  static uint16_t Load(const void* p) {
    uint16_t tmp;
    std::memcpy(&tmp, p, 2);
    return little_endian::ToHost(tmp);
  }
};

template <>
struct EndianHelper<4> {
  static uint32_t Load(const void* p) {
    uint32_t tmp;
    std::memcpy(&tmp, p, 4);
    return little_endian::ToHost(tmp);
  }
};

template <>
struct EndianHelper<8> {
  static uint64_t Load(const void* p) {
    uint64_t tmp;
    std::memcpy(&tmp, p, 8);
    return little_endian::ToHost(tmp);
  }
};

template <typename T>
T UnalignedLoad(const char* p) {
  auto tmp = EndianHelper<sizeof(T)>::Load(p);
  T res;
  memcpy(&res, &tmp, sizeof(T));
  return res;
}
template <typename T, typename Void,
          typename = std::enable_if_t<std::is_same<Void, void>::value>>
T UnalignedLoad(const Void* p) {
  return UnalignedLoad<T>(reinterpret_cast<const char*>(p));
}

PROTOBUF_EXPORT
std::pair<const char*, uint32_t> VarintParseSlow32(const char* p, uint32_t res);
PROTOBUF_EXPORT
std::pair<const char*, uint64_t> VarintParseSlow64(const char* p, uint32_t res);

inline const char* VarintParseSlow(const char* p, uint32_t res, uint32_t* out) {
  auto tmp = VarintParseSlow32(p, res);
  *out = tmp.second;
  return tmp.first;
}

inline const char* VarintParseSlow(const char* p, uint32_t res, uint64_t* out) {
  auto tmp = VarintParseSlow64(p, res);
  *out = tmp.second;
  return tmp.first;
}

#if defined(__aarch64__) && !defined(_MSC_VER)
// Generally, speaking, the ARM-optimized Varint decode algorithm is to extract
// and concatenate all potentially valid data bits, compute the actual length
// of the Varint, and mask off the data bits which are not actually part of the
// result.  More detail on the two main parts is shown below.
//
// 1) Extract and concatenate all potentially valid data bits.
//    Two ARM-specific features help significantly:
//    a) Efficient and non-destructive bit extraction (UBFX)
//    b) A single instruction can perform both an OR with a shifted
//       second operand in one cycle.  E.g., the following two lines do the same
//       thing
//       ```result = operand_1 | (operand2 << 7);```
//       ```ORR %[result], %[operand_1], %[operand_2], LSL #7```
//    The figure below shows the implementation for handling four chunks.
//
// Bits   32    31-24    23   22-16    15    14-8      7     6-0
//      +----+---------+----+---------+----+---------+----+---------+
//      |CB 3| Chunk 3 |CB 2| Chunk 2 |CB 1| Chunk 1 |CB 0| Chunk 0 |
//      +----+---------+----+---------+----+---------+----+---------+
//                |              |              |              |
//               UBFX           UBFX           UBFX           UBFX    -- cycle 1
//                |              |              |              |
//                V              V              V              V
//               Combined LSL #7 and ORR     Combined LSL #7 and ORR  -- cycle 2
//                                 |             |
//                                 V             V
//                            Combined LSL #14 and ORR                -- cycle 3
//                                       |
//                                       V
//                                Parsed bits 0-27
//
//
// 2) Calculate the index of the cleared continuation bit in order to determine
//    where the encoded Varint ends and the size of the decoded value.  The
//    easiest way to do this is mask off all data bits, leaving just the
//    continuation bits.  We actually need to do the masking on an inverted
//    copy of the data, which leaves a 1 in all continuation bits which were
//    originally clear.  The number of trailing zeroes in this value indicates
//    the size of the Varint.
//
//  AND  0x80    0x80    0x80    0x80    0x80    0x80    0x80    0x80
//
// Bits   63      55      47      39      31      23      15       7
//      +----+--+----+--+----+--+----+--+----+--+----+--+----+--+----+--+
// ~    |CB 7|  |CB 6|  |CB 5|  |CB 4|  |CB 3|  |CB 2|  |CB 1|  |CB 0|  |
//      +----+--+----+--+----+--+----+--+----+--+----+--+----+--+----+--+
//         |       |       |       |       |       |       |       |
//         V       V       V       V       V       V       V       V
// Bits   63      55      47      39      31      23      15       7
//      +----+--+----+--+----+--+----+--+----+--+----+--+----+--+----+--+
//      |~CB 7|0|~CB 6|0|~CB 5|0|~CB 4|0|~CB 3|0|~CB 2|0|~CB 1|0|~CB 0|0|
//      +----+--+----+--+----+--+----+--+----+--+----+--+----+--+----+--+
//                                      |
//                                     CTZ
//                                      V
//                     Index of first cleared continuation bit
//
//
// While this is implemented in C++ significant care has been taken to ensure
// the compiler emits the best instruction sequence.  In some cases we use the
// following two functions to manipulate the compiler's scheduling decisions.
//
// Controls compiler scheduling by telling it that the first value is modified
// by the second value the callsite.  This is useful if non-critical path
// instructions are too aggressively scheduled, resulting in a slowdown of the
// actual critical path due to opportunity costs.  An example usage is shown
// where a false dependence of num_bits on result is added to prevent checking
// for a very unlikely error until all critical path instructions have been
// fetched.
//
// ```
// num_bits = <multiple operations to calculate new num_bits value>
// result = <multiple operations to calculate result>
// num_bits = ValueBarrier(num_bits, result);
// if (num_bits == 63) {
//   ABSL_LOG(FATAL) << "Invalid num_bits value";
// }
// ```
// Falsely indicate that the specific value is modified at this location.  This
// prevents code which depends on this value from being scheduled earlier.
template <typename V1Type>
PROTOBUF_ALWAYS_INLINE V1Type ValueBarrier(V1Type value1) {
  asm("" : "+r"(value1));
  return value1;
}

template <typename V1Type, typename V2Type>
PROTOBUF_ALWAYS_INLINE V1Type ValueBarrier(V1Type value1, V2Type value2) {
  asm("" : "+r"(value1) : "r"(value2));
  return value1;
}

// Performs a 7 bit UBFX (Unsigned Bit Extract) starting at the indicated bit.
static PROTOBUF_ALWAYS_INLINE uint64_t Ubfx7(uint64_t data, uint64_t start) {
  return ValueBarrier((data >> start) & 0x7f);
}

PROTOBUF_ALWAYS_INLINE uint64_t ExtractAndMergeTwoChunks(uint64_t data,
                                                         uint64_t first_byte) {
  ABSL_DCHECK_LE(first_byte, 6U);
  uint64_t first = Ubfx7(data, first_byte * 8);
  uint64_t second = Ubfx7(data, (first_byte + 1) * 8);
  return ValueBarrier(first | (second << 7));
}

struct SlowPathEncodedInfo {
  const char* p;
  uint64_t last8;
  uint64_t valid_bits;
  uint64_t valid_chunk_bits;
  uint64_t masked_cont_bits;
};

// Performs multiple actions which are identical between 32 and 64 bit Varints
// in order to compute the length of the encoded Varint and compute the new
// of p.
PROTOBUF_ALWAYS_INLINE SlowPathEncodedInfo
ComputeLengthAndUpdateP(const char* p) {
  SlowPathEncodedInfo result;
  // Load the last two bytes of the encoded Varint.
  std::memcpy(&result.last8, p + 2, sizeof(result.last8));
  uint64_t mask = ValueBarrier(0x8080808080808080);
  // Only set continuation bits remain
  result.masked_cont_bits = ValueBarrier(mask & ~result.last8);
  // The first cleared continuation bit is the most significant 1 in the
  // reversed value.  Result is undefined for an input of 0 and we handle that
  // case below.
  result.valid_bits = absl::countr_zero(result.masked_cont_bits);
  // Calculates the number of chunks in the encoded Varint.  This value is low
  // by three as neither the cleared continuation chunk nor the first two chunks
  // are counted.
  uint64_t set_continuation_bits = result.valid_bits >> 3;
  // Update p to point past the encoded Varint.
  result.p = p + set_continuation_bits + 3;
  // Calculate number of valid data bits in the decoded value so invalid bits
  // can be masked off.  Value is too low by 14 but we account for that when
  // calculating the mask.
  result.valid_chunk_bits = result.valid_bits - set_continuation_bits;
  return result;
}

PROTOBUF_ALWAYS_INLINE std::pair<const char*, uint64_t> VarintParseSlowArm64(
    const char* p, uint64_t first8) {
  constexpr uint64_t kResultMaskUnshifted = 0xffffffffffffc000ULL;
  constexpr uint64_t kFirstResultBitChunk2 = 2 * 7;
  constexpr uint64_t kFirstResultBitChunk4 = 4 * 7;
  constexpr uint64_t kFirstResultBitChunk6 = 6 * 7;
  constexpr uint64_t kFirstResultBitChunk8 = 8 * 7;

  SlowPathEncodedInfo info = ComputeLengthAndUpdateP(p);
  // Extract data bits from the low six chunks.  This includes chunks zero and
  // one which we already know are valid.
  uint64_t merged_01 = ExtractAndMergeTwoChunks(first8, /*first_chunk=*/0);
  uint64_t merged_23 = ExtractAndMergeTwoChunks(first8, /*first_chunk=*/2);
  uint64_t merged_45 = ExtractAndMergeTwoChunks(first8, /*first_chunk=*/4);
  // Low 42 bits of decoded value.
  uint64_t result = merged_01 | (merged_23 << kFirstResultBitChunk2) |
                    (merged_45 << kFirstResultBitChunk4);
  // This immediate ends in 14 zeroes since valid_chunk_bits is too low by 14.
  uint64_t result_mask = kResultMaskUnshifted << info.valid_chunk_bits;
  //  iff the Varint i invalid.
  if (ABSL_PREDICT_FALSE(info.masked_cont_bits == 0)) {
    return {nullptr, 0};
  }
  // Test for early exit if Varint does not exceed 6 chunks.  Branching on one
  // bit is faster on ARM than via a compare and branch.
  if (ABSL_PREDICT_FALSE((info.valid_bits & 0x20) != 0)) {
    // Extract data bits from high four chunks.
    uint64_t merged_67 = ExtractAndMergeTwoChunks(first8, /*first_chunk=*/6);
    // Last two chunks come from last two bytes of info.last8.
    uint64_t merged_89 =
        ExtractAndMergeTwoChunks(info.last8, /*first_chunk=*/6);
    result |= merged_67 << kFirstResultBitChunk6;
    result |= merged_89 << kFirstResultBitChunk8;
    // Handle an invalid Varint with all 10 continuation bits set.
  }
  // Mask off invalid data bytes.
  result &= ~result_mask;
  return {info.p, result};
}

// See comments in VarintParseSlowArm64 for a description of the algorithm.
// Differences in the 32 bit version are noted below.
PROTOBUF_ALWAYS_INLINE std::pair<const char*, uint32_t> VarintParseSlowArm32(
    const char* p, uint64_t first8) {
  constexpr uint64_t kResultMaskUnshifted = 0xffffffffffffc000ULL;
  constexpr uint64_t kFirstResultBitChunk1 = 1 * 7;
  constexpr uint64_t kFirstResultBitChunk3 = 3 * 7;

  // This also skips the slop bytes.
  SlowPathEncodedInfo info = ComputeLengthAndUpdateP(p);
  // Extract data bits from chunks 1-4.  Chunk zero is merged in below.
  uint64_t merged_12 = ExtractAndMergeTwoChunks(first8, /*first_chunk=*/1);
  uint64_t merged_34 = ExtractAndMergeTwoChunks(first8, /*first_chunk=*/3);
  first8 = ValueBarrier(first8, p);
  uint64_t result = Ubfx7(first8, /*start=*/0);
  result = ValueBarrier(result | merged_12 << kFirstResultBitChunk1);
  result = ValueBarrier(result | merged_34 << kFirstResultBitChunk3);
  uint64_t result_mask = kResultMaskUnshifted << info.valid_chunk_bits;
  result &= ~result_mask;
  // It is extremely unlikely that a Varint is invalid so checking that
  // condition isn't on the critical path. Here we make sure that we don't do so
  // until result has been computed.
  info.masked_cont_bits = ValueBarrier(info.masked_cont_bits, result);
  if (ABSL_PREDICT_FALSE(info.masked_cont_bits == 0)) {
    return {nullptr, 0};
  }
  return {info.p, result};
}

static const char* VarintParseSlowArm(const char* p, uint32_t* out,
                                      uint64_t first8) {
  auto tmp = VarintParseSlowArm32(p, first8);
  *out = tmp.second;
  return tmp.first;
}

static const char* VarintParseSlowArm(const char* p, uint64_t* out,
                                      uint64_t first8) {
  auto tmp = VarintParseSlowArm64(p, first8);
  *out = tmp.second;
  return tmp.first;
}
#endif

// The caller must ensure that p points to at least 10 valid bytes.
template <typename T>
[[nodiscard]] const char* VarintParse(const char* p, T* out) {
  AssertBytesAreReadable(p, 10);
#if defined(__aarch64__) && defined(ABSL_IS_LITTLE_ENDIAN) && !defined(_MSC_VER)
  // This optimization is not supported in big endian mode
  uint64_t first8;
  std::memcpy(&first8, p, sizeof(first8));
  if (ABSL_PREDICT_TRUE((first8 & 0x80) == 0)) {
    *out = static_cast<uint8_t>(first8);
    return p + 1;
  }
  if (ABSL_PREDICT_TRUE((first8 & 0x8000) == 0)) {
    uint64_t chunk1;
    uint64_t chunk2;
    // Extracting the two chunks this way gives a speedup for this path.
    chunk1 = Ubfx7(first8, 0);
    chunk2 = Ubfx7(first8, 8);
    *out = chunk1 | (chunk2 << 7);
    return p + 2;
  }
  return VarintParseSlowArm(p, out, first8);
#else   // __aarch64__
  auto ptr = reinterpret_cast<const uint8_t*>(p);
  uint32_t res = ptr[0];
  if ((res & 0x80) == 0) {
    *out = res;
    return p + 1;
  }
  return VarintParseSlow(p, res, out);
#endif  // __aarch64__
}

// Used for tags, could read up to 5 bytes which must be available.
// Caller must ensure it's safe to call.

PROTOBUF_EXPORT
std::pair<const char*, uint32_t> ReadTagFallback(const char* p, uint32_t res);

// Same as ParseVarint but only accept 5 bytes at most.
inline const char* ReadTag(const char* p, uint32_t* out,
                           uint32_t /*max_tag*/ = 0) {
  uint32_t res = static_cast<uint8_t>(p[0]);
  if (res < 128) {
    *out = res;
    return p + 1;
  }
  uint32_t second = static_cast<uint8_t>(p[1]);
  res += (second - 1) << 7;
  if (second < 128) {
    *out = res;
    return p + 2;
  }
  auto tmp = ReadTagFallback(p, res);
  *out = tmp.second;
  return tmp.first;
}

// As above, but optimized to consume very few registers while still being fast,
// ReadTagInlined is useful for callers that don't mind the extra code but would
// like to avoid an extern function call causing spills into the stack.
//
// Two support routines for ReadTagInlined come first...
template <class T>
[[nodiscard]] PROTOBUF_ALWAYS_INLINE constexpr T RotateLeft(T x,
                                                            int s) noexcept {
  return static_cast<T>(x << (s & (std::numeric_limits<T>::digits - 1))) |
         static_cast<T>(x >> ((-s) & (std::numeric_limits<T>::digits - 1)));
}

[[nodiscard]] PROTOBUF_ALWAYS_INLINE uint64_t
RotRight7AndReplaceLowByte(uint64_t res, const char& byte) {
  // TODO: remove the inline assembly
#if defined(__x86_64__) && defined(__GNUC__)
  // This will only use one register for `res`.
  // `byte` comes as a reference to allow the compiler to generate code like:
  //
  //   rorq    $7, %rcx
  //   movb    1(%rax), %cl
  //
  // which avoids loading the incoming bytes into a separate register first.
  asm("ror $7,%0\n\t"
      "movb %1,%b0"
      : "+r"(res)
      : "m"(byte));
#else
  res = RotateLeft(res, -7);
  res = res & ~0xFF;
  res |= 0xFF & byte;
#endif
  return res;
}

PROTOBUF_ALWAYS_INLINE const char* ReadTagInlined(const char* ptr,
                                                  uint32_t* out) {
  uint64_t res = 0xFF & ptr[0];
  if (ABSL_PREDICT_FALSE(res >= 128)) {
    res = RotRight7AndReplaceLowByte(res, ptr[1]);
    if (ABSL_PREDICT_FALSE(res & 0x80)) {
      res = RotRight7AndReplaceLowByte(res, ptr[2]);
      if (ABSL_PREDICT_FALSE(res & 0x80)) {
        res = RotRight7AndReplaceLowByte(res, ptr[3]);
        if (ABSL_PREDICT_FALSE(res & 0x80)) {
          // Note: this wouldn't work if res were 32-bit,
          // because then replacing the low byte would overwrite
          // the bottom 4 bits of the result.
          res = RotRight7AndReplaceLowByte(res, ptr[4]);
          if (ABSL_PREDICT_FALSE(res & 0x80)) {
            // The proto format does not permit longer than 5-byte encodings for
            // tags.
            *out = 0;
            return nullptr;
          }
          *out = static_cast<uint32_t>(RotateLeft(res, 28));
#if defined(__GNUC__)
          // Note: this asm statement prevents the compiler from
          // trying to share the "return ptr + constant" among all
          // branches.
          asm("" : "+r"(ptr));
#endif
          return ptr + 5;
        }
        *out = static_cast<uint32_t>(RotateLeft(res, 21));
        return ptr + 4;
      }
      *out = static_cast<uint32_t>(RotateLeft(res, 14));
      return ptr + 3;
    }
    *out = static_cast<uint32_t>(RotateLeft(res, 7));
    return ptr + 2;
  }
  *out = static_cast<uint32_t>(res);
  return ptr + 1;
}

// Decode 2 consecutive bytes of a varint and returns the value, shifted left
// by 1. It simultaneous updates *ptr to *ptr + 1 or *ptr + 2 depending if the
// first byte's continuation bit is set.
// If bit 15 of return value is set (equivalent to the continuation bits of both
// bytes being set) the varint continues, otherwise the parse is done. On x86
// movsx eax, dil
// and edi, eax
// add eax, edi
// adc [rsi], 1
inline uint32_t DecodeTwoBytes(const char** ptr) {
  uint32_t value = UnalignedLoad<uint16_t>(*ptr);
  // Sign extend the low byte continuation bit
  uint32_t x = static_cast<int8_t>(value);
  value &= x;  // Mask out the high byte iff no continuation
  // This add is an amazing operation, it cancels the low byte continuation bit
  // from y transferring it to the carry. Simultaneously it also shifts the 7
  // LSB left by one tightly against high byte varint bits. Hence value now
  // contains the unpacked value shifted left by 1.
  value += x;
  // Use the carry to update the ptr appropriately.
  *ptr += value < x ? 2 : 1;
  return value;
}

// More efficient varint parsing for big varints
inline const char* ParseBigVarint(const char* p, uint64_t* out) {
  auto pnew = p;
  auto tmp = DecodeTwoBytes(&pnew);
  uint64_t res = tmp >> 1;
  if (ABSL_PREDICT_TRUE(static_cast<std::int16_t>(tmp) >= 0)) {
    *out = res;
    return pnew;
  }
  for (std::uint32_t i = 1; i < 5; i++) {
    pnew = p + 2 * i;
    tmp = DecodeTwoBytes(&pnew);
    res += (static_cast<std::uint64_t>(tmp) - 2) << (14 * i - 1);
    if (ABSL_PREDICT_TRUE(static_cast<std::int16_t>(tmp) >= 0)) {
      *out = res;
      return pnew;
    }
  }
  return nullptr;
}

PROTOBUF_EXPORT
std::pair<const char*, int32_t> ReadSizeFallback(const char* p, uint32_t first);

// Used for length prefixes. Could read up to 5 bytes, but no more than
// necessary for a single varint. The caller must ensure enough bytes are
// available. Additionally it makes sure the unsigned value fits in an int32_t,
// otherwise returns nullptr. Caller must ensure it is safe to call.
inline uint32_t ReadSize(const char** pp) {
  auto p = *pp;
  uint32_t res = static_cast<uint8_t>(p[0]);
  if (res < 128) {
    *pp = p + 1;
    return res;
  }
  auto x = ReadSizeFallback(p, res);
  *pp = x.first;
  return x.second;
}

// Some convenience functions to simplify the generated parse loop code.
// Returning the value and updating the buffer pointer allows for nicer
// function composition. We rely on the compiler to inline this.
// Also in debug compiles having local scoped variables tend to generated
// stack frames that scale as O(num fields).
inline uint64_t ReadVarint64(const char** p) {
  uint64_t tmp;
  *p = VarintParse(*p, &tmp);
  return tmp;
}

inline uint32_t ReadVarint32(const char** p) {
  uint32_t tmp;
  *p = VarintParse(*p, &tmp);
  return tmp;
}

inline int64_t ReadVarintZigZag64(const char** p) {
  uint64_t tmp;
  *p = VarintParse(*p, &tmp);
  return WireFormatLite::ZigZagDecode64(tmp);
}

inline int32_t ReadVarintZigZag32(const char** p) {
  uint64_t tmp;
  *p = VarintParse(*p, &tmp);
  return WireFormatLite::ZigZagDecode32(static_cast<uint32_t>(tmp));
}

template <typename Func>
[[nodiscard]] PROTOBUF_ALWAYS_INLINE const char*
ParseContext::ParseLengthDelimitedInlined(const char* ptr, const Func& func) {
  LimitToken old;
  ptr = ReadSizeAndPushLimitAndDepthInlined(ptr, &old);
  if (ptr == nullptr) return ptr;
  auto old_depth = depth_;
  PROTOBUF_ALWAYS_INLINE_CALL ptr = func(ptr);
  if (ptr != nullptr) ABSL_DCHECK_EQ(old_depth, depth_);
  depth_++;
  if (!PopLimit(std::move(old))) return nullptr;
  return ptr;
}

template <typename Func>
[[nodiscard]] PROTOBUF_ALWAYS_INLINE const char*
ParseContext::ParseGroupInlined(const char* ptr, uint32_t start_tag,
                                const Func& func) {
  if (--depth_ < 0) return nullptr;
  group_depth_++;
  auto old_depth = depth_;
  auto old_group_depth = group_depth_;
  PROTOBUF_ALWAYS_INLINE_CALL ptr = func(ptr);
  if (ptr != nullptr) {
    ABSL_DCHECK_EQ(old_depth, depth_);
    ABSL_DCHECK_EQ(old_group_depth, group_depth_);
  }
  group_depth_--;
  depth_++;
  if (ABSL_PREDICT_FALSE(!ConsumeEndGroup(start_tag))) return nullptr;
  return ptr;
}

inline const char* ParseContext::ReadSizeAndPushLimitAndDepthInlined(
    const char* ptr, LimitToken* old_limit) {
  int size = ReadSize(&ptr);
  if (ABSL_PREDICT_FALSE(!ptr) || depth_ <= 0) {
    return nullptr;
  }
  *old_limit = PushLimit(ptr, size);
  --depth_;
  return ptr;
}

template <typename Tag, typename T>
const char* EpsCopyInputStream::ReadRepeatedFixed(const char* ptr,
                                                  Tag expected_tag,
                                                  RepeatedField<T>* out) {
  do {
    out->Add(UnalignedLoad<T>(ptr));
    ptr += sizeof(T);
    if (ABSL_PREDICT_FALSE(ptr >= limit_end_)) return ptr;
  } while (UnalignedLoad<Tag>(ptr) == expected_tag && (ptr += sizeof(Tag)));
  return ptr;
}

// Add any of the following lines to debug which parse function is failing.

#define GOOGLE_PROTOBUF_ASSERT_RETURN(predicate, ret) \
  if (!(predicate)) {                                  \
    /*  ::raise(SIGINT);  */                           \
    /*  ABSL_LOG(ERROR) << "Parse failure";  */        \
    return ret;                                        \
  }

#define GOOGLE_PROTOBUF_PARSER_ASSERT(predicate) \
  GOOGLE_PROTOBUF_ASSERT_RETURN(predicate, nullptr)

template <typename T>
const char* EpsCopyInputStream::ReadPackedFixed(const char* ptr, int size,
                                                RepeatedField<T>* out) {
  GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
  int nbytes = BytesAvailable(ptr);
  while (size > nbytes) {
    int num = nbytes / sizeof(T);
    int old_entries = out->size();
    out->Reserve(old_entries + num);
    int block_size = num * sizeof(T);
    auto dst = out->AddNAlreadyReserved(num);
#ifdef ABSL_IS_LITTLE_ENDIAN
    std::memcpy(dst, ptr, block_size);
#else
    for (int i = 0; i < num; i++)
      dst[i] = UnalignedLoad<T>(ptr + i * sizeof(T));
#endif
    size -= block_size;
    if (limit_ <= kSlopBytes) return nullptr;
    ptr = Next();
    if (ptr == nullptr) return nullptr;
    ptr += kSlopBytes - (nbytes - block_size);
    nbytes = BytesAvailable(ptr);
  }
  int num = size / sizeof(T);
  int block_size = num * sizeof(T);
  if (num == 0) return size == block_size ? ptr : nullptr;
  int old_entries = out->size();
  out->Reserve(old_entries + num);
  auto dst = out->AddNAlreadyReserved(num);
#ifdef ABSL_IS_LITTLE_ENDIAN
  ABSL_CHECK(dst != nullptr) << out << "," << num;
  std::memcpy(dst, ptr, block_size);
#else
  for (int i = 0; i < num; i++) dst[i] = UnalignedLoad<T>(ptr + i * sizeof(T));
#endif
  ptr += block_size;
  if (size != block_size) return nullptr;
  return ptr;
}

template <typename Add>
const char* ReadPackedVarintArray(const char* ptr, const char* end, Add add) {
  while (ptr < end) {
    uint64_t varint;
    ptr = VarintParse(ptr, &varint);
    if (ptr == nullptr) return nullptr;
    add(varint);
  }
  return ptr;
}

template <typename Add, typename SizeCb>
const char* EpsCopyInputStream::ReadPackedVarint(const char* ptr, Add add,
                                                 SizeCb size_callback) {
  int size = ReadSize(&ptr);
  size_callback(size);

  GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
  int chunk_size = static_cast<int>(buffer_end_ - ptr);
  while (size > chunk_size) {
    ptr = ReadPackedVarintArray(ptr, buffer_end_, add);
    if (ptr == nullptr) return nullptr;
    int overrun = static_cast<int>(ptr - buffer_end_);
    ABSL_DCHECK(overrun >= 0 && overrun <= kSlopBytes);
    if (size - chunk_size <= kSlopBytes) {
      // The current buffer contains all the information needed, we don't need
      // to flip buffers. However we must parse from a buffer with enough space
      // so we are not prone to a buffer overflow.
      char buf[kSlopBytes + 10] = {};
      std::memcpy(buf, buffer_end_, kSlopBytes);
      ABSL_CHECK_LE(size - chunk_size, kSlopBytes);
      auto end = buf + (size - chunk_size);
      auto res = ReadPackedVarintArray(buf + overrun, end, add);
      if (res == nullptr || res != end) return nullptr;
      return buffer_end_ + (res - buf);
    }
    size -= overrun + chunk_size;
    ABSL_DCHECK_GT(size, 0);
    // We must flip buffers
    if (limit_ <= kSlopBytes) return nullptr;
    ptr = Next();
    if (ptr == nullptr) return nullptr;
    ptr += overrun;
    chunk_size = static_cast<int>(buffer_end_ - ptr);
  }
  auto end = ptr + size;
  ptr = ReadPackedVarintArray(ptr, end, add);
  return end == ptr ? ptr : nullptr;
}

// Helper for verification of utf8
PROTOBUF_EXPORT
bool VerifyUTF8(absl::string_view s, const char* field_name);

inline bool VerifyUTF8(const std::string* s, const char* field_name) {
  return VerifyUTF8(*s, field_name);
}

// All the string parsers with or without UTF checking and for all CTypes.
[[nodiscard]] PROTOBUF_EXPORT const char* InlineGreedyStringParser(
    std::string* s, const char* ptr, ParseContext* ctx);

[[nodiscard]] inline const char* InlineCordParser(::absl::Cord* cord,
                                                  const char* ptr,
                                                  ParseContext* ctx) {
  int size = ReadSize(&ptr);
  if (!ptr) return nullptr;
  return ctx->ReadCord(ptr, size, cord);
}


template <typename T>
[[nodiscard]] const char* FieldParser(uint64_t tag, T& field_parser,
                                      const char* ptr, ParseContext* ctx) {
  uint32_t number = tag >> 3;
  GOOGLE_PROTOBUF_PARSER_ASSERT(number != 0);
  using WireType = internal::WireFormatLite::WireType;
  switch (tag & 7) {
    case WireType::WIRETYPE_VARINT: {
      uint64_t value;
      ptr = VarintParse(ptr, &value);
      GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
      field_parser.AddVarint(number, value);
      break;
    }
    case WireType::WIRETYPE_FIXED64: {
      uint64_t value = UnalignedLoad<uint64_t>(ptr);
      ptr += 8;
      field_parser.AddFixed64(number, value);
      break;
    }
    case WireType::WIRETYPE_LENGTH_DELIMITED: {
      ptr = field_parser.ParseLengthDelimited(number, ptr, ctx);
      GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
      break;
    }
    case WireType::WIRETYPE_START_GROUP: {
      ptr = field_parser.ParseGroup(number, ptr, ctx);
      GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
      break;
    }
    case WireType::WIRETYPE_END_GROUP: {
      ABSL_LOG(FATAL) << "Can't happen";
      break;
    }
    case WireType::WIRETYPE_FIXED32: {
      uint32_t value = UnalignedLoad<uint32_t>(ptr);
      ptr += 4;
      field_parser.AddFixed32(number, value);
      break;
    }
    default:
      return nullptr;
  }
  return ptr;
}

template <typename T>
[[nodiscard]] const char* WireFormatParser(T& field_parser, const char* ptr,
                                           ParseContext* ctx) {
  while (!ctx->Done(&ptr)) {
    uint32_t tag;
    ptr = ReadTag(ptr, &tag);
    GOOGLE_PROTOBUF_PARSER_ASSERT(ptr != nullptr);
    if (tag == 0 || (tag & 7) == 4) {
      ctx->SetLastTag(tag);
      return ptr;
    }
    ptr = FieldParser(tag, field_parser, ptr, ctx);
    GOOGLE_PROTOBUF_PARSER_ASSERT(ptr != nullptr);
  }
  return ptr;
}

// The packed parsers parse repeated numeric primitives directly into  the
// corresponding field

// These are packed varints
[[nodiscard]] PROTOBUF_EXPORT const char* PackedInt32Parser(void* object,
                                                            const char* ptr,
                                                            ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedUInt32Parser(void* object,
                                                             const char* ptr,
                                                             ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedInt64Parser(void* object,
                                                            const char* ptr,
                                                            ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedUInt64Parser(void* object,
                                                             const char* ptr,
                                                             ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedSInt32Parser(void* object,
                                                             const char* ptr,
                                                             ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedSInt64Parser(void* object,
                                                             const char* ptr,
                                                             ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedEnumParser(void* object,
                                                           const char* ptr,
                                                           ParseContext* ctx);

template <typename T, typename Validator>
[[nodiscard]] const char* PackedEnumParserArg(void* object, const char* ptr,
                                              ParseContext* ctx,
                                              Validator validator,
                                              InternalMetadata* metadata,
                                              int field_num) {
  return ctx->ReadPackedVarint(
      ptr, [object, validator, metadata, field_num](int32_t val) {
        if (validator.IsValid(val)) {
          static_cast<RepeatedField<int>*>(object)->Add(val);
        } else {
          WriteVarint(field_num, val, metadata->mutable_unknown_fields<T>());
        }
      });
}

[[nodiscard]] PROTOBUF_EXPORT const char* PackedBoolParser(void* object,
                                                           const char* ptr,
                                                           ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedFixed32Parser(
    void* object, const char* ptr, ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedSFixed32Parser(
    void* object, const char* ptr, ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedFixed64Parser(
    void* object, const char* ptr, ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedSFixed64Parser(
    void* object, const char* ptr, ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedFloatParser(void* object,
                                                            const char* ptr,
                                                            ParseContext* ctx);
[[nodiscard]] PROTOBUF_EXPORT const char* PackedDoubleParser(void* object,
                                                             const char* ptr,
                                                             ParseContext* ctx);

// This is the only recursive parser.
[[nodiscard]] PROTOBUF_EXPORT const char* UnknownGroupLiteParse(
    std::string* unknown, const char* ptr, ParseContext* ctx);
// This is a helper to for the UnknownGroupLiteParse but is actually also
// useful in the generated code. It uses overload on std::string* vs
// UnknownFieldSet* to make the generated code isomorphic between full and lite.
[[nodiscard]] PROTOBUF_EXPORT const char* UnknownFieldParse(
    uint32_t tag, std::string* unknown, const char* ptr, ParseContext* ctx);

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_PARSE_CONTEXT_H__
