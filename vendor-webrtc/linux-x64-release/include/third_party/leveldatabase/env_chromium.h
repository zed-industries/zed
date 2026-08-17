// Copyright (c) 2013 The LevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#ifndef THIRD_PARTY_LEVELDATABASE_ENV_CHROMIUM_H_
#define THIRD_PARTY_LEVELDATABASE_ENV_CHROMIUM_H_

#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "base/containers/linked_list.h"
#include "base/containers/span.h"
#include "base/files/file.h"
#include "base/files/file_path.h"
#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "leveldb/cache.h"
#include "leveldb/db.h"
#include "leveldb/env.h"
#include "leveldb/export.h"
#include "port/port_chromium.h"

namespace base {
namespace trace_event {
class MemoryAllocatorDump;
class ProcessMemoryDump;
}  // namespace trace_event
}  // namespace base

namespace storage {
class FilesystemProxy;
}

namespace leveldb_env {

// These entries map to values in tools/metrics/histograms/histograms.xml. New
// values should be appended at the end.
enum MethodID {
  kSequentialFileRead,
  kSequentialFileSkip,
  kRandomAccessFileRead,
  kWritableFileAppend,
  kWritableFileClose,
  kWritableFileFlush,
  kWritableFileSync,
  kNewSequentialFile,
  kNewRandomAccessFile,
  kNewWritableFile,
  kObsoleteDeleteFile,
  kCreateDir,
  kObsoleteDeleteDir,
  kGetFileSize,
  kRenameFile,
  kLockFile,
  kUnlockFile,
  kGetTestDirectory,
  kNewLogger,
  kSyncParent,
  kGetChildren,
  kNewAppendableFile,
  kRemoveFile,
  kRemoveDir,
  kNumEntries
};

// leveldb::Status::Code values are mapped to these values for UMA logging.
// Do not change/delete these values as you will break reporting for older
// copies of Chrome. Only add new values to the end.
enum LevelDBStatusValue {
  LEVELDB_STATUS_OK = 0,
  LEVELDB_STATUS_NOT_FOUND,
  LEVELDB_STATUS_CORRUPTION,
  LEVELDB_STATUS_NOT_SUPPORTED,
  LEVELDB_STATUS_INVALID_ARGUMENT,
  LEVELDB_STATUS_IO_ERROR,
  LEVELDB_STATUS_MAX
};

LEVELDB_EXPORT LevelDBStatusValue
GetLevelDBStatusUMAValue(const leveldb::Status& s);

using DatabaseErrorReportingCallback =
    base::RepeatingCallback<void(const leveldb::Status&)>;

// Create the default leveldb options object suitable for leveldb operations.
struct LEVELDB_EXPORT Options : public leveldb::Options {
  Options();

  // Called when there is a error during the Get() call. Intended for metrics
  // reporting.
  DatabaseErrorReportingCallback on_get_error;
  // Called when there is a error during the Write() call, which is called for
  // Write(), Put() and Delete(). Intended for metrics reporting.
  DatabaseErrorReportingCallback on_write_error;
};

LEVELDB_EXPORT const char* MethodIDToString(MethodID method);

leveldb::Status LEVELDB_EXPORT
MakeIOError(leveldb::Slice filename,
            const std::string& message,
            MethodID method,
            base::File::Error error = base::File::FILE_OK);

enum ErrorParsingResult {
  METHOD_ONLY,
  METHOD_AND_BFE,
  NONE,
};

ErrorParsingResult LEVELDB_EXPORT
ParseMethodAndError(const leveldb::Status& status,
                    MethodID* method,
                    base::File::Error* error);
LEVELDB_EXPORT int GetCorruptionCode(const leveldb::Status& status);
LEVELDB_EXPORT int GetNumCorruptionCodes();
LEVELDB_EXPORT std::string GetCorruptionMessage(const leveldb::Status& status);
LEVELDB_EXPORT bool IndicatesDiskFull(const leveldb::Status& status);

// Returns the name for a temporary database copy during RewriteDB().
LEVELDB_EXPORT std::string DatabaseNameForRewriteDB(
    const std::string& original_name);

// Determine the appropriate leveldb write buffer size to use. The default size
// (4MB) may result in a log file too large to be compacted given the available
// storage space. This function will return smaller values for smaller disks,
// and the default leveldb value for larger disks.
//
// |disk_space| is the logical partition size (in bytes), and *not* available
// space. A value of -1 will return leveldb's default write buffer size.
LEVELDB_EXPORT extern size_t WriteBufferSize(int64_t disk_space);

// Thread safety: `ChromiumEnv` is safe to use from multiple threads as long as
// it's created and destroyed safely. In Chromium, ChromiumEnv is created via a
// NoDestructor singleton, so as a function-local static, construction is
// thread-safe as of C++11. The NoDestructor-wrapped instance is never
// destroyed.
class LEVELDB_EXPORT ChromiumEnv : public leveldb::Env {
 public:
  using ScheduleFunc = void(void*);

