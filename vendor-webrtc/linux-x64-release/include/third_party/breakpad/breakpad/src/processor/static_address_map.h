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

// static_address_map.h: StaticAddressMap.
//
// StaticAddressMap is a wrapper class of StaticMap, just as AddressMap wraps
// std::map.  StaticAddressMap provides read-only Retrieve() operation, similar
// as AddressMap.  However, the difference between StaticAddressMap and
// AddressMap is that StaticAddressMap does not support dynamic operation
// Store() due to the static nature of the underlying StaticMap.
//
// See address_map.h for reference.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_STATIC_ADDRESS_MAP_H__
#define PROCESSOR_STATIC_ADDRESS_MAP_H__

#include "processor/static_map-inl.h"

namespace google_breakpad {

// AddressType MUST be a basic type, e.g.: integer types etc
// EntryType could be a complex type, so we retrieve its pointer instead.
template<typename AddressType, typename EntryType>
class StaticAddressMap {
 public:
  StaticAddressMap(): map_() { }
  explicit StaticAddressMap(const char* map_data): map_(map_data) { }

  // Locates the entry stored at the highest address less than or equal to
  // the address argument.  If there is no such range, returns false.  The
  // entry is returned in entry, which is a required argument.  If
  // entry_address is not NULL, it will be set to the address that the entry
  // was stored at.
  bool Retrieve(const AddressType& address,
                const EntryType*& entry, AddressType* entry_address) const;

 private:
  friend class ModuleComparer;
  // Convenience types.
  typedef StaticAddressMap* SelfPtr;
  typedef StaticMap<AddressType, EntryType> AddressToEntryMap;
  typedef typename AddressToEntryMap::const_iterator MapConstIterator;

  AddressToEntryMap map_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_STATIC_ADDRESS_MAP_H__

