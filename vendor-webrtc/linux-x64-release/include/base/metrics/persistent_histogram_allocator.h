// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_PERSISTENT_HISTOGRAM_ALLOCATOR_H_
#define BASE_METRICS_PERSISTENT_HISTOGRAM_ALLOCATOR_H_

#include <atomic>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"
#include "base/metrics/histogram_base.h"
#include "base/metrics/persistent_memory_allocator.h"
#include "base/metrics/ranges_manager.h"
#include "base/process/process_handle.h"
#include "base/synchronization/lock.h"
#include "build/build_config.h"

namespace base {

class BucketRanges;
class FilePath;
class PersistentSampleMapRecords;
class PersistentSparseHistogramDataManager;
class UnsafeSharedMemoryRegion;

// A data manager for sparse histograms so each instance of such doesn't have
// to separately iterate over the entire memory segment.
class BASE_EXPORT PersistentSparseHistogramDataManager {
 public:
  // Constructs the data manager. The allocator must live longer than any
  // managers that reference it.
  explicit PersistentSparseHistogramDataManager(
      PersistentMemoryAllocator* allocator);

  PersistentSparseHistogramDataManager(
      const PersistentSparseHistogramDataManager&) = delete;
  PersistentSparseHistogramDataManager& operator=(
      const PersistentSparseHistogramDataManager&) = delete;

  ~PersistentSparseHistogramDataManager();

  // Returns an object that manages persistent-sample-map records for a given
  // `id`. The returned object queries `this` for records. Hence, the returned
  // object must not outlive `this`.
  std::unique_ptr<PersistentSampleMapRecords> CreateSampleMapRecords(
      uint64_t id);

  // Convenience method that gets the object for a given reference so callers
  // don't have to also keep their own pointer to the appropriate allocator.
  template <typename T>
  T* GetAsObject(PersistentMemoryAllocator::Reference ref) {
    return allocator_->GetAsObject<T>(ref);
  }

 private:
  friend class PersistentSampleMapRecords;

  struct ReferenceAndSample {
    PersistentMemoryAllocator::Reference reference;
    HistogramBase::Sample32 value;
  };

  // Gets the vector holding records for a given sample-map id.
  std::vector<ReferenceAndSample>* GetSampleMapRecordsWhileLocked(uint64_t id)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Returns sample-map records belonging to the specified `sample_map_records`.
  // Only records found that were not yet seen by `sample_map_records` will be
  // returned, determined by its `seen_` field. Records found for other
  // sample-maps are held for later use without having to iterate again. This
  // should be called only from a PersistentSampleMapRecords object because
  // those objects have a contract that there are no other threads accessing the
  // internal records_ field of the object that is passed in. If `until_value`
  // is set and a sample is found with said value, the search will stop early
  // and the last entry in the returned vector will be that sample.
  // Note: The returned vector is not guaranteed to contain all unseen records
  // for `sample_map_records`. If this is needed, then repeatedly call this
  // until an empty vector is returned, which definitely means that
  // `sample_map_records` has seen all its records.
  std::vector<PersistentMemoryAllocator::Reference> LoadRecords(
      PersistentSampleMapRecords* sample_map_records,
      std::optional<HistogramBase::Sample32> until_value);

  // Weak-pointer to the allocator used by the sparse histograms.
  raw_ptr<PersistentMemoryAllocator> allocator_;

  // Iterator within the allocator for finding sample records.
  PersistentMemoryAllocator::Iterator record_iterator_ GUARDED_BY(lock_);

  // Mapping of sample-map IDs to their sample records.
  std::map<uint64_t, std::unique_ptr<std::vector<ReferenceAndSample>>>
      sample_records_ GUARDED_BY(lock_);

  base::Lock lock_;
};

// This class manages sample-records used by a PersistentSampleMap container
// that underlies a persistent SparseHistogram object. It is broken out into a
// top-level class so that it can be forward-declared in other header files
// rather than include this entire file as would be necessary if it were
// declared within the PersistentSparseHistogramDataManager class above.
class BASE_EXPORT PersistentSampleMapRecords {
 public:
  // Constructs an instance of this class. The manager object must live longer
  // than all instances of this class that reference it, which is not usually
  // a problem since these objects are generally managed from within that
  // manager instance. The same caveats apply for for the `records` vector.
  PersistentSampleMapRecords(
      PersistentSparseHistogramDataManager* data_manager,
      uint64_t sample_map_id,
      std::vector<PersistentSparseHistogramDataManager::ReferenceAndSample>*
          records);

