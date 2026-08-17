// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_IMPLICIT_WEAK_MESSAGE_H__
#define GOOGLE_PROTOBUF_IMPLICIT_WEAK_MESSAGE_H__

#include <cstddef>
#include <string>

#include "google/protobuf/arena.h"
#include "google/protobuf/generated_message_tctable_decl.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/repeated_field.h"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

// Must be included last.
#include "google/protobuf/port_def.inc"

// This file is logically internal-only and should only be used by protobuf
// generated code.

namespace google {
namespace protobuf {
namespace internal {

// An implementation of MessageLite that treats all data as unknown. This type
// acts as a placeholder for an implicit weak field in the case where the true
// message type does not get linked into the binary.
class PROTOBUF_EXPORT ImplicitWeakMessage final : public MessageLite {
 public:
  ImplicitWeakMessage() : ImplicitWeakMessage(nullptr) {}
  explicit constexpr ImplicitWeakMessage(ConstantInitialized);
  ImplicitWeakMessage(const ImplicitWeakMessage&) = delete;
  ImplicitWeakMessage& operator=(const ImplicitWeakMessage&) = delete;

  // Arena enabled constructors: for internal use only.
  ImplicitWeakMessage(internal::InternalVisibility, Arena* arena)
      : ImplicitWeakMessage(arena) {}

  // TODO: make this constructor private
  explicit ImplicitWeakMessage(Arena* arena)
      : MessageLite(arena, class_data_.base()),
        data_(Arena::Create<std::string>(arena)) {}

  ~ImplicitWeakMessage() PROTOBUF_FINAL { delete data_; }

  static const ImplicitWeakMessage& default_instance();

  const ClassData* GetClassData() const PROTOBUF_FINAL;

  void Clear() PROTOBUF_FINAL { data_->clear(); }

  size_t ByteSizeLong() const PROTOBUF_FINAL {
    size_t size = data_ == nullptr ? 0 : data_->size();
    cached_size_.Set(internal::ToCachedSize(size));
    return size;
  }

  uint8_t* _InternalSerialize(
      uint8_t* target, io::EpsCopyOutputStream* stream) const PROTOBUF_FINAL {
    if (data_ == nullptr) {
      return target;
    }
    return stream->WriteRaw(data_->data(), static_cast<int>(data_->size()),
                            target);
  }

  using InternalArenaConstructable_ = void;
  using DestructorSkippable_ = void;

  static PROTOBUF_CC const char* ParseImpl(ImplicitWeakMessage* msg,
                                           const char* ptr, ParseContext* ctx);

 private:
  static const TcParseTable<0> table_;
  static const ClassDataLite<1> class_data_;

  static void MergeImpl(MessageLite&, const MessageLite&);

  static void DestroyImpl(MessageLite& msg) {
    static_cast<ImplicitWeakMessage&>(msg).~ImplicitWeakMessage();
  }
  static size_t ByteSizeLongImpl(const MessageLite& msg) {
    return static_cast<const ImplicitWeakMessage&>(msg).ByteSizeLong();
  }

  static uint8_t* _InternalSerializeImpl(const MessageLite& msg,
                                         uint8_t* target,
                                         io::EpsCopyOutputStream* stream) {
    return static_cast<const ImplicitWeakMessage&>(msg)._InternalSerialize(
        target, stream);
  }

  // This std::string is allocated on the heap, but we use a raw pointer so that
  // the default instance can be constant-initialized. In the const methods, we
  // have to handle the possibility of data_ being null.
  std::string* data_;
  google::protobuf::internal::CachedSize cached_size_{};
};

struct ImplicitWeakMessageDefaultType;
extern ImplicitWeakMessageDefaultType implicit_weak_message_default_instance;

// A type handler for use with implicit weak repeated message fields.
template <typename ImplicitWeakType>
class ImplicitWeakTypeHandler {
 public:
  typedef MessageLite Type;
  static constexpr bool Moveable = false;

  static inline MessageLite* NewFromPrototype(const MessageLite* prototype,
                                              Arena* arena = nullptr) {
    return prototype->New(arena);
  }

