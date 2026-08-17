/*
 * Copyright 2021 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_FLATBUFFER_BUILDER_H_
#define FLATBUFFERS_FLATBUFFER_BUILDER_H_

#include <algorithm>
#include <cstdint>
#include <functional>
#include <initializer_list>
#include <type_traits>

#include "flatbuffers/allocator.h"
#include "flatbuffers/array.h"
#include "flatbuffers/base.h"
#include "flatbuffers/buffer.h"
#include "flatbuffers/buffer_ref.h"
#include "flatbuffers/default_allocator.h"
#include "flatbuffers/detached_buffer.h"
#include "flatbuffers/stl_emulation.h"
#include "flatbuffers/string.h"
#include "flatbuffers/struct.h"
#include "flatbuffers/table.h"
#include "flatbuffers/vector.h"
#include "flatbuffers/vector_downward.h"
#include "flatbuffers/verifier.h"

namespace flatbuffers {

// Converts a Field ID to a virtual table offset.
inline voffset_t FieldIndexToOffset(voffset_t field_id) {
  // Should correspond to what EndTable() below builds up.
  const voffset_t fixed_fields =
      2 * sizeof(voffset_t);  // Vtable size and Object Size.
  size_t offset = fixed_fields + field_id * sizeof(voffset_t);
  FLATBUFFERS_ASSERT(offset < std::numeric_limits<voffset_t>::max());
  return static_cast<voffset_t>(offset);
}

template<typename T, typename Alloc = std::allocator<T>>
const T *data(const std::vector<T, Alloc> &v) {
  // Eventually the returned pointer gets passed down to memcpy, so
  // we need it to be non-null to avoid undefined behavior.
  static uint8_t t;
  return v.empty() ? reinterpret_cast<const T *>(&t) : &v.front();
}
template<typename T, typename Alloc = std::allocator<T>>
T *data(std::vector<T, Alloc> &v) {
  // Eventually the returned pointer gets passed down to memcpy, so
  // we need it to be non-null to avoid undefined behavior.
  static uint8_t t;
  return v.empty() ? reinterpret_cast<T *>(&t) : &v.front();
}

/// @addtogroup flatbuffers_cpp_api
/// @{
/// @class FlatBufferBuilder
/// @brief Helper class to hold data needed in creation of a FlatBuffer.
/// To serialize data, you typically call one of the `Create*()` functions in
/// the generated code, which in turn call a sequence of `StartTable`/
/// `PushElement`/`AddElement`/`EndTable`, or the builtin `CreateString`/
/// `CreateVector` functions. Do this is depth-first order to build up a tree to
/// the root. `Finish()` wraps up the buffer ready for transport.
template<bool Is64Aware = false> class FlatBufferBuilderImpl {
 public:
  // This switches the size type of the builder, based on if its 64-bit aware
  // (uoffset64_t) or not (uoffset_t).
  typedef
      typename std::conditional<Is64Aware, uoffset64_t, uoffset_t>::type SizeT;

  /// @brief Default constructor for FlatBufferBuilder.
  /// @param[in] initial_size The initial size of the buffer, in bytes. Defaults
  /// to `1024`.
  /// @param[in] allocator An `Allocator` to use. If null will use
  /// `DefaultAllocator`.
  /// @param[in] own_allocator Whether the builder/vector should own the
  /// allocator. Defaults to / `false`.
  /// @param[in] buffer_minalign Force the buffer to be aligned to the given
  /// minimum alignment upon reallocation. Only needed if you intend to store
  /// types with custom alignment AND you wish to read the buffer in-place
  /// directly after creation.
  explicit FlatBufferBuilderImpl(
      size_t initial_size = 1024, Allocator *allocator = nullptr,
      bool own_allocator = false,
      size_t buffer_minalign = AlignOf<largest_scalar_t>())
      : buf_(initial_size, allocator, own_allocator, buffer_minalign,
             static_cast<SizeT>(Is64Aware ? FLATBUFFERS_MAX_64_BUFFER_SIZE
                                          : FLATBUFFERS_MAX_BUFFER_SIZE)),
        num_field_loc(0),
        max_voffset_(0),
        length_of_64_bit_region_(0),
        nested(false),
        finished(false),
        minalign_(1),
        force_defaults_(false),
        dedup_vtables_(true),
        string_pool(nullptr) {
    EndianCheck();
  }

  /// @brief Move constructor for FlatBufferBuilder.
  FlatBufferBuilderImpl(FlatBufferBuilderImpl &&other) noexcept
      : buf_(1024, nullptr, false, AlignOf<largest_scalar_t>(),
             static_cast<SizeT>(Is64Aware ? FLATBUFFERS_MAX_64_BUFFER_SIZE
                                          : FLATBUFFERS_MAX_BUFFER_SIZE)),
        num_field_loc(0),
        max_voffset_(0),
        length_of_64_bit_region_(0),
        nested(false),
        finished(false),
        minalign_(1),
        force_defaults_(false),
        dedup_vtables_(true),
        string_pool(nullptr) {
    EndianCheck();
    // Default construct and swap idiom.
    // Lack of delegating constructors in vs2010 makes it more verbose than
    // needed.
    Swap(other);
  }

  /// @brief Move assignment operator for FlatBufferBuilder.
  FlatBufferBuilderImpl &operator=(FlatBufferBuilderImpl &&other) noexcept {
    // Move construct a temporary and swap idiom
    FlatBufferBuilderImpl temp(std::move(other));
    Swap(temp);
    return *this;
  }

  void Swap(FlatBufferBuilderImpl &other) {
    using std::swap;
    buf_.swap(other.buf_);
    swap(num_field_loc, other.num_field_loc);
    swap(max_voffset_, other.max_voffset_);
    swap(length_of_64_bit_region_, other.length_of_64_bit_region_);
    swap(nested, other.nested);
    swap(finished, other.finished);
    swap(minalign_, other.minalign_);
    swap(force_defaults_, other.force_defaults_);
    swap(dedup_vtables_, other.dedup_vtables_);
    swap(string_pool, other.string_pool);
  }

  ~FlatBufferBuilderImpl() {
    if (string_pool) delete string_pool;
  }

  void Reset() {
    Clear();       // clear builder state
    buf_.reset();  // deallocate buffer
  }

  /// @brief Reset all the state in this FlatBufferBuilder so it can be reused
  /// to construct another buffer.
  void Clear() {
    ClearOffsets();
    buf_.clear();
    nested = false;
    finished = false;
    minalign_ = 1;
    length_of_64_bit_region_ = 0;
    if (string_pool) string_pool->clear();
  }

  /// @brief The current size of the serialized buffer, counting from the end.
  /// @return Returns an `SizeT` with the current size of the buffer.
  SizeT GetSize() const { return buf_.size(); }

  /// @brief The current size of the serialized buffer relative to the end of
  /// the 32-bit region.
  /// @return Returns an `uoffset_t` with the current size of the buffer.
  template<bool is_64 = Is64Aware>
  // Only enable this method for the 64-bit builder, as only that builder is
  // concerned with the 32/64-bit boundary, and should be the one to bare any
  // run time costs.
  typename std::enable_if<is_64, uoffset_t>::type GetSizeRelative32BitRegion()
      const {
    //[32-bit region][64-bit region]
    //         [XXXXXXXXXXXXXXXXXXX] GetSize()
    //               [YYYYYYYYYYYYY] length_of_64_bit_region_
    //         [ZZZZ]                return size
    return static_cast<uoffset_t>(GetSize() - length_of_64_bit_region_);
  }

  template<bool is_64 = Is64Aware>
  // Only enable this method for the 32-bit builder.
  typename std::enable_if<!is_64, uoffset_t>::type GetSizeRelative32BitRegion()
      const {
    return static_cast<uoffset_t>(GetSize());
  }

  /// @brief Get the serialized buffer (after you call `Finish()`).
  /// @return Returns an `uint8_t` pointer to the FlatBuffer data inside the
  /// buffer.
  uint8_t *GetBufferPointer() const {
    Finished();
    return buf_.data();
  }

  /// @brief Get the serialized buffer (after you call `Finish()`) as a span.
  /// @return Returns a constructed flatbuffers::span that is a view over the
  /// FlatBuffer data inside the buffer.
  flatbuffers::span<uint8_t> GetBufferSpan() const {
    Finished();
    return flatbuffers::span<uint8_t>(buf_.data(), buf_.size());
  }

  /// @brief Get a pointer to an unfinished buffer.
  /// @return Returns a `uint8_t` pointer to the unfinished buffer.
  uint8_t *GetCurrentBufferPointer() const { return buf_.data(); }

  /// @brief Get the released DetachedBuffer.
  /// @return A `DetachedBuffer` that owns the buffer and its allocator.
  DetachedBuffer Release() {
    Finished();
    DetachedBuffer buffer = buf_.release();
    Clear();
    return buffer;
  }

  /// @brief Get the released pointer to the serialized buffer.
  /// @param size The size of the memory block containing
  /// the serialized `FlatBuffer`.
  /// @param offset The offset from the released pointer where the finished
  /// `FlatBuffer` starts.
  /// @return A raw pointer to the start of the memory block containing
  /// the serialized `FlatBuffer`.
  /// @remark If the allocator is owned, it gets deleted when the destructor is
  /// called.
  uint8_t *ReleaseRaw(size_t &size, size_t &offset) {
    Finished();
    uint8_t *raw = buf_.release_raw(size, offset);
    Clear();
    return raw;
  }

  /// @brief get the minimum alignment this buffer needs to be accessed
  /// properly. This is only known once all elements have been written (after
  /// you call Finish()). You can use this information if you need to embed
  /// a FlatBuffer in some other buffer, such that you can later read it
  /// without first having to copy it into its own buffer.
  size_t GetBufferMinAlignment() const {
    Finished();
    return minalign_;
  }

  /// @cond FLATBUFFERS_INTERNAL
  void Finished() const {
    // If you get this assert, you're attempting to get access a buffer
    // which hasn't been finished yet. Be sure to call
    // FlatBufferBuilder::Finish with your root table.
    // If you really need to access an unfinished buffer, call
    // GetCurrentBufferPointer instead.
    FLATBUFFERS_ASSERT(finished);
  }
  /// @endcond

  /// @brief In order to save space, fields that are set to their default value
  /// don't get serialized into the buffer.
  /// @param[in] fd When set to `true`, always serializes default values that
  /// are set. Optional fields which are not set explicitly, will still not be
  /// serialized.
  void ForceDefaults(bool fd) { force_defaults_ = fd; }

  /// @brief By default vtables are deduped in order to save space.
  /// @param[in] dedup When set to `true`, dedup vtables.
  void DedupVtables(bool dedup) { dedup_vtables_ = dedup; }

  /// @cond FLATBUFFERS_INTERNAL
  void Pad(size_t num_bytes) { buf_.fill(num_bytes); }

  void TrackMinAlign(size_t elem_size) {
    if (elem_size > minalign_) minalign_ = elem_size;
  }

  void Align(size_t elem_size) {
    TrackMinAlign(elem_size);
    buf_.fill(PaddingBytes(buf_.size(), elem_size));
  }

  void PushFlatBuffer(const uint8_t *bytes, size_t size) {
    PushBytes(bytes, size);
    finished = true;
  }

  void PushBytes(const uint8_t *bytes, size_t size) { buf_.push(bytes, size); }

  void PopBytes(size_t amount) { buf_.pop(amount); }

  template<typename T> void AssertScalarT() {
    // The code assumes power of 2 sizes and endian-swap-ability.
    static_assert(flatbuffers::is_scalar<T>::value, "T must be a scalar type");
  }

  // Write a single aligned scalar to the buffer
  template<typename T, typename ReturnT = uoffset_t>
  ReturnT PushElement(T element) {
    AssertScalarT<T>();
    Align(sizeof(T));
    buf_.push_small(EndianScalar(element));
    return CalculateOffset<ReturnT>();
  }

  template<typename T, template<typename> class OffsetT = Offset>
  uoffset_t PushElement(OffsetT<T> off) {
    // Special case for offsets: see ReferTo below.
    return PushElement(ReferTo(off.o));
  }

  // When writing fields, we track where they are, so we can create correct
  // vtables later.
  void TrackField(voffset_t field, uoffset_t off) {
    FieldLoc fl = { off, field };
    buf_.scratch_push_small(fl);
    num_field_loc++;
    if (field > max_voffset_) { max_voffset_ = field; }
  }

  // Like PushElement, but additionally tracks the field this represents.
  template<typename T> void AddElement(voffset_t field, T e, T def) {
    // We don't serialize values equal to the default.
    if (IsTheSameAs(e, def) && !force_defaults_) return;
    TrackField(field, PushElement(e));
  }

  template<typename T> void AddElement(voffset_t field, T e) {
    TrackField(field, PushElement(e));
  }

  template<typename T> void AddOffset(voffset_t field, Offset<T> off) {
    if (off.IsNull()) return;  // Don't store.
    AddElement(field, ReferTo(off.o), static_cast<uoffset_t>(0));
  }

  template<typename T> void AddOffset(voffset_t field, Offset64<T> off) {
    if (off.IsNull()) return;  // Don't store.
    AddElement(field, ReferTo(off.o), static_cast<uoffset64_t>(0));
  }

  template<typename T> void AddStruct(voffset_t field, const T *structptr) {
    if (!structptr) return;  // Default, don't store.
    Align(AlignOf<T>());
    buf_.push_small(*structptr);
    TrackField(field, CalculateOffset<uoffset_t>());
  }

  void AddStructOffset(voffset_t field, uoffset_t off) {
    TrackField(field, off);
  }

  // Offsets initially are relative to the end of the buffer (downwards).
  // This function converts them to be relative to the current location
  // in the buffer (when stored here), pointing upwards.
  uoffset_t ReferTo(uoffset_t off) {
    // Align to ensure GetSizeRelative32BitRegion() below is correct.
    Align(sizeof(uoffset_t));
    // 32-bit offsets are relative to the tail of the 32-bit region of the
    // buffer. For most cases (without 64-bit entities) this is equivalent to
    // size of the whole buffer (e.g. GetSize())
    return ReferTo(off, GetSizeRelative32BitRegion());
  }

  uoffset64_t ReferTo(uoffset64_t off) {
    // Align to ensure GetSize() below is correct.
    Align(sizeof(uoffset64_t));
    // 64-bit offsets are relative to tail of the whole buffer
    return ReferTo(off, GetSize());
  }

  template<typename T, typename T2> T ReferTo(const T off, const T2 size) {
    FLATBUFFERS_ASSERT(off && off <= size);
    return size - off + static_cast<T>(sizeof(T));
  }

  template<typename T> T ReferTo(const T off, const T size) {
    FLATBUFFERS_ASSERT(off && off <= size);
    return size - off + static_cast<T>(sizeof(T));
  }

  void NotNested() {
    // If you hit this, you're trying to construct a Table/Vector/String
    // during the construction of its parent table (between the MyTableBuilder
    // and table.Finish().
    // Move the creation of these sub-objects to above the MyTableBuilder to
    // not get this assert.
    // Ignoring this assert may appear to work in simple cases, but the reason
    // it is here is that storing objects in-line may cause vtable offsets
    // to not fit anymore. It also leads to vtable duplication.
    FLATBUFFERS_ASSERT(!nested);
    // If you hit this, fields were added outside the scope of a table.
    FLATBUFFERS_ASSERT(!num_field_loc);
  }

  // From generated code (or from the parser), we call StartTable/EndTable
  // with a sequence of AddElement calls in between.
  uoffset_t StartTable() {
    NotNested();
    nested = true;
    return GetSizeRelative32BitRegion();
  }

  // This finishes one serialized object by generating the vtable if it's a
  // table, comparing it against existing vtables, and writing the
  // resulting vtable offset.
  uoffset_t EndTable(uoffset_t start) {
    // If you get this assert, a corresponding StartTable wasn't called.
    FLATBUFFERS_ASSERT(nested);
    // Write the vtable offset, which is the start of any Table.
    // We fill its value later.
    // This is relative to the end of the 32-bit region.
    const uoffset_t vtable_offset_loc =
        static_cast<uoffset_t>(PushElement<soffset_t>(0));
    // Write a vtable, which consists entirely of voffset_t elements.
    // It starts with the number of offsets, followed by a type id, followed
    // by the offsets themselves. In reverse:
    // Include space for the last offset and ensure empty tables have a
    // minimum size.
    max_voffset_ =
        (std::max)(static_cast<voffset_t>(max_voffset_ + sizeof(voffset_t)),
                   FieldIndexToOffset(0));
    buf_.fill_big(max_voffset_);
    const uoffset_t table_object_size = vtable_offset_loc - start;
    // Vtable use 16bit offsets.
    FLATBUFFERS_ASSERT(table_object_size < 0x10000);
    WriteScalar<voffset_t>(buf_.data() + sizeof(voffset_t),
                           static_cast<voffset_t>(table_object_size));
    WriteScalar<voffset_t>(buf_.data(), max_voffset_);
    // Write the offsets into the table
    for (auto it = buf_.scratch_end() - num_field_loc * sizeof(FieldLoc);
         it < buf_.scratch_end(); it += sizeof(FieldLoc)) {
      auto field_location = reinterpret_cast<FieldLoc *>(it);
      const voffset_t pos =
          static_cast<voffset_t>(vtable_offset_loc - field_location->off);
      // If this asserts, it means you've set a field twice.
      FLATBUFFERS_ASSERT(
          !ReadScalar<voffset_t>(buf_.data() + field_location->id));
      WriteScalar<voffset_t>(buf_.data() + field_location->id, pos);
    }
    ClearOffsets();
    auto vt1 = reinterpret_cast<voffset_t *>(buf_.data());
    auto vt1_size = ReadScalar<voffset_t>(vt1);
    auto vt_use = GetSizeRelative32BitRegion();
    // See if we already have generated a vtable with this exact same
    // layout before. If so, make it point to the old one, remove this one.
    if (dedup_vtables_) {
      for (auto it = buf_.scratch_data(); it < buf_.scratch_end();
           it += sizeof(uoffset_t)) {
        auto vt_offset_ptr = reinterpret_cast<uoffset_t *>(it);
        auto vt2 = reinterpret_cast<voffset_t *>(buf_.data_at(*vt_offset_ptr));
        auto vt2_size = ReadScalar<voffset_t>(vt2);
        if (vt1_size != vt2_size || 0 != memcmp(vt2, vt1, vt1_size)) continue;
        vt_use = *vt_offset_ptr;
        buf_.pop(GetSizeRelative32BitRegion() - vtable_offset_loc);
        break;
      }
    }
    // If this is a new vtable, remember it.
    if (vt_use == GetSizeRelative32BitRegion()) {
      buf_.scratch_push_small(vt_use);
    }
    // Fill the vtable offset we created above.
    // The offset points from the beginning of the object to where the vtable is
    // stored.
    // Offsets default direction is downward in memory for future format
    // flexibility (storing all vtables at the start of the file).
    WriteScalar(buf_.data_at(vtable_offset_loc + length_of_64_bit_region_),
                static_cast<soffset_t>(vt_use) -
                    static_cast<soffset_t>(vtable_offset_loc));
    nested = false;
    return vtable_offset_loc;
  }

  FLATBUFFERS_ATTRIBUTE([[deprecated("call the version above instead")]])
  uoffset_t EndTable(uoffset_t start, voffset_t /*numfields*/) {
    return EndTable(start);
  }

  // This checks a required field has been set in a given table that has
  // just been constructed.
  template<typename T> void Required(Offset<T> table, voffset_t field) {
    auto table_ptr = reinterpret_cast<const Table *>(buf_.data_at(table.o));
    bool ok = table_ptr->GetOptionalFieldOffset(field) != 0;
    // If this fails, the caller will show what field needs to be set.
    FLATBUFFERS_ASSERT(ok);
    (void)ok;
  }

  uoffset_t StartStruct(size_t alignment) {
    Align(alignment);
    return GetSizeRelative32BitRegion();
  }

  uoffset_t EndStruct() { return GetSizeRelative32BitRegion(); }

  void ClearOffsets() {
    buf_.scratch_pop(num_field_loc * sizeof(FieldLoc));
    num_field_loc = 0;
    max_voffset_ = 0;
  }

  // Aligns such that when "len" bytes are written, an object can be written
  // after it (forward in the buffer) with "alignment" without padding.
  void PreAlign(size_t len, size_t alignment) {
    if (len == 0) return;
    TrackMinAlign(alignment);
    buf_.fill(PaddingBytes(GetSize() + len, alignment));
  }

  // Aligns such than when "len" bytes are written, an object of type `AlignT`
  // can be written after it (forward in the buffer) without padding.
  template<typename AlignT> void PreAlign(size_t len) {
    AssertScalarT<AlignT>();
    PreAlign(len, AlignOf<AlignT>());
  }
  /// @endcond

  /// @brief Store a string in the buffer, which can contain any binary data.
  /// @param[in] str A const char pointer to the data to be stored as a string.
  /// @param[in] len The number of bytes that should be stored from `str`.
  /// @return Returns the offset in the buffer where the string starts.
  template<template<typename> class OffsetT = Offset>
  OffsetT<String> CreateString(const char *str, size_t len) {
    CreateStringImpl(str, len);
    return OffsetT<String>(
        CalculateOffset<typename OffsetT<String>::offset_type>());
  }

  /// @brief Store a string in the buffer, which is null-terminated.
  /// @param[in] str A const char pointer to a C-string to add to the buffer.
  /// @return Returns the offset in the buffer where the string starts.
  template<template<typename> class OffsetT = Offset>
  OffsetT<String> CreateString(const char *str) {
    return CreateString<OffsetT>(str, strlen(str));
  }

  /// @brief Store a string in the buffer, which is null-terminated.
  /// @param[in] str A char pointer to a C-string to add to the buffer.
  /// @return Returns the offset in the buffer where the string starts.
  template<template<typename> class OffsetT = Offset>
  OffsetT<String> CreateString(char *str) {
    return CreateString<OffsetT>(str, strlen(str));
  }

  /// @brief Store a string in the buffer, which can contain any binary data.
  /// @param[in] str A const reference to a std::string to store in the buffer.
  /// @return Returns the offset in the buffer where the string starts.
  template<template<typename> class OffsetT = Offset>
  OffsetT<String> CreateString(const std::string &str) {
    return CreateString<OffsetT>(str.c_str(), str.length());
  }

  // clang-format off
  #ifdef FLATBUFFERS_HAS_STRING_VIEW
  /// @brief Store a string in the buffer, which can contain any binary data.
  /// @param[in] str A const string_view to copy in to the buffer.
  /// @return Returns the offset in the buffer where the string starts.
  template<template <typename> class OffsetT = Offset>
  OffsetT<String>CreateString(flatbuffers::string_view str) {
    return CreateString<OffsetT>(str.data(), str.size());
  }
  #endif // FLATBUFFERS_HAS_STRING_VIEW
  // clang-format on

  /// @brief Store a string in the buffer, which can contain any binary data.
  /// @param[in] str A const pointer to a `String` struct to add to the buffer.
  /// @return Returns the offset in the buffer where the string starts
  template<template<typename> class OffsetT = Offset>
  OffsetT<String> CreateString(const String *str) {
    return str ? CreateString<OffsetT>(str->c_str(), str->size()) : 0;
  }

  /// @brief Store a string in the buffer, which can contain any binary data.
  /// @param[in] str A const reference to a std::string like type with support
  /// of T::data() and T::length() to store in the buffer.
  /// @return Returns the offset in the buffer where the string starts.
  template<template<typename> class OffsetT = Offset,
           // No need to explicitly declare the T type, let the compiler deduce
           // it.
           int &...ExplicitArgumentBarrier, typename T>
  OffsetT<String> CreateString(const T &str) {
    return CreateString<OffsetT>(str.data(), str.length());
  }

  /// @brief Store a string in the buffer, which can contain any binary data.
  /// If a string with this exact contents has already been serialized before,
  /// instead simply returns the offset of the existing string. This uses a map
  /// stored on the heap, but only stores the numerical offsets.
  /// @param[in] str A const char pointer to the data to be stored as a string.
  /// @param[in] len The number of bytes that should be stored from `str`.
  /// @return Returns the offset in the buffer where the string starts.
  Offset<String> CreateSharedString(const char *str, size_t len) {
    FLATBUFFERS_ASSERT(FLATBUFFERS_GENERAL_HEAP_ALLOC_OK);
    if (!string_pool) {
      string_pool = new StringOffsetMap(StringOffsetCompare(buf_));
    }

    const size_t size_before_string = buf_.size();
    // Must first serialize the string, since the set is all offsets into
    // buffer.
    const Offset<String> off = CreateString<Offset>(str, len);
    auto it = string_pool->find(off);
    // If it exists we reuse existing serialized data!
    if (it != string_pool->end()) {
      // We can remove the string we serialized.
      buf_.pop(buf_.size() - size_before_string);
      return *it;
    }
    // Record this string for future use.
    string_pool->insert(off);
    return off;
  }

