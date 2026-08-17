// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_SCOPED_AUTHORIZATIONREF_H_
#define BASE_MAC_SCOPED_AUTHORIZATIONREF_H_

#include <Security/Authorization.h>

#include <utility>

#include "base/base_export.h"
#include "base/check.h"

// `ScopedAuthorizationRef` maintains ownership of an `AuthorizationRef`.  It is
// patterned after the `unique_ptr` interface.

namespace base::mac {

class BASE_EXPORT ScopedAuthorizationRef {
 public:
  explicit ScopedAuthorizationRef(AuthorizationRef authorization = nullptr)
      : authorization_(authorization) {}

  ScopedAuthorizationRef(const ScopedAuthorizationRef&) = delete;
  ScopedAuthorizationRef& operator=(const ScopedAuthorizationRef&) = delete;

  ScopedAuthorizationRef(ScopedAuthorizationRef&& that)
      : authorization_(std::exchange(that.authorization_, nullptr)) {}
  ScopedAuthorizationRef& operator=(ScopedAuthorizationRef&& that) {
    authorization_ = std::exchange(that.authorization_, nullptr);
    return *this;
  }

  ~ScopedAuthorizationRef() {
    if (authorization_) {
      FreeInternal();
    }
  }

  void reset(AuthorizationRef authorization = nullptr) {
    if (authorization_ != authorization) {
      if (authorization_) {
        FreeInternal();
      }
      authorization_ = authorization;
    }
  }

  bool operator==(AuthorizationRef that) const {
    return authorization_ == that;
  }

  bool operator!=(AuthorizationRef that) const {
    return authorization_ != that;
  }

  operator AuthorizationRef() const { return authorization_; }

  explicit operator bool() const { return authorization_ != nullptr; }

  // This is to be used only to take ownership of objects that are created
  // by pass-by-pointer create functions. To enforce this, require that the
  // object be reset to NULL before this may be used.
  [[nodiscard]] AuthorizationRef* InitializeInto() {
    DCHECK(!authorization_);
    return &authorization_;
  }

  AuthorizationRef get() const { return authorization_; }

  // ScopedAuthorizationRef::release() is like std::unique_ptr<>::release. It is
  // NOT a wrapper for AuthorizationFree(). To force a ScopedAuthorizationRef
  // object to call AuthorizationFree(), use ScopedAuthorizationRef::reset().
  [[nodiscard]] AuthorizationRef release() {
    AuthorizationRef temp = authorization_;
    authorization_ = nullptr;
    return temp;
  }

 private:
  // Calling AuthorizationFree, defined in Security.framework, from an inline
  // function, results in link errors when linking dynamically with
  // libbase.dylib. So wrap the call in an un-inlined method. This method
  // doesn't check if |authorization_| is null; that check should be in the
  // inlined callers.
  void FreeInternal();

  AuthorizationRef authorization_;
};

}  // namespace base::mac

#endif  // BASE_MAC_SCOPED_AUTHORIZATIONREF_H_