  static inline void Delete(MessageLite* value, Arena* arena) {
    if (arena == nullptr) {
      delete value;
    }
  }
  static inline Arena* GetArena(MessageLite* value) {
    return value->GetArena();
  }
  static inline void Clear(MessageLite* value) { value->Clear(); }
  static void Merge(const MessageLite& from, MessageLite* to) {
    to->CheckTypeAndMergeFrom(from);
  }
};

}  // namespace internal

template <typename T>
struct WeakRepeatedPtrField {
  using InternalArenaConstructable_ = void;
  using DestructorSkippable_ = void;

  using TypeHandler = internal::ImplicitWeakTypeHandler<T>;

  constexpr WeakRepeatedPtrField() : weak() {}
  WeakRepeatedPtrField(const WeakRepeatedPtrField& rhs)
      : WeakRepeatedPtrField(nullptr, rhs) {}

  // Arena enabled constructors: for internal use only.
  WeakRepeatedPtrField(internal::InternalVisibility, Arena* arena)
      : WeakRepeatedPtrField(arena) {}
  WeakRepeatedPtrField(internal::InternalVisibility, Arena* arena,
                       const WeakRepeatedPtrField& rhs)
      : WeakRepeatedPtrField(arena, rhs) {}

  // TODO: make this constructor private
  explicit WeakRepeatedPtrField(Arena* arena) : weak(arena) {}

  ~WeakRepeatedPtrField() {
    if (weak.NeedsDestroy()) {
      weak.DestroyProtos();
    }
  }

  typedef internal::RepeatedPtrIterator<MessageLite> iterator;
  typedef internal::RepeatedPtrIterator<const MessageLite> const_iterator;
  typedef internal::RepeatedPtrOverPtrsIterator<MessageLite*, void*>
      pointer_iterator;
  typedef internal::RepeatedPtrOverPtrsIterator<const MessageLite* const,
                                                const void* const>
      const_pointer_iterator;

  bool empty() const { return base().empty(); }
  iterator begin() { return iterator(base().raw_data()); }
  const_iterator begin() const { return iterator(base().raw_data()); }
  const_iterator cbegin() const { return begin(); }
  iterator end() { return begin() + base().size(); }
  const_iterator end() const { return begin() + base().size(); }
  const_iterator cend() const { return end(); }
  pointer_iterator pointer_begin() {
    return pointer_iterator(base().raw_mutable_data());
  }
  const_pointer_iterator pointer_begin() const {
    return const_pointer_iterator(base().raw_data());
  }
  pointer_iterator pointer_end() {
    return pointer_iterator(base().raw_mutable_data() + base().size());
  }
  const_pointer_iterator pointer_end() const {
    return const_pointer_iterator(base().raw_data() + base().size());
  }

  T* Add() { return weak.Add(); }
  void Clear() { base().template Clear<TypeHandler>(); }
  void MergeFrom(const WeakRepeatedPtrField& other) {
    if (other.empty()) return;
    base().template MergeFrom<MessageLite>(other.base());
  }
  void InternalSwap(WeakRepeatedPtrField* PROTOBUF_RESTRICT other) {
    base().InternalSwap(&other->base());
  }

  const internal::RepeatedPtrFieldBase& base() const { return weak; }
  internal::RepeatedPtrFieldBase& base() { return weak; }
  // Union disables running the destructor. Which would create a strong link.
  // Instead we explicitly destroy the underlying base through the virtual
  // destructor.
  union {
    RepeatedPtrField<T> weak;
  };

  static constexpr size_t InternalGetArenaOffset(
      internal::InternalVisibility visibility) {
    return decltype(weak)::InternalGetArenaOffset(visibility);
  }

 private:
  WeakRepeatedPtrField(Arena* arena, const WeakRepeatedPtrField& rhs)
      : WeakRepeatedPtrField(arena) {
    MergeFrom(rhs);
  }
};

}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_IMPLICIT_WEAK_MESSAGE_H__
