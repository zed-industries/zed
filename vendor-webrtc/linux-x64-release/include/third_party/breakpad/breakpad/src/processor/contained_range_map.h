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

// contained_range_map.h: Hierarchically-organized range maps.
//
// A contained range map is similar to a standard range map, except it allows
// objects to be organized hierarchically.  A contained range map allows
// objects to contain other objects.  It is not sensitive to the order that
// objects are added to the map: larger, more general, containing objects
// may be added either before or after smaller, more specific, contained
// ones.
//
// Contained range maps guarantee that each object may only contain smaller
// objects than itself, and that a parent object may only contain child
// objects located entirely within the parent's address space.  Attempts
// to introduce objects (via StoreRange) that violate these rules will fail.
// Retrieval (via RetrieveRange) always returns the most specific (smallest)
// object that contains the address being queried.  Note that while it is
// not possible to insert two objects into a map that have exactly the same
// geometry (base address and size), it is possible to completely mask a
// larger object by inserting smaller objects that entirely fill the larger
// object's address space.
//
// Internally, contained range maps are implemented as a tree.  Each tree
// node except for the root node describes an object in the map.  Each node
// maintains its list of children in a map similar to a standard range map,
// keyed by the highest address that each child occupies.  Each node's
// children occupy address ranges entirely within the node.  The root node
// is the only node directly accessible to the user, and represents the
// entire address space.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_CONTAINED_RANGE_MAP_H__
#define PROCESSOR_CONTAINED_RANGE_MAP_H__


#include <map>
#include <vector>


namespace google_breakpad {

// Forward declarations (for later friend declarations of specialized template).
template<class, class> class ContainedRangeMapSerializer;

template<typename AddressType, typename EntryType>
class ContainedRangeMap {
 public:
  // The default constructor creates a ContainedRangeMap with no geometry
  // and no entry, and as such is only suitable for the root node of a
  // ContainedRangeMap tree.
  explicit ContainedRangeMap(bool allow_equal_range = false)
      : base_(),
        entry_(),
        map_(nullptr),
        allow_equal_range_(allow_equal_range) {}

  ~ContainedRangeMap();

  // Inserts a range into the map.  If the new range is encompassed by
  // an existing child range, the new range is passed into the child range's
  // StoreRange method.  If the new range encompasses any existing child
  // ranges, those child ranges are moved to the new range, becoming
  // grandchildren of this ContainedRangeMap.  Returns false for a
  // parameter error, or if the ContainedRangeMap hierarchy guarantees
  // would be violated.
  bool StoreRange(const AddressType& base,
                  const AddressType& size,
                  const EntryType& entry);

  // Retrieves the most specific (smallest) descendant range encompassing
  // the specified address.  This method will only return entries held by
  // child ranges, and not the entry contained by |this|.  This is necessary
  // to support a sparsely-populated root range.  If no descendant range
  // encompasses the address, returns false.
  bool RetrieveRange(const AddressType& address, EntryType* entries) const;

  // Retrieves the vector of entries encompassing the specified address from the
  // innermost entry to the outermost entry.
  bool RetrieveRanges(const AddressType& address,
                      std::vector<const EntryType*>& entries) const;

  // Removes all children.  Note that Clear only removes descendants,
  // leaving the node on which it is called intact.  Because the only
  // meaningful things contained by a root node are descendants, this
  // is sufficient to restore an entire ContainedRangeMap to its initial
  // empty state when called on the root node.
  void Clear();

 private:
  friend class ContainedRangeMapSerializer<AddressType, EntryType>;
  friend class ModuleComparer;

  // AddressToRangeMap stores pointers.  This makes reparenting simpler in
  // StoreRange, because it doesn't need to copy entire objects.
  typedef std::map<AddressType, ContainedRangeMap*> AddressToRangeMap;
  typedef typename AddressToRangeMap::const_iterator MapConstIterator;
  typedef typename AddressToRangeMap::iterator MapIterator;
  typedef typename AddressToRangeMap::value_type MapValue;

  // Creates a new ContainedRangeMap with the specified base address, entry,
  // and initial child map, which may be NULL.  This is only used internally
  // by ContainedRangeMap when it creates a new child.
  ContainedRangeMap(const AddressType& base,
                    const EntryType& entry,
                    AddressToRangeMap* map,
                    bool allow_equal_range)
      : base_(base),
        entry_(entry),
        map_(map),
        allow_equal_range_(allow_equal_range) {}

  // The base address of this range.  The high address does not need to
  // be stored, because it is used as the key to an object in its parent's
  // map, and all ContainedRangeMaps except for the root range are contained
  // within maps.  The root range does not actually contain an entry, so its
  // base_ field is meaningless, and the fact that it has no parent and thus
  // no key is unimportant.  For this reason, the base_ field should only be
  // is accessed on child ContainedRangeMap objects, and never on |this|.
  const AddressType base_;

  // The entry corresponding to this range.  The root range does not
  // actually contain an entry, so its entry_ field is meaningless.  For
  // this reason, the entry_ field should only be accessed on child
  // ContainedRangeMap objects, and never on |this|.
  const EntryType entry_;

  // The map containing child ranges, keyed by each child range's high
  // address.  This is a pointer to avoid allocating map structures for
  // leaf nodes, where they are not needed.
  AddressToRangeMap* map_;

  // Whether or not we allow storing an entry into a range that equals to
  // existing range in the map. Default is false.
  // If this is true, the newly added range will become a child of existing
  // innermost range which has same base and size.
  bool allow_equal_range_;
};


}  // namespace google_breakpad


#endif  // PROCESSOR_CONTAINED_RANGE_MAP_H__
