/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_UTILITY_INCLUDE_JVM_ANDROID_H_
#define MODULES_UTILITY_INCLUDE_JVM_ANDROID_H_

#include <jni.h>

#include <memory>
#include <string>

#include "api/sequence_checker.h"
#include "modules/utility/include/helpers_android.h"

namespace webrtc {

// RAII JavaVM AttachCurrentThread/DetachCurrentThread object.
//
// The JNI interface pointer (JNIEnv) is valid only in the current thread.
// Should another thread need to access the Java VM, it must first call
// AttachCurrentThread() to attach itself to the VM and obtain a JNI interface
// pointer. The native thread remains attached to the VM until it calls
// DetachCurrentThread() to detach.
class JvmThreadConnector {
 public:
  JvmThreadConnector();
  ~JvmThreadConnector();

 private:
  SequenceChecker thread_checker_;
  bool attached_;
};

// This class is created by the NativeRegistration class and is used to wrap
// the actual Java object handle (jobject) on which we can call methods from
// C++ in to Java. See example in JVM for more details.
// TODO(henrika): extend support for type of function calls.
class GlobalRef {
 public:
  GlobalRef(JNIEnv* jni, jobject object);
  ~GlobalRef();

  jboolean CallBooleanMethod(jmethodID methodID, ...);
  jint CallIntMethod(jmethodID methodID, ...);
  void CallVoidMethod(jmethodID methodID, ...);

 private:
  JNIEnv* const jni_;
  const jobject j_object_;
};

// Wraps the jclass object on which we can call GetMethodId() functions to
// query method IDs.
class JavaClass {
 public:
  JavaClass(JNIEnv* jni, jclass clazz) : jni_(jni), j_class_(clazz) {}
  ~JavaClass() {}

  jmethodID GetMethodId(const char* name, const char* signature);
  jmethodID GetStaticMethodId(const char* name, const char* signature);
  jobject CallStaticObjectMethod(jmethodID methodID, ...);
  jint CallStaticIntMethod(jmethodID methodID, ...);

 protected:
  JNIEnv* const jni_;
  jclass const j_class_;
};

// Adds support of the NewObject factory method to the JavaClass class.
// See example in JVM for more details on how to use it.
class NativeRegistration : public JavaClass {
 public:
  NativeRegistration(JNIEnv* jni, jclass clazz);
  ~NativeRegistration();

  std::unique_ptr<GlobalRef> NewObject(const char* name,
                                       const char* signature,
                                       ...);

 private:
  JNIEnv* const jni_;
};

// This class is created by the JVM class and is used to expose methods that
// needs the JNI interface pointer but its main purpose is to create a
// NativeRegistration object given name of a Java class and a list of native
// methods. See example in JVM for more details.
class JNIEnvironment {
 public:
  explicit JNIEnvironment(JNIEnv* jni);
  ~JNIEnvironment();

  // Registers native methods with the Java class specified by `name`.
  // Note that the class name must be one of the names in the static
  // `loaded_classes` array defined in jvm_android.cc.
  // This method must be called on the construction thread.
  std::unique_ptr<NativeRegistration> RegisterNatives(
      const char* name,
      const JNINativeMethod* methods,
      int num_methods);

  // Converts from Java string to std::string.
  // This method must be called on the construction thread.
  std::string JavaToStdString(const jstring& j_string);

 private:
  SequenceChecker thread_checker_;
  JNIEnv* const jni_;
};

// Main class for working with Java from C++ using JNI in WebRTC.
//
// Example usage:
//
//   // At initialization (e.g. in JNI_OnLoad), call JVM::Initialize.
//   JNIEnv* jni = ::base::android::AttachCurrentThread();
//   JavaVM* jvm = NULL;
//   jni->GetJavaVM(&jvm);
//   webrtc::JVM::Initialize(jvm);
//
//   // Header (.h) file of example class called User.
//   std::unique_ptr<JNIEnvironment> env;
//   std::unique_ptr<NativeRegistration> reg;
//   std::unique_ptr<GlobalRef> obj;
//
//   // Construction (in .cc file) of User class.
//   User::User() {
//     // Calling thread must be attached to the JVM.
//     env = JVM::GetInstance()->environment();
//     reg = env->RegisterNatives("org/webrtc/WebRtcTest", ,);
//     obj = reg->NewObject("<init>", ,);
//   }
//
//   // Each User method can now use `reg` and `obj` and call Java functions
//   // in WebRtcTest.java, e.g. boolean init() {}.
//   bool User::Foo() {
//     jmethodID id = reg->GetMethodId("init", "()Z");
//     return obj->CallBooleanMethod(id);
//   }
//
//   // And finally, e.g. in JNI_OnUnLoad, call JVM::Uninitialize.
//   JVM::Uninitialize();
class JVM {
 public:
  // Stores global handles to the Java VM interface.
  // Should be called once on a thread that is attached to the JVM.
  static void Initialize(JavaVM* jvm);
  // Like the method above but also passes the context to the ContextUtils
  // class. This method should be used by pure-C++ Android users that can't call
  // ContextUtils.initialize directly.
  static void Initialize(JavaVM* jvm, jobject context);
  // Clears handles stored in Initialize(). Must be called on same thread as
  // Initialize().
  static void Uninitialize();
  // Gives access to the global Java VM interface pointer, which then can be
  // used to create a valid JNIEnvironment object or to get a JavaClass object.
  static JVM* GetInstance();

  // Creates a JNIEnvironment object.
  // This method returns a NULL pointer if AttachCurrentThread() has not been
  // called successfully. Use the AttachCurrentThreadIfNeeded class if needed.
  std::unique_ptr<JNIEnvironment> environment();

  // Returns a JavaClass object given class `name`.
  // Note that the class name must be one of the names in the static
  // `loaded_classes` array defined in jvm_android.cc.
  // This method must be called on the construction thread.
  JavaClass GetClass(const char* name);

  // TODO(henrika): can we make these private?
  JavaVM* jvm() const { return jvm_; }

 protected:
  JVM(JavaVM* jvm);
  ~JVM();

 private:
  JNIEnv* jni() const { return GetEnv(jvm_); }

  SequenceChecker thread_checker_;
  JavaVM* const jvm_;
};

}  // namespace webrtc

#endif  // MODULES_UTILITY_INCLUDE_JVM_ANDROID_H_