  PersistentSampleMapRecords(const PersistentSampleMapRecords&) = delete;
  PersistentSampleMapRecords& operator=(const PersistentSampleMapRecords&) =
      delete;

  ~PersistentSampleMapRecords();

  // Gets next references to persistent sample-map records. If `until_value` is
  // passed, and said value is found, then it will be the last element in the
  // returned vector. The type and layout of the data being referenced is
  // defined entirely within the PersistentSampleMap class.
  // Note: The returned vector is not guaranteed to contain all unseen records
  // for `this`. If this is needed, then repeatedly call this until an empty
  // vector is returned, which definitely means that `this` has seen all its
  // records.
  std::vector<PersistentMemoryAllocator::Reference> GetNextRecords(
      std::optional<HistogramBase::Sample32> until_value);

  // Creates a new persistent sample-map record for sample `value` and returns
  // a reference to it.
  PersistentMemoryAllocator::Reference CreateNew(HistogramBase::Sample32 value);

  // Convenience method that gets the object for a given reference so callers
  // don't have to also keep their own pointer to the appropriate allocator.
  // This is expected to be used with the SampleRecord structure defined inside
  // the persistent_sample_map.cc file but since that isn't exported (for
  // cleanliness of the interface), a template is defined that will be
  // resolved when used inside that file.
  template <typename T>
  T* GetAsObject(PersistentMemoryAllocator::Reference ref) {
    return data_manager_->GetAsObject<T>(ref);
  }

 private:
  friend PersistentSparseHistogramDataManager;

  // Weak-pointer to the parent data-manager object.
  raw_ptr<PersistentSparseHistogramDataManager> data_manager_;

  // ID of PersistentSampleMap to which these records apply.
  const uint64_t sample_map_id_;

  // This is the count of how many "records" have already been read by `this`.
  size_t seen_ = 0;

  // This is the set of records found during iteration through memory, owned by
  // the parent manager. When GetNextRecords() is called, this is looked up to
  // find new references. Access to this vector should only be done while
  // holding the parent manager's lock.
  raw_ptr<std::vector<PersistentSparseHistogramDataManager::ReferenceAndSample>>
      records_;
};

// This class manages histograms created within a PersistentMemoryAllocator.
class BASE_EXPORT PersistentHistogramAllocator {
 public:
  // A reference to a histogram. While this is implemented as PMA::Reference,
  // it is not conceptually the same thing. Outside callers should always use
  // a Reference matching the class it is for and not mix the two.
  using Reference = PersistentMemoryAllocator::Reference;

  // Iterator used for fetching persistent histograms from an allocator.
  // It is lock-free and thread-safe.
  // See PersistentMemoryAllocator::Iterator for more information.
  class BASE_EXPORT Iterator {
   public:
    // Constructs an iterator on a given `allocator`, starting at the beginning.
    // The allocator must live beyond the lifetime of the iterator.
    explicit Iterator(PersistentHistogramAllocator* allocator);

    Iterator(const Iterator&) = delete;
    Iterator& operator=(const Iterator&) = delete;

    // Gets the next histogram from persistent memory; returns null if there
    // are no more histograms to be found. This may still be called again
    // later to retrieve any new histograms added in the meantime.
    std::unique_ptr<HistogramBase> GetNext() { return GetNextWithIgnore(0); }

    // Gets the next histogram from persistent memory, ignoring one particular
    // reference in the process. Pass `ignore` of zero (0) to ignore nothing.
    std::unique_ptr<HistogramBase> GetNextWithIgnore(Reference ignore);

   private:
    // Weak-pointer to histogram allocator being iterated over.
    raw_ptr<PersistentHistogramAllocator> allocator_;

    // The iterator used for stepping through objects in persistent memory.
    // It is lock-free and thread-safe which is why this class is also such.
    PersistentMemoryAllocator::Iterator memory_iter_;
  };

  // A PersistentHistogramAllocator is constructed from a PersistentMemory-
  // Allocator object of which it takes ownership.
  explicit PersistentHistogramAllocator(
      std::unique_ptr<PersistentMemoryAllocator> memory);

  PersistentHistogramAllocator(const PersistentHistogramAllocator&) = delete;
  PersistentHistogramAllocator& operator=(const PersistentHistogramAllocator&) =
      delete;

  virtual ~PersistentHistogramAllocator();

