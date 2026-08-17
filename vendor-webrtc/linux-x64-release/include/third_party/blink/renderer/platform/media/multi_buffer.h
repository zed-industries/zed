// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_H_

#include <stddef.h>
#include <stdint.h>

#include <functional>
#include <limits>
#include <map>
#include <memory>
#include <set>
#include <unordered_map>
#include <vector>

#include "base/containers/flat_map.h"
#include "base/containers/lru_cache.h"
#include "base/hash/hash.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "build/build_config.h"
#include "media/base/data_buffer.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/media/interval_map.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

// Used to identify a block of data in the multibuffer.
// Our blocks are 32kb (1 << 15), so our maximum cacheable file size
// is 1 << (15 + 31) = 64Tb
typedef int32_t MultiBufferBlockId;
class MultiBuffer;

// This type is used to identify a block in the LRU, which is shared between
// multibuffers.
typedef std::pair<MultiBuffer*, MultiBufferBlockId> MultiBufferGlobalBlockId;

}  // namespace blink

namespace std {

template <>
struct hash<blink::MultiBufferGlobalBlockId> {
  std::size_t operator()(const blink::MultiBufferGlobalBlockId& key) const {
    return base::HashInts(reinterpret_cast<uintptr_t>(key.first), key.second);
  }
};

}  // namespace std

namespace blink {

// Freeing a lot of blocks can be expensive, to keep thing
// flowing smoothly we only free a maximum of |kMaxFreesPerAdd|
// blocks when a new block is added to the cache.
const int kMaxFreesPerAdd = 10;

// There is a simple logic for creating, destroying and deferring
// data providers. Every data provider has a look-ahead region and
// a look-behind region. If there are readers in the look-ahead
// region, we keep reading. If not, but there are readers in the
// look-behind region, we defer. If there are no readers in either
// region, we destroy the data provider.

// When new readers are added, new data providers are created if
// the new reader doesn't fall into the look-ahead region of
// an existing data provider.

// This is the size of the look-ahead region.
const int kMaxWaitForWriterOffset = 5;

// This is the size of the look-behind region.
const int kMaxWaitForReaderOffset = 50;

// MultiBuffers are multi-reader multi-writer cache/buffers with
// prefetching and pinning. Data is stored internally in ref-counted
// blocks of identical size. |block_size_shift| is log2 of the block
// size.
//
// Users should inherit this class and implement CreateWriter().
// TODO(hubbe): Make the multibuffer respond to memory pressure.
class PLATFORM_EXPORT MultiBuffer {
 public:
  // Interface for clients wishing to read data out of this cache.
  // Note: It might look tempting to replace this with a callback,
  // but we keep and compare pointers to Readers internally.
  class Reader {
   public:
    Reader() = default;
    Reader(const Reader&) = delete;
    Reader& operator=(const Reader&) = delete;
    virtual ~Reader() = default;
    // Notifies the reader that the range of available blocks has changed.
    // The reader must call MultiBuffer::Observe() to activate this callback.
    virtual void NotifyAvailableRange(
        const Interval<MultiBufferBlockId>& range) = 0;
  };

  // DataProvider is the interface that MultiBuffer
  // uses to get data into the cache.
  class DataProvider {
   public:
    virtual ~DataProvider() = default;

    // Returns the block number that is to be returned
    // by the next Read() call.
    virtual MultiBufferBlockId Tell() const = 0;

    // Returns true if one (or more) blocks are
    // availble to read.
    virtual bool Available() const = 0;

    // Returns how many bytes are available, note that Available() may still
    // return false even if AvailableBytes() returns a value greater than
    // zero if less than a full block is available.
    virtual int64_t AvailableBytes() const = 0;

    // Returns the next block. Only valid if Available()
    // returns true. Last block might be of a smaller size
    // and after the last block we will get an end-of-stream
    // DataBuffer.
    virtual scoped_refptr<media::DataBuffer> Read() = 0;

    // Ask the data provider to stop giving us data.
    // It's ok if the effect is not immediate.
    virtual void SetDeferred(bool deferred) = 0;

    // Ask the provider if it has been deferred too long.
    virtual bool IsStale() const = 0;
  };

  // MultiBuffers use a global shared LRU to free memory.
  // This effectively means that recently used multibuffers can
  // borrow memory from less recently used ones.
  class PLATFORM_EXPORT GlobalLRU : public ThreadSafeRefCounted<GlobalLRU> {
   public:
    typedef MultiBufferGlobalBlockId GlobalBlockId;
    explicit GlobalLRU(scoped_refptr<base::SingleThreadTaskRunner> task_runner);
    GlobalLRU(const GlobalLRU&) = delete;
    GlobalLRU& operator=(const GlobalLRU&) = delete;