#ifdef FLATBUFFERS_HAS_STRING_VIEW
  /// @brief Store a string in the buffer, which can contain any binary data.
  /// If a string with this exact contents has already been serialized before,
  /// instead simply returns the offset of the existing string. This uses a map
  /// stored on the heap, but only stores the numerical offsets.
  /// @param[in] str A const std::string_view to store in the buffer.
  /// @return Returns the offset in the buffer where the string starts
  Offset<String> CreateSharedString(const flatbuffers::string_view str) {
    return CreateSharedString(str.data(), str.size());
  }
#else
  /// @brief Store a string in the buffer, which null-terminated.
  /// If a string with this exact contents has already been serialized before,
  /// instead simply returns the offset of the existing string. This uses a map
  /// stored on the heap, but only stores the numerical offsets.
  /// @param[in] str A const char pointer to a C-string to add to the buffer.
  /// @return Returns the offset in the buffer where the string starts.
  Offset<String> CreateSharedString(const char *str) {
    return CreateSharedString(str, strlen(str));
  }

  /// @brief Store a string in the buffer, which can contain any binary data.
  /// If a string with this exact contents has already been serialized before,
  /// instead simply returns the offset of the existing string. This uses a map
  /// stored on the heap, but only stores the numerical offsets.
  /// @param[in] str A const reference to a std::string to store in the buffer.
  /// @return Returns the offset in the buffer where the string starts.
  Offset<String> CreateSharedString(const std::string &str) {
    return CreateSharedString(str.c_str(), str.length());
  }
