// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_IMPORTANT_FILE_WRITER_H_
#define BASE_FILES_IMPORTANT_FILE_WRITER_H_

#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <variant>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/files/file.h"
#include "base/files/file_path.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/sequence_checker.h"
#include "base/time/time.h"
#include "base/timer/timer.h"

namespace base {

class SequencedTaskRunner;

// Helper for atomically writing a file to ensure that it won't be corrupted by
// *application* crash during write (implemented as create, flush, rename).
//
// As an added benefit, ImportantFileWriter makes it less likely that the file
// is corrupted by *system* crash, though even if the ImportantFileWriter call
// has already returned at the time of the crash it is not specified which
// version of the file (old or new) is preserved. And depending on system
// configuration (hardware and software) a significant likelihood of file
// corruption may remain, thus using ImportantFileWriter is not a valid
// substitute for file integrity checks and recovery codepaths for malformed
// files.
//
// Also note that ImportantFileWriter can be *really* slow (cf. File::Flush()
// for details) and thus please don't block shutdown on ImportantFileWriter.
class BASE_EXPORT ImportantFileWriter {
 public:
  // Promise-like callback that returns (via output parameter) the serialized
  // data to be written. This callback is invoked on the sequence where I/O
  // operations are executed. Returning false indicates an error.
  using BackgroundDataProducerCallback =
      base::OnceCallback<std::optional<std::string>()>;

  // Used by ScheduleSave to lazily provide the data to be saved. Allows us
  // to also batch data serializations.
  class BASE_EXPORT DataSerializer {
   public:
    // Returns a string for serialisation when successful, or a nullopt in case
    // it failed to generate the data. Will be called on the same thread on
    // which ImportantFileWriter has been created.
    virtual std::optional<std::string> SerializeData() = 0;

   protected:
    virtual ~DataSerializer() = default;
  };

  // Same as DataSerializer but allows the caller to move some of the
  // serialization logic to the sequence where I/O operations are executed.
  class BASE_EXPORT BackgroundDataSerializer {
   public:
    // Returns a promise-like callback that, when invoked, will produce the
    // serialized string. This getter itself will be called on the same thread
    // on which ImportantFileWriter has been created, but the callback will be
    // invoked from the sequence where I/O operations are executed.
    virtual BackgroundDataProducerCallback
    GetSerializedDataProducerForBackgroundSequence() = 0;

   protected:
    virtual ~BackgroundDataSerializer() = default;
  };

  // Save |data| to |path| in an atomic manner. Blocks and writes data on the
  // current thread. Does not guarantee file integrity across system crash (see
  // the class comment above).
  static bool WriteFileAtomically(
      const FilePath& path,
      std::string_view data,
      std::string_view histogram_suffix = std::string_view());

  // Initialize the writer.
  // |path| is the name of file to write.
  // |task_runner| is the SequencedTaskRunner instance where on which we will
  // execute file I/O operations.
  // All non-const methods, ctor and dtor must be called on the same thread.
  ImportantFileWriter(const FilePath& path,
                      scoped_refptr<SequencedTaskRunner> task_runner,
                      std::string_view histogram_suffix = std::string_view());

  // Same as above, but with a custom commit interval.
  ImportantFileWriter(const FilePath& path,
                      scoped_refptr<SequencedTaskRunner> task_runner,
                      TimeDelta interval,
                      std::string_view histogram_suffix = std::string_view());

  ImportantFileWriter(const ImportantFileWriter&) = delete;
  ImportantFileWriter& operator=(const ImportantFileWriter&) = delete;

  // You have to ensure that there are no pending writes at the moment
  // of destruction.
  ~ImportantFileWriter();

  const FilePath& path() const LIFETIME_BOUND { return path_; }

  // Returns true if there is a scheduled write pending which has not yet
  // been started.
  bool HasPendingWrite() const;

  // Save |data| to target filename. Does not block. If there is a pending write
  // scheduled by ScheduleWrite(), it is cancelled.
  void WriteNow(std::string data);

  // Schedule a save to target filename. Data will be serialized and saved
  // to disk after the commit interval. If another ScheduleWrite is issued
  // before that, only one serialization and write to disk will happen, and
  // the most recent |serializer| will be used. This operation does not block.
  // |serializer| should remain valid through the lifetime of
  // ImportantFileWriter.
  void ScheduleWrite(DataSerializer* serializer);

