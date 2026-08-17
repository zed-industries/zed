/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_TESTING_PLATFORM_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_TESTING_PLATFORM_SUPPORT_H_

#include <memory>
#include <utility>

#include "base/auto_reset.h"
#include "base/check_op.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/test/scoped_feature_list.h"
#include "third_party/blink/public/platform/platform.h"
#include "third_party/blink/renderer/platform/heap/heap_test_utilities.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8-platform.h"

namespace base {
class TestDiscardableMemoryAllocator;
}

namespace blink {

// A base class to override Platform methods for testing.  You can override the
// behavior by subclassing TestingPlatformSupport or using
// ScopedTestingPlatformSupport (see below).
class TestingPlatformSupport : public Platform {
 public:
  TestingPlatformSupport();
  TestingPlatformSupport(const TestingPlatformSupport&) = delete;
  TestingPlatformSupport& operator=(const TestingPlatformSupport&) = delete;

  ~TestingPlatformSupport() override;

  // Platform:
  WebString DefaultLocale() override;
  WebData GetDataResource(int resource_id,
                          ui::ResourceScaleFactor scale_factor) override;
  std::string GetDataResourceString(int resource_id) override;
  ThreadSafeBrowserInterfaceBrokerProxy* GetBrowserInterfaceBroker() override;
  bool IsThreadedAnimationEnabled() override;
  std::unique_ptr<blink::WebV8ValueConverter> CreateWebV8ValueConverter()
      override;

  virtual void RunUntilIdle();
  void SetThreadedAnimationEnabled(bool enabled);

  virtual const base::Clock* GetClock() const;
  virtual const base::TickClock* GetTickClock() const;
  virtual base::TimeTicks NowTicks() const;

  // Overrides the handling of GetInterface on the platform's associated
  // interface provider.
  class ScopedOverrideMojoInterface {
   public:
    using GetInterfaceCallback =
        base::RepeatingCallback<void(const char*,
                                     mojo::ScopedMessagePipeHandle)>;
    explicit ScopedOverrideMojoInterface(GetInterfaceCallback);
    ~ScopedOverrideMojoInterface();

   private:
    base::AutoReset<GetInterfaceCallback> auto_reset_;
  };

 protected:
  class TestingBrowserInterfaceBroker;

  const raw_ptr<Platform> old_platform_;
  scoped_refptr<TestingBrowserInterfaceBroker> interface_broker_;

 private:
  bool is_threaded_animation_enabled_ = false;
};

// ScopedTestingPlatformSupport<MyTestingPlatformSupport> can be used to
// override Platform::current() with MyTestingPlatformSupport, like this:
//
// #include
// "third_party/blink/renderer/platform/testing/testing_platform_support.h"
//
// TEST_F(SampleTest, sampleTest) {
//   ScopedTestingPlatformSupport<MyTestingPlatformSupport> platform;
//   ...
//   // You can call methods of MyTestingPlatformSupport.
//   EXPECT_TRUE(platform->myMethodIsCalled());
//
//   // Another instance can be nested.
//   {
//     // Constructor's arguments can be passed like this.
//     Arg arg;
//     ScopedTestingPlatformSupport<MyAnotherTestingPlatformSupport, const Arg&>
//         another_platform(args);
//     ...
//   }
//
//   // Here the original MyTestingPlatformSupport should be restored.
// }
template <class T, typename... Args>
class ScopedTestingPlatformSupport final {
 public:
  explicit ScopedTestingPlatformSupport(Args&&... args) {
    testing_platform_support_ =
        std::make_unique<T>(std::forward<Args>(args)...);
    original_platform_ = Platform::Current();
    DCHECK(original_platform_);
    Platform::SetCurrentPlatformForTesting(testing_platform_support_.get());
  }
  ScopedTestingPlatformSupport(const ScopedTestingPlatformSupport&) = delete;
  ScopedTestingPlatformSupport& operator=(const ScopedTestingPlatformSupport&) =
      delete;
  ~ScopedTestingPlatformSupport() {
    DCHECK_EQ(testing_platform_support_.get(), Platform::Current());
    testing_platform_support_.reset();
    Platform::SetCurrentPlatformForTesting(original_platform_);
  }

  const T* operator->() const { return testing_platform_support_.get(); }
  T* operator->() { return testing_platform_support_.get(); }

  const T& operator*() const { return *testing_platform_support_; }
  T& operator*() { return *testing_platform_support_; }

  T* GetTestingPlatformSupport() { return testing_platform_support_.get(); }

 private:
  std::unique_ptr<T> testing_platform_support_;
  raw_ptr<Platform> original_platform_;
};

class ScopedUnittestsEnvironmentSetup final {
  STACK_ALLOCATED();

 public:
  ScopedUnittestsEnvironmentSetup(int argc, char** argv);
  ScopedUnittestsEnvironmentSetup(const ScopedUnittestsEnvironmentSetup&) =
      delete;
  ScopedUnittestsEnvironmentSetup& operator=(
      const ScopedUnittestsEnvironmentSetup&) = delete;
  ~ScopedUnittestsEnvironmentSetup();

 private:
  base::test::ScopedFeatureList scoped_feature_list_;
  std::unique_ptr<base::TestDiscardableMemoryAllocator>
      discardable_memory_allocator_;
  std::unique_ptr<Platform> dummy_platform_;
  std::unique_ptr<v8::Platform> v8_platform_for_heap_testing_;
  std::unique_ptr<TestingPlatformSupport> testing_platform_support_;
  std::optional<HeapPointersOnStackScope> conservative_gc_scope_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_TESTING_PLATFORM_SUPPORT_H_