    // Free elements from cache if possible.
    // Don't free more than |max_to_free| blocks.
    void TryFree(int64_t max_to_free);

    // Free as much memory from the cache as possible.
    // Only used during critical memory pressure.
    void TryFreeAll();

    // Like TryFree, but only frees blocks if the data
    // number of the blocks in the cache is too large.
    void Prune(int64_t max_to_free);

    // Returns true if there are prunable blocks.
    bool Pruneable() const;

    // Incremnt the amount of data used by all multibuffers.
    void IncrementDataSize(int64_t blocks);

    // Each multibuffer is allowed a certain amount of memory,
    // that memory is registered by calling this function.
    // The memory is actually shared by all multibuffers.
    // When the total amount of memory used by all multibuffers
    // is greater than what has been registered here, we use the
    // LRU to decide what blocks to free first.
    void IncrementMaxSize(int64_t blocks);

    // LRU operations.
    void Use(MultiBuffer* multibuffer, MultiBufferBlockId id);
    void Remove(MultiBuffer* multibuffer, MultiBufferBlockId id);
    void Insert(MultiBuffer* multibuffer, MultiBufferBlockId id);
    bool Contains(MultiBuffer* multibuffer, MultiBufferBlockId id);
    int64_t Size() const;

   private:
    friend class ThreadSafeRefCounted<GlobalLRU>;
    ~GlobalLRU();

    // Schedule background pruning, if needed.
    void SchedulePrune();

    // Perform background pruning.
    void PruneTask();

    // Max number of blocks.
    int64_t max_size_;

    // Sum of all multibuffer::data_.size().
    int64_t data_size_;

    // True if there is a call to the background pruning outstanding.
    bool background_pruning_pending_;

    // The LRU should contain all blocks which are not pinned from
    // all multibuffers.
    base::HashingLRUCacheSet<MultiBufferGlobalBlockId> lru_;

    // Where we run our tasks.
    scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  };

  MultiBuffer(int32_t block_size_shift, scoped_refptr<GlobalLRU> global_lru);
  MultiBuffer(const MultiBuffer&) = delete;
  MultiBuffer& operator=(const MultiBuffer&) = delete;
  virtual ~MultiBuffer();

  // Identifies a block in the cache.
  // Block numbers can be calculated from byte positions as:
  // block_num = byte_pos >> block_size_shift
  typedef MultiBufferBlockId BlockId;
  typedef std::unordered_map<BlockId, scoped_refptr<media::DataBuffer>> DataMap;

  // Registers a reader at the given position.
  // If the cache does not already contain |pos|, it will activate
  // or create data providers to make sure that the block becomes
  // available soon. If |pos| is already in the cache, no action is
  // taken, it simply lets the cache know that this reader is likely
  // to read pos+1, pos+2.. soon.
  //
  // Registered readers will be notified when the available range
  // at their position changes. The available range at |pos| is a range
  // from A to B where: A <= |pos|, B >= |pos| and all blocks in [A..B)
  // are present in the cache.  When this changes, we will call
  // NotifyAvailableRange() on the reader.
  void AddReader(const BlockId& pos, Reader* reader);

  // Unregister a reader at block |pos|.
  // Often followed by a call to AddReader(pos + 1, ...);
  // Idempotent.
  void RemoveReader(const BlockId& pos, Reader* reader);

  // Immediately remove writers at or before |pos| if nobody needs them.
  // Note that we can't really do this in StopWaitFor(), because it's very
  // likely that StopWaitFor() is immediately followed by a call to WaitFor().
  // It is also a bad idea to wait for the writers to clean themselves up when
  // they try to provide unwanted data to the cache. Besides the obvoius
  // inefficiency, it will also cause the http_cache to bypass the disk/memory
  // cache if we have multiple simultaneous requests going against the same
  // url.
  void CleanupWriters(const BlockId& pos);

  // Returns true if block |pos| is available in the cache.
  bool Contains(const BlockId& pos) const;

  // Returns the next unavailable block at or after |pos|.
  BlockId FindNextUnavailable(const BlockId& pos) const;

  // Change the pin count for a range of data blocks.
  // Note that blocks do not have to be present in the
  // cache to be pinned.
  // Examples:
  // Pin block 3, 4 & 5: PinRange(3, 6, 1);
  // Unpin block 4 & 5: PinRange(4, 6, -1);
  void PinRange(const BlockId& from, const BlockId& to, int32_t how_much);

