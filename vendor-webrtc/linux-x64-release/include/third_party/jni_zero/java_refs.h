// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef JNI_ZERO_JAVA_REFS_H_
#define JNI_ZERO_JAVA_REFS_H_

#include <jni.h>

#include <type_traits>
#include <utility>

#include "third_party/jni_zero/jni_export.h"
#include "third_party/jni_zero/logging.h"

namespace jni_zero {

// Creates a new local reference frame, in which at least a given number of
// local references can be created. Note that local references already created
// in previous local frames are still valid in the current local frame.
class JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalFrame {
 public:
  explicit ScopedJavaLocalFrame(JNIEnv* env);
  ScopedJavaLocalFrame(JNIEnv* env, int capacity);

  ScopedJavaLocalFrame(const ScopedJavaLocalFrame&) = delete;
  ScopedJavaLocalFrame& operator=(const ScopedJavaLocalFrame&) = delete;

  ~ScopedJavaLocalFrame();

 private:
  // This class is only good for use on the thread it was created on so
  // it's safe to cache the non-threadsafe JNIEnv* inside this object.
  JNIEnv* env_;
};

// Forward declare the generic java reference template class.
template <typename T>
class JavaRef;

// Template specialization of JavaRef, which acts as the base class for all
// other JavaRef<> template types. This allows you to e.g. pass
// ScopedJavaLocalRef<jstring> into a function taking const JavaRef<jobject>&
template <>
class JNI_ZERO_COMPONENT_BUILD_EXPORT JavaRef<jobject> {
 public:
  // Initializes a null reference.
  constexpr JavaRef() {}

  // Allow nullptr to be converted to JavaRef. This avoids having to declare an
  // empty JavaRef just to pass null to a function, and makes C++ "nullptr" and
  // Java "null" equivalent.
  constexpr JavaRef(std::nullptr_t) {}

  JavaRef(const JavaRef&) = delete;
  JavaRef& operator=(const JavaRef&) = delete;

  // Public to allow destruction of null JavaRef objects.
  ~JavaRef() {}

  // TODO(torne): maybe rename this to get() for consistency with unique_ptr
  // once there's fewer unnecessary uses of it in the codebase.
  jobject obj() const { return obj_; }

  explicit operator bool() const { return obj_ != nullptr; }

  // Deprecated. Just use bool conversion.
  // TODO(torne): replace usage and remove this.
  bool is_null() const { return obj_ == nullptr; }

 protected:
// Takes ownership of the |obj| reference passed; requires it to be a local
// reference type.
#if JNI_ZERO_DCHECK_IS_ON()
  // Implementation contains a DCHECK; implement out-of-line when DCHECK_IS_ON.
  JavaRef(JNIEnv* env, jobject obj);
#else
  JavaRef(JNIEnv* env, jobject obj) : obj_(obj) {}
#endif

  // Used for move semantics. obj_ must have been released first if non-null.
  void steal(JavaRef&& other) {
    obj_ = other.obj_;
    other.obj_ = nullptr;
  }

  // The following are implementation detail convenience methods, for
  // use by the sub-classes.
  JNIEnv* SetNewLocalRef(JNIEnv* env, jobject obj);
  void SetNewGlobalRef(JNIEnv* env, jobject obj);
  void ResetLocalRef(JNIEnv* env);
  void ResetGlobalRef();

  jobject ReleaseInternal() {
    jobject obj = obj_;
    obj_ = nullptr;
    return obj;
  }

 private:
  jobject obj_ = nullptr;
};

// Forward declare the object array reader for the convenience function.
template <typename T>
class JavaObjectArrayReader;

// Generic base class for ScopedJavaLocalRef and ScopedJavaGlobalRef. Useful
// for allowing functions to accept a reference without having to mandate
// whether it is a local or global type.
template <typename T>
class JavaRef : public JavaRef<jobject> {
 public:
  constexpr JavaRef() {}
  constexpr JavaRef(std::nullptr_t) {}

  JavaRef(const JavaRef&) = delete;
  JavaRef& operator=(const JavaRef&) = delete;

  ~JavaRef() {}

  T obj() const { return static_cast<T>(JavaRef<jobject>::obj()); }

  // Get a JavaObjectArrayReader for the array pointed to by this reference.
  // Only defined for JavaRef<jobjectArray>.
  // You must pass the type of the array elements (usually jobject) as the
  // template parameter.
  template <typename ElementType,
            typename T_ = T,
            typename = std::enable_if_t<std::is_same_v<T_, jobjectArray>>>
  JavaObjectArrayReader<ElementType> ReadElements() const {
    return JavaObjectArrayReader<ElementType>(*this);
  }