  // Direct access to underlying memory allocator. If the segment is shared
  // across threads or processes, reading data through these values does
  // not guarantee consistency. Use with care. Do not write.
  PersistentMemoryAllocator* memory_allocator() {
    return memory_allocator_.get();
  }

  // Implement the "metadata" API of a PersistentMemoryAllocator, forwarding
  // those requests to the real one.
  uint64_t Id() const { return memory_allocator_->Id(); }
  const void* data() const { return memory_allocator_->data(); }
  size_t length() const { return memory_allocator_->length(); }
  size_t size() const { return memory_allocator_->size(); }
  size_t used() const { return memory_allocator_->used(); }

  // Returns the internal name of this allocator (possibly an empty string).
  // The returned string_view references a bounded span within the shared
  // memory region. As such, it should be treated as a volatile but bounded
  // block of memory. In particular, clients should respect the 'length()' of
  // the returned view instead of relying on a terminating NUL char.
  std::string_view Name() const { return memory_allocator_->Name(); }

  // Recreate a Histogram from data held in persistent memory. Though this
  // object will be local to the current process, the sample data will be
  // shared with all other threads referencing it. This method takes a `ref`
  // to where the top-level histogram data may be found in this allocator.
  // This method will return null if any problem is detected with the data.
  std::unique_ptr<HistogramBase> GetHistogram(Reference ref);

  // Allocates a new persistent histogram. The returned histogram will not
  // be able to be located by other allocators until it is "finalized".
  std::unique_ptr<HistogramBase> AllocateHistogram(
      HistogramType histogram_type,
      std::string_view name,
      int minimum,
      int maximum,
      const BucketRanges* bucket_ranges,
      int32_t flags,
      Reference* ref_ptr);

  // Finalize the creation of the histogram, making it available to other
  // processes if `registered` (as in: added to the StatisticsRecorder) is
  // True, forgetting it otherwise.
  void FinalizeHistogram(Reference ref, bool registered);

  // Merges the data in a persistent histogram with one held globally by the
  // StatisticsRecorder, updating the "logged" samples within the passed
  // object so that repeated merges are allowed. Don't call this on a "global"
  // allocator because histograms created there will already be in the SR.
  // Returns whether the merge was successful; if false, the histogram did not
  // have the same shape (different types or buckets), or we couldn't get a
  // target histogram from the statistic recorder.
  bool MergeHistogramDeltaToStatisticsRecorder(HistogramBase* histogram);

  // As above but merge the "final" delta. No update of "logged" samples is
  // done which means it can operate on read-only objects. It's essential,
  // however, not to call this more than once or those final samples will
  // get recorded again. Returns whether the merge was successful; if false, the
  // histogram did not have the same shape (different types or buckets), or we
  // couldn't get a target histogram from the statistic recorder.
  bool MergeHistogramFinalDeltaToStatisticsRecorder(
      const HistogramBase* histogram);

  // Returns an object that manages persistent-sample-map records for a given
  // `id`. The returned object queries `sparse_histogram_data_manager_` for
  // records. Hence, the returned object must not outlive
  // `sparse_histogram_data_manager_` (and hence `this`).
  std::unique_ptr<PersistentSampleMapRecords> CreateSampleMapRecords(
      uint64_t id);

  // Creates internal histograms for tracking memory use and allocation sizes
  // for allocator of `name` (which can simply be the result of Name()). This
  // is done separately from construction for situations such as when the
  // histograms will be backed by memory provided by this very allocator.
  //
  // IMPORTANT: tools/metrics/histograms/metadata/uma/histograms.xml must
  // be updated with the following histograms for each `name` param:
  //    UMA.PersistentAllocator.name.UsedPct
  void CreateTrackingHistograms(std::string_view name);
  void UpdateTrackingHistograms();

  // Sets the internal `ranges_manager_`, which will be used by the allocator to
  // register BucketRanges. Takes ownership of the passed `ranges_manager`.
  //
  // WARNING: Since histograms may be created from `this` from multiple threads,
  // for example through a direct call to CreateHistogram(), or while iterating
  // through `this`, then the passed manager may also be accessed concurrently.
  // Hence, care must be taken to ensure that either:
  //   1) The passed manager is threadsafe (see ThreadSafeRangesManager), or
  //   2) `this` is not used concurrently.
  void SetRangesManager(RangesManager* ranges_manager);

  // Clears the internal `last_created_` reference so testing can validate
  // operation without that optimization.
  void ClearLastCreatedReferenceForTesting();

