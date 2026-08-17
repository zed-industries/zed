// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_HISTOGRAM_BASE_H_
#define BASE_METRICS_HISTOGRAM_BASE_H_

#include <limits.h>
#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <memory>
#include <string>
#include <string_view>

#include "base/atomicops.h"
#include "base/base_export.h"
#include "base/strings/durable_string_view.h"
#include "base/time/time.h"
#include "base/values.h"

namespace base {

class Value;
class HistogramBase;
class HistogramSamples;
class Pickle;
class PickleIterator;

////////////////////////////////////////////////////////////////////////////////
// This enum is used to facilitate deserialization of histograms from other
// processes into the browser. If you create another class that inherits from
// HistogramBase, add new histogram types and names below.

enum HistogramType {
  HISTOGRAM,
  LINEAR_HISTOGRAM,
  BOOLEAN_HISTOGRAM,
  CUSTOM_HISTOGRAM,
  SPARSE_HISTOGRAM,
  DUMMY_HISTOGRAM,
};

// Controls the verbosity of the information when the histogram is serialized to
// a JSON.
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base.metrics
enum JSONVerbosityLevel {
  // The histogram is completely serialized.
  JSON_VERBOSITY_LEVEL_FULL,
  // The bucket information is not serialized.
  JSON_VERBOSITY_LEVEL_OMIT_BUCKETS,
};

std::string HistogramTypeToString(HistogramType type);

// This enum is used for reporting how many histograms and of what types and
// variations are being created. It has to be in the main .h file so it is
// visible to files that define the various histogram types.
enum HistogramReport {
  // Count the number of reports created. The other counts divided by this
  // number will give the average per run of the program.
  HISTOGRAM_REPORT_CREATED = 0,

  // Count the total number of histograms created. It is the limit against
  // which all others are compared.
  HISTOGRAM_REPORT_HISTOGRAM_CREATED = 1,

  // Count the total number of histograms looked-up. It's better to cache
  // the result of a single lookup rather than do it repeatedly.
  HISTOGRAM_REPORT_HISTOGRAM_LOOKUP = 2,

  // These count the individual histogram types. This must follow the order
  // of HistogramType above.
  HISTOGRAM_REPORT_TYPE_LOGARITHMIC = 3,
  HISTOGRAM_REPORT_TYPE_LINEAR = 4,
  HISTOGRAM_REPORT_TYPE_BOOLEAN = 5,
  HISTOGRAM_REPORT_TYPE_CUSTOM = 6,
  HISTOGRAM_REPORT_TYPE_SPARSE = 7,

  // These indicate the individual flags that were set.
  HISTOGRAM_REPORT_FLAG_UMA_TARGETED = 8,
  HISTOGRAM_REPORT_FLAG_UMA_STABILITY = 9,
  HISTOGRAM_REPORT_FLAG_PERSISTENT = 10,

  // This must be last.
  HISTOGRAM_REPORT_MAX = 11
};

// Create or find existing histogram that matches the pickled info.
// Returns NULL if the pickled data has problems.
BASE_EXPORT HistogramBase* DeserializeHistogramInfo(base::PickleIterator* iter);

////////////////////////////////////////////////////////////////////////////////

class BASE_EXPORT HistogramBase {
 public:
  using Sample32 = int32_t;              // Used for samples.
  using AtomicCount = subtle::Atomic32;  // Used to count samples.
  using Count32 = int32_t;  // Used to manipulate counts in temporaries.

  static const Sample32 kSampleType_MAX;  // INT_MAX

  enum Flags : uint16_t {
    kNoFlags = 0x0,

    // Histogram should be UMA uploaded.
    kUmaTargetedHistogramFlag = 0x1,

    // Indicates that this is a stability histogram. This flag exists to specify
    // which histograms should be included in the initial stability log. Please
    // refer to |MetricsService::PrepareInitialStabilityLog|.
    kUmaStabilityHistogramFlag = kUmaTargetedHistogramFlag | 0x2,

    // Indicates that the histogram was pickled to be sent across an IPC
    // Channel. If we observe this flag on a histogram being aggregated into
    // after IPC, then we are running in a single process mode, and the
    // aggregation should not take place (as we would be aggregating back into
    // the source histogram!).
    kIPCSerializationSourceFlag = 0x10,

    // Indicates that a callback exists for when a new sample is recorded on
    // this histogram. We store this as a flag with the histogram since
    // histograms can be in performance critical code, and this allows us
    // to shortcut looking up the callback if it doesn't exist.
    kCallbackExists = 0x20,

    // Indicates that the histogram is held in "persistent" memory and may
    // be accessible between processes. This is only possible if such a
    // memory segment has been created/attached, used to create a Persistent-
    // MemoryAllocator, and that loaded into the Histogram module before this
    // histogram is created.
    kIsPersistent = 0x40,
  };