  // Constructs a ChromiumEnv instance with an unrestricted FilesystemProxy
  // instance that performs direct filesystem access.
  ChromiumEnv();

  // Temporary debugging ctor.
  explicit ChromiumEnv(bool log_lock_errors);

  // Constructs a ChromiumEnv instance with a custom FilesystemProxy instance.
  explicit ChromiumEnv(std::unique_ptr<storage::FilesystemProxy> filesystem);

  ~ChromiumEnv() override;

  bool FileExists(const std::string& fname) override;
  leveldb::Status GetChildren(const std::string& dir,
                              std::vector<std::string>* result) override;
  leveldb::Status RemoveFile(const std::string& fname) override;
  leveldb::Status CreateDir(const std::string& name) override;
  leveldb::Status RemoveDir(const std::string& name) override;
  leveldb::Status GetFileSize(const std::string& fname,
                              uint64_t* size) override;
  leveldb::Status RenameFile(const std::string& src,
                             const std::string& dst) override;
  leveldb::Status LockFile(const std::string& fname,
                           leveldb::FileLock** lock) override;
  leveldb::Status UnlockFile(leveldb::FileLock* lock) override;
  void Schedule(ScheduleFunc*, void* arg) override;
  void StartThread(void (*function)(void* arg), void* arg) override;
  leveldb::Status GetTestDirectory(std::string* path) override;
  uint64_t NowMicros() override;
  void SleepForMicroseconds(int micros) override;
  leveldb::Status NewSequentialFile(const std::string& fname,
                                    leveldb::SequentialFile** result) override;
  leveldb::Status NewRandomAccessFile(
      const std::string& fname,
      leveldb::RandomAccessFile** result) override;
  leveldb::Status NewWritableFile(const std::string& fname,
                                  leveldb::WritableFile** result) override;
  leveldb::Status NewAppendableFile(const std::string& fname,
                                    leveldb::WritableFile** result) override;
  leveldb::Status NewLogger(const std::string& fname,
                            leveldb::Logger** result) override;
  void SetReadOnlyFileLimitForTesting(int max_open_files);

 protected:
  static const char* FileErrorString(base::File::Error error);

 private:
  void RemoveBackupFiles(const base::FilePath& dir);

  // When `log_lock_errors_` is true, this env will emit extra metrics for
  // locking failures. TODO(crbug.com/340398745): remove this.
  bool log_lock_errors_ = false;

  // `FilesystemProxy` is thread-safe.
  const std::unique_ptr<storage::FilesystemProxy> filesystem_;

  base::Lock mu_;
  base::FilePath test_directory_ GUARDED_BY(mu_);

  // `leveldb::Cache` is thread-safe.
  std::unique_ptr<leveldb::Cache> file_cache_;
};

// Tracks databases open via OpenDatabase() method and exposes them to
// memory-infra. The class is thread safe.
class LEVELDB_EXPORT DBTracker {
 public:
  enum SharedReadCacheUse : int {
    // Use for databases whose access pattern is dictated by browser code.
    SharedReadCacheUse_Browser = 0,
    // Use for databases whose access pattern is directly influenced by Web
    // APIs, like Indexed DB, etc.
    SharedReadCacheUse_Web,
    SharedReadCacheUse_Unified,   // When Web == Browser.
    SharedReadCacheUse_InMemory,  // Shared by all in-memory databases.
    SharedReadCacheUse_NumCacheUses
  };

  // DBTracker singleton instance.
  static DBTracker* GetInstance();

  DBTracker(const DBTracker&) = delete;
  DBTracker& operator=(const DBTracker&) = delete;

  // Returns the memory-infra dump for |tracked_db|. Can be used to attach
  // additional info to the database dump, or to properly attribute memory
  // usage in memory dump providers that also dump |tracked_db|.
  // Note that |tracked_db| should be a live database instance produced by
  // OpenDatabase() method or leveldb_env::OpenDB() function.
  static base::trace_event::MemoryAllocatorDump* GetOrCreateAllocatorDump(
      base::trace_event::ProcessMemoryDump* pmd,
      leveldb::DB* tracked_db);