 protected:
  // The structure used to hold histogram data in persistent memory. It is
  // defined and used entirely within the .cc file.
  struct PersistentHistogramData;

  // Gets the reference of the last histogram created, used to avoid
  // trying to import what was just created.
  Reference last_created() {
    return last_created_.load(std::memory_order_relaxed);
  }

  // Gets the next histogram in persistent data based on iterator while
  // ignoring a particular reference if it is found.
  std::unique_ptr<HistogramBase> GetNextHistogramWithIgnore(Iterator* iter,
                                                            Reference ignore);

 private:
  // Create a histogram based on saved (persistent) information about it.
  // `histogram_data_ptr` refers to a variable length structure, ending
  // with the histogram name. `durable_name` must be a durable string view
  // starting at `histogram_data_ptr->name` and with some length bounded by the
  // `alloc_size` returned from the `memory_allocator`.
  std::unique_ptr<HistogramBase> CreateHistogram(
      PersistentHistogramData* histogram_data_ptr,
      DurableStringView durable_name);

  // Gets or creates an object in the global StatisticsRecorder matching
  // the `histogram` passed. Null is returned if one was not found and
  // one could not be created.
  HistogramBase* GetOrCreateStatisticsRecorderHistogram(
      const HistogramBase* histogram);

  // The memory allocator that provides the actual histogram storage.
  std::unique_ptr<PersistentMemoryAllocator> memory_allocator_;

  // The RangesManager that the allocator will register its BucketRanges with.
  // If this is null (default), the BucketRanges will be registered with the
  // global statistics recorder. Used when loading self-contained metrics coming
  // from a previous session. Registering the BucketRanges with the global
  // statistics recorder could create unnecessary contention, and a low amount
  // of extra memory.
  std::unique_ptr<base::RangesManager> ranges_manager_;

  // The data-manager used to improve performance of sparse histograms.
  PersistentSparseHistogramDataManager sparse_histogram_data_manager_;

