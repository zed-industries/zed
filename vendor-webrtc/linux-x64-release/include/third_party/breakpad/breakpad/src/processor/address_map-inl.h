// Copyright 2006 Google LLC
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

// address_map-inl.h: Address map implementation.
//
// See address_map.h for documentation.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_ADDRESS_MAP_INL_H__
#define PROCESSOR_ADDRESS_MAP_INL_H__

#include "processor/address_map.h"

#include <assert.h>

#include "processor/logging.h"

namespace google_breakpad {

template<typename AddressType, typename EntryType>
bool AddressMap<AddressType, EntryType>::Store(const AddressType& address,
                                               const EntryType& entry) {
  // Ensure that the specified address doesn't conflict with something already
  // in the map.
  if (map_.find(address) != map_.end()) {
    BPLOG(INFO) << "Store failed, address " << HexString(address) <<
                   " is already present";
    return false;
  }

  map_.insert(MapValue(address, entry));
  return true;
}

template<typename AddressType, typename EntryType>
bool AddressMap<AddressType, EntryType>::Retrieve(
    const AddressType& address,
    EntryType* entry, AddressType* entry_address) const {
  BPLOG_IF(ERROR, !entry) << "AddressMap::Retrieve requires |entry|";
  assert(entry);

  // upper_bound gives the first element whose key is greater than address,
  // but we want the first element whose key is less than or equal to address.
  // Decrement the iterator to get there, but not if the upper_bound already
  // points to the beginning of the map - in that case, address is lower than
  // the lowest stored key, so return false.
  MapConstIterator iterator = map_.upper_bound(address);
  if (iterator == map_.begin())
    return false;
  --iterator;

  *entry = iterator->second;
  if (entry_address)
    *entry_address = iterator->first;

  return true;
}

template<typename AddressType, typename EntryType>
void AddressMap<AddressType, EntryType>::Clear() {
  map_.clear();
}

}  // namespace google_breakpad

#endif  // PROCESSOR_ADDRESS_MAP_INL_H__