 protected:
  JavaRef(JNIEnv* env, T obj) : JavaRef<jobject>(env, obj) {}
};

// Holds a local reference to a JNI method parameter.
// Method parameters should not be deleted, and so this class exists purely to
// wrap them as a JavaRef<T> in the JNI binding generator. Do not use in new
// code.
// TODO(crbug.com/40425392): Remove all uses of JavaParamRef to use JavaRef
// instead.
template <typename T>
class JavaParamRef : public JavaRef<T> {
 public:
  // Assumes that |obj| is a parameter passed to a JNI method from Java.
  // Does not assume ownership as parameters should not be deleted.
  JavaParamRef(JNIEnv* env, T obj) : JavaRef<T>(env, obj) {}

  // Allow nullptr to be converted to JavaParamRef. Some unit tests call JNI
  // methods directly from C++ and pass null for objects which are not actually
  // used by the implementation (e.g. the caller object); allow this to keep
  // working.
  JavaParamRef(std::nullptr_t) {}

  JavaParamRef(const JavaParamRef&) = delete;
  JavaParamRef& operator=(const JavaParamRef&) = delete;

  ~JavaParamRef() {}

  operator T() const { return JavaRef<T>::obj(); }
};

// Holds a local reference to a Java object. The local reference is scoped
// to the lifetime of this object.
// Instances of this class may hold onto any JNIEnv passed into it until
// destroyed. Therefore, since a JNIEnv is only suitable for use on a single
// thread, objects of this class must be created, used, and destroyed, on a
// single thread.
// Therefore, this class should only be used as a stack-based object and from a
// single thread. If you wish to have the reference outlive the current
// callstack (e.g. as a class member) or you wish to pass it across threads,
// use a ScopedJavaGlobalRef instead.
template <typename T>
class ScopedJavaLocalRef : public JavaRef<T> {
 public:
  // Take ownership of a bare jobject. This does not create a new reference.
  // This should only be used by JNI helper functions, or in cases where code
  // must call JNIEnv methods directly.
  static ScopedJavaLocalRef Adopt(JNIEnv* env, T obj) {
    return ScopedJavaLocalRef(env, obj);
  }

  constexpr ScopedJavaLocalRef() {}
  constexpr ScopedJavaLocalRef(std::nullptr_t) {}

  // Copy constructor. This is required in addition to the copy conversion
  // constructor below.
  ScopedJavaLocalRef(const ScopedJavaLocalRef& other) : env_(other.env_) {
    JavaRef<T>::SetNewLocalRef(env_, other.obj());
  }

  // Copy conversion constructor.
  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  ScopedJavaLocalRef(const ScopedJavaLocalRef<U>& other) : env_(other.env_) {
    JavaRef<T>::SetNewLocalRef(env_, other.obj());
  }

  // Move constructor. This is required in addition to the move conversion
  // constructor below.
  ScopedJavaLocalRef(ScopedJavaLocalRef&& other) : env_(other.env_) {
    JavaRef<T>::steal(std::move(other));
  }

  // Move conversion constructor.
  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  ScopedJavaLocalRef(ScopedJavaLocalRef<U>&& other) : env_(other.env_) {
    JavaRef<T>::steal(std::move(other));
  }

  // Constructor for other JavaRef types.
  explicit ScopedJavaLocalRef(const JavaRef<T>& other) { Reset(other); }

  ScopedJavaLocalRef(JNIEnv* env, const JavaRef<T>& other) { Reset(other); }

  // Assumes that |obj| is a local reference to a Java object and takes
  // ownership of this local reference.
  // TODO(torne): make legitimate uses call Adopt() instead, and make this
  // private.
  ScopedJavaLocalRef(JNIEnv* env, T obj) : JavaRef<T>(env, obj), env_(env) {}

  ~ScopedJavaLocalRef() { Reset(); }

  // Null assignment, for disambiguation.
  ScopedJavaLocalRef& operator=(std::nullptr_t) {
    Reset();
    return *this;
  }

  // Copy assignment.
  ScopedJavaLocalRef& operator=(const ScopedJavaLocalRef& other) {
    Reset(other);
    return *this;
  }

  // Copy conversion assignment.
  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  ScopedJavaLocalRef& operator=(const ScopedJavaLocalRef<U>& other) {
    Reset(other);
    return *this;
  }

