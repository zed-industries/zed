// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_TEST_COMPONENT_CONTEXT_FOR_PROCESS_H_
#define BASE_FUCHSIA_TEST_COMPONENT_CONTEXT_FOR_PROCESS_H_

#include <fidl/fuchsia.io/cpp/fidl.h>

#include <memory>
#include <string_view>

#include "base/base_export.h"
#include "base/containers/span.h"

namespace sys {
class ComponentContext;
class OutgoingDirectory;
class ServiceDirectory;
}  // namespace sys

namespace base {

class FilteredServiceDirectory;

// Replaces the process-global sys::ComponentContext (as returned by the
// base::ComponentContextForProcess() function) with an empty instance which the
// calling test can configure, and restores the original when deleted.
//
// The test ComponentContext runs on the test main thread, which means that:
// - Tests using TestComponentContextForProcess must instantiate a
//   [SingleThread]TaskEnvironment with UI or IO main-thread-type.
// - If all services exposed via the test ComponentContext run on the test
//   main thread, and the code under test does as well, then RunUntilIdle() can
//   normally be used to "flush" any pending FIDL requests and related work.
//   Note that this is not true if any services, or code under test, use threads
//   or processes!
//
// The test ComponentContext is typically instantiated within a test body or
// test base-class:
//
//   TEST(MyFunkyTest, IsFunky) {
//     TestComponentContextForProcess test_context;
//     // Configure the |test_context|.
//     // Run tests of code that uses ComponentContextForProcess().
//   }
//
// By default created context doesn't expose any services. Services from the
// original process-global ComponentContext (usually the environment in which
// the test process is running), can be exposed through the |test_context| with
// AddServices(), during test setup:
//
//   test_context.AddServices({fuchsia::memorypressure::Provider::Name_, ...});
//   // ... Execute tests which use fuchsia.memorypressure.Provider ...
//
// Alternatively InitialState::kCloneAll can be passed to the constructor to
// expose all services listed in /svc, e.g.:
//
//   TestComponentContextForProcess test_context(
//       TestComponentContextForProcess::InitialState::kCloneAll);
//
// Fake/mock implementations can be exposed via additional_services():
//
//   ScopedServiceBinding<funky::Service> binding(
//       test_context.additional_services(), &funky_implementation);
//   // ... Execute tests which use funky.Service.
//
// Services published to the process' ComponentContext by code-under-test
// can be accessed via published_services():
//
//   funky::HamsterPtr hamster_service;
//   test_context.published_services()->Connect(hamster_service.NewRequest());
//   // ... Execute tests which exercise the funky.Hamster implementation.
//
class BASE_EXPORT TestComponentContextForProcess {
 public:
  enum class InitialState {
    kEmpty,
    kCloneAll,
  };

  explicit TestComponentContextForProcess(
      InitialState initial_state = InitialState::kEmpty);
  ~TestComponentContextForProcess();

  TestComponentContextForProcess(const TestComponentContextForProcess&) =
      delete;
  TestComponentContextForProcess& operator=(
      const TestComponentContextForProcess&) = delete;

  // Returns an OutgoingDirectory into which additional services may be
  // published for use by the code-under test.
  sys::OutgoingDirectory* additional_services();

  // Allows the specified service(s) from the original ComponentContext to be
  // exposed via the test default ComponentContext.
  void AddService(std::string_view service);
  void AddServices(base::span<const std::string_view> services);

  // Returns the directory of services that the code under test has published
  // to its outgoing service directory.
  std::shared_ptr<sys::ServiceDirectory> published_services() const {
    return published_services_;
  }

  fidl::UnownedClientEnd<fuchsia_io::Directory> published_services_natural();

 private:
  std::unique_ptr<sys::ComponentContext> old_context_;

  std::unique_ptr<FilteredServiceDirectory> context_services_;
  std::shared_ptr<sys::ServiceDirectory> published_services_;
  fidl::ClientEnd<fuchsia_io::Directory> published_services_natural_;
};

}  // namespace base

#endif  // BASE_FUCHSIA_TEST_COMPONENT_CONTEXT_FOR_PROCESS_H_