  // Histogram data inconsistency types.
  enum Inconsistency : uint32_t {
    NO_INCONSISTENCIES = 0x0,
    RANGE_CHECKSUM_ERROR = 0x1,
    BUCKET_ORDER_ERROR = 0x2,
    COUNT_HIGH_ERROR = 0x4,
    COUNT_LOW_ERROR = 0x8,

    NEVER_EXCEEDED_VALUE = 0x10,
  };

  // Construct the base histogram. The name is not copied; it's up to the
  // caller to ensure that it lives at least as long as this object.
  explicit HistogramBase(DurableStringView name);

  HistogramBase(const HistogramBase&) = delete;
  HistogramBase& operator=(const HistogramBase&) = delete;

  virtual ~HistogramBase();

  std::string_view histogram_name() const {
    return {histogram_name_, histogram_name_length_};
  }

  // Compares |name| to the histogram name and triggers a DCHECK if they do not
  // match. This is a helper function used by histogram macros, which results in
  // in more compact machine code being generated by the macros.
  virtual void CheckName(std::string_view name) const;

  // Get a unique ID for this histogram's samples.
  virtual uint64_t name_hash() const = 0;

  // Operations with Flags enum.
  int32_t flags() const { return flags_.load(std::memory_order_relaxed); }
  void SetFlags(int32_t flags);
  void ClearFlags(int32_t flags);
  bool HasFlags(int32_t flags) const;

  virtual HistogramType GetHistogramType() const = 0;

  // Whether the histogram has construction arguments as parameters specified.
  // For histograms that don't have the concept of minimum, maximum or
  // bucket_count, this function always returns false.
  virtual bool HasConstructionArguments(Sample32 expected_minimum,
                                        Sample32 expected_maximum,
                                        size_t expected_bucket_count) const = 0;

  virtual void Add(Sample32 value) = 0;

  // In Add function the |value| bucket is increased by one, but in some use
  // cases we need to increase this value by an arbitrary integer. AddCount
  // function increases the |value| bucket by |count|. |count| should be greater
  // than or equal to 1.
  virtual void AddCount(Sample32 value, int count) = 0;

  // Similar to above but divides |count| by the |scale| amount. Probabilistic
  // rounding is used to yield a reasonably accurate total when many samples
  // are added. Methods for common cases of scales 1000 and 1024 are included.
  // The ScaledLinearHistogram (which can also used be for enumerations) may be
  // a better (and faster) solution.
  void AddScaled(Sample32 value, int count, int scale);
  void AddKilo(Sample32 value, int count);  // scale=1000
  void AddKiB(Sample32 value, int count);   // scale=1024

  // Convenient functions that call Add(Sample32).
  void AddTime(const TimeDelta& time) { AddTimeMillisecondsGranularity(time); }
  void AddTimeMillisecondsGranularity(const TimeDelta& time);
  // Note: AddTimeMicrosecondsGranularity() drops the report if this client
  // doesn't have a high-resolution clock.
  void AddTimeMicrosecondsGranularity(const TimeDelta& time);
  void AddBoolean(bool value);

  virtual bool AddSamples(const HistogramSamples& samples) = 0;
  virtual bool AddSamplesFromPickle(base::PickleIterator* iter) = 0;

  // Serialize the histogram info into |pickle|.
  // Note: This only serializes the construction arguments of the histogram, but
  // does not serialize the samples.
  void SerializeInfo(base::Pickle* pickle) const;

  // Try to find out data corruption from histogram and the samples.
  // The returned value is a combination of Inconsistency enum.
  virtual uint32_t FindCorruption(const HistogramSamples& samples) const;

  // Snapshot the current complete set of sample data.
  // Note that histogram data is stored per-process. The browser process
  // periodically ingests data from subprocesses. As such, the browser
  // process can see histogram data from any process but other processes
  // can only see histogram data recorded in the subprocess.
  // Moreover, the data returned here may not be up to date:
  // - this function does not use a lock so data might not be synced
  //   (e.g., across cpu caches)
  // - in the browser process, the data from subprocesses may not have
  //   synced data from subprocesses via MergeHistogramDeltas() recently.
  //
  // Override with atomic/locked snapshot if needed.
  // NOTE: this data can overflow for long-running sessions. It should be
  // handled with care and this method is recommended to be used only
  // in about:histograms and test code.
  virtual std::unique_ptr<HistogramSamples> SnapshotSamples() const = 0;

  // Returns a copy of the samples that have not yet been logged. To mark the
  // returned samples as logged, see MarkSamplesAsLogged().
  //
  // See additional caveats by SnapshotSamples().
  //
  // WARNING: This may be called from a background thread by the metrics
  // collection system. Do not make a call to this unless it was properly vetted
  // by someone familiar with the system.
  // TODO(crbug.com/40119012): Consider gating this behind a PassKey, so that
  // eventually, only StatisticsRecorder can use this.
  virtual std::unique_ptr<HistogramSamples> SnapshotUnloggedSamples() const = 0;