  // Calls PinRange for each range in |ranges|, convenience
  // function for applying multiple changes to the pinned ranges.
  void PinRanges(const IntervalMap<BlockId, int32_t>& ranges);

  // Returns a continous (but possibly empty) list of blocks starting at
  // |from| up to, but not including |to|. This function is thread safe.
  void GetBlocksThreadsafe(
      const BlockId& from,
      const BlockId& to,
      std::vector<scoped_refptr<media::DataBuffer>>* output);

  // Increment max cache size by |size| (counted in blocks).
  void IncrementMaxSize(int64_t size);

  // Returns how many bytes have been received by the data providers at position
  // |block|, which have not yet been submitted to the multibuffer cache.
  // The returned number should be less than the size of one block.
  int64_t UncommittedBytesAt(const BlockId& block);

  // Caller takes ownership of 'provider', cache will
  // not call it anymore.
  std::unique_ptr<DataProvider> RemoveProvider(DataProvider* provider);

  // Add a writer to this cache. Cache takes ownership, and may
  // destroy |provider| later. (Not during this call.)
  void AddProvider(std::unique_ptr<DataProvider> provider);

  // Transfer all data from |other| to this.
  void MergeFrom(MultiBuffer* other);

  // Accessors.
  const DataMap& map() const { return data_; }
  int32_t block_size_shift() const { return block_size_shift_; }

  // Setters.
  void SetIsClientAudioElement(bool is_client_audio_element) {
    is_client_audio_element_ = is_client_audio_element;
  }

  // Callback which notifies us that a data provider has
  // some data for us. Also called when it might be appropriate
  // for a provider in a deferred state to wake up.
  void OnDataProviderEvent(DataProvider* provider);

  size_t writer_index_size_for_testing() const { return writer_index_.size(); }

 protected:
  // Create a new writer at |pos| and return it.
  // Users needs to implemement this method.
  virtual std::unique_ptr<DataProvider> CreateWriter(
      const BlockId& pos,
      bool is_client_audio_element) = 0;

  virtual bool RangeSupported() const = 0;

  // Called when the cache becomes empty. Implementations can use this
  // as a signal for when we should free this object and any metadata
  // that goes with it.
  virtual void OnEmpty();

 private:
  // For testing.
  friend class TestMultiBuffer;

  enum ProviderState {
    ProviderStateDead,
    ProviderStateDefer,
    ProviderStateLoad
  };

  // Can be overriden for testing.
  virtual void Prune(size_t max_to_free);

  // Remove the given blocks from the multibuffer, called from
  // GlobalLRU::Prune().
  void ReleaseBlocks(const std::vector<MultiBufferBlockId>& blocks);

  // Figure out what state a writer at |pos| should be in.
  ProviderState SuggestProviderState(const BlockId& pos, bool is_stale) const;

  // Returns true if a writer at |pos| is colliding with
  // output of another writer.
  bool ProviderCollision(const BlockId& pos) const;

  // Call NotifyAvailableRange(new_range) on all readers waiting
  // for a block in |observer_range|
  void NotifyAvailableRange(const Interval<MultiBufferBlockId>& observer_range,
                            const Interval<MultiBufferBlockId>& new_range);

  // Max number of blocks.
  int64_t max_size_;

  // log2 of block size.
  int32_t block_size_shift_;

  // Is the client an audio element?
  bool is_client_audio_element_ = false;

  // Stores the actual data.
  DataMap data_ ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/40760651)");

  // protects data_
  // Note that because data_ is only modified on the a single thread,
  // we don't need to lock this if we're reading data from the same thread.
  // Instead, we only lock this when:
  //   * modifying data_
  //   * reading data_ from another thread
  base::Lock data_lock_;

  // Keeps track of readers waiting for data.
  base::flat_map<MultiBufferBlockId, std::set<raw_ptr<Reader, SetExperimental>>>
      readers_;

  // Keeps track of writers by their position.
  // The writers are owned by this class.
  base::flat_map<BlockId, std::unique_ptr<DataProvider>> writer_index_;

  // Gloabally shared LRU, decides which block to free next.
  scoped_refptr<GlobalLRU> lru_;

  // Keeps track of what blocks are pinned. If block p is pinned,
  // then pinned_[p] > 0. Pinned blocks cannot be freed and should not
  // be present in |lru_|.
  IntervalMap<BlockId, int32_t> pinned_;

  // present_[block] should be 1 for all blocks that are present
  // and 0 for all blocks that are not. Used to quickly figure out
  // ranges of available/unavailable blocks without iterating.
  IntervalMap<BlockId, int32_t> present_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_H_
