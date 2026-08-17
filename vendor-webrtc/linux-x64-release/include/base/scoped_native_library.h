// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SCOPED_NATIVE_LIBRARY_H_
#define BASE_SCOPED_NATIVE_LIBRARY_H_

#include "base/base_export.h"
#include "base/native_library.h"
#include "base/scoped_generic.h"

namespace base {

class FilePath;

struct BASE_EXPORT NativeLibraryTraits {
  // It's assumed that this is a fast inline function with little-to-no
  // penalty for duplicate calls. This must be a static function even
  // for stateful traits.
  static NativeLibrary InvalidValue() { return nullptr; }

  // This free function will not be called if library == InvalidValue()!
  static void Free(NativeLibrary library);
};

// A class which encapsulates a base::NativeLibrary object available only in a
// scope.
// This class automatically unloads the loaded library in its destructor.
class BASE_EXPORT ScopedNativeLibrary
    : public ScopedGeneric<NativeLibrary, NativeLibraryTraits> {
 public:
  // Initializes with a NULL library.
  ScopedNativeLibrary();

  // Takes ownership of the given library handle.
  explicit ScopedNativeLibrary(NativeLibrary library);

  // Opens the given library and manages its lifetime.
  explicit ScopedNativeLibrary(const FilePath& library_path);

  // Move constructor. Takes ownership of handle stored in |scoped_library|
  ScopedNativeLibrary(ScopedNativeLibrary&& scoped_library);

  // Move assignment operator. Takes ownership of handle stored in
  // |scoped_library|.
  ScopedNativeLibrary& operator=(ScopedNativeLibrary&& scoped_library) =
      default;

  ScopedNativeLibrary(const ScopedNativeLibrary&) = delete;
  ScopedNativeLibrary& operator=(const ScopedNativeLibrary&) = delete;

  ~ScopedNativeLibrary() override;

  void* GetFunctionPointer(const char* function_name) const;

  const NativeLibraryLoadError* GetError() const;

 private:
  NativeLibraryLoadError error_;
};

}  // namespace base

#endif  // BASE_SCOPED_NATIVE_LIBRARY_H_
