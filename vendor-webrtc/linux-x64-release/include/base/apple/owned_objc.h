// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_OWNED_OBJC_H_
#define BASE_APPLE_OWNED_OBJC_H_

#include <memory>
#include <string>

#include "base/base_export.h"
#include "build/blink_buildflags.h"
#include "build/build_config.h"

// This file defines wrappers to allow C++ code to hold references to
// Objective-C objects (either strong or weak) without being Objective-C++ code
// themselves.
//
// WHEN NOT TO USE:
// - Do not use these for pure Objective-C++ code. For that code, simply use
//   Objective-C types as normal.
// - Do not use as a member variable in an Objective-C++ class where the header
//   is included from C++ files. Use the pimpl idiom instead:
//   https://chromium.googlesource.com/chromium/src/+/main/docs/mac/mixing_cpp_and_objc.md
//
// Use these wrappers only in the situation where C++ code is passing
// Objective-C framework objects around, instead of using double-declaration.

#if __OBJC__

#define GENERATE_STRONG_OBJC_TYPE(name) @class name;
#define GENERATE_STRONG_OBJC_PROTOCOL(name) @protocol name;
#define GENERATE_WEAK_OBJC_TYPE(name) @class name;
#define GENERATE_WEAK_OBJC_PROTOCOL(name) @protocol name;

#include "base/apple/owned_objc_types.h"

#undef GENERATE_STRONG_OBJC_TYPE
#undef GENERATE_STRONG_OBJC_PROTOCOL
#undef GENERATE_WEAK_OBJC_TYPE
#undef GENERATE_WEAK_OBJC_PROTOCOL

#endif  // __OBJC__

// Define this class two ways: the full-fledged way that allows Objective-C code
// to fully construct and access the inner Objective-C object, and a
// C++-compatible way that does not expose any Objective-C code and only allows
// default construction and validity checking.
#if __OBJC__
#define OWNED_TYPE_DECL_OBJC_ADDITIONS(classname, objctype) \
  explicit classname(objctype obj);                         \
  objctype Get() const;
#else
#define OWNED_TYPE_DECL_OBJC_ADDITIONS(classname, objctype)
#endif  // __OBJC__

#define OWNED_OBJC_DECL(classname, objctype)                 \
  namespace base::apple {                                    \
  class BASE_EXPORT classname {                              \
   public:                                                   \
    /* Default-construct in a null state. */                 \
    classname();                                             \
    ~classname();                                            \
    classname(const classname&);                             \
    classname& operator=(const classname&);                  \
    /* Returns whether the object contains a valid object.*/ \
    explicit operator bool() const;                          \
    /* Comparisons. */                                       \
    bool operator==(const classname& other) const;           \
    bool operator!=(const classname& other) const;           \
    /* Logging (to allow CHECK_EQ, etc). */                  \
    std::string ToString() const;                            \
    /* Objective-C-only constructor and getter. */           \
    OWNED_TYPE_DECL_OBJC_ADDITIONS(classname, objctype)      \
                                                             \
   private:                                                  \
    struct ObjCStorage;                                      \
    std::unique_ptr<ObjCStorage> objc_storage_;              \
  };                                                         \
  }  // namespace base::apple

#define GENERATE_STRONG_OBJC_TYPE(name) OWNED_OBJC_DECL(Owned##name, name*)
#define GENERATE_STRONG_OBJC_PROTOCOL(name) \
  OWNED_OBJC_DECL(Owned##name, id<name>)
#define GENERATE_WEAK_OBJC_TYPE(name) OWNED_OBJC_DECL(Weak##name, name*)
#define GENERATE_WEAK_OBJC_PROTOCOL(name) OWNED_OBJC_DECL(Weak##name, id<name>)

#include "base/apple/owned_objc_types.h"

#undef GENERATE_STRONG_OBJC_TYPE
#undef GENERATE_STRONG_OBJC_PROTOCOL
#undef GENERATE_WEAK_OBJC_TYPE
#undef GENERATE_WEAK_OBJC_PROTOCOL
#undef OWNED_OBJC_DECL
#undef OWNED_TYPE_DECL_OBJC_ADDITIONS

#endif  // BASE_APPLE_OWNED_OBJC_H_
