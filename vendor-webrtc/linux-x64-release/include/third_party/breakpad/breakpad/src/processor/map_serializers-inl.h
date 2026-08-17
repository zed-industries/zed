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
// map_serializers_inl.h: implementation for serializing std::map and its
// wrapper classes.
//
// See map_serializers.h for documentation.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_MAP_SERIALIZERS_INL_H__
#define PROCESSOR_MAP_SERIALIZERS_INL_H__

#include <map>
#include <string>

#include "processor/map_serializers.h"
#include "processor/simple_serializer.h"

#include "processor/address_map-inl.h"
#include "processor/range_map-inl.h"
#include "processor/contained_range_map-inl.h"

#include "processor/logging.h"

namespace google_breakpad {

template<typename Key, typename Value>
size_t StdMapSerializer<Key, Value>::SizeOf(
    const std::map<Key, Value>& m) const {
  size_t size = 0;
  size_t header_size = (1 + m.size()) * sizeof(uint64_t);
  size += header_size;

  typename std::map<Key, Value>::const_iterator iter;
  for (iter = m.begin(); iter != m.end(); ++iter) {
    size += key_serializer_.SizeOf(iter->first);
    size += value_serializer_.SizeOf(iter->second);
  }
  return size;
}

template<typename Key, typename Value>
char* StdMapSerializer<Key, Value>::Write(const std::map<Key, Value>& m,
                                          char* dest) const {
  if (!dest) {
    BPLOG(ERROR) << "StdMapSerializer failed: write to NULL address.";
    return nullptr;
  }
  char* start_address = dest;

  // Write header:
  // Number of nodes.
  dest = SimpleSerializer<uint64_t>::Write(m.size(), dest);
  // Nodes offsets.
  uint64_t* offsets = reinterpret_cast<uint64_t*>(dest);
  dest += sizeof(uint64_t) * m.size();

  char* key_address = dest;
  dest += sizeof(Key) * m.size();

  // Traverse map.
  typename std::map<Key, Value>::const_iterator iter;
  int64_t index = 0;
  for (iter = m.begin(); iter != m.end(); ++iter, ++index) {
    offsets[index] = static_cast<uint64_t>(dest - start_address);
    key_address = key_serializer_.Write(iter->first, key_address);
    dest = value_serializer_.Write(iter->second, dest);
  }
  return dest;
}

template<typename Key, typename Value>
char* StdMapSerializer<Key, Value>::Serialize(
    const std::map<Key, Value>& m, uint64_t* size) const {
  // Compute size of memory to be allocated.
  uint64_t size_to_alloc = SizeOf(m);
  // Allocate memory.
  char* serialized_data = new char[size_to_alloc];
  if (!serialized_data) {
    BPLOG(INFO) << "StdMapSerializer memory allocation failed.";
    if (size) *size = 0;
    return nullptr;
  }
  // Write serialized data into memory.
  Write(m, serialized_data);

  if (size) *size = size_to_alloc;
  return serialized_data;
}

template<typename Address, typename Entry>
size_t RangeMapSerializer<Address, Entry>::SizeOf(
    const RangeMap<Address, Entry>& m) const {
  size_t size = 0;
  size_t header_size = (1 + m.map_.size()) * sizeof(uint64_t);
  size += header_size;

  typename std::map<Address, Range>::const_iterator iter;
  for (iter = m.map_.begin(); iter != m.map_.end(); ++iter) {
    // Size of key (high address).
    size += address_serializer_.SizeOf(iter->first);
    // Size of base (low address).
    size += address_serializer_.SizeOf(iter->second.base());
    // Size of entry.
    size += entry_serializer_.SizeOf(iter->second.entry());
  }
  return size;
}

template<typename Address, typename Entry>
char* RangeMapSerializer<Address, Entry>::Write(
    const RangeMap<Address, Entry>& m, char* dest) const {
  if (!dest) {
    BPLOG(ERROR) << "RangeMapSerializer failed: write to NULL address.";
    return nullptr;
  }
  char* start_address = dest;

  // Write header:
  // Number of nodes.
  dest = SimpleSerializer<uint64_t>::Write(m.map_.size(), dest);
  // Nodes offsets.
  uint64_t* offsets = reinterpret_cast<uint64_t*>(dest);
  dest += sizeof(uint64_t) * m.map_.size();

  char* key_address = dest;
  dest += sizeof(Address) * m.map_.size();

  // Traverse map.
  typename std::map<Address, Range>::const_iterator iter;
  int64_t index = 0;
  for (iter = m.map_.begin(); iter != m.map_.end(); ++iter, ++index) {
    offsets[index] = static_cast<uint64_t>(dest - start_address);
    key_address = address_serializer_.Write(iter->first, key_address);
    dest = address_serializer_.Write(iter->second.base(), dest);
    dest = entry_serializer_.Write(iter->second.entry(), dest);
  }
  return dest;
}

template<typename Address, typename Entry>
char* RangeMapSerializer<Address, Entry>::Serialize(
    const RangeMap<Address, Entry>& m, uint64_t* size) const {
  // Compute size of memory to be allocated.
  uint64_t size_to_alloc = SizeOf(m);
  // Allocate memory.
  char* serialized_data = new char[size_to_alloc];
  if (!serialized_data) {
    BPLOG(INFO) << "RangeMapSerializer memory allocation failed.";
    if (size) *size = 0;
    return nullptr;
  }

  // Write serialized data into memory.
  Write(m, serialized_data);

  if (size) *size = size_to_alloc;
  return serialized_data;
}


template<class AddrType, class EntryType>
size_t ContainedRangeMapSerializer<AddrType, EntryType>::SizeOf(
    const ContainedRangeMap<AddrType, EntryType>* m) const {
  size_t size = 0;
  size_t header_size = addr_serializer_.SizeOf(m->base_)
                       + entry_serializer_.SizeOf(m->entry_)
                       + sizeof(uint64_t);
  size += header_size;
  // In case m.map_ == NULL, we treat it as an empty map:
  size += sizeof(uint64_t);
  if (m->map_) {
    size += m->map_->size() * sizeof(uint64_t);
    typename Map::const_iterator iter;
    for (iter = m->map_->begin(); iter != m->map_->end(); ++iter) {
      size += addr_serializer_.SizeOf(iter->first);
      // Recursive calculation of size:
      size += SizeOf(iter->second);
    }
  }
  return size;
}

template<class AddrType, class EntryType>
char* ContainedRangeMapSerializer<AddrType, EntryType>::Write(
    const ContainedRangeMap<AddrType, EntryType>* m, char* dest) const {
  if (!dest) {
    BPLOG(ERROR) << "StdMapSerializer failed: write to NULL address.";
    return nullptr;
  }
  dest = addr_serializer_.Write(m->base_, dest);
  dest = SimpleSerializer<uint64_t>::Write(entry_serializer_.SizeOf(m->entry_),
                                            dest);
  dest = entry_serializer_.Write(m->entry_, dest);

  // Write map<<AddrType, ContainedRangeMap*>:
  char* map_address = dest;
  if (m->map_ == nullptr) {
    dest = SimpleSerializer<uint64_t>::Write(0, dest);
  } else {
    dest = SimpleSerializer<uint64_t>::Write(m->map_->size(), dest);
    uint64_t* offsets = reinterpret_cast<uint64_t*>(dest);
    dest += sizeof(uint64_t) * m->map_->size();

    char* key_address = dest;
    dest += sizeof(AddrType) * m->map_->size();

    // Traverse map.
    typename Map::const_iterator iter;
    int64_t index = 0;
    for (iter = m->map_->begin(); iter != m->map_->end(); ++iter, ++index) {
      offsets[index] = static_cast<uint64_t>(dest - map_address);
      key_address = addr_serializer_.Write(iter->first, key_address);
      // Recursively write.
      dest = Write(iter->second, dest);
    }
  }
  return dest;
}

template<class AddrType, class EntryType>
char* ContainedRangeMapSerializer<AddrType, EntryType>::Serialize(
    const ContainedRangeMap<AddrType, EntryType>* m, uint64_t* size) const {
  uint64_t size_to_alloc = SizeOf(m);
  // Allocating memory.
  char* serialized_data = new char[size_to_alloc];
  if (!serialized_data) {
    BPLOG(INFO) << "ContainedRangeMapSerializer memory allocation failed.";
    if (size) *size = 0;
    return nullptr;
  }
  Write(m, serialized_data);
  if (size) *size = size_to_alloc;
  return serialized_data;
}

}  // namespace google_breakpad

#endif  // PROCESSOR_MAP_SERIALIZERS_INL_H__
