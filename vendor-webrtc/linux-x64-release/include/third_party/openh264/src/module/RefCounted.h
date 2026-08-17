/*
 * Copyright 2015, Mozilla Foundation and contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef __RefCount_h__
#define __RefCount_h__

#include <stdint.h>
#include <assert.h>

extern GMPPlatformAPI* g_platform_api;

inline GMPMutex* GMPCreateMutex() {
  GMPMutex* mutex;
  if (!g_platform_api) {
    return nullptr;
  }
  GMPErr err = g_platform_api->createmutex(&mutex);
  assert(mutex);
  return GMP_FAILED(err) ? nullptr : mutex;
}

class AutoLock {
public:
  explicit AutoLock(GMPMutex* aMutex)
    : mMutex(aMutex)
  {
    assert(aMutex);
    if (mMutex) {
      mMutex->Acquire();
    }
  }
  ~AutoLock() {
    if (mMutex) {
      mMutex->Release();
    }
  }
private:
  GMPMutex* mMutex;
};

class AtomicRefCount {
public:
  explicit AtomicRefCount(uint32_t aValue)
    : mCount(aValue)
    , mMutex(GMPCreateMutex())
  {
    assert(mMutex);
  }
  ~AtomicRefCount()
  {
    if (mMutex) {
      mMutex->Destroy();
    }
  }
  uint32_t operator--() {
    AutoLock lock(mMutex);
    return --mCount;
  }
  uint32_t operator++() {
    AutoLock lock(mMutex);
    return ++mCount;
  }
  operator uint32_t() {
    AutoLock lock(mMutex);
    return mCount;
  }
private:
  uint32_t mCount;
  GMPMutex* mMutex;
};

// Note: Thread safe.
class RefCounted {
public:
  void AddRef() {
    ++mRefCount;
  }

  uint32_t Release() {
    uint32_t newCount = --mRefCount;
    if (!newCount) {
      delete this;
    }
    return newCount;
  }

protected:
  RefCounted()
    : mRefCount(0)
  {
  }
  virtual ~RefCounted()
  {
    assert(!mRefCount);
  }
  AtomicRefCount mRefCount;
};

template<class T>
class RefPtr {
public:
  explicit RefPtr(T* aPtr) : mPtr(nullptr) {
    Assign(aPtr);
  }
  ~RefPtr() {
    Assign(nullptr);
  }
  T* operator->() const { return mPtr; }

  RefPtr& operator=(T* aVal) {
    Assign(aVal);
    return *this;
  }

private:
  void Assign(T* aPtr) {
    if (mPtr) {
      mPtr->Release();
    }
    mPtr = aPtr;
    if (mPtr) {
      aPtr->AddRef();
    }
  }
  T* mPtr;
};

#endif // __RefCount_h__
