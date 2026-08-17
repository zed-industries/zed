// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LEVELDATABASE_LEVELDB_CHROME_H_
#define THIRD_PARTY_LEVELDATABASE_LEVELDB_CHROME_H_

#include <memory>
#include <string>

#include "base/files/file_path.h"
#include "leveldb/cache.h"
#include "leveldb/env.h"
#include "leveldb/export.h"
#include "leveldb/options.h"
#include "third_party/leveldatabase/src/db/filename.h"

namespace base {
namespace trace_event {
class MemoryAllocatorDump;
class ProcessMemoryDump;
}  // namespace trace_event
}  // namespace base

namespace leveldb_chrome {

// Return the shared leveldb block cache for web APIs. The caller *does not*
// own the returned instance. Thread safety:
//
// * `leveldb::Cache` is thread-safe to use
// * `GetSharedWebBlockCache()` is thread-safe to call (it instantiates and
//   returns a local static)
LEVELDB_EXPORT leveldb::Cache* GetSharedWebBlockCache();

// Return the shared leveldb block cache for browser (non web) APIs. The caller
// *does not* own the returned instance.
LEVELDB_EXPORT leveldb::Cache* GetSharedBrowserBlockCache();

// Return the shared leveldb block cache for in-memory Envs. The caller *does
// not* own the returned instance.
LEVELDB_EXPORT leveldb::Cache* GetSharedInMemoryBlockCache();

// Determine if a leveldb::Env stores the file data in RAM.
LEVELDB_EXPORT bool IsMemEnv(const leveldb::Env* env);

// Creates an in-memory Env for which all files are stored in the heap.
// This wraps leveldb::NewMemEnv to add memory-infra logging.
// if |base_env| is null then leveldb::Env::Default() will be used.
LEVELDB_EXPORT std::unique_ptr<leveldb::Env> NewMemEnv(
    const std::string& name,
    leveldb::Env* base_env = nullptr);

// If filename is a leveldb file, store the type of the file in *type.
// The number encoded in the filename is stored in *number.
// Returns true if the filename was successfully parsed.
LEVELDB_EXPORT bool ParseFileName(const std::string& filename,
                                  uint64_t* number,
                                  leveldb::FileType* type);

// Corrupt a closed database for testing purposes. After calling this function
// leveldb::OpenDB(...) will return a status where IsCorruption() returns true.
// Returns true if the database was successfully corrupted, false if not.
// Note: This function will fail if |db_path| does not exist.
LEVELDB_EXPORT bool CorruptClosedDBForTesting(const base::FilePath& db_path);

// Check that the database path in |db_path| "appears" to be a valid leveldb
// database using the provided |env|. This function *does not* open or verify
// that the database referred to by |db_path| is a valid database, only that it
// appears to be one.
LEVELDB_EXPORT bool PossiblyValidDB(const base::FilePath& db_path,
                                    leveldb::Env* env);

// Fully delete the leveldb database specified by |db_path|. leveldb::DestroyDB
// will only delete files that it creates. Other files, if present, will be
// ignored and left behind after leveldb::DestroyDB returns. This function will
// delete the entire database directory.
//
// Note: Can be used with in-memory Env's.
LEVELDB_EXPORT leveldb::Status DeleteDB(const base::FilePath& db_path,
                                        const leveldb::Options& options);

// Returns the memory-infra dump for |tracked_memenv|.
// Do not call this function, instead call
// leveldb_env::DBTracker::GetOrCreateAllocatorDump().
// TODO(crbug.com/762598) Can be made private as part of leveldb cleanup.
base::trace_event::MemoryAllocatorDump* GetEnvAllocatorDump(
    base::trace_event::ProcessMemoryDump* pmd,
    leveldb::Env* tracked_memenv);

// Dump all tracked in-memory env's to the |pmd|. Do not call - this is a
// private function for leveldb_env::DBTracker.
// TODO(crbug.com/762598) Can be made private as part of leveldb cleanup.
void DumpAllTrackedEnvs(base::trace_event::ProcessMemoryDump* pmd);

}  // namespace leveldb_chrome

#endif  // THIRD_PARTY_LEVELDATABASE_LEVELDB_CHROME_H_