  // Same as above but uses the BackgroundDataSerializer API.
  void ScheduleWriteWithBackgroundDataSerializer(
      BackgroundDataSerializer* serializer);

  // Serialize data pending to be saved and execute write on background thread.
  void DoScheduledWrite();

  // Registers |before_next_write_callback| and |after_next_write_callback| to
  // be synchronously invoked from WriteFileAtomically() before its next write
  // and after its next write, respectively. The boolean passed to
  // |after_next_write_callback| indicates whether the write was successful.
  // Both callbacks must be thread safe as they will be called on |task_runner_|
  // and may be called during Chrome shutdown.
  // If called more than once before a write is scheduled on |task_runner|, the
  // latest callbacks clobber the others.
  void RegisterOnNextWriteCallbacks(
      OnceClosure before_next_write_callback,
      OnceCallback<void(bool success)> after_next_write_callback);

  TimeDelta commit_interval() const { return commit_interval_; }

  // Overrides the timer to use for scheduling writes with |timer_override|.
  void SetTimerForTesting(OneShotTimer* timer_override);

#if defined(UNIT_TEST)
  size_t previous_data_size() const { return previous_data_size_; }
#endif
  void set_previous_data_size(size_t previous_data_size) {
    previous_data_size_ = previous_data_size;
  }

  // Allows tests to call the given callback instead of ReplaceFile().
  using ReplaceFileCallback =
      RepeatingCallback<bool(const FilePath&, const FilePath&, File::Error*)>;
  void SetReplaceFileCallbackForTesting(ReplaceFileCallback callback);

 private:
  const OneShotTimer& timer() const LIFETIME_BOUND {
    return timer_override_ ? *timer_override_ : timer_;
  }
  OneShotTimer& timer() LIFETIME_BOUND {
    return timer_override_ ? *timer_override_ : timer_;
  }

  // Same as WriteNow() but it uses a promise-like signature that allows running
  // custom logic in the background sequence.
  void WriteNowWithBackgroundDataProducer(
      BackgroundDataProducerCallback background_producer);

  // Helper function to call WriteFileAtomically() with a promise-like callback
  // producing a std::string.
  static void ProduceAndWriteStringToFileAtomically(
      const FilePath& path,
      BackgroundDataProducerCallback data_producer_for_background_sequence,
      OnceClosure before_write_callback,
      OnceCallback<void(bool success)> after_write_callback,
      ReplaceFileCallback replace_file_callback,
      const std::string& histogram_suffix);

  // Writes |data| to |path|, recording histograms with an optional
  // |histogram_suffix|. |from_instance| indicates whether the call originates
  // from an instance of ImportantFileWriter or a direct call to
  // WriteFileAtomically. When false, the directory containing |path| is added
  // to the set cleaned by the ImportantFileWriterCleaner (Windows only).
  static bool WriteFileAtomicallyImpl(
      const FilePath& path,
      std::string_view data,
      std::string_view histogram_suffix,
      bool from_instance,
      ReplaceFileCallback replace_file_callback);

  void ClearPendingWrite();

  // Invoked synchronously on the next write event.
  OnceClosure before_next_write_callback_;
  OnceCallback<void(bool success)> after_next_write_callback_;

  // Path being written to.
  const FilePath path_;

  // TaskRunner for the thread on which file I/O can be done.
  const scoped_refptr<SequencedTaskRunner> task_runner_;

  // Timer used to schedule commit after ScheduleWrite.
  OneShotTimer timer_;

  // An override for |timer_| used for testing.
  raw_ptr<OneShotTimer> timer_override_ = nullptr;

  // Serializer which will provide the data to be saved.
  std::variant<std::monostate, DataSerializer*, BackgroundDataSerializer*>
      serializer_;

  // Time delta after which scheduled data will be written to disk.
  const TimeDelta commit_interval_;

  // Custom histogram suffix.
  const std::string histogram_suffix_;

  // Memorizes the amount of data written on the previous write. This helps
  // preallocating memory for the data serialization. It is only used for
  // scheduled writes.
  size_t previous_data_size_ = 0;

  ReplaceFileCallback replace_file_callback_;

  SEQUENCE_CHECKER(sequence_checker_);

  WeakPtrFactory<ImportantFileWriter> weak_factory_{this};
};

}  // namespace base

#endif  // BASE_FILES_IMPORTANT_FILE_WRITER_H_
