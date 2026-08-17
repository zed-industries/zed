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

// address_map.h: Address maps.
//
// An address map contains a set of objects keyed by address.  Objects are
// retrieved from the map by returning the object with the highest key less
// than or equal to the lookup key.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_ADDRESS_MAP_H__
#define PROCESSOR_ADDRESS_MAP_H__

#include <map>

namespace google_breakpad {

// Forward declarations (for later friend declarations).
template<class, class> class AddressMapSerializer;

template<typename AddressType, typename EntryType>
class AddressMap {
 public:
  AddressMap() : map_() {}

  // Inserts an entry into the map.  Returns false without storing the entry
  // if an entry is already stored in the map at the same address as specified
  // by the address argument.
  bool Store(const AddressType& address, const EntryType& entry);

  // Locates the entry stored at the highest address less than or equal to
  // the address argument.  If there is no such range, returns false.  The
  // entry is returned in entry, which is a required argument.  If
  // entry_address is not NULL, it will be set to the address that the entry
  // was stored at.
  bool Retrieve(const AddressType& address,
                EntryType* entry, AddressType* entry_address) const;

  // Empties the address map, restoring it to the same state as when it was
  // initially created.
  void Clear();

 private:
  friend class AddressMapSerializer<AddressType, EntryType>;
  friend class ModuleComparer;

  // Convenience types.
  typedef std::map<AddressType, EntryType> AddressToEntryMap;
  typedef typename AddressToEntryMap::const_iterator MapConstIterator;
  typedef typename AddressToEntryMap::value_type MapValue;

  // Maps the address of each entry to an EntryType.
  AddressToEntryMap map_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_ADDRESS_MAP_H__
