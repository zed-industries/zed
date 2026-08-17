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

#ifndef FLATBUFFERS_TABLE_H_
#define FLATBUFFERS_TABLE_H_

#include "flatbuffers/base.h"
#include "flatbuffers/verifier.h"

namespace flatbuffers {

// "tables" use an offset table (possibly shared) that allows fields to be
// omitted and added at will, but uses an extra indirection to read.
class Table {
 public:
  const uint8_t *GetVTable() const {
    return data_ - ReadScalar<soffset_t>(data_);
  }

  // This gets the field offset for any of the functions below it, or 0
  // if the field was not present.
  voffset_t GetOptionalFieldOffset(voffset_t field) const {
    // The vtable offset is always at the start.
    auto vtable = GetVTable();
    // The first element is the size of the vtable (fields + type id + itself).
    auto vtsize = ReadScalar<voffset_t>(vtable);
    // If the field we're accessing is outside the vtable, we're reading older
    // data, so it's the same as if the offset was 0 (not present).
    return field < vtsize ? ReadScalar<voffset_t>(vtable + field) : 0;
  }

  template<typename T> T GetField(voffset_t field, T defaultval) const {
    auto field_offset = GetOptionalFieldOffset(field);
    return field_offset ? ReadScalar<T>(data_ + field_offset) : defaultval;
  }

  template<typename P, typename OffsetSize = uoffset_t>
  P GetPointer(voffset_t field) {
    auto field_offset = GetOptionalFieldOffset(field);
    auto p = data_ + field_offset;
    return field_offset ? reinterpret_cast<P>(p + ReadScalar<OffsetSize>(p))
                        : nullptr;
  }
  template<typename P, typename OffsetSize = uoffset_t>
  P GetPointer(voffset_t field) const {
    return const_cast<Table *>(this)->GetPointer<P, OffsetSize>(field);
  }

  template<typename P> P GetPointer64(voffset_t field) {
    return GetPointer<P, uoffset64_t>(field);
  }

  template<typename P> P GetPointer64(voffset_t field) const {
    return GetPointer<P, uoffset64_t>(field);
  }

  template<typename P> P GetStruct(voffset_t field) const {
    auto field_offset = GetOptionalFieldOffset(field);
    auto p = const_cast<uint8_t *>(data_ + field_offset);
    return field_offset ? reinterpret_cast<P>(p) : nullptr;
  }

  template<typename Raw, typename Face>
  flatbuffers::Optional<Face> GetOptional(voffset_t field) const {
    auto field_offset = GetOptionalFieldOffset(field);
    auto p = data_ + field_offset;
    return field_offset ? Optional<Face>(static_cast<Face>(ReadScalar<Raw>(p)))
                        : Optional<Face>();
  }

  template<typename T> bool SetField(voffset_t field, T val, T def) {
    auto field_offset = GetOptionalFieldOffset(field);
    if (!field_offset) return IsTheSameAs(val, def);
    WriteScalar(data_ + field_offset, val);
    return true;
  }
  template<typename T> bool SetField(voffset_t field, T val) {
    auto field_offset = GetOptionalFieldOffset(field);
    if (!field_offset) return false;
    WriteScalar(data_ + field_offset, val);
    return true;
  }

  bool SetPointer(voffset_t field, const uint8_t *val) {
    auto field_offset = GetOptionalFieldOffset(field);
    if (!field_offset) return false;
    WriteScalar(data_ + field_offset,
                static_cast<uoffset_t>(val - (data_ + field_offset)));
    return true;
  }

  uint8_t *GetAddressOf(voffset_t field) {
    auto field_offset = GetOptionalFieldOffset(field);
    return field_offset ? data_ + field_offset : nullptr;
  }
  const uint8_t *GetAddressOf(voffset_t field) const {
    return const_cast<Table *>(this)->GetAddressOf(field);
  }

  bool CheckField(voffset_t field) const {
    return GetOptionalFieldOffset(field) != 0;
  }

  // Verify the vtable of this table.
  // Call this once per table, followed by VerifyField once per field.
  bool VerifyTableStart(Verifier &verifier) const {
    return verifier.VerifyTableStart(data_);
  }

  // Verify a particular field.
  template<typename T>
  bool VerifyField(const Verifier &verifier, voffset_t field,
                   size_t align) const {
    // Calling GetOptionalFieldOffset should be safe now thanks to
    // VerifyTable().
    auto field_offset = GetOptionalFieldOffset(field);
    // Check the actual field.
    return !field_offset || verifier.VerifyField<T>(data_, field_offset, align);
  }

  // VerifyField for required fields.
  template<typename T>
  bool VerifyFieldRequired(const Verifier &verifier, voffset_t field,
                           size_t align) const {
    auto field_offset = GetOptionalFieldOffset(field);
    return verifier.Check(field_offset != 0) &&
           verifier.VerifyField<T>(data_, field_offset, align);
  }

  // Versions for offsets.
  template<typename OffsetT = uoffset_t>
  bool VerifyOffset(const Verifier &verifier, voffset_t field) const {
    auto field_offset = GetOptionalFieldOffset(field);
    return !field_offset || verifier.VerifyOffset<OffsetT>(data_, field_offset);
  }

  template<typename OffsetT = uoffset_t>
  bool VerifyOffsetRequired(const Verifier &verifier, voffset_t field) const {
    auto field_offset = GetOptionalFieldOffset(field);
    return verifier.Check(field_offset != 0) &&
           verifier.VerifyOffset<OffsetT>(data_, field_offset);
  }

  bool VerifyOffset64(const Verifier &verifier, voffset_t field) const {
    return VerifyOffset<uoffset64_t>(verifier, field);
  }

  bool VerifyOffset64Required(const Verifier &verifier, voffset_t field) const {
    return VerifyOffsetRequired<uoffset64_t>(verifier, field);
  }

 private:
  // private constructor & copy constructor: you obtain instances of this
  // class by pointing to existing data only
  Table();
  Table(const Table &other);
  Table &operator=(const Table &);

  uint8_t data_[1];
};

// This specialization allows avoiding warnings like:
// MSVC C4800: type: forcing value to bool 'true' or 'false'.
template<>
inline flatbuffers::Optional<bool> Table::GetOptional<uint8_t, bool>(
    voffset_t field) const {
  auto field_offset = GetOptionalFieldOffset(field);
  auto p = data_ + field_offset;
  return field_offset ? Optional<bool>(ReadScalar<uint8_t>(p) != 0)
                      : Optional<bool>();
}

}  // namespace flatbuffers

#endif  // FLATBUFFERS_TABLE_H_