#endif

  /// @brief Store a string in the buffer, which can contain any binary data.
  /// If a string with this exact contents has already been serialized before,
  /// instead simply returns the offset of the existing string. This uses a map
  /// stored on the heap, but only stores the numerical offsets.
  /// @param[in] str A const pointer to a `String` struct to add to the buffer.
  /// @return Returns the offset in the buffer where the string starts
  Offset<String> CreateSharedString(const String *str) {
    return str ? CreateSharedString(str->c_str(), str->size()) : 0;
  }

  /// @cond FLATBUFFERS_INTERNAL
  template<typename LenT = uoffset_t, typename ReturnT = uoffset_t>
  ReturnT EndVector(size_t len) {
    FLATBUFFERS_ASSERT(nested);  // Hit if no corresponding StartVector.
    nested = false;
    return PushElement<LenT, ReturnT>(static_cast<LenT>(len));
  }

  template<template<typename> class OffsetT = Offset, typename LenT = uint32_t>
  void StartVector(size_t len, size_t elemsize, size_t alignment) {
    NotNested();
    nested = true;
    // Align to the Length type of the vector (either 32-bit or 64-bit), so
    // that the length of the buffer can be added without padding.
    PreAlign<LenT>(len * elemsize);
    PreAlign(len * elemsize, alignment);  // Just in case elemsize > uoffset_t.
  }

  template<typename T, template<typename> class OffsetT = Offset,
           typename LenT = uint32_t>
  void StartVector(size_t len) {
    return StartVector<OffsetT, LenT>(len, sizeof(T), AlignOf<T>());
  }

  // Call this right before StartVector/CreateVector if you want to force the
  // alignment to be something different than what the element size would
  // normally dictate.
  // This is useful when storing a nested_flatbuffer in a vector of bytes,
  // or when storing SIMD floats, etc.
  void ForceVectorAlignment(const size_t len, const size_t elemsize,
                            const size_t alignment) {
    if (len == 0) return;
    FLATBUFFERS_ASSERT(VerifyAlignmentRequirements(alignment));
    PreAlign(len * elemsize, alignment);
  }

  template<bool is_64 = Is64Aware>
  typename std::enable_if<is_64, void>::type ForceVectorAlignment64(
      const size_t len, const size_t elemsize, const size_t alignment) {
    // If you hit this assertion, you are trying to force alignment on a
    // vector with offset64 after serializing a 32-bit offset.
    FLATBUFFERS_ASSERT(GetSize() == length_of_64_bit_region_);

    // Call through.
    ForceVectorAlignment(len, elemsize, alignment);

    // Update the 64 bit region.
    length_of_64_bit_region_ = GetSize();
  }

  // Similar to ForceVectorAlignment but for String fields.
  void ForceStringAlignment(size_t len, size_t alignment) {
    if (len == 0) return;
    FLATBUFFERS_ASSERT(VerifyAlignmentRequirements(alignment));
    PreAlign((len + 1) * sizeof(char), alignment);
  }

  /// @endcond

  /// @brief Serialize an array into a FlatBuffer `vector`.
  /// @tparam T The data type of the array elements.
  /// @tparam OffsetT the type of offset to return
  /// @tparam VectorT the type of vector to cast to.
  /// @param[in] v A pointer to the array of type `T` to serialize into the
  /// buffer as a `vector`.
  /// @param[in] len The number of elements to serialize.
  /// @return Returns a typed `TOffset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, template<typename...> class OffsetT = Offset,
           template<typename...> class VectorT = Vector>
  OffsetT<VectorT<T>> CreateVector(const T *v, size_t len) {
    // The type of the length field in the vector.
    typedef typename VectorT<T>::size_type LenT;
    typedef typename OffsetT<VectorT<T>>::offset_type offset_type;
    // If this assert hits, you're specifying a template argument that is
    // causing the wrong overload to be selected, remove it.
    AssertScalarT<T>();
    StartVector<T, OffsetT, LenT>(len);
    if (len > 0) {
      // clang-format off
      #if FLATBUFFERS_LITTLEENDIAN
        PushBytes(reinterpret_cast<const uint8_t *>(v), len * sizeof(T));
      #else
        if (sizeof(T) == 1) {
          PushBytes(reinterpret_cast<const uint8_t *>(v), len);
        } else {
          for (auto i = len; i > 0; ) {
            PushElement(v[--i]);
          }
        }
      #endif
      // clang-format on
    }
    return OffsetT<VectorT<T>>(EndVector<LenT, offset_type>(len));
  }

  /// @brief Serialize an array like object into a FlatBuffer `vector`.
  /// @tparam T The data type of the array elements.
  /// @tparam C The type of the array.
  /// @param[in] array A reference to an array like object of type `T` to
  /// serialize into the buffer as a `vector`.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, class C> Offset<Vector<T>> CreateVector(const C &array) {
    return CreateVector(array.data(), array.size());
  }

  /// @brief Serialize an initializer list into a FlatBuffer `vector`.
  /// @tparam T The data type of the initializer list elements.
  /// @param[in] v The value of the initializer list.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T>
  Offset<Vector<T>> CreateVector(std::initializer_list<T> v) {
    return CreateVector(v.begin(), v.size());
  }

  template<typename T>
  Offset<Vector<Offset<T>>> CreateVector(const Offset<T> *v, size_t len) {
    StartVector<Offset<T>>(len);
    for (auto i = len; i > 0;) { PushElement(v[--i]); }
    return Offset<Vector<Offset<T>>>(EndVector(len));
  }

  /// @brief Serialize a `std::vector` into a FlatBuffer `vector`.
  /// @tparam T The data type of the `std::vector` elements.
  /// @param v A const reference to the `std::vector` to serialize into the
  /// buffer as a `vector`.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename Alloc = std::allocator<T>>
  Offset<Vector<T>> CreateVector(const std::vector<T, Alloc> &v) {
    return CreateVector(data(v), v.size());
  }

  template<template<typename...> class VectorT = Vector64,
           int &...ExplicitArgumentBarrier, typename T>
  Offset64<VectorT<T>> CreateVector64(const std::vector<T> &v) {
    return CreateVector<T, Offset64, VectorT>(data(v), v.size());
  }

  // vector<bool> may be implemented using a bit-set, so we can't access it as
  // an array. Instead, read elements manually.
  // Background: https://isocpp.org/blog/2012/11/on-vectorbool
  Offset<Vector<uint8_t>> CreateVector(const std::vector<bool> &v) {
    StartVector<uint8_t>(v.size());
    for (auto i = v.size(); i > 0;) {
      PushElement(static_cast<uint8_t>(v[--i]));
    }
    return Offset<Vector<uint8_t>>(EndVector(v.size()));
  }

  /// @brief Serialize values returned by a function into a FlatBuffer `vector`.
  /// This is a convenience function that takes care of iteration for you.
  /// @tparam T The data type of the `std::vector` elements.
  /// @param f A function that takes the current iteration 0..vector_size-1 and
  /// returns any type that you can construct a FlatBuffers vector out of.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T>
  Offset<Vector<T>> CreateVector(size_t vector_size,
                                 const std::function<T(size_t i)> &f) {
    FLATBUFFERS_ASSERT(FLATBUFFERS_GENERAL_HEAP_ALLOC_OK);
    std::vector<T> elems(vector_size);
    for (size_t i = 0; i < vector_size; i++) elems[i] = f(i);
    return CreateVector(elems);
  }

  /// @brief Serialize values returned by a function into a FlatBuffer `vector`.
  /// This is a convenience function that takes care of iteration for you. This
  /// uses a vector stored on the heap to store the intermediate results of the
  /// iteration.
  /// @tparam T The data type of the `std::vector` elements.
  /// @param f A function that takes the current iteration 0..vector_size-1,
  /// and the state parameter returning any type that you can construct a
  /// FlatBuffers vector out of.
  /// @param state State passed to f.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename F, typename S>
  Offset<Vector<T>> CreateVector(size_t vector_size, F f, S *state) {
    FLATBUFFERS_ASSERT(FLATBUFFERS_GENERAL_HEAP_ALLOC_OK);
    std::vector<T> elems(vector_size);
    for (size_t i = 0; i < vector_size; i++) elems[i] = f(i, state);
    return CreateVector(elems);
  }

  /// @brief Serialize a `std::vector<StringType>` into a FlatBuffer `vector`.
  /// whereas StringType is any type that is accepted by the CreateString()
  /// overloads.
  /// This is a convenience function for a common case.
  /// @param v A const reference to the `std::vector` to serialize into the
  /// buffer as a `vector`.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename StringType = std::string,
           typename Alloc = std::allocator<StringType>>
  Offset<Vector<Offset<String>>> CreateVectorOfStrings(
      const std::vector<StringType, Alloc> &v) {
    return CreateVectorOfStrings(v.cbegin(), v.cend());
  }

  /// @brief Serialize a collection of Strings into a FlatBuffer `vector`.
  /// This is a convenience function for a common case.
  /// @param begin The beginning iterator of the collection
  /// @param end The ending iterator of the collection
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<class It>
  Offset<Vector<Offset<String>>> CreateVectorOfStrings(It begin, It end) {
    auto distance = std::distance(begin, end);
    FLATBUFFERS_ASSERT(distance >= 0);
    auto size = static_cast<size_t>(distance);
    auto scratch_buffer_usage = size * sizeof(Offset<String>);
    // If there is not enough space to store the offsets, there definitely won't
    // be enough space to store all the strings. So ensuring space for the
    // scratch region is OK, for if it fails, it would have failed later.
    buf_.ensure_space(scratch_buffer_usage);
    for (auto it = begin; it != end; ++it) {
      buf_.scratch_push_small(CreateString(*it));
    }
    StartVector<Offset<String>>(size);
    for (size_t i = 1; i <= size; i++) {
      // Note we re-evaluate the buf location each iteration to account for any
      // underlying buffer resizing that may occur.
      PushElement(*reinterpret_cast<Offset<String> *>(
          buf_.scratch_end() - i * sizeof(Offset<String>)));
    }
    buf_.scratch_pop(scratch_buffer_usage);
    return Offset<Vector<Offset<String>>>(EndVector(size));
  }

  /// @brief Serialize an array of structs into a FlatBuffer `vector`.
  /// @tparam T The data type of the struct array elements.
  /// @param[in] v A pointer to the array of type `T` to serialize into the
  /// buffer as a `vector`.
  /// @param[in] len The number of elements to serialize.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, template<typename...> class OffsetT = Offset,
           template<typename...> class VectorT = Vector>
  OffsetT<VectorT<const T *>> CreateVectorOfStructs(const T *v, size_t len) {
    // The type of the length field in the vector.
    typedef typename VectorT<T>::size_type LenT;
    typedef typename OffsetT<VectorT<const T *>>::offset_type offset_type;

    StartVector<OffsetT, LenT>(len, sizeof(T), AlignOf<T>());
    if (len > 0) {
      PushBytes(reinterpret_cast<const uint8_t *>(v), sizeof(T) * len);
    }
    return OffsetT<VectorT<const T *>>(EndVector<LenT, offset_type>(len));
  }

  /// @brief Serialize an array of structs into a FlatBuffer `vector`.
  /// @tparam T The data type of the struct array elements.
  /// @param[in] filler A function that takes the current iteration
  /// 0..vector_size-1 and a pointer to the struct that must be filled.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  /// This is mostly useful when flatbuffers are generated with mutation
  /// accessors.
  template<typename T>
  Offset<Vector<const T *>> CreateVectorOfStructs(
      size_t vector_size, const std::function<void(size_t i, T *)> &filler) {
    T *structs = StartVectorOfStructs<T>(vector_size);
    for (size_t i = 0; i < vector_size; i++) {
      filler(i, structs);
      structs++;
    }
    return EndVectorOfStructs<T>(vector_size);
  }

  /// @brief Serialize an array of structs into a FlatBuffer `vector`.
  /// @tparam T The data type of the struct array elements.
  /// @param[in] f A function that takes the current iteration 0..vector_size-1,
  /// a pointer to the struct that must be filled and the state argument.
  /// @param[in] state Arbitrary state to pass to f.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  /// This is mostly useful when flatbuffers are generated with mutation
  /// accessors.
  template<typename T, typename F, typename S>
  Offset<Vector<const T *>> CreateVectorOfStructs(size_t vector_size, F f,
                                                  S *state) {
    T *structs = StartVectorOfStructs<T>(vector_size);
    for (size_t i = 0; i < vector_size; i++) {
      f(i, structs, state);
      structs++;
    }
    return EndVectorOfStructs<T>(vector_size);
  }

  /// @brief Serialize a `std::vector` of structs into a FlatBuffer `vector`.
  /// @tparam T The data type of the `std::vector` struct elements.
  /// @param[in] v A const reference to the `std::vector` of structs to
  /// serialize into the buffer as a `vector`.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, template<typename...> class OffsetT = Offset,
           template<typename...> class VectorT = Vector,
           typename Alloc = std::allocator<T>>
  OffsetT<VectorT<const T *>> CreateVectorOfStructs(
      const std::vector<T, Alloc> &v) {
    return CreateVectorOfStructs<T, OffsetT, VectorT>(data(v), v.size());
  }

  template<template<typename...> class VectorT = Vector64, int &..., typename T>
  Offset64<VectorT<const T *>> CreateVectorOfStructs64(
      const std::vector<T> &v) {
    return CreateVectorOfStructs<T, Offset64, VectorT>(data(v), v.size());
  }

  /// @brief Serialize an array of native structs into a FlatBuffer `vector`.
  /// @tparam T The data type of the struct array elements.
  /// @tparam S The data type of the native struct array elements.
  /// @param[in] v A pointer to the array of type `S` to serialize into the
  /// buffer as a `vector`.
  /// @param[in] len The number of elements to serialize.
  /// @param[in] pack_func Pointer to a function to convert the native struct
  /// to the FlatBuffer struct.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename S>
  Offset<Vector<const T *>> CreateVectorOfNativeStructs(
      const S *v, size_t len, T (*const pack_func)(const S &)) {
    FLATBUFFERS_ASSERT(pack_func);
    auto structs = StartVectorOfStructs<T>(len);
    for (size_t i = 0; i < len; i++) { structs[i] = pack_func(v[i]); }
    return EndVectorOfStructs<T>(len);
  }

  /// @brief Serialize an array of native structs into a FlatBuffer `vector`.
  /// @tparam T The data type of the struct array elements.
  /// @tparam S The data type of the native struct array elements.
  /// @param[in] v A pointer to the array of type `S` to serialize into the
  /// buffer as a `vector`.
  /// @param[in] len The number of elements to serialize.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename S>
  Offset<Vector<const T *>> CreateVectorOfNativeStructs(const S *v,
                                                        size_t len) {
    extern T Pack(const S &);
    return CreateVectorOfNativeStructs(v, len, Pack);
  }

  /// @brief Serialize a `std::vector` of native structs into a FlatBuffer
  /// `vector`.
  /// @tparam T The data type of the `std::vector` struct elements.
  /// @tparam S The data type of the `std::vector` native struct elements.
  /// @param[in] v A const reference to the `std::vector` of structs to
  /// serialize into the buffer as a `vector`.
  /// @param[in] pack_func Pointer to a function to convert the native struct
  /// to the FlatBuffer struct.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename S, typename Alloc = std::allocator<T>>
  Offset<Vector<const T *>> CreateVectorOfNativeStructs(
      const std::vector<S, Alloc> &v, T (*const pack_func)(const S &)) {
    return CreateVectorOfNativeStructs<T, S>(data(v), v.size(), pack_func);
  }

  /// @brief Serialize a `std::vector` of native structs into a FlatBuffer
  /// `vector`.
  /// @tparam T The data type of the `std::vector` struct elements.
  /// @tparam S The data type of the `std::vector` native struct elements.
  /// @param[in] v A const reference to the `std::vector` of structs to
  /// serialize into the buffer as a `vector`.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename S, typename Alloc = std::allocator<S>>
  Offset<Vector<const T *>> CreateVectorOfNativeStructs(
      const std::vector<S, Alloc> &v) {
    return CreateVectorOfNativeStructs<T, S>(data(v), v.size());
  }

  /// @cond FLATBUFFERS_INTERNAL
  template<typename T> struct StructKeyComparator {
    bool operator()(const T &a, const T &b) const {
      return a.KeyCompareLessThan(&b);
    }
  };
  /// @endcond

  /// @brief Serialize a `std::vector` of structs into a FlatBuffer `vector`
  /// in sorted order.
  /// @tparam T The data type of the `std::vector` struct elements.
  /// @param[in] v A const reference to the `std::vector` of structs to
  /// serialize into the buffer as a `vector`.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename Alloc = std::allocator<T>>
  Offset<Vector<const T *>> CreateVectorOfSortedStructs(
      std::vector<T, Alloc> *v) {
    return CreateVectorOfSortedStructs(data(*v), v->size());
  }

  /// @brief Serialize a `std::vector` of native structs into a FlatBuffer
  /// `vector` in sorted order.
  /// @tparam T The data type of the `std::vector` struct elements.
  /// @tparam S The data type of the `std::vector` native struct elements.
  /// @param[in] v A const reference to the `std::vector` of structs to
  /// serialize into the buffer as a `vector`.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename S, typename Alloc = std::allocator<T>>
  Offset<Vector<const T *>> CreateVectorOfSortedNativeStructs(
      std::vector<S, Alloc> *v) {
    return CreateVectorOfSortedNativeStructs<T, S>(data(*v), v->size());
  }

  /// @brief Serialize an array of structs into a FlatBuffer `vector` in sorted
  /// order.
  /// @tparam T The data type of the struct array elements.
  /// @param[in] v A pointer to the array of type `T` to serialize into the
  /// buffer as a `vector`.
  /// @param[in] len The number of elements to serialize.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T>
  Offset<Vector<const T *>> CreateVectorOfSortedStructs(T *v, size_t len) {
    std::stable_sort(v, v + len, StructKeyComparator<T>());
    return CreateVectorOfStructs(v, len);
  }

  /// @brief Serialize an array of native structs into a FlatBuffer `vector` in
  /// sorted order.
  /// @tparam T The data type of the struct array elements.
  /// @tparam S The data type of the native struct array elements.
  /// @param[in] v A pointer to the array of type `S` to serialize into the
  /// buffer as a `vector`.
  /// @param[in] len The number of elements to serialize.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename S>
  Offset<Vector<const T *>> CreateVectorOfSortedNativeStructs(S *v,
                                                              size_t len) {
    extern T Pack(const S &);
    auto structs = StartVectorOfStructs<T>(len);
    for (size_t i = 0; i < len; i++) { structs[i] = Pack(v[i]); }
    std::stable_sort(structs, structs + len, StructKeyComparator<T>());
    return EndVectorOfStructs<T>(len);
  }

  /// @cond FLATBUFFERS_INTERNAL
  template<typename T> struct TableKeyComparator {
    explicit TableKeyComparator(vector_downward<SizeT> &buf) : buf_(buf) {}
    TableKeyComparator(const TableKeyComparator &other) : buf_(other.buf_) {}
    bool operator()(const Offset<T> &a, const Offset<T> &b) const {
      auto table_a = reinterpret_cast<T *>(buf_.data_at(a.o));
      auto table_b = reinterpret_cast<T *>(buf_.data_at(b.o));
      return table_a->KeyCompareLessThan(table_b);
    }
    vector_downward<SizeT> &buf_;

   private:
    FLATBUFFERS_DELETE_FUNC(
        TableKeyComparator &operator=(const TableKeyComparator &other));
  };
  /// @endcond

  /// @brief Serialize an array of `table` offsets as a `vector` in the buffer
  /// in sorted order.
  /// @tparam T The data type that the offset refers to.
  /// @param[in] v An array of type `Offset<T>` that contains the `table`
  /// offsets to store in the buffer in sorted order.
  /// @param[in] len The number of elements to store in the `vector`.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T>
  Offset<Vector<Offset<T>>> CreateVectorOfSortedTables(Offset<T> *v,
                                                       size_t len) {
    std::stable_sort(v, v + len, TableKeyComparator<T>(buf_));
    return CreateVector(v, len);
  }

  /// @brief Serialize an array of `table` offsets as a `vector` in the buffer
  /// in sorted order.
  /// @tparam T The data type that the offset refers to.
  /// @param[in] v An array of type `Offset<T>` that contains the `table`
  /// offsets to store in the buffer in sorted order.
  /// @return Returns a typed `Offset` into the serialized data indicating
  /// where the vector is stored.
  template<typename T, typename Alloc = std::allocator<T>>
  Offset<Vector<Offset<T>>> CreateVectorOfSortedTables(
      std::vector<Offset<T>, Alloc> *v) {
    return CreateVectorOfSortedTables(data(*v), v->size());
  }

  /// @brief Specialized version of `CreateVector` for non-copying use cases.
  /// Write the data any time later to the returned buffer pointer `buf`.
  /// @param[in] len The number of elements to store in the `vector`.
  /// @param[in] elemsize The size of each element in the `vector`.
  /// @param[out] buf A pointer to a `uint8_t` pointer that can be
  /// written to at a later time to serialize the data into a `vector`
  /// in the buffer.
  uoffset_t CreateUninitializedVector(size_t len, size_t elemsize,
                                      size_t alignment, uint8_t **buf) {
    NotNested();
    StartVector(len, elemsize, alignment);
    buf_.make_space(len * elemsize);
    const uoffset_t vec_start = GetSizeRelative32BitRegion();
    auto vec_end = EndVector(len);
    *buf = buf_.data_at(vec_start);
    return vec_end;
  }

  FLATBUFFERS_ATTRIBUTE([[deprecated("call the version above instead")]])
  uoffset_t CreateUninitializedVector(size_t len, size_t elemsize,
                                      uint8_t **buf) {
    return CreateUninitializedVector(len, elemsize, elemsize, buf);
  }

  /// @brief Specialized version of `CreateVector` for non-copying use cases.
  /// Write the data any time later to the returned buffer pointer `buf`.
  /// @tparam T The data type of the data that will be stored in the buffer
  /// as a `vector`.
  /// @param[in] len The number of elements to store in the `vector`.
  /// @param[out] buf A pointer to a pointer of type `T` that can be
  /// written to at a later time to serialize the data into a `vector`
  /// in the buffer.
  template<typename T>
  Offset<Vector<T>> CreateUninitializedVector(size_t len, T **buf) {
    AssertScalarT<T>();
    return CreateUninitializedVector(len, sizeof(T), AlignOf<T>(),
                                     reinterpret_cast<uint8_t **>(buf));
  }

  template<typename T>
  Offset<Vector<const T *>> CreateUninitializedVectorOfStructs(size_t len,
                                                               T **buf) {
    return CreateUninitializedVector(len, sizeof(T), AlignOf<T>(),
                                     reinterpret_cast<uint8_t **>(buf));
  }

  // @brief Create a vector of scalar type T given as input a vector of scalar
  // type U, useful with e.g. pre "enum class" enums, or any existing scalar
  // data of the wrong type.
  template<typename T, typename U>
  Offset<Vector<T>> CreateVectorScalarCast(const U *v, size_t len) {
    AssertScalarT<T>();
    AssertScalarT<U>();
    StartVector<T>(len);
    for (auto i = len; i > 0;) { PushElement(static_cast<T>(v[--i])); }
    return Offset<Vector<T>>(EndVector(len));
  }

  /// @brief Write a struct by itself, typically to be part of a union.
  template<typename T> Offset<const T *> CreateStruct(const T &structobj) {
    NotNested();
    Align(AlignOf<T>());
    buf_.push_small(structobj);
    return Offset<const T *>(
        CalculateOffset<typename Offset<const T *>::offset_type>());
  }

  /// @brief Finish serializing a buffer by writing the root offset.
  /// @param[in] file_identifier If a `file_identifier` is given, the buffer
  /// will be prefixed with a standard FlatBuffers file header.
  template<typename T>
  void Finish(Offset<T> root, const char *file_identifier = nullptr) {
    Finish(root.o, file_identifier, false);
  }

  /// @brief Finish a buffer with a 32 bit size field pre-fixed (size of the
  /// buffer following the size field). These buffers are NOT compatible
  /// with standard buffers created by Finish, i.e. you can't call GetRoot
  /// on them, you have to use GetSizePrefixedRoot instead.
  /// All >32 bit quantities in this buffer will be aligned when the whole
  /// size pre-fixed buffer is aligned.
  /// These kinds of buffers are useful for creating a stream of FlatBuffers.
  template<typename T>
  void FinishSizePrefixed(Offset<T> root,
                          const char *file_identifier = nullptr) {
    Finish(root.o, file_identifier, true);
  }

  void SwapBufAllocator(FlatBufferBuilderImpl &other) {
    buf_.swap_allocator(other.buf_);
  }

  /// @brief The length of a FlatBuffer file header.
  static const size_t kFileIdentifierLength =
      ::flatbuffers::kFileIdentifierLength;

 protected:
  // You shouldn't really be copying instances of this class.
  FlatBufferBuilderImpl(const FlatBufferBuilderImpl &);
  FlatBufferBuilderImpl &operator=(const FlatBufferBuilderImpl &);

  void Finish(uoffset_t root, const char *file_identifier, bool size_prefix) {
    // A buffer can only be finished once. To reuse a builder use `clear()`.
    FLATBUFFERS_ASSERT(!finished);

    NotNested();
    buf_.clear_scratch();

    const size_t prefix_size = size_prefix ? sizeof(SizeT) : 0;
    // Make sure we track the alignment of the size prefix.
    TrackMinAlign(prefix_size);

    const size_t root_offset_size = sizeof(uoffset_t);
    const size_t file_id_size = file_identifier ? kFileIdentifierLength : 0;

    // This will cause the whole buffer to be aligned.
    PreAlign(prefix_size + root_offset_size + file_id_size, minalign_);

    if (file_identifier) {
      FLATBUFFERS_ASSERT(strlen(file_identifier) == kFileIdentifierLength);
      PushBytes(reinterpret_cast<const uint8_t *>(file_identifier),
                kFileIdentifierLength);
    }
    PushElement(ReferTo(root));  // Location of root.
    if (size_prefix) { PushElement(GetSize()); }
    finished = true;
  }

  struct FieldLoc {
    uoffset_t off;
    voffset_t id;
  };

  vector_downward<SizeT> buf_;

  // Accumulating offsets of table members while it is being built.
  // We store these in the scratch pad of buf_, after the vtable offsets.
  uoffset_t num_field_loc;
  // Track how much of the vtable is in use, so we can output the most compact
  // possible vtable.
  voffset_t max_voffset_;

  // This is the length of the 64-bit region of the buffer. The buffer supports
  // 64-bit offsets by forcing serialization of those elements in the "tail"
  // region of the buffer (i.e. "64-bit region"). To properly keep track of
  // offsets that are referenced from the tail of the buffer to not overflow
  // their size (e.g. Offset is a uint32_t type), the boundary of the 32-/64-bit
  // regions must be tracked.
  //
  // [    Complete FlatBuffer     ]
  // [32-bit region][64-bit region]
  //               ^              ^
  //               |              Tail of the buffer.
  //               |
  //               Tail of the 32-bit region of the buffer.
  //
  // This keeps track of the size of the 64-bit region so that the tail of the
  // 32-bit region can be calculated as `GetSize() - length_of_64_bit_region_`.
  //
  // This will remain 0 if no 64-bit offset types are added to the buffer.
  size_t length_of_64_bit_region_;

  // Ensure objects are not nested.
  bool nested;

  // Ensure the buffer is finished before it is being accessed.
  bool finished;

  size_t minalign_;

  bool force_defaults_;  // Serialize values equal to their defaults anyway.

  bool dedup_vtables_;

  struct StringOffsetCompare {
    explicit StringOffsetCompare(const vector_downward<SizeT> &buf)
        : buf_(&buf) {}
    bool operator()(const Offset<String> &a, const Offset<String> &b) const {
      auto stra = reinterpret_cast<const String *>(buf_->data_at(a.o));
      auto strb = reinterpret_cast<const String *>(buf_->data_at(b.o));
      return StringLessThan(stra->data(), stra->size(), strb->data(),
                            strb->size());
    }
    const vector_downward<SizeT> *buf_;
  };

  // For use with CreateSharedString. Instantiated on first use only.
  typedef std::set<Offset<String>, StringOffsetCompare> StringOffsetMap;
  StringOffsetMap *string_pool;

 private:
  void CanAddOffset64() {
    // If you hit this assertion, you are attempting to add a 64-bit offset to
    // a 32-bit only builder. This is because the builder has overloads that
    // differ only on the offset size returned: e.g.:
    //
    //   FlatBufferBuilder builder;
    //   Offset64<String> string_offset = builder.CreateString<Offset64>();
    //
    // Either use a 64-bit aware builder, or don't try to create an Offset64
    // return type.
    //
    // TODO(derekbailey): we can probably do more enable_if to avoid this
    // looking like its possible to the user.
    static_assert(Is64Aware, "cannot add 64-bit offset to a 32-bit builder");

    // If you hit this assertion, you are attempting to add an 64-bit offset
    // item after already serializing a 32-bit item. All 64-bit offsets have to
    // added to the tail of the buffer before any 32-bit items can be added.
    // Otherwise some items might not be addressable due to the maximum range of
    // the 32-bit offset.
    FLATBUFFERS_ASSERT(GetSize() == length_of_64_bit_region_);
  }

  /// @brief Store a string in the buffer, which can contain any binary data.
  /// @param[in] str A const char pointer to the data to be stored as a string.
  /// @param[in] len The number of bytes that should be stored from `str`.
  /// @return Returns the offset in the buffer where the string starts.
  void CreateStringImpl(const char *str, size_t len) {
    NotNested();
    PreAlign<uoffset_t>(len + 1);  // Always 0-terminated.
    buf_.fill(1);
    PushBytes(reinterpret_cast<const uint8_t *>(str), len);
    PushElement(static_cast<uoffset_t>(len));
  }

  // Allocates space for a vector of structures.
  // Must be completed with EndVectorOfStructs().
  template<typename T, template<typename> class OffsetT = Offset>
  T *StartVectorOfStructs(size_t vector_size) {
    StartVector<OffsetT>(vector_size, sizeof(T), AlignOf<T>());
    return reinterpret_cast<T *>(buf_.make_space(vector_size * sizeof(T)));
  }

  // End the vector of structures in the flatbuffers.
  // Vector should have previously be started with StartVectorOfStructs().
  template<typename T, template<typename> class OffsetT = Offset>
  OffsetT<Vector<const T *>> EndVectorOfStructs(size_t vector_size) {
    return OffsetT<Vector<const T *>>(
        EndVector<typename Vector<const T *>::size_type,
                  typename OffsetT<Vector<const T *>>::offset_type>(
            vector_size));
  }

  template<typename T>
  typename std::enable_if<std::is_same<T, uoffset_t>::value, T>::type
  CalculateOffset() {
    // Default to the end of the 32-bit region. This may or may not be the end
    // of the buffer, depending on if any 64-bit offsets have been added.
    return GetSizeRelative32BitRegion();
  }

  // Specializations to handle the 64-bit CalculateOffset, which is relative to
  // end of the buffer.
  template<typename T>
  typename std::enable_if<std::is_same<T, uoffset64_t>::value, T>::type
  CalculateOffset() {
    // This should never be compiled in when not using a 64-bit builder.
    static_assert(Is64Aware, "invalid 64-bit offset in 32-bit builder");

    // Store how big the 64-bit region of the buffer is, so we can determine
    // where the 32/64 bit boundary is.
    length_of_64_bit_region_ = GetSize();

    return length_of_64_bit_region_;
  }
};
/// @}

