// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PATH_SERVICE_H_
#define BASE_PATH_SERVICE_H_

#include "base/base_export.h"
#include "base/base_paths.h"
#include "base/gtest_prod_util.h"
#include "build/build_config.h"

namespace base {

class FilePath;
class ScopedPathOverride;

// The path service is a global table mapping keys to file system paths.  It is
// OK to use this service from multiple threads.
//
class BASE_EXPORT PathService {
 public:
  // Populates |path| with a special directory or file. Returns true on success,
  // in which case |path| is guaranteed to have a non-empty value. On failure,
  // |path| will not be changed.
  static bool Get(int key, FilePath* path);

  // Returns the corresponding path; CHECKs that the operation succeeds.
  static FilePath CheckedGet(int key);

  // Overrides the path to a special directory or file.  This cannot be used to
  // change the value of DIR_CURRENT, but that should be obvious.  Also, if the
  // path specifies a directory that does not exist, the directory will be
  // created by this method.  This method returns true if successful.
  //
  // If the given path is relative, then it will be resolved against
  // DIR_CURRENT.
  //
  // WARNING: Consumers of PathService::Get may expect paths to be constant
  // over the lifetime of the app, so this method should be used with caution.
  //
  // Unit tests generally should use ScopedPathOverride instead. Overrides from
  // one test should not carry over to another.
  static bool Override(int key, const FilePath& path);

  // This function does the same as PathService::Override but it takes extra
  // parameters:
  // - |is_absolute| indicates that |path| has already been expanded into an
  // absolute path, otherwise MakeAbsoluteFilePath() will be used. This is
  // useful to override paths that may not exist yet, since MakeAbsoluteFilePath
  // fails for those. Note that MakeAbsoluteFilePath also expands symbolic
  // links, even if path.IsAbsolute() is already true.
  // - |create| guides whether the directory to be overriden must
  // be created in case it doesn't exist already.
  static bool OverrideAndCreateIfNeeded(int key,
                                        const FilePath& path,
                                        bool is_absolute,
                                        bool create);

  // Returns whether an override is present for a special directory or file.
  static bool IsOverriddenForTesting(int key);

  // To extend the set of supported keys, you can register a path provider,
  // which is just a function mirroring PathService::Get.  The ProviderFunc
  // returns false if it cannot provide a non-empty path for the given key.
  // Otherwise, true is returned.
  //
  // WARNING: This function could be called on any thread from which the
  // PathService is used, so a the ProviderFunc MUST BE THREADSAFE.
  //
  typedef bool (*ProviderFunc)(int, FilePath*);

  // Call to register a path provider.  You must specify the range "[key_start,
  // key_end)" of supported path keys.
  static void RegisterProvider(ProviderFunc provider,
                               int key_start,
                               int key_end);

  // Disable internal cache.
  static void DisableCache();

 private:
  friend class ScopedPathOverride;
  FRIEND_TEST_ALL_PREFIXES(PathServiceTest, RemoveOverride);

  // Removes an override for a special directory or file. Returns true if there
  // was an override to remove or false if none was present.
  static bool RemoveOverrideForTests(int key);
};

}  // namespace base

#endif  // BASE_PATH_SERVICE_H_
