// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_PARCEL_H_
#define IPCZ_SRC_IPCZ_PARCEL_H_

#include <algorithm>
#include <atomic>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <variant>
#include <vector>

#include "ipcz/api_object.h"
#include "ipcz/fragment.h"
#include "ipcz/ipcz.h"
#include "ipcz/message.h"
#include "ipcz/node_link.h"
#include "ipcz/node_link_memory.h"
#include "ipcz/sequence_number.h"
#include "third_party/abseil-cpp/absl/base/macros.h"
#include "third_party/abseil-cpp/absl/container/inlined_vector.h"
#include "third_party/abseil-cpp/absl/types/span.h"
#include "util/ref_counted.h"

namespace ipcz {

class NodeLink;
class NodeLinkMemory;

// Represents a parcel queued within a portal, either for inbound retrieval or
// outgoing transfer.
class Parcel {
 public:
  // Arbitrary hard cap on the number of subparcels which can be embedded within
  // a parcel, to mitigate the potential for abuse.
  static constexpr size_t kMaxSubparcelsPerParcel = 1024;

  Parcel();
  explicit Parcel(SequenceNumber sequence_number);
  Parcel(const Parcel& other) = delete;
  Parcel& operator=(const Parcel& other) = delete;
  ~Parcel();

  void set_sequence_number(SequenceNumber n) { sequence_number_ = n; }
  SequenceNumber sequence_number() const { return sequence_number_; }

  void set_num_subparcels(size_t num_subparcels) {
    num_subparcels_ = num_subparcels;
  }
  size_t num_subparcels() const { return num_subparcels_; }

  void set_subparcel_index(size_t index) { subparcel_index_ = index; }
  size_t subparcel_index() const { return subparcel_index_; }

  // Indicates whether this Parcel is empty, meaning its data and objects have
  // been fully consumed.
  bool empty() const { return data_view().empty() && objects_view().empty(); }

  // Sets this Parcel's data to the contents of `data_view`, backed by a subset
  // of the memory within `buffer`. Any prior data in the Parcel is discarded.
  void SetDataFromMessage(Message::ReceivedDataBuffer buffer,
                          absl::Span<uint8_t> data_view);

  // Attaches the given set of `objects` to this Parcel.
  void SetObjects(std::vector<Ref<APIObject>> objects);

  // Allocates `num_bytes` of storage for this Parcel's data. If `memory` is
  // non-null then its fragment pool is the preferred allocation source.
  // Otherwise memory is allocated and zero-initialized on the heap, and the
  // data placed therein will be inlined within any message that transmits this
  // parcel.
  //
  // If `memory` is non-null and `allow_partial` is true, this may allocate less
  // memory than requested if some reasonable amount of space is still available
  // within `memory`.
  //
  // Upon return, data_view() references the allocated memory wherever it
  // resides.
  //
  // If the Parcel had any data attached prior to this call, it is discarded and
  // replaced with the newly allocated storage.
  void AllocateData(size_t num_bytes,
                    bool allow_partial,
                    NodeLinkMemory* memory);

  // Configures this Parcel to adopt its data fragment from the `fragment`
  // belonging to `memory`. `fragment` must be addressable and must have a valid
  // FragmentHeader at the start describing the data which follows. Otherwise
  // this returns false.
  bool AdoptDataFragment(Ref<NodeLinkMemory> memory, const Fragment& fragment);

  void set_remote_source(Ref<NodeLink> source) {
    remote_source_ = std::move(source);
  }
  const Ref<NodeLink>& remote_source() const { return remote_source_; }

  absl::Span<uint8_t> data_view() { return data_.view; }
  absl::Span<const uint8_t> data_view() const { return data_.view; }

  size_t data_size() const { return data_view().size(); }

  bool has_data_fragment() const {
    return std::holds_alternative<DataFragment>(data_.storage);
  }
  const Fragment& data_fragment() const {
    ABSL_ASSERT(has_data_fragment());
    return std::get<DataFragment>(data_.storage).fragment();
  }
  const Ref<NodeLinkMemory>& data_fragment_memory() const {
    ABSL_ASSERT(has_data_fragment());
    return std::get<DataFragment>(data_.storage).memory();
  }

  absl::Span<Ref<APIObject>> objects_view() const { return objects_.view; }

  size_t num_objects() const { return objects_view().size(); }

  // Commits `num_bytes` of data to this Parcel's data fragment. This MUST be
  // called after populating the Parcel's data, and it must be called by the
  // same thread that populated the data. If the parcel's data is inlined rather
  // than stored in a fragment, this is a no-op.
  void CommitData(size_t num_bytes);

