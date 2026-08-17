// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_ANDROID_UTIL_ASSET_MANAGER_UTIL_H_
#define MEDIAPIPE_ANDROID_UTIL_ASSET_MANAGER_UTIL_H_

#ifdef __ANDROID__
#include <android/asset_manager.h>
#include <android/asset_manager_jni.h>
#include <jni.h>

#include <string>

#include "mediapipe/framework/port/canonical_errors.h"
#include "mediapipe/framework/port/singleton.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/port/statusor.h"

namespace mediapipe {

// Thin wrapper over AAssetManager provided by JNI. This class is meant to be
// used as a singleton. (Singleton<AssetManager>::get()->Initialize/ReadFile...)
//
// USAGE: call one of Initialize* functions from a JNI function that has access
// to a context/activity/etc. (See
// mediapipe/java/com/google/mediapipe/framework/AndroidAssetUtil.java
// for initialization method you can use right away.) This initializes the
// AssetManager and MediaPipe's GetResourceContents should be able to read
// resources from the assets folder using AssetManager::ReadFile()
//
// NOTE: initialization should happen strictly once and guaranteed to complete
// before any possible use, otherwise it cannot be used safely across multiple
// threads.
class AssetManager {
 public:
  AssetManager(const AssetManager&) = delete;
  AssetManager& operator=(const AssetManager&) = delete;

  // Returns the asset manager if it has been set by one of Initialize*
  // functions, otherwise returns nullptr.
  AAssetManager* GetAssetManager();

  // Returns true if AAssetManager was successfully initialized.
  bool InitializeFromAssetManager(JNIEnv* env, jobject local_asset_manager,
                                  const std::string& cache_dir_path);

  // Returns true if AAssetManager was successfully initialized.
  // cache_dir_path should be set to activity.getCacheDir().getAbsolutePath().
  // We could get it from the activity, but we have the Java layer pass it
  // directly for convenience.
  bool InitializeFromActivity(JNIEnv* env, jobject activity,
                              const std::string& cache_dir_path);

  // Returns true if AAssetManager was successfully initialized.
  ABSL_DEPRECATED("Use one of alternate Initialize* functions instead.")
  bool InitializeFromAssetManager(JNIEnv* env, jobject local_asset_manager);

  // Returns true if AAssetManager was successfully initialized.
  // cache_dir_path should be set to context.getCacheDir().getAbsolutePath().
  // We could get it from the context, but we have the Java layer pass it
  // directly for convenience.
  bool InitializeFromContext(JNIEnv* env, jobject context,
                             const std::string& cache_dir_path);

  // Checks if a file exists. Returns true on success, false otherwise. If it
  // does exist, then 'is_dir' will be set to indicate whether the file is a
  // directory.
  bool FileExists(const std::string& filename, bool* is_dir = nullptr);

  // Reads a file into output. Returns true on success, false otherwise.
  bool ReadFile(const std::string& filename, std::string* output);

  // Reads the raw bytes referred to by the supplied content URI. Returns true
  // on success, false otherwise.
  absl::Status ReadContentUri(const std::string& content_uri,
                              std::string* output);

  // Returns the path to the Android cache directory. Will be empty if
  // AssetManager hasn't been initialized.
  const std::string& GetCacheDirPath();

  // Caches the contents of the given asset as a file, and returns a path to
  // that file. This can be used to pass an asset to APIs that require a path
  // to a filesystem file.
  // NOTE: this is _not_ thread-safe, e.g. if two threads are requesting the
  // same file
  absl::StatusOr<std::string> CachedFileFromAsset(
      const std::string& asset_path);

 private:
  // Private constructor since this class is meant to be a singleton.
  AssetManager() = default;

  // Pointer to asset manager from JNI.
  AAssetManager* asset_manager_ = nullptr;

  // The context from which assets should be loaded.
  jobject context_;

  // Path to the Android cache directory for our context.
  std::string cache_dir_path_;

  friend class Singleton<AssetManager>;
};

}  // namespace mediapipe

#endif  // __ANDROID__

#endif  // MEDIAPIPE_ANDROID_UTIL_ASSET_MANAGER_UTIL_H_