  // Marks the passed |samples| as logged. More formally, the |samples| passed
  // will not appear in the samples returned by a subsequent call to
  // SnapshotDelta().
  //
  // See additional caveats by SnapshotSamples().
  //
  // WARNING: This may be called from a background thread by the metrics
  // collection system. Do not make a call to this unless it was properly vetted
  // by someone familiar with the system.
  // TODO(crbug.com/40119012): Consider gating this behind a PassKey, so that
  // eventually, only StatisticsRecorder can use this.
  virtual void MarkSamplesAsLogged(const HistogramSamples& samples) = 0;

  // Calculate the change (delta) in histogram counts since the previous call
  // to this method. Each successive call will return only those counts changed
  // since the last call. Calls to MarkSamplesAsLogged() will also affect the
  // samples returned. Logically, this function is equivalent to a call to
  // SnapshotUnloggedSamples() followed by a call to MarkSamplesAsLogged().
  //
  // See additional caveats by SnapshotSamples().
  //
  // WARNING: This may be called from a background thread by the metrics
  // collection system. Do not make a call to this unless it was properly vetted
  // by someone familiar with the system.
  virtual std::unique_ptr<HistogramSamples> SnapshotDelta() = 0;

  // Calculate the change (delta) in histogram counts since the previous call
  // to SnapshotDelta() but do so without modifying any internal data as to
  // what was previous logged. After such a call, no further calls to this
  // method or to SnapshotDelta() should be done as the result would include
  // data previously returned. Because no internal data is changed, this call
  // can be made on "const" histograms such as those with data held in
  // read-only memory.
  //
  // See additional caveats by SnapshotSamples().
  virtual std::unique_ptr<HistogramSamples> SnapshotFinalDelta() const = 0;

  // The following method provides graphical histogram displays.
  virtual void WriteAscii(std::string* output) const;

  // Returns histograms data as a Dict (or an empty dict if not available),
  // with the following format:
  // {"header": "Name of the histogram with samples, mean, and/or flags",
  // "body": "ASCII histogram representation"}
  virtual base::Value::Dict ToGraphDict() const = 0;

  // Produce a JSON representation of the histogram with |verbosity_level| as
  // the serialization verbosity. This is implemented with the help of
  // GetParameters and GetCountAndBucketData; overwrite them to customize the
  // output.
  void WriteJSON(std::string* output, JSONVerbosityLevel verbosity_level) const;

 protected:
  enum ReportActivity { HISTOGRAM_CREATED, HISTOGRAM_LOOKUP };

  struct BASE_EXPORT CountAndBucketData {
    Count32 count;
    int64_t sum;
    Value::List buckets;

    CountAndBucketData(Count32 count, int64_t sum, Value::List buckets);
    ~CountAndBucketData();

    CountAndBucketData(CountAndBucketData&& other);
    CountAndBucketData& operator=(CountAndBucketData&& other);
  };

  // Subclasses should implement this function to make SerializeInfo work.
  virtual void SerializeInfoImpl(base::Pickle* pickle) const = 0;

  // Writes information about the construction parameters in |params|.
  virtual Value::Dict GetParameters() const = 0;

  // Returns information about the current (non-empty) buckets and their sample
  // counts to |buckets|, the total sample count to |count| and the total sum
  // to |sum|.
  CountAndBucketData GetCountAndBucketData() const;

  // Produces an actual graph (set of blank vs non blank char's) for a bucket.
  void WriteAsciiBucketGraph(double x_count,
                             int line_length,
                             std::string* output) const;

  // Return a string description of what goes in a given bucket.
  const std::string GetSimpleAsciiBucketRange(Sample32 sample) const;

  // Write textual description of the bucket contents (relative to histogram).
  // Output is the count in the buckets, as well as the percentage.
  void WriteAsciiBucketValue(Count32 current,
                             double scaled_sum,
                             std::string* output) const;

  // Retrieves the registered callbacks for this histogram, if any, and runs
  // them passing |sample| as the parameter.
  void FindAndRunCallbacks(Sample32 sample) const;

  // Gets a view to a permanent string that can be used for histogram objects
  // when the original string is not a code constant or held in persistent
  // memory.
  static DurableStringView GetPermanentName(std::string_view name);

 private:
  friend class HistogramBaseTest;
  friend class HistogramThreadsafeTest;

  // A pointer to permanent storage where the histogram name is held. This can
  // be code space or the output of GetPermanentName() or any other storage
  // that is expected to never be deallocated or modified.
  const char* const histogram_name_;

  // The length of the string pointed to by `histogram_name_`. This is stored
  // to avoid having to recalculate the length every time `histogram_name_` is
  // used and to avoid reading beyond the strings alloc in the event of memory
  // tampering or corruption.
  const uint16_t histogram_name_length_;

  // Additional information about the histogram.
  std::atomic<uint16_t> flags_{0};
};

}  // namespace base

#endif  // BASE_METRICS_HISTOGRAM_BASE_H_
