// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_WTF_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_WTF_TEST_HELPER_H_

#include <type_traits>

#include "base/memory/scoped_refptr.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace WTF {

class DestructCounter {
  USING_FAST_MALLOC(DestructCounter);

 public:
  explicit DestructCounter(int i, int* destruct_number)
      : i_(i), destruct_number_(destruct_number) {}

  ~DestructCounter() { ++(*destruct_number_); }
  int Get() const { return i_; }

 private:
  int i_;
  int* destruct_number_;
};

class MoveOnly {
  DISALLOW_NEW();

 public:
  explicit MoveOnly(int i = 0) : i_(i) {}

  MoveOnly(MoveOnly&& other) : i_(other.i_) { other.i_ = 0; }

  MoveOnly& operator=(MoveOnly&& other) {
    if (this != &other) {
      i_ = other.i_;
      other.i_ = 0;
    }
    return *this;
  }

  int Value() const { return i_; }

 private:
  int i_;
};

class MoveOnlyHashValue {
 public:
  // kEmpty and kDeleted have special meanings when MoveOnlyHashValue is used as
  // the key of a hash table.
  enum { kEmpty = 0, kDeleted = -1, kMovedOut = -2 };

  explicit MoveOnlyHashValue(int value = kEmpty, int id = 0)
      : value_(value), id_(id) {}
  MoveOnlyHashValue(MoveOnlyHashValue&& other)
      : value_(other.value_), id_(other.id_) {
    other.value_ = kMovedOut;
    other.id_ = 0;
  }
  MoveOnlyHashValue& operator=(MoveOnlyHashValue&& other) {
    value_ = other.value_;
    id_ = other.id_;
    other.value_ = kMovedOut;
    other.id_ = 0;
    return *this;
  }

  int Value() const { return value_; }
  // id() is used for distinguishing MoveOnlys with the same value().
  int Id() const { return id_; }

 private:
  MoveOnlyHashValue(const MoveOnlyHashValue&) = delete;
  MoveOnlyHashValue& operator=(const MoveOnlyHashValue&) = delete;

  int value_;
  int id_;
};

struct MoveOnlyHashTraits : public GenericHashTraits<MoveOnlyHashValue> {
  // This is actually true, but we pretend that it's false to disable the
  // optimization.
  static const bool kEmptyValueIsZero = false;

  static bool IsEmptyValue(const MoveOnlyHashValue& value) {
    return value.Value() == MoveOnlyHashValue::kEmpty;
  }
  static void ConstructDeletedValue(MoveOnlyHashValue& slot) {
    slot = MoveOnlyHashValue(MoveOnlyHashValue::kDeleted);
  }
  static bool IsDeletedValue(const MoveOnlyHashValue& value) {
    return value.Value() == MoveOnlyHashValue::kDeleted;
  }
  static unsigned GetHash(const MoveOnlyHashValue& value) {
    return WTF::GetHash(value.Value());
  }
  static bool Equal(const MoveOnlyHashValue& left,
                    const MoveOnlyHashValue& right) {
    return left.Value() == right.Value();
  }
};

template <>
struct HashTraits<MoveOnlyHashValue> : MoveOnlyHashTraits {};

class CountCopy final {
 public:
  static int* const kDeletedValue;

  CountCopy() : counter_(nullptr) {}
  explicit CountCopy(int* counter) : counter_(counter) {}
  explicit CountCopy(int& counter) : counter_(&counter) {}
  CountCopy(const CountCopy& other) : counter_(other.counter_) {
    if (counter_ && counter_ != kDeletedValue)
      ++*counter_;
  }
  CountCopy& operator=(const CountCopy& other) {
    counter_ = other.counter_;
    if (counter_ && counter_ != kDeletedValue)
      ++*counter_;
    return *this;
  }

  const int* Counter() const { return counter_; }

 private:
  int* counter_;
};

struct CountCopyHashTraits : public GenericHashTraits<CountCopy> {
  static const bool kEmptyValueIsZero = false;
  static bool IsEmptyValue(const CountCopy& value) { return !value.Counter(); }
  static void ConstructDeletedValue(CountCopy& slot) {
    slot = CountCopy(CountCopy::kDeletedValue);
  }
  static bool IsDeletedValue(const CountCopy& value) {
    return value.Counter() == CountCopy::kDeletedValue;
  }
  static unsigned GetHash(const CountCopy& value) {
    return WTF::GetHash(value.Counter());
  }
  static bool Equal(const CountCopy& left, const CountCopy& right) {
    return left.Counter() == right.Counter();
  }
};