// Hack to `FlatBufferBuilder` mean `FlatBufferBuilder<false>` or
// `FlatBufferBuilder<>`, where the template < > syntax is required.
using FlatBufferBuilder = FlatBufferBuilderImpl<false>;
using FlatBufferBuilder64 = FlatBufferBuilderImpl<true>;

// These are external due to GCC not allowing them in the class.
// See: https://stackoverflow.com/q/8061456/868247
template<>
template<>
inline Offset64<String> FlatBufferBuilder64::CreateString(const char *str,
                                                          size_t len) {
  CanAddOffset64();
  CreateStringImpl(str, len);
  return Offset64<String>(
      CalculateOffset<typename Offset64<String>::offset_type>());
}

// Used to distinguish from real Offsets.
template<typename T = void> struct EmptyOffset {};

// TODO(derekbailey): it would be nice to combine these two methods.
template<>
template<>
inline void FlatBufferBuilder64::StartVector<Offset64, uint32_t>(
    size_t len, size_t elemsize, size_t alignment) {
  CanAddOffset64();
  StartVector<EmptyOffset, uint32_t>(len, elemsize, alignment);
}

template<>
template<>
inline void FlatBufferBuilder64::StartVector<Offset64, uint64_t>(
    size_t len, size_t elemsize, size_t alignment) {
  CanAddOffset64();
  StartVector<EmptyOffset, uint64_t>(len, elemsize, alignment);
}

/// Helpers to get a typed pointer to objects that are currently being built.
/// @warning Creating new objects will lead to reallocations and invalidates
/// the pointer!
template<typename T>
T *GetMutableTemporaryPointer(FlatBufferBuilder &fbb, Offset<T> offset) {
  return reinterpret_cast<T *>(fbb.GetCurrentBufferPointer() + fbb.GetSize() -
                               offset.o);
}

template<typename T>
const T *GetTemporaryPointer(const FlatBufferBuilder &fbb, Offset<T> offset) {
  return reinterpret_cast<const T *>(fbb.GetCurrentBufferPointer() +
                                     fbb.GetSize() - offset.o);
}

}  // namespace flatbuffers

#endif  // FLATBUFFERS_FLATBUFFER_BUILDER_H_