  // Relinquishes ownership of this Parcel's data fragment, if applicable. This
  // prevents the fragment from being freed upon Parcel destruction.
  void ReleaseDataFragment();

  // Filling `out_handles` (of size N) with handles to the first N APIObjects in
  // objects_view(). The front of objects_view() is also advanced by N,
  // effectively removing the objects from this parcel.
  //
  // Note that the size of `out_handles` must not be larger than the size of
  // objects_view().
  void ConsumeHandles(absl::Span<IpczHandle> out_handles);

  void SetEnvelope(DriverObject envelope);

  // Produces a log-friendly description of the Parcel, useful for various
  // debugging log messages.
  std::string Describe() const;

 private:
  // When Parcel data is in a shared memory fragment, this header sits at the
  // front of the fragment.
  struct IPCZ_ALIGN(8) FragmentHeader {
    // The size in bytes of the parcel data which immediately follows this
    // header. Must not extend beyond the bounds of the fragment itself.
    //
    // Access to this atomic is also used to synchronize access to the parcel
    // data. Writes to the fragment must be finalized by calling CommitData(),
    // and any node receiving this parcel must adopt the fragment by calling
    // AdoptDataFragment().
    std::atomic<uint32_t> size;

    // Reserved padding for 8-byte parcel data alignment.
    std::atomic<uint32_t> reserved;
  };

  // Holds a Fragment and a reference to its backing memory. Used when a
  // Parcel's data lives in shared memory. This implements exclusive ownership
  // of the underlying fragment, and upon destruction it releases the fragment
  // back into the memory pool.
  class DataFragment {
   public:
    DataFragment() = default;
    DataFragment(Ref<NodeLinkMemory> memory, const Fragment& fragment)
        : memory_(std::move(memory)), fragment_(fragment) {
      // Parcels can only be given data fragments which are already addressable.
      ABSL_ASSERT(is_valid());
    }
    DataFragment(const DataFragment& other) = delete;
    DataFragment& operator=(const DataFragment& other) = delete;
    ~DataFragment();

    bool is_valid() const { return memory_ && fragment_.is_addressable(); }

    const Fragment& fragment() const { return fragment_; }
    const Ref<NodeLinkMemory>& memory() const { return memory_; }

    [[nodiscard]] Fragment release();
    void reset();

   private:
    Ref<NodeLinkMemory> memory_;
    Fragment fragment_;
  };

  // A variant backing type for the parcel's data. Data may be in shared memory,
  // heap-allocated and initialized from within the Parcel, or heap-allocated by
  // a received Message and moved into the Parcel from there.
  using DataStorage = std::variant<std::monostate,
                                   DataFragment,
                                   std::vector<uint8_t>,
                                   Message::ReceivedDataBuffer>;

  // Groups a DataStorage with a view into its data. This defines its own move
  // construction and assignment operators to ensure that moved-from data is
  // cleared. Note that the view may reference only a subset of the data within
  // the DataStorage. This subset is considered the Parcel's data.
  struct DataStorageWithView {
    DataStorageWithView() = default;
    DataStorageWithView(const DataStorageWithView& other) = delete;
    DataStorageWithView& operator=(const DataStorageWithView& other) = delete;
    ~DataStorageWithView() = default;

    DataStorage storage;
    absl::Span<uint8_t> view;
  };

  // Groups a vector of object attachments along with a view into that vector.
  // Note that the view may reference only a subset of the elements within the
  // vector if some objects have been removed or not-yet-attached.
  struct ObjectStorageWithView {
    ObjectStorageWithView() = default;
    ObjectStorageWithView(const ObjectStorageWithView&) = delete;
    ObjectStorageWithView& operator=(const ObjectStorageWithView&) = delete;
    ~ObjectStorageWithView() = default;

    std::vector<Ref<APIObject>> storage;
    absl::Span<Ref<APIObject>> view;
  };

  SequenceNumber sequence_number_{0};

  // If this Parcel was received from a remote node, this tracks the NodeLink
  // which received it.
  Ref<NodeLink> remote_source_;

  // Concrete storage for the parcel's data, along with a view the data not yet
  // consumed.
  DataStorageWithView data_;

  // The set of APIObjects attached to this parcel, and a view of the objects
  // not yet consumed from it.
  ObjectStorageWithView objects_;

  // By default, all parcels have a single subparcel (theirself) at index 0. On
  // any Parcel that exists as a subparcel of another, these fields will be
  // updated by the containing Parcel as needed.
  size_t num_subparcels_ = 1;
  size_t subparcel_index_ = 0;

  DriverObject envelope_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_PARCEL_H_