  // A reference to the last-created histogram in the allocator, used to avoid
  // trying to import what was just created.
  std::atomic<Reference> last_created_ = 0;
};

// A special case of the PersistentHistogramAllocator that operates on a
// global scale, collecting histograms created through standard macros and
// the FactoryGet() method.
class BASE_EXPORT GlobalHistogramAllocator
    : public PersistentHistogramAllocator {
 public:
  GlobalHistogramAllocator(const GlobalHistogramAllocator&) = delete;
  GlobalHistogramAllocator& operator=(const GlobalHistogramAllocator&) = delete;

  ~GlobalHistogramAllocator() override;

  // Create a global allocator using the passed-in memory `base`, `size`, and
  // other parameters. Ownership of the memory segment remains with the caller.
  static void CreateWithPersistentMemory(void* base,
                                         size_t size,
                                         size_t page_size,
                                         uint64_t id,
                                         std::string_view name);

  // Create a global allocator using an internal block of memory of the
  // specified `size` taken from the heap.
  static void CreateWithLocalMemory(size_t size,
                                    uint64_t id,
                                    std::string_view name);

#if !BUILDFLAG(IS_NACL)
  // Create a global allocator by memory-mapping a `file`. If the file does
  // not exist, it will be created with the specified `size`. If the file does
  // exist, the allocator will use and add to its contents, ignoring the passed
  // size in favor of the existing size. Returns whether the global allocator
  // was set. If `exclusive_write` is true, the file will be opened in a mode
  // that disallows multiple concurrent writers (no effect on non-Windows).
  static bool CreateWithFile(const FilePath& file_path,
                             size_t size,
                             uint64_t id,
                             std::string_view name,
                             bool exclusive_write = false);

  // Creates a new file at `active_path`. If it already exists, it will first be
  // moved to `base_path`. In all cases, any old file at `base_path` will be
  // removed. If `spare_path` is non-empty and exists, that will be renamed and
  // used as the active file. Otherwise, the file will be created using the
  // given size, id, and name. Returns whether the global allocator was set.
  static bool CreateWithActiveFile(const FilePath& base_path,
                                   const FilePath& active_path,
                                   const FilePath& spare_path,
                                   size_t size,
                                   uint64_t id,
                                   std::string_view name);

  // Uses ConstructBaseActivePairFilePaths() to build a pair of file names which
  // are then used for CreateWithActiveFile(). `name` is used for both the
  // internal name for the allocator and also for the name of the file inside
  // `dir`.
  static bool CreateWithActiveFileInDir(const FilePath& dir,
                                        size_t size,
                                        uint64_t id,
                                        std::string_view name);

  // Constructs a filename using a name.
  static FilePath ConstructFilePath(const FilePath& dir, std::string_view name);

  // Constructs a filename using a name for an "active" file.
  static FilePath ConstructFilePathForActiveFile(const FilePath& dir,
                                                 std::string_view name);

  // Like above but with timestamp and pid for use in upload directories.
  static FilePath ConstructFilePathForUploadDir(const FilePath& dir,
                                                std::string_view name,
                                                base::Time stamp,
                                                ProcessId pid);

  // Override that uses the current time stamp and current process id.
  static FilePath ConstructFilePathForUploadDir(const FilePath& dir,
                                                std::string_view name);

  // Parses a filename to extract name, timestamp, and pid.
  static bool ParseFilePath(const FilePath& path,
                            std::string* out_name,
                            Time* out_stamp,
                            ProcessId* out_pid);

  // Create a "spare" file that can later be made the "active" file. This
  // should be done on a background thread if possible.
  static bool CreateSpareFile(const FilePath& spare_path, size_t size);
#endif

  // Create a global allocator using a block of shared memory accessed
  // through the given `region`. The allocator maps the shared memory into
  // current process's virtual address space and frees it upon destruction.
  // The memory will continue to live if other processes have access to it.
  static void CreateWithSharedMemoryRegion(
      const UnsafeSharedMemoryRegion& region);

  // Sets a GlobalHistogramAllocator for globally storing histograms in
  // a space that can be persisted or shared between processes. There is only
  // ever one allocator for all such histograms created by a single process.
  // This takes ownership of the object and should be called as soon as
  // possible during startup to capture as many histograms as possible and
  // while operating single-threaded so there are no race-conditions. Note that
  // the `allocator` will never be destroyed including tests.
  static void Set(GlobalHistogramAllocator* allocator);

  // Gets a pointer to the global histogram allocator. Returns null if none
  // exists.
  static GlobalHistogramAllocator* Get();

  // This access to the persistent allocator is only for testing; it extracts
  // the current allocator completely. This allows easy creation of histograms
  // within persistent memory segments which can then be extracted and used in
  // other ways. Do not destroy the returned allocator since already created
  // histograms may still keep pointers to allocated memory.
  static GlobalHistogramAllocator* ReleaseForTesting();

  // Stores a pathname to which the contents of this allocator should be saved
  // in order to persist the data for a later use.
  void SetPersistentLocation(const FilePath& location);

  // Retrieves a previously set pathname to which the contents of this allocator
  // are to be saved.
  const FilePath& GetPersistentLocation() const LIFETIME_BOUND;

  // Returns whether the contents of this allocator are being saved to a
  // persistent file on disk.
  bool HasPersistentLocation() const;

  // Moves the file being used to persist this allocator's data to the directory
  // specified by `dir`. Returns whether the operation was successful.
  bool MovePersistentFile(const FilePath& dir);

  // Writes the internal data to a previously set location. This is generally
  // called when a process is exiting from a section of code that may not know
  // the filesystem. The data is written in an atomic manner. The return value
  // indicates success.
  bool WriteToPersistentLocation();

  // If there is a global metrics file being updated on disk, mark it to be
  // deleted when the process exits.
  void DeletePersistentLocation();

 private:
  friend class StatisticsRecorder;

  // Creates a new global histogram allocator.
  explicit GlobalHistogramAllocator(
      std::unique_ptr<PersistentMemoryAllocator> memory);

  // Import new histograms from the global histogram allocator. It's possible
  // for other processes to create histograms in the active memory segment;
  // this adds those to the internal list of known histograms to avoid creating
  // duplicates that would have to be merged during reporting. Every call to
  // this method resumes from the last entry it saw; it costs nothing if
  // nothing new has been added.
  void ImportHistogramsToStatisticsRecorder();

  // Builds a FilePath for a metrics file.
  static FilePath MakeMetricsFilePath(const FilePath& dir,
                                      std::string_view name);

  // Import always continues from where it left off, making use of a single
  // iterator to continue the work.
  Iterator import_iterator_;

  // The location to which the data should be persisted.
  FilePath persistent_location_;
};

}  // namespace base

#endif  // BASE_METRICS_PERSISTENT_HISTOGRAM_ALLOCATOR_H_
