// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef _JNI_ZERO_SAMPLE_FOR_TESTS_H_
#define _JNI_ZERO_SAMPLE_FOR_TESTS_H_

#include <jni.h>
#include <map>
#include <string>
#include <vector>
#include "third_party/jni_zero/jni_zero.h"

namespace jni_zero::tests {

enum class MyEnum {
  kFirstOption = 0,
  kSecondOption = 1,
  kMaxValue = kSecondOption
};

// This file is used to:
// - document the best practices and guidelines on JNI usage.
// - ensure sample_for_tests_jni.h compiles and the functions declared in it
//   are as expected.
//
// Methods are called directly from Java. More documentation in
// SampleForTests.java. See README.md for the build rules necessary for JNI to
// be used in an APK.
//
class CPPClass {
 public:
  CPPClass();

  CPPClass(const CPPClass&) = delete;
  CPPClass(CPPClass&&) = default;
  CPPClass& operator=(const CPPClass&) = delete;

  ~CPPClass();

  // Java @CalledByNative methods implicitly available to C++ via the _jni.h
  // file included in the .cc file.

  class InnerClass {
   public:
    jdouble MethodOtherP0(JNIEnv* env,
                          const jni_zero::JavaParamRef<jobject>& caller);
  };

  void Destroy(JNIEnv* env,
               const jni_zero::JavaParamRef<jobject>& caller,
               std::vector<uint8_t>& bytes);

  jint Method(JNIEnv* env,
              const jni_zero::JavaParamRef<jobject>& caller,
              std::vector<std::string>& strings);

  void AddStructB(JNIEnv* env,
                  const jni_zero::JavaParamRef<jobject>& caller,
                  const jni_zero::JavaParamRef<jobject>& structb);

  void IterateAndDoSomethingWithStructB(
      JNIEnv* env,
      const jni_zero::JavaParamRef<jobject>& caller);

  jni_zero::ScopedJavaLocalRef<jstring> ReturnAString(
      JNIEnv* env,
      const jni_zero::JavaParamRef<jobject>& caller);

 private:
  std::map<long, std::string> map_;
};

}  // namespace jni_zero::tests

#endif  // _JNI_ZERO_SAMPLE_FOR_TESTS_H_
