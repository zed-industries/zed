/*
 *  Copyright 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains Macros for creating proxies for webrtc MediaStream and
// PeerConnection classes.

// The proxied objects are initialized with either one or two thread
// objects that operations can be proxied to: The primary and secondary
// threads.
// In common usage, the primary thread will be the PeerConnection's
// signaling thread, and the secondary thread will be either the
// PeerConnection's worker thread or the PeerConnection's network thread.

//
// Example usage:
//
// class TestInterface : public RefCountInterface {
//  public:
//   std::string FooA() = 0;
//   std::string FooB(bool arg1) const = 0;
//   std::string FooC(bool arg1) = 0;
//  };
//
// Note that return types can not be a const reference.
//
// class Test : public TestInterface {
// ... implementation of the interface.
// };
//
// BEGIN_PROXY_MAP(Test)
//   PROXY_PRIMARY_THREAD_DESTRUCTOR()
//   PROXY_METHOD0(std::string, FooA)
//   PROXY_CONSTMETHOD1(std::string, FooB, arg1)
//   PROXY_SECONDARY_METHOD1(std::string, FooC, arg1)
// END_PROXY_MAP()
//
// Where the destructor and first two methods are invoked on the primary
// thread, and the third is invoked on the secondary thread.
//
// The proxy can be created using
//
//   TestProxy::Create(Thread* signaling_thread, Thread* worker_thread,
//                     TestInterface*).
//
// The variant defined with BEGIN_PRIMARY_PROXY_MAP is unaware of
// the secondary thread, and invokes all methods on the primary thread.
//

#ifndef PC_PROXY_H_
#define PC_PROXY_H_

#include <stddef.h>

#include <memory>
#include <string>
#include <tuple>
#include <type_traits>
#include <utility>

#include "api/make_ref_counted.h"
#include "api/scoped_refptr.h"
#include "api/task_queue/task_queue_base.h"
#include "rtc_base/event.h"
#include "rtc_base/string_utils.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread.h"
#include "rtc_base/trace_event.h"

#if !defined(RTC_DISABLE_PROXY_TRACE_EVENTS) && !defined(WEBRTC_CHROMIUM_BUILD)
#define RTC_DISABLE_PROXY_TRACE_EVENTS
#endif

namespace webrtc {

template <typename R>
class ReturnType {
 public:
  template <typename C, typename M, typename... Args>
  void Invoke(C* c, M m, Args&&... args) {
    r_ = (c->*m)(std::forward<Args>(args)...);
  }

  R moved_result() { return std::move(r_); }

 private:
  R r_;
};

template <>
class ReturnType<void> {
 public:
  template <typename C, typename M, typename... Args>
  void Invoke(C* c, M m, Args&&... args) {
    (c->*m)(std::forward<Args>(args)...);
  }

  void moved_result() {}
};

template <typename C, typename R, typename... Args>
class MethodCall {
 public:
  typedef R (C::*Method)(Args...);
  MethodCall(C* c, Method m, Args&&... args)
      : c_(c),
        m_(m),
        args_(std::forward_as_tuple(std::forward<Args>(args)...)) {}

  R Marshal(Thread* t) {
    if (t->IsCurrent()) {
      Invoke(std::index_sequence_for<Args...>());
    } else {
      t->PostTask([this] {
        Invoke(std::index_sequence_for<Args...>());
        event_.Set();
      });
      event_.Wait(Event::kForever);
    }
    return r_.moved_result();
  }

 private:
  template <size_t... Is>
  void Invoke(std::index_sequence<Is...>) {
    r_.Invoke(c_, m_, std::move(std::get<Is>(args_))...);
  }

  C* c_;
  Method m_;
  ReturnType<R> r_;
  std::tuple<Args&&...> args_;
  Event event_;
};

template <typename C, typename R, typename... Args>
class ConstMethodCall {
 public:
  typedef R (C::*Method)(Args...) const;
  ConstMethodCall(const C* c, Method m, Args&&... args)
      : c_(c),
        m_(m),
        args_(std::forward_as_tuple(std::forward<Args>(args)...)) {}

  R Marshal(Thread* t) {
    if (t->IsCurrent()) {
      Invoke(std::index_sequence_for<Args...>());
    } else {
      t->PostTask([this] {
        Invoke(std::index_sequence_for<Args...>());
        event_.Set();
      });
      event_.Wait(Event::kForever);
    }
    return r_.moved_result();
  }

 private:
  template <size_t... Is>
  void Invoke(std::index_sequence<Is...>) {
    r_.Invoke(c_, m_, std::move(std::get<Is>(args_))...);
  }

  const C* c_;
  Method m_;
  ReturnType<R> r_;
  std::tuple<Args&&...> args_;
  Event event_;
};

#define PROXY_STRINGIZE_IMPL(x) #x
#define PROXY_STRINGIZE(x) PROXY_STRINGIZE_IMPL(x)

// Helper macros to reduce code duplication.
#define PROXY_MAP_BOILERPLATE(class_name)                              \
  template <class INTERNAL_CLASS>                                      \
  class class_name##ProxyWithInternal;                                 \
  typedef class_name##ProxyWithInternal<class_name##Interface>         \
      class_name##Proxy;                                               \
  template <class INTERNAL_CLASS>                                      \
  class class_name##ProxyWithInternal : public class_name##Interface { \
   protected:                                                          \
    static constexpr char proxy_name_[] = #class_name "Proxy";         \
    typedef class_name##Interface C;                                   \
                                                                       \
   public:                                                             \
    const INTERNAL_CLASS* internal() const {                           \
      return c();                                                      \
    }                                                                  \
    INTERNAL_CLASS* internal() {                                       \
      return c();                                                      \
    }

// clang-format off
// clang-format would put the semicolon alone,
// leading to a presubmit error (cpplint.py)
#define END_PROXY_MAP(class_name)                                       \
  };                                                                    \
  template <class INTERNAL_CLASS>                                       \
  constexpr char class_name##ProxyWithInternal<INTERNAL_CLASS>::proxy_name_[];
// clang-format on

#define PRIMARY_PROXY_MAP_BOILERPLATE(class_name)                \
 protected:                                                      \
  class_name##ProxyWithInternal(Thread* primary_thread,          \
                                scoped_refptr<INTERNAL_CLASS> c) \
      : primary_thread_(primary_thread), c_(std::move(c)) {}     \
                                                                 \
 private:                                                        \
  mutable Thread* primary_thread_;

#define SECONDARY_PROXY_MAP_BOILERPLATE(class_name)              \
 protected:                                                      \
  class_name##ProxyWithInternal(Thread* primary_thread,          \
                                Thread* secondary_thread,        \
                                scoped_refptr<INTERNAL_CLASS> c) \
      : primary_thread_(primary_thread),                         \
        secondary_thread_(secondary_thread),                     \
        c_(std::move(c)) {}                                      \
                                                                 \
 private:                                                        \
  mutable Thread* primary_thread_;                               \
  mutable Thread* secondary_thread_;

// Note that the destructor is protected so that the proxy can only be
// destroyed via RefCountInterface.
#define REFCOUNTED_PROXY_MAP_BOILERPLATE(class_name)            \
 protected:                                                     \
  ~class_name##ProxyWithInternal() {                            \
    MethodCall<class_name##ProxyWithInternal, void> call(       \
        this, &class_name##ProxyWithInternal::DestroyInternal); \
    call.Marshal(destructor_thread());                          \
  }                                                             \
                                                                \
 private:                                                       \
  const INTERNAL_CLASS* c() const {                             \
    return c_.get();                                            \
  }                                                             \
  INTERNAL_CLASS* c() {                                         \
    return c_.get();                                            \
  }                                                             \
  void DestroyInternal() {                                      \
    c_ = nullptr;                                               \
  }                                                             \
  scoped_refptr<INTERNAL_CLASS> c_;

// Note: This doesn't use a unique_ptr, because it intends to handle a corner
// case where an object's deletion triggers a callback that calls back into
// this proxy object. If relying on a unique_ptr to delete the object, its
// inner pointer would be set to null before this reentrant callback would have
// a chance to run, resulting in a segfault.
#define OWNED_PROXY_MAP_BOILERPLATE(class_name)                 \
 public:                                                        \
  ~class_name##ProxyWithInternal() {                            \
    MethodCall<class_name##ProxyWithInternal, void> call(       \
        this, &class_name##ProxyWithInternal::DestroyInternal); \
    call.Marshal(destructor_thread());                          \
  }                                                             \
                                                                \
 private:                                                       \
  const INTERNAL_CLASS* c() const {                             \
    return c_;                                                  \
  }                                                             \
  INTERNAL_CLASS* c() {                                         \
    return c_;                                                  \
  }                                                             \
  void DestroyInternal() {                                      \
    delete c_;                                                  \
  }                                                             \
  INTERNAL_CLASS* c_;

#define BEGIN_PRIMARY_PROXY_MAP(class_name)                                \
  PROXY_MAP_BOILERPLATE(class_name)                                        \
  PRIMARY_PROXY_MAP_BOILERPLATE(class_name)                                \
  REFCOUNTED_PROXY_MAP_BOILERPLATE(class_name)                             \
 public:                                                                   \
  static scoped_refptr<class_name##ProxyWithInternal> Create(              \
      Thread* primary_thread, scoped_refptr<INTERNAL_CLASS> c) {           \
    return make_ref_counted<class_name##ProxyWithInternal>(primary_thread, \
                                                           std::move(c));  \
  }

#define BEGIN_PROXY_MAP(class_name)                           \
  PROXY_MAP_BOILERPLATE(class_name)                           \
  SECONDARY_PROXY_MAP_BOILERPLATE(class_name)                 \
  REFCOUNTED_PROXY_MAP_BOILERPLATE(class_name)                \
 public:                                                      \
  static scoped_refptr<class_name##ProxyWithInternal> Create( \
      Thread* primary_thread, Thread* secondary_thread,       \
      scoped_refptr<INTERNAL_CLASS> c) {                      \
    return make_ref_counted<class_name##ProxyWithInternal>(   \
        primary_thread, secondary_thread, std::move(c));      \
  }

#define PROXY_PRIMARY_THREAD_DESTRUCTOR() \
 private:                                 \
  Thread* destructor_thread() const {     \
    return primary_thread_;               \
  }                                       \
                                          \
 public:  // NOLINTNEXTLINE

#define PROXY_SECONDARY_THREAD_DESTRUCTOR() \
 private:                                   \
  Thread* destructor_thread() const {       \
    return secondary_thread_;               \
  }                                         \
                                            \
 public:  // NOLINTNEXTLINE

#if defined(RTC_DISABLE_PROXY_TRACE_EVENTS)
#define TRACE_BOILERPLATE(method) \
  do {                            \
  } while (0)
#else  // if defined(RTC_DISABLE_PROXY_TRACE_EVENTS)
#define TRACE_BOILERPLATE(method)                          \
  static constexpr auto class_and_method_name =            \
      webrtc::MakeCompileTimeString(proxy_name_)           \
          .Concat(webrtc::MakeCompileTimeString("::"))     \
          .Concat(webrtc::MakeCompileTimeString(#method)); \
  TRACE_EVENT0("webrtc", class_and_method_name.string)

#endif  // if defined(RTC_DISABLE_PROXY_TRACE_EVENTS)

#define PROXY_METHOD0(r, method)            \
  r method() override {                     \
    TRACE_BOILERPLATE(method);              \
    MethodCall<C, r> call(c(), &C::method); \
    return call.Marshal(primary_thread_);   \
  }

#define PROXY_CONSTMETHOD0(r, method)            \
  r method() const override {                    \
    TRACE_BOILERPLATE(method);                   \
    ConstMethodCall<C, r> call(c(), &C::method); \
    return call.Marshal(primary_thread_);        \
  }

#define PROXY_METHOD1(r, method, t1)                           \
  r method(t1 a1) override {                                   \
    TRACE_BOILERPLATE(method);                                 \
    MethodCall<C, r, t1> call(c(), &C::method, std::move(a1)); \
    return call.Marshal(primary_thread_);                      \
  }

#define PROXY_CONSTMETHOD1(r, method, t1)                           \
  r method(t1 a1) const override {                                  \
    TRACE_BOILERPLATE(method);                                      \
    ConstMethodCall<C, r, t1> call(c(), &C::method, std::move(a1)); \
    return call.Marshal(primary_thread_);                           \
  }

#define PROXY_METHOD2(r, method, t1, t2)                          \
  r method(t1 a1, t2 a2) override {                               \
    TRACE_BOILERPLATE(method);                                    \
    MethodCall<C, r, t1, t2> call(c(), &C::method, std::move(a1), \
                                  std::move(a2));                 \
    return call.Marshal(primary_thread_);                         \
  }

#define PROXY_METHOD3(r, method, t1, t2, t3)                          \
  r method(t1 a1, t2 a2, t3 a3) override {                            \
    TRACE_BOILERPLATE(method);                                        \
    MethodCall<C, r, t1, t2, t3> call(c(), &C::method, std::move(a1), \
                                      std::move(a2), std::move(a3));  \
    return call.Marshal(primary_thread_);                             \
  }

#define PROXY_METHOD4(r, method, t1, t2, t3, t4)                          \
  r method(t1 a1, t2 a2, t3 a3, t4 a4) override {                         \
    TRACE_BOILERPLATE(method);                                            \
    MethodCall<C, r, t1, t2, t3, t4> call(c(), &C::method, std::move(a1), \
                                          std::move(a2), std::move(a3),   \
                                          std::move(a4));                 \
    return call.Marshal(primary_thread_);                                 \
  }

#define PROXY_METHOD5(r, method, t1, t2, t3, t4, t5)                          \
  r method(t1 a1, t2 a2, t3 a3, t4 a4, t5 a5) override {                      \
    TRACE_BOILERPLATE(method);                                                \
    MethodCall<C, r, t1, t2, t3, t4, t5> call(c(), &C::method, std::move(a1), \
                                              std::move(a2), std::move(a3),   \
                                              std::move(a4), std::move(a5));  \
    return call.Marshal(primary_thread_);                                     \
  }

// Define methods which should be invoked on the secondary thread.
#define PROXY_SECONDARY_METHOD0(r, method)  \
  r method() override {                     \
    TRACE_BOILERPLATE(method);              \
    MethodCall<C, r> call(c(), &C::method); \
    return call.Marshal(secondary_thread_); \
  }

#define PROXY_SECONDARY_CONSTMETHOD0(r, method)  \
  r method() const override {                    \
    TRACE_BOILERPLATE(method);                   \
    ConstMethodCall<C, r> call(c(), &C::method); \
    return call.Marshal(secondary_thread_);      \
  }

#define PROXY_SECONDARY_METHOD1(r, method, t1)                 \
  r method(t1 a1) override {                                   \
    TRACE_BOILERPLATE(method);                                 \
    MethodCall<C, r, t1> call(c(), &C::method, std::move(a1)); \
    return call.Marshal(secondary_thread_);                    \
  }

#define PROXY_SECONDARY_CONSTMETHOD1(r, method, t1)                 \
  r method(t1 a1) const override {                                  \
    TRACE_BOILERPLATE(method);                                      \
    ConstMethodCall<C, r, t1> call(c(), &C::method, std::move(a1)); \
    return call.Marshal(secondary_thread_);                         \
  }

#define PROXY_SECONDARY_METHOD2(r, method, t1, t2)                \
  r method(t1 a1, t2 a2) override {                               \
    TRACE_BOILERPLATE(method);                                    \
    MethodCall<C, r, t1, t2> call(c(), &C::method, std::move(a1), \
                                  std::move(a2));                 \
    return call.Marshal(secondary_thread_);                       \
  }

#define PROXY_SECONDARY_CONSTMETHOD2(r, method, t1, t2)                \
  r method(t1 a1, t2 a2) const override {                              \
    TRACE_BOILERPLATE(method);                                         \
    ConstMethodCall<C, r, t1, t2> call(c(), &C::method, std::move(a1), \
                                       std::move(a2));                 \
    return call.Marshal(secondary_thread_);                            \
  }

#define PROXY_SECONDARY_METHOD3(r, method, t1, t2, t3)                \
  r method(t1 a1, t2 a2, t3 a3) override {                            \
    TRACE_BOILERPLATE(method);                                        \
    MethodCall<C, r, t1, t2, t3> call(c(), &C::method, std::move(a1), \
                                      std::move(a2), std::move(a3));  \
    return call.Marshal(secondary_thread_);                           \
  }

#define PROXY_SECONDARY_CONSTMETHOD3(r, method, t1, t2)                    \
  r method(t1 a1, t2 a2, t3 a3) const override {                           \
    TRACE_BOILERPLATE(method);                                             \
    ConstMethodCall<C, r, t1, t2, t3> call(c(), &C::method, std::move(a1), \
                                           std::move(a2), std::move(a3));  \
    return call.Marshal(secondary_thread_);                                \
  }

// For use when returning purely const state (set during construction).
// Use with caution. This method should only be used when the return value will
// always be the same.
#define BYPASS_PROXY_CONSTMETHOD0(r, method) \
  r method() const override {                \
    TRACE_BOILERPLATE(method);               \
    return c_->method();                     \
  }
// Allows a custom implementation of a method where the otherwise proxied
// implementation can do a more efficient, yet thread-safe, job than the proxy
// can do by default or when more flexibility is needed than can be provided
// by a proxy.
// Note that calls to these methods should be expected to be made from unknown
// threads.
#define BYPASS_PROXY_METHOD0(r, method) \
  r method() override {                 \
    TRACE_BOILERPLATE(method);          \
    return c_->method();                \
  }

// The 1 argument version of `BYPASS_PROXY_METHOD0`.
#define BYPASS_PROXY_METHOD1(r, method, t1) \
  r method(t1 a1) override {                \
    TRACE_BOILERPLATE(method);              \
    return c_->method(std::move(a1));       \
  }

// The 2 argument version of `BYPASS_PROXY_METHOD0`.
#define BYPASS_PROXY_METHOD2(r, method, t1, t2)      \
  r method(t1 a1, t2 a2) override {                  \
    TRACE_BOILERPLATE(method);                       \
    return c_->method(std::move(a1), std::move(a2)); \
  }
}  // namespace webrtc

#endif  //  PC_PROXY_H_