template <>
struct HashTraits<CountCopy> : CountCopyHashTraits {};

template <typename T>
class ValueInstanceCount final {
 public:
  static int* const kDeletedValue;

  ValueInstanceCount() : counter_(nullptr), value_(T()) {}
  explicit ValueInstanceCount(int* counter, T value = T())
      : counter_(counter), value_(value) {
    if (counter_ && counter_ != kDeletedValue)
      ++*counter_;
  }
  ValueInstanceCount(const ValueInstanceCount& other)
      : counter_(other.counter_), value_(other.value_) {
    if (counter_ && counter_ != kDeletedValue)
      ++*counter_;
  }
  ValueInstanceCount& operator=(const ValueInstanceCount& other) {
    if (counter_ && counter_ != kDeletedValue)
      --*counter_;
    counter_ = other.counter_;
    value_ = other.value_;
    if (counter_ && counter_ != kDeletedValue)
      ++*counter_;
    return *this;
  }
  ~ValueInstanceCount() {
    if (counter_ && counter_ != kDeletedValue)
      --*counter_;
  }

  const int* Counter() const { return counter_; }
  const T& Value() const { return value_; }

 private:
  int* counter_;
  T value_;
};

template <typename T>
struct ValueInstanceCountHashTraits
    : public GenericHashTraits<ValueInstanceCount<T>> {
  static const bool kEmptyValueIsZero = false;
  static bool IsEmptyValue(const ValueInstanceCount<T>& value) {
    return !value.Counter();
  }
  static void ConstructDeletedValue(ValueInstanceCount<T>& slot) {
    slot = ValueInstanceCount<T>(ValueInstanceCount<T>::kDeletedValue);
  }
  static bool IsDeletedValue(const ValueInstanceCount<T>& value) {
    return value.Counter() == ValueInstanceCount<T>::kDeletedValue;
  }
  static unsigned GetHash(const ValueInstanceCount<T>& value) {
    return WTF::GetHash(value.Counter());
  }
  static bool Equal(const ValueInstanceCount<T>& left,
                    const ValueInstanceCount<T>& right) {
    return left.Counter() == right.Counter();
  }
};

template <typename T>
struct HashTraits<ValueInstanceCount<T>>
    : public ValueInstanceCountHashTraits<T> {};

class DummyRefCounted : public RefCounted<DummyRefCounted> {
 public:
  DummyRefCounted(bool& is_deleted) : is_deleted_(is_deleted) {
    is_deleted_ = false;
  }
  ~DummyRefCounted() {
    DCHECK(!is_deleted_);
    is_deleted_ = true;
  }

  void AddRef() {
    DCHECK(!is_deleted_);
    WTF::RefCounted<DummyRefCounted>::AddRef();
    ++ref_invokes_count_;
  }

  void Release() {
    DCHECK(!is_deleted_);
    WTF::RefCounted<DummyRefCounted>::Release();
  }

  static int ref_invokes_count_;

 private:
  bool& is_deleted_;
};

struct Dummy {
  Dummy(bool& deleted) : deleted(deleted) {}

  ~Dummy() { deleted = true; }

  bool& deleted;
};

// WrappedInt class will fail if it was memmoved or memcpyed.
extern HashSet<void*> g_constructed_wrapped_ints;
class WrappedInt {
 public:
  WrappedInt(int i = 0) : original_this_ptr_(this), i_(i) {
    g_constructed_wrapped_ints.insert(this);
  }

  WrappedInt(const WrappedInt& other) : original_this_ptr_(this), i_(other.i_) {
    g_constructed_wrapped_ints.insert(this);
  }

  WrappedInt& operator=(const WrappedInt& other) {
    i_ = other.i_;
    return *this;
  }

  ~WrappedInt() {
    EXPECT_EQ(original_this_ptr_, this);
    EXPECT_TRUE(g_constructed_wrapped_ints.Contains(this));
    g_constructed_wrapped_ints.erase(this);
  }

  int Get() const { return i_; }

 private:
  void* original_this_ptr_;
  int i_;
};

class LivenessCounter {
 public:
  void AddRef() { live_++; }
  void Release() { live_--; }

  static unsigned live_;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_WTF_TEST_HELPER_H_