  // Move assignment.
  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  ScopedJavaLocalRef& operator=(ScopedJavaLocalRef<U>&& other) {
    env_ = other.env_;
    Reset();
    JavaRef<T>::steal(std::move(other));
    return *this;
  }

  // Assignment for other JavaRef types.
  ScopedJavaLocalRef& operator=(const JavaRef<T>& other) {
    Reset(other);
    return *this;
  }

  void Reset() { JavaRef<T>::ResetLocalRef(env_); }

  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  void Reset(const ScopedJavaLocalRef<U>& other) {
    // We can copy over env_ here as |other| instance must be from the same
    // thread as |this| local ref. (See class comment for multi-threading
    // limitations, and alternatives).
    env_ = JavaRef<T>::SetNewLocalRef(other.env_, other.obj());
  }

  void Reset(const JavaRef<T>& other) {
    // If |env_| was not yet set (is still null) it will be attached to the
    // current thread in SetNewLocalRef().
    env_ = JavaRef<T>::SetNewLocalRef(env_, other.obj());
  }

  // Releases the local reference to the caller. The caller *must* delete the
  // local reference when it is done with it. Note that calling a Java method
  // is *not* a transfer of ownership and Release() should not be used.
  T Release() { return static_cast<T>(JavaRef<T>::ReleaseInternal()); }

  // Alias for Release(). For use in templates when global refs are invalid.
  T ReleaseLocal() { return static_cast<T>(JavaRef<T>::ReleaseInternal()); }

 private:
  // This class is only good for use on the thread it was created on so
  // it's safe to cache the non-threadsafe JNIEnv* inside this object.
  JNIEnv* env_ = nullptr;

  // Prevent ScopedJavaLocalRef(JNIEnv*, T obj) from being used to take
  // ownership of a JavaParamRef's underlying object - parameters are not
  // allowed to be deleted and so should not be owned by ScopedJavaLocalRef.
  // TODO(torne): this can be removed once JavaParamRef no longer has an
  // implicit conversion back to T.
  ScopedJavaLocalRef(JNIEnv* env, const JavaParamRef<T>& other);

  // Friend required to get env_ from conversions.
  template <typename U>
  friend class ScopedJavaLocalRef;

  // Avoids JavaObjectArrayReader having to accept and store its own env.
  template <typename U>
  friend class JavaObjectArrayReader;
};

// Holds a global reference to a Java object. The global reference is scoped
// to the lifetime of this object. This class does not hold onto any JNIEnv*
// passed to it, hence it is safe to use across threads (within the constraints
// imposed by the underlying Java object that it references).
template <typename T>
class ScopedJavaGlobalRef : public JavaRef<T> {
 public:
  constexpr ScopedJavaGlobalRef() {}
  constexpr ScopedJavaGlobalRef(std::nullptr_t) {}

  // Copy constructor. This is required in addition to the copy conversion
  // constructor below.
  ScopedJavaGlobalRef(const ScopedJavaGlobalRef& other) { Reset(other); }

  // Copy conversion constructor.
  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  ScopedJavaGlobalRef(const ScopedJavaGlobalRef<U>& other) {
    Reset(other);
  }

  // Move constructor. This is required in addition to the move conversion
  // constructor below.
  ScopedJavaGlobalRef(ScopedJavaGlobalRef&& other) {
    JavaRef<T>::steal(std::move(other));
  }

  // Move conversion constructor.
  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  ScopedJavaGlobalRef(ScopedJavaGlobalRef<U>&& other) {
    JavaRef<T>::steal(std::move(other));
  }

  // Conversion constructor for other JavaRef types.
  explicit ScopedJavaGlobalRef(const JavaRef<T>& other) { Reset(other); }

  ScopedJavaGlobalRef(JNIEnv* env, const JavaRef<T>& other) {
    JavaRef<T>::SetNewGlobalRef(env, other.obj());
  }

  // Create a new global reference to the object.
  // Deprecated. Don't use bare jobjects; use a JavaRef as the input.
  ScopedJavaGlobalRef(JNIEnv* env, T obj) { Reset(env, obj); }

  ~ScopedJavaGlobalRef() { Reset(); }

  // Null assignment, for disambiguation.
  ScopedJavaGlobalRef& operator=(std::nullptr_t) {
    Reset();
    return *this;
  }

  // Copy assignment.
  ScopedJavaGlobalRef& operator=(const ScopedJavaGlobalRef& other) {
    Reset(other);
    return *this;
  }