  // Returns the memory-infra dump for |tracked_memenv|. Can be used to attach
  // additional info to the database dump, or to properly attribute memory
  // usage in memory dump providers that also dump |tracked_memenv|.
  // Note that |tracked_memenv| should be a live Env instance produced by
  // leveldb_chrome::NewMemEnv().
  static base::trace_event::MemoryAllocatorDump* GetOrCreateAllocatorDump(
      base::trace_event::ProcessMemoryDump* pmd,
      leveldb::Env* tracked_memenv);

  // Provides extra information about a tracked database.
  class TrackedDB : public leveldb::DB {
   public:
    // Name that OpenDatabase() was called with.
    virtual const std::string& name() const = 0;

    // Options used when opening the database.
    virtual SharedReadCacheUse block_cache_type() const = 0;
  };

  // Opens a database and starts tracking it. As long as the opened database
  // is alive (i.e. its instance is not destroyed) the database is exposed to
  // memory-infra and is enumerated by VisitDatabases() method.
  // This function is an implementation detail of leveldb_env::OpenDB(), and
  // has similar guarantees regarding |dbptr| argument.
  leveldb::Status OpenDatabase(const leveldb_env::Options& options,
                               const std::string& name,
                               TrackedDB** dbptr);

 private:
  class MemoryDumpProvider;
  class TrackedDBImpl;

  using DatabaseVisitor = base::RepeatingCallback<void(TrackedDB*)>;

  friend class ChromiumEnvDBTrackerTest;
  FRIEND_TEST_ALL_PREFIXES(ChromiumEnvDBTrackerTest, IsTrackedDB);
  FRIEND_TEST_ALL_PREFIXES(ChromiumEnvDBTrackerTest, MemoryDumpCreation);
  FRIEND_TEST_ALL_PREFIXES(ChromiumEnvDBTrackerTest, MemEnvMemoryDumpCreation);

  DBTracker();
  ~DBTracker();

  // Calls |visitor| for each live database. The database is live from the
  // point it was returned from OpenDatabase() and up until its instance is
  // destroyed.
  // The databases may be visited in an arbitrary order.
  // This function takes a lock, preventing any database from being opened or
  // destroyed (but doesn't lock the databases themselves).
  void VisitDatabases(const DatabaseVisitor& visitor);

  // Checks if |db| is tracked.
  bool IsTrackedDB(const leveldb::DB* db) const;

  void DatabaseOpened(TrackedDBImpl* database);
  void DatabaseDestroyed(TrackedDBImpl* database);

  // Protect databases_ and mdp_ members.
  mutable base::Lock databases_lock_;
  base::LinkedList<TrackedDBImpl> databases_;
  std::unique_ptr<MemoryDumpProvider> mdp_;
};

// Opens a database with the specified "name" and "options" (see note) and
// exposes it to Chrome's tracing (see DBTracker for details). The function
// guarantees that:
//   1. |dbptr| is not touched on failure
//   2. |dbptr| is not NULL on success
//
// Note that some `options` may not be honored, for example in the case of
// in-memory databases, the block cache is disabled and a minimum write buffer
// size is used to conserve memory.
LEVELDB_EXPORT leveldb::Status OpenDB(const leveldb_env::Options& options,
                                      const std::string& name,
                                      std::unique_ptr<leveldb::DB>* dbptr);

// Overrides OpenDB with the given closure.
using DBFactoryMethod =
    base::RepeatingCallback<leveldb::Status(const leveldb_env::Options&,
                                            const std::string&,
                                            std::unique_ptr<leveldb::DB>*)>;
LEVELDB_EXPORT void SetDBFactoryForTesting(DBFactoryMethod factory);

// Copies the content of |dbptr| into a fresh database to remove traces of
// deleted data. |options| and |name| of the old database are required to create
// an identical copy. |dbptr| will be replaced with the new database on success.
// If the rewrite fails e.g. because we can't write to the temporary location,
// the old db is returned if possible, otherwise |*dbptr| can become NULL.
LEVELDB_EXPORT leveldb::Status RewriteDB(const leveldb_env::Options& options,
                                         const std::string& name,
                                         std::unique_ptr<leveldb::DB>* dbptr);

LEVELDB_EXPORT std::string_view MakeStringView(const leveldb::Slice& s);
LEVELDB_EXPORT leveldb::Slice MakeSlice(std::string_view s);
LEVELDB_EXPORT leveldb::Slice MakeSlice(base::span<const uint8_t> s);

}  // namespace leveldb_env

#endif  // THIRD_PARTY_LEVELDATABASE_ENV_CHROMIUM_H_
