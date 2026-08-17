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

#ifndef MEDIAPIPE_OBJC_CFHOLDER_H_
#define MEDIAPIPE_OBJC_CFHOLDER_H_

#import <CoreFoundation/CoreFoundation.h>

/// Manages ownership of a CoreFoundation type (any type that can be passed
/// to CFRetain/CFRelease).
template <typename T>
class CFHolder {
 public:
  /// Default constructor gives a NULL ref.
  CFHolder() : _object(NULL) {}

  /// Constructor with the basic ref type. Retains it.
  explicit CFHolder(T object) : _object(RetainIfNotNull(object)) {}

  /// Copy constructor.
  CFHolder(const CFHolder& other) : _object(RetainIfNotNull(*other)) {}

  /// Move constructor.
  CFHolder(CFHolder&& other) : _object(*other) { other._object = NULL; }

  /// Destructor releases the held object.
  ~CFHolder() { ReleaseIfNotNull(_object); }

  /// Dereference to access the held object.
  T operator*() const { return _object; }

  /// Assigning from another CFHolder adds a reference.
  CFHolder& operator=(const CFHolder& other) { return reset(*other); }

  /// Move assignment does not add a reference.
  CFHolder& operator=(CFHolder&& other) {
    // C++11 allows its library implementation to assume that rvalue reference
    // arguments are not aliased. See 17.6.4.9 in the standard document.
    ReleaseIfNotNull(_object);
    _object = other._object;
    other._object = NULL;
    return *this;
  }

  /// Equality and inequality operators.
  bool operator==(const CFHolder& other) const {
    return _object == other._object;
  }
  bool operator!=(const CFHolder& other) const { return !operator==(other); }
  bool operator==(T object) const { return _object == object; }
  bool operator!=(T object) const { return !operator==(object); }

  /// Sets the managed object.
  CFHolder& reset(T object) {
    T old = _object;
    _object = RetainIfNotNull(object);
    ReleaseIfNotNull(old);
    return *this;
  }

  /// Takes ownership of the object. Does not retain.
  CFHolder& adopt(T object) {
    ReleaseIfNotNull(_object);
    _object = object;
    return *this;
  }

 private:
  static inline T RetainIfNotNull(T object) {
    if (object) CFRetain(object);
    return object;
  }

  static inline void ReleaseIfNotNull(T object) {
    if (object) CFRelease(object);
  }
  T _object;
};

/// Using these functions allows template argument deduction (i.e. you do not
/// need to specify the type of object the holder holds, it is inferred from
/// the argument.
template <typename T>
CFHolder<T>* NewCFHolder(T object) {
  return new CFHolder<T>(object);
}

template <typename T>
CFHolder<T> MakeCFHolder(T object) {
  return CFHolder<T>(object);
}

template <typename T>
CFHolder<T> MakeCFHolderAdopting(T object) {
  return CFHolder<T>().adopt(object);
}

#endif  // MEDIAPIPE_OBJC_CFHOLDER_H_
