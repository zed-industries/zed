// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_REF_COUNTED_DELETE_ON_SEQUENCE_H_
#define BASE_MEMORY_REF_COUNTED_DELETE_ON_SEQUENCE_H_

#include <utility>

#include "base/check.h"
#include "base/location.h"
#include "base/memory/ref_counted.h"
#include "base/task/sequenced_task_runner.h"

namespace base {

// RefCountedDeleteOnSequence is similar to RefCountedThreadSafe, and ensures
// that the object will be deleted on a specified sequence.
//
// Sample usage:
// class Foo : public RefCountedDeleteOnSequence<Foo> {
//
//   Foo(scoped_refptr<SequencedTaskRunner> task_runner)
//       : RefCountedDeleteOnSequence<Foo>(std::move(task_runner)) {}
//   ...
//  private:
//   friend class RefCountedDeleteOnSequence<Foo>;
//   friend class DeleteHelper<Foo>;
//
//   ~Foo();
// };
template <class T>
class RefCountedDeleteOnSequence : public subtle::RefCountedThreadSafeBase {
 public:
  using RefCountPreferenceTag = subtle::StartRefCountFromZeroTag;

  // A SequencedTaskRunner for the current sequence can be acquired by calling
  // SequencedTaskRunner::GetCurrentDefault().
  explicit RefCountedDeleteOnSequence(
      scoped_refptr<SequencedTaskRunner> owning_task_runner)
      : subtle::RefCountedThreadSafeBase(subtle::GetRefCountPreference<T>()),
        owning_task_runner_(std::move(owning_task_runner)) {
    DCHECK(owning_task_runner_);
  }

  RefCountedDeleteOnSequence(const RefCountedDeleteOnSequence&) = delete;
  RefCountedDeleteOnSequence& operator=(const RefCountedDeleteOnSequence&) =
      delete;

  void AddRef() const { AddRefImpl(subtle::GetRefCountPreference<T>()); }

  void Release() const {
    if (subtle::RefCountedThreadSafeBase::Release()) {
      DestructOnSequence();
    }
  }

 protected:
  friend class DeleteHelper<RefCountedDeleteOnSequence>;
  ~RefCountedDeleteOnSequence() = default;

  SequencedTaskRunner* owning_task_runner() {
    return owning_task_runner_.get();
  }
  const SequencedTaskRunner* owning_task_runner() const {
    return owning_task_runner_.get();
  }

 private:
  void DestructOnSequence() const {
    const T* t = static_cast<const T*>(this);
    if (owning_task_runner_->RunsTasksInCurrentSequence()) {
      delete t;
    } else {
      owning_task_runner_->DeleteSoon(FROM_HERE, t);
    }
  }

  void AddRefImpl(subtle::StartRefCountFromZeroTag) const {
    subtle::RefCountedThreadSafeBase::AddRef();
  }

  void AddRefImpl(subtle::StartRefCountFromOneTag) const {
    subtle::RefCountedThreadSafeBase::AddRefWithCheck();
  }

  const scoped_refptr<SequencedTaskRunner> owning_task_runner_;
};

}  // namespace base

#endif  // BASE_MEMORY_REF_COUNTED_DELETE_ON_SEQUENCE_H_
