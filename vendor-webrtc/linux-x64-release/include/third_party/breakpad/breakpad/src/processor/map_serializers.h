// Copyright 2010 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// map_serializers.h: defines templates for serializing std::map and its
// wrappers: AddressMap, RangeMap, and ContainedRangeMap.
//
// Author: Siyang Xie (lambxsy@google.com)


#ifndef PROCESSOR_MAP_SERIALIZERS_H__
#define PROCESSOR_MAP_SERIALIZERS_H__

#include <map>
#include <string>

#include "processor/simple_serializer.h"

#include "processor/address_map-inl.h"
#include "processor/range_map-inl.h"
#include "processor/contained_range_map-inl.h"

namespace google_breakpad {

// StdMapSerializer allocates memory and serializes an std::map instance into a
// chunk of memory data.
template<typename Key, typename Value>
class StdMapSerializer {
 public:
  // Calculate the memory size of serialized data.
  size_t SizeOf(const std::map<Key, Value>& m) const;

  // Writes the serialized data to memory with start address = dest,
  // and returns the "end" of data, i.e., return the address follow the final
  // byte of data.
  // NOTE: caller has to allocate enough memory before invoke Write() method.
  char* Write(const std::map<Key, Value>& m, char* dest) const;

  // Serializes a std::map object into a chunk of memory data with format
  // described in "StaticMap.h" comment.
  // Returns a pointer to the serialized data.  If size != NULL, *size is set
  // to the size of serialized data, i.e., SizeOf(m).
  // Caller has the ownership of memory allocated as "new char[]".
  char* Serialize(const std::map<Key, Value>& m, uint64_t* size) const;

 private:
  SimpleSerializer<Key> key_serializer_;
  SimpleSerializer<Value> value_serializer_;
};

// AddressMapSerializer allocates memory and serializes an AddressMap into a
// chunk of memory data.
template<typename Addr, typename Entry>
class AddressMapSerializer {
 public:
  // Calculate the memory size of serialized data.
  size_t SizeOf(const AddressMap<Addr, Entry>& m) const {
    return std_map_serializer_.SizeOf(m.map_);
  }

  // Write the serialized data to specified memory location.  Return the "end"
  // of data, i.e., return the address after the final byte of data.
  // NOTE: caller has to allocate enough memory before invoke Write() method.
  char* Write(const AddressMap<Addr, Entry>& m, char* dest) const {
    return std_map_serializer_.Write(m.map_, dest);
  }

  // Serializes an AddressMap object into a chunk of memory data.
  // Returns a pointer to the serialized data.  If size != NULL, *size is set
  // to the size of serialized data, i.e., SizeOf(m).
  // Caller has the ownership of memory allocated as "new char[]".
  char* Serialize(const AddressMap<Addr, Entry>& m, uint64_t* size) const {
    return std_map_serializer_.Serialize(m.map_, size);
  }

 private:
  // AddressMapSerializer is a simple wrapper of StdMapSerializer, just as
  // AddressMap is a simple wrapper of std::map.
  StdMapSerializer<Addr, Entry> std_map_serializer_;
};

// RangeMapSerializer allocates memory and serializes a RangeMap instance into a
// chunk of memory data.
template<typename Address, typename Entry>
class RangeMapSerializer {
 public:
  // Calculate the memory size of serialized data.
  size_t SizeOf(const RangeMap<Address, Entry>& m) const;

  // Write the serialized data to specified memory location.  Return the "end"
  // of data, i.e., return the address after the final byte of data.
  // NOTE: caller has to allocate enough memory before invoke Write() method.
  char* Write(const RangeMap<Address, Entry>& m, char* dest) const;

  // Serializes a RangeMap object into a chunk of memory data.
  // Returns a pointer to the serialized data.  If size != NULL, *size is set
  // to the size of serialized data, i.e., SizeOf(m).
  // Caller has the ownership of memory allocated as "new char[]".
  char* Serialize(const RangeMap<Address, Entry>& m, uint64_t* size) const;

 private:
  // Convenient type name for Range.
  typedef typename RangeMap<Address, Entry>::Range Range;

  // Serializer for RangeMap's key and Range::base_.
  SimpleSerializer<Address> address_serializer_;
  // Serializer for RangeMap::Range::entry_.
  SimpleSerializer<Entry> entry_serializer_;
};

// ContainedRangeMapSerializer allocates memory and serializes a
// ContainedRangeMap instance into a chunk of memory data.
template<class AddrType, class EntryType>
class ContainedRangeMapSerializer {
 public:
  // Calculate the memory size of serialized data.
  size_t SizeOf(const ContainedRangeMap<AddrType, EntryType>* m) const;

  // Write the serialized data to specified memory location.  Return the "end"
  // of data, i.e., return the address after the final byte of data.
  // NOTE: caller has to allocate enough memory before invoke Write() method.
  char* Write(const ContainedRangeMap<AddrType, EntryType>* m,
              char* dest) const;

  // Serializes a ContainedRangeMap object into a chunk of memory data.
  // Returns a pointer to the serialized data.  If size != NULL, *size is set
  // to the size of serialized data, i.e., SizeOf(m).
  // Caller has the ownership of memory allocated as "new char[]".
  char* Serialize(const ContainedRangeMap<AddrType, EntryType>* m,
                  uint64_t* size) const;

 private:
  // Convenient type name for the underlying map type.
  typedef std::map<AddrType, ContainedRangeMap<AddrType, EntryType>*> Map;

  // Serializer for addresses and entries stored in ContainedRangeMap.
  SimpleSerializer<AddrType> addr_serializer_;
  SimpleSerializer<EntryType> entry_serializer_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_MAP_SERIALIZERS_H__
