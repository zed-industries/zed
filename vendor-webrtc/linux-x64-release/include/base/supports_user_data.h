// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SUPPORTS_USER_DATA_H_
#define BASE_SUPPORTS_USER_DATA_H_

#include <map>
#include <memory>

#include "base/base_export.h"
#include "base/memory/scoped_refptr.h"
#include "base/sequence_checker.h"

namespace base {

// This is a helper for classes that want to allow users to stash random data by
// key. At destruction all the objects will be destructed.
class BASE_EXPORT SupportsUserData {
 public:
  SupportsUserData();
  SupportsUserData(SupportsUserData&&);
  SupportsUserData& operator=(SupportsUserData&&);
  SupportsUserData(const SupportsUserData&) = delete;
  SupportsUserData& operator=(const SupportsUserData&) = delete;

  // Derive from this class and add your own data members to associate extra
  // information with this object. Alternatively, add this as a public base
  // class to any class with a virtual destructor.
  class BASE_EXPORT Data {
   public:
    virtual ~Data() = default;

    // Returns a copy of |this|; null if copy is not supported.
    virtual std::unique_ptr<Data> Clone();
  };

  // The user data allows the clients to associate data with this object.
  // |key| must not be null--that value is too vulnerable for collision.
  // NOTE: SetUserData() with an empty unique_ptr behaves the same as
  // RemoveUserData().
  Data* GetUserData(const void* key) const;
  [[nodiscard]] std::unique_ptr<Data> TakeUserData(const void* key);
  void SetUserData(const void* key, std::unique_ptr<Data> data);
  void RemoveUserData(const void* key);

  // Adds all data from |other|, that is clonable, to |this|. That is, this
  // iterates over the data in |other|, and any data that returns non-null from
  // Clone() is added to |this|.
  void CloneDataFrom(const SupportsUserData& other);

  // SupportsUserData is not thread-safe, and on debug build will assert it is
  // only used on one execution sequence. Calling this method allows the caller
  // to hand the SupportsUserData instance across execution sequences. Use only
  // if you are taking full control of the synchronization of that hand over.
  void DetachFromSequence();

 protected:
  virtual ~SupportsUserData();

  // Clear all user data from this object. This can be used if the subclass
  // needs to provide reset functionality.
  void ClearAllUserData();

 private:
  struct Impl;
  std::unique_ptr<Impl> impl_;
  bool in_clear_ = false;
  // Guards usage of |impl_|
  SEQUENCE_CHECKER(sequence_checker_);
};

// Adapter class that releases a refcounted object when the
// SupportsUserData::Data object is deleted.
template <typename T>
class UserDataAdapter : public SupportsUserData::Data {
 public:
  static T* Get(const SupportsUserData* supports_user_data, const void* key) {
    UserDataAdapter* data =
        static_cast<UserDataAdapter*>(supports_user_data->GetUserData(key));
    return data ? static_cast<T*>(data->object_.get()) : nullptr;
  }

  explicit UserDataAdapter(T* object) : object_(object) {}
  UserDataAdapter(const UserDataAdapter&) = delete;
  UserDataAdapter& operator=(const UserDataAdapter&) = delete;
  ~UserDataAdapter() override = default;

  T* release() { return object_.release(); }

 private:
  scoped_refptr<T> const object_;
};

}  // namespace base

#endif  // BASE_SUPPORTS_USER_DATA_H_