  // Copy conversion assignment.
  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  ScopedJavaGlobalRef& operator=(const ScopedJavaGlobalRef<U>& other) {
    Reset(other);
    return *this;
  }

  // Move assignment.
  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  ScopedJavaGlobalRef& operator=(ScopedJavaGlobalRef<U>&& other) {
    Reset();
    JavaRef<T>::steal(std::move(other));
    return *this;
  }

  // Assignment for other JavaRef types.
  ScopedJavaGlobalRef& operator=(const JavaRef<T>& other) {
    Reset(other);
    return *this;
  }

  void Reset() { JavaRef<T>::ResetGlobalRef(); }

  template <typename U,
            typename = std::enable_if_t<std::is_convertible_v<U, T>>>
  void Reset(const ScopedJavaGlobalRef<U>& other) {
    Reset(nullptr, other.obj());
  }

  void Reset(const JavaRef<T>& other) { Reset(nullptr, other.obj()); }

  // Deprecated. You can just use Reset(const JavaRef&).
  void Reset(JNIEnv* env, const JavaParamRef<T>& other) {
    Reset(env, other.obj());
  }

  // Deprecated. Don't use bare jobjects; use a JavaRef as the input.
  void Reset(JNIEnv* env, T obj) { JavaRef<T>::SetNewGlobalRef(env, obj); }

  // Releases the global reference to the caller. The caller *must* delete the
  // global reference when it is done with it. Note that calling a Java method
  // is *not* a transfer of ownership and Release() should not be used.
  T Release() { return static_cast<T>(JavaRef<T>::ReleaseInternal()); }

  // Create a local reference.
  ScopedJavaLocalRef<T> AsLocalRef(JNIEnv* env) const {
    T j_obj = JavaRef<T>::obj();
    if (!j_obj) {
      return nullptr;
    }
    return ScopedJavaLocalRef<T>::Adopt(
        env, static_cast<T>(env->NewLocalRef(j_obj)));
  }
};

// Wrapper for working with weak references.
class JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaGlobalWeakRef {
 public:
  ScopedJavaGlobalWeakRef() = default;
  ScopedJavaGlobalWeakRef(const ScopedJavaGlobalWeakRef& orig);
  ScopedJavaGlobalWeakRef(ScopedJavaGlobalWeakRef&& orig) : obj_(orig.obj_) {
    orig.obj_ = nullptr;
  }
  ScopedJavaGlobalWeakRef(JNIEnv* env, const JavaRef<jobject>& obj);
  ~ScopedJavaGlobalWeakRef() { reset(); }

  void operator=(const ScopedJavaGlobalWeakRef& rhs) { Assign(rhs); }
  void operator=(ScopedJavaGlobalWeakRef&& rhs) { std::swap(obj_, rhs.obj_); }

  ScopedJavaLocalRef<jobject> get(JNIEnv* env) const;

  // Returns true if the weak reference has not been initialized to point at
  // an object (or ḣas had reset() called).
  // Do not call this to test if the object referred to still exists! The weak
  // reference remains initialized even if the target object has been collected.
  bool is_uninitialized() const { return obj_ == nullptr; }

  void reset();

 private:
  void Assign(const ScopedJavaGlobalWeakRef& rhs);

  jweak obj_ = nullptr;
};

// A global JavaRef that will never be released.
template <typename T>
class JNI_ZERO_COMPONENT_BUILD_EXPORT LeakedJavaGlobalRef : public JavaRef<T> {
 public:
  constexpr LeakedJavaGlobalRef() = default;
  constexpr LeakedJavaGlobalRef(std::nullptr_t) {}

  LeakedJavaGlobalRef(const LeakedJavaGlobalRef& other) = delete;
  LeakedJavaGlobalRef(const LeakedJavaGlobalRef&& other) = delete;
  ~LeakedJavaGlobalRef() = default;

  void Reset(JNIEnv* env, const JavaRef<T>& j_object) {
    JNI_ZERO_DCHECK(JavaRef<T>::obj() == nullptr);
    JavaRef<T>::SetNewGlobalRef(env, j_object.obj());
  }

  // Create a local reference.
  ScopedJavaLocalRef<T> AsLocalRef(JNIEnv* env) const {
    T j_obj = JavaRef<T>::obj();
    if (!j_obj) {
      return nullptr;
    }
    return ScopedJavaLocalRef<T>::Adopt(
        env, static_cast<T>(env->NewLocalRef(j_obj)));
  }
};
}  // namespace jni_zero

#endif  // JNI_ZERO_JAVA_REFS_H_
