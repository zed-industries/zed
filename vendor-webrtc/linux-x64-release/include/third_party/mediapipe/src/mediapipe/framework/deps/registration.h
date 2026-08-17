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

#ifndef MEDIAPIPE_DEPS_REGISTRATION_H_
#define MEDIAPIPE_DEPS_REGISTRATION_H_

#include <algorithm>
#include <functional>
#include <string>
#include <tuple>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#include <utility>

#include "absl/base/macros.h"
#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/meta/type_traits.h"
#include "absl/strings/str_join.h"
#include "absl/strings/str_split.h"
#include "absl/strings/string_view.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/deps/registration_token.h"
#include "mediapipe/framework/port/canonical_errors.h"
#include "mediapipe/framework/port/statusor.h"

namespace mediapipe {

// Usage:
//
// === Defining a registry ================================================
//
//  class Widget {};
//
//  using WidgetRegistry =
//      GlobalFactoryRegistry<unique_ptr<Widget>,                 // return
//                            unique_ptr<Gadget>, const Thing*>   // args
//
// === Registering an implementation =======================================
//
//  class MyWidget : public Widget {
//    static unique_ptr<Widget> Create(unique_ptr<Gadget> arg,
//                                     const Thing* thing) {
//      return MakeUnique<Widget>(std::move(arg), thing);
//    }
//    ...
//  };
//
//  REGISTER_FACTORY_FUNCTION_QUALIFIED(
//      WidgetRegistry, widget_registration,
//      ::my_ns::MyWidget, MyWidget::Create);
//
// === Using std::function =================================================
//
//  class Client {};
//
//  using ClientRegistry =
//      GlobalFactoryRegistry<absl::StatusOr<unique_ptr<Client>>;
//
//  class MyClient : public Client {
//   public:
//    MyClient(unique_ptr<Backend> backend)
//      : backend_(std::move(backend)) {}
//   private:
//     const std::unique_ptr<Backend> backend_;
//  };
//
//  // Any std::function that returns a Client is valid to pass here. Below,
//  // we use a lambda.
//  REGISTER_FACTORY_FUNCTION_QUALIFIED(
//      ClientRegistry, client_registration,
//      ::my_ns::MyClient,
//      []() {
//        auto backend = absl::make_unique<Backend>("/path/to/backend");
//        const absl::Status status = backend->Init();
//        if (!status.ok()) {
//          return status;
//        }
//        std::unique_ptr<Client> client
//            = absl::make_unique<MyClient>(std::move(backend));
//        return client;
//      });
//
// === Using the registry to create instances ==============================
//
//  // Registry will return absl::StatusOr<Object>
//  absl::StatusOr<unique_ptr<Widget>> s_or_widget =
//      WidgetRegistry::CreateByName(
//          "my_ns.MyWidget", std::move(gadget), thing);
//  // Registry will return NOT_FOUND if the name is unknown.
//  if (!s_or_widget.ok()) ... // handle error
//  DoStuffWithWidget(std::move(s_or_widget).value());
//
//  // It's also possible to find an instance by name within a source namespace.
//  auto s_or_widget = WidgetRegistry::CreateByNameInNamespace(
//      "my_ns.sub_namespace", "MyWidget");
//
//  // It's also possible to just check if a name is registered without creating
//  // an instance.
//  bool registered = WidgetRegistry::IsRegistered("my_ns::MyWidget");
//
//  // It's also possible to iterate through all registered function names.
//  // This might be useful if clients outside of your codebase are registering
//  // plugins.
//  for (const auto& name : WidgetRegistry::GetRegisteredNames()) {
//    absl::StatusOr<unique_ptr<Widget>> s_or_widget =
//        WidgetRegistry::CreateByName(name, std::move(gadget), thing);
//    ...
//  }
//
// === Injecting instances for testing =====================================
//
// Unregister unregisterer(WidgetRegistry::Register(
//     "MockWidget",
//      [](unique_ptr<Gadget> arg, const Thing* thing) {
//        ...
//      }));

namespace registration_internal {
inline constexpr char kCxxSep[] = "::";
inline constexpr char kNameSep[] = ".";

template <typename T>
struct WrapStatusOr {
  using type = absl::StatusOr<T>;
};

// Specialization to avoid double-wrapping types that are already StatusOrs.
template <typename T>
struct WrapStatusOr<absl::StatusOr<T>> {
  using type = absl::StatusOr<T>;
};

// Defining a member of this type causes P to be ODR-used, which forces its
// instantiation if it's a static member of a template.
// Previously we depended on the pointer's value to determine whether the size
// of a character array is 0 or 1, forcing it to be instantiated so the
// compiler can determine the object's layout. But using it as a template
// argument is more compact.
template <auto* P>
struct ForceStaticInstantiation {
#ifdef _MSC_VER
  // Just having it as the template argument does not count as a use for
  // MSVC.
  static constexpr bool Use() { return P != nullptr; }
  char force_static[Use()];
#endif  // _MSC_VER
};

}  // namespace registration_internal

class NamespaceAllowlist {
 public:
  static const absl::flat_hash_set<std::string>& TopNamespaces();
};

template <typename R, typename... Args>
class FunctionRegistry {
 public:
  using Function = std::function<R(Args...)>;
  using ReturnType = typename registration_internal::WrapStatusOr<R>::type;

  FunctionRegistry() {}
  FunctionRegistry(const FunctionRegistry&) = delete;
  FunctionRegistry& operator=(const FunctionRegistry&) = delete;

  RegistrationToken Register(absl::string_view name, Function func)
      ABSL_LOCKS_EXCLUDED(lock_) {
    std::string normalized_name = GetNormalizedName(name);
    absl::WriterMutexLock lock(&lock_);
    std::string adjusted_name = GetAdjustedName(normalized_name);
    if (adjusted_name != normalized_name) {
      functions_.insert(std::make_pair(adjusted_name, func));
    }
    if (functions_.insert(std::make_pair(normalized_name, std::move(func)))
            .second) {
      return RegistrationToken(
          [this, normalized_name]() { Unregister(normalized_name); });
    }
    ABSL_LOG(FATAL) << "Function with name " << name << " already registered.";
    return RegistrationToken([]() {});
  }

  // Force 'args' to be deduced by templating the function, instead of just
  // accepting Args. This is necessary to make 'args' a forwarding reference as
  // opposed to a plain rvalue reference.
  // https://isocpp.org/blog/2012/11/universal-references-in-c11-scott-meyers
  //
  // The absl::enable_if_t is used to disable this method if Args2 are not
  // convertible to Args. This will allow the compiler to identify the offending
  // line (i.e. the line where the method is called) in the first error message,
  // rather than nesting it multiple levels down the error stack.
  template <typename... Args2,
            absl::enable_if_t<std::is_convertible<std::tuple<Args2...>,
                                                  std::tuple<Args...>>::value,
                              int> = 0>
  ReturnType Invoke(absl::string_view name, Args2&&... args)
      ABSL_LOCKS_EXCLUDED(lock_) {
    Function function;
    {
      absl::ReaderMutexLock lock(&lock_);
      auto it = functions_.find(name);
      if (it == functions_.end()) {
        return absl::NotFoundError(
            absl::StrCat("No registered object with name: ", name));
      }
      function = it->second;
    }
    return function(std::forward<Args2>(args)...);
  }

  // Invokes the specified factory function and returns the result.
  // Namespaces in |name| and |ns| are separated by kNameSep.
  template <typename... Args2>
  ReturnType Invoke(absl::string_view ns, absl::string_view name,
                    Args2&&... args) ABSL_LOCKS_EXCLUDED(lock_) {
    return Invoke(GetQualifiedName(ns, name), args...);
  }

  // Note that it's possible for registered implementations to be subsequently
  // unregistered, though this will never happen with registrations made via
  // MEDIAPIPE_REGISTER_FACTORY_FUNCTION.
  bool IsRegistered(absl::string_view name) const ABSL_LOCKS_EXCLUDED(lock_) {
    absl::ReaderMutexLock lock(&lock_);
    return functions_.count(name) != 0;
  }

  // Returns true if the specified factory function is available.
  // Namespaces in |name| and |ns| are separated by kNameSep.
  bool IsRegistered(absl::string_view ns, absl::string_view name) const
      ABSL_LOCKS_EXCLUDED(lock_) {
    return IsRegistered(GetQualifiedName(ns, name));
  }

  // Returns a vector of all registered function names.
  // Note that it's possible for registered implementations to be subsequently
  // unregistered, though this will never happen with registrations made via
  // MEDIAPIPE_REGISTER_FACTORY_FUNCTION.
  std::unordered_set<std::string> GetRegisteredNames() const
      ABSL_LOCKS_EXCLUDED(lock_) {
    absl::ReaderMutexLock lock(&lock_);
    std::unordered_set<std::string> names;
    std::for_each(functions_.cbegin(), functions_.cend(),
                  [&names](const std::pair<const std::string, Function>& pair) {
                    names.insert(pair.first);
                  });
    return names;
  }

  // Normalizes a C++ qualified name.  Validates the name qualification.
  // The name must be either unqualified or fully qualified with a leading "::".
  // The leading "::" in a fully qualified name is stripped.
  std::string GetNormalizedName(absl::string_view name) {
    using ::mediapipe::registration_internal::kCxxSep;
    std::vector<std::string> names = absl::StrSplit(name, kCxxSep);
    if (names[0].empty()) {
      names.erase(names.begin());
    } else {
      ABSL_CHECK_EQ(1u, names.size())
          << "A registered class name must be either fully qualified "
          << "with a leading :: or unqualified, got: " << name << ".";
    }
    return absl::StrJoin(names, kCxxSep);
  }

  // Returns the registry key for a name specified within a namespace.
  // Namespaces are separated by kNameSep.
  std::string GetQualifiedName(absl::string_view ns,
                               absl::string_view name) const {
    using ::mediapipe::registration_internal::kCxxSep;
    using ::mediapipe::registration_internal::kNameSep;
    std::vector<std::string> names = absl::StrSplit(name, kNameSep);
    if (names[0].empty()) {
      names.erase(names.begin());
      return absl::StrJoin(names, kCxxSep);
    }
    std::string cxx_name = absl::StrJoin(names, kCxxSep);
    if (ns.empty()) {
      return cxx_name;
    }
    std::vector<std::string> spaces = absl::StrSplit(ns, kNameSep);
    absl::ReaderMutexLock lock(&lock_);
    while (!spaces.empty()) {
      std::string cxx_ns = absl::StrJoin(spaces, kCxxSep);
      std::string qualified_name = absl::StrCat(cxx_ns, kCxxSep, cxx_name);
      if (functions_.count(qualified_name)) {
        return qualified_name;
      }
      spaces.pop_back();
    }
    return cxx_name;
  }

  // Returns a type name with '.' separated namespaces.
  static std::string GetLookupName(const absl::string_view cxx_type_name) {
    constexpr absl::string_view kCxxSep = "::";
    constexpr absl::string_view kNameSep = ".";
    std::vector<absl::string_view> names =
        absl::StrSplit(cxx_type_name, kCxxSep);
    if (names[0].empty()) {
      names.erase(names.begin());
    }
    return absl::StrJoin(names, kNameSep);
  }

 private:
  mutable absl::Mutex lock_;
  absl::flat_hash_map<std::string, Function> functions_ ABSL_GUARDED_BY(lock_);

  // For names included in NamespaceAllowlist, strips the namespace.
  std::string GetAdjustedName(absl::string_view name) {
    using ::mediapipe::registration_internal::kCxxSep;
    std::vector<std::string> names = absl::StrSplit(name, kCxxSep);
    std::string base_name = names.back();
    names.pop_back();
    std::string ns = absl::StrJoin(names, kCxxSep);
    if (NamespaceAllowlist::TopNamespaces().count(ns)) {
      return base_name;
    }
    return std::string(name);
  }

  void Unregister(absl::string_view name) {
    absl::WriterMutexLock lock(&lock_);
    std::string adjusted_name = GetAdjustedName(name);
    if (adjusted_name != name) {
      functions_.erase(adjusted_name);
    }
    functions_.erase(name);
  }
};

template <typename R, typename... Args>
class GlobalFactoryRegistry {
  using Functions = FunctionRegistry<R, Args...>;

 public:
  static RegistrationToken Register(absl::string_view name,
                                    typename Functions::Function func) {
    return functions()->Register(name, std::move(func));
  }

  // Invokes the specified factory function and returns the result.
  // If using namespaces with this registry, the variant with a namespace
  // argument should be used.
  template <typename... Args2>
  static typename Functions::ReturnType CreateByName(absl::string_view name,
                                                     Args2&&... args) {
    return functions()->Invoke(name, std::forward<Args2>(args)...);
  }

  // Returns true if the specified factory function is available.
  // If using namespaces with this registry, the variant with a namespace
  // argument should be used.
  static bool IsRegistered(absl::string_view name) {
    return functions()->IsRegistered(name);
  }

  static std::unordered_set<std::string> GetRegisteredNames() {
    return functions()->GetRegisteredNames();
  }

  // Invokes the specified factory function and returns the result.
  // Namespaces in |name| and |ns| are separated by kNameSep.
  // See comments re: use of Args2 and absl::enable_if_t on Invoke.
  template <typename... Args2,
            absl::enable_if_t<std::is_convertible<std::tuple<Args2...>,
                                                  std::tuple<Args...>>::value,
                              int> = 0>
  static typename Functions::ReturnType CreateByNameInNamespace(
      absl::string_view ns, absl::string_view name, Args2&&... args) {
    return functions()->Invoke(ns, name, std::forward<Args2>(args)...);
  }

  // Returns true if the specified factory function is available.
  // Namespaces in |name| and |ns| are separated by kNameSep.
  static bool IsRegistered(absl::string_view ns, absl::string_view name) {
    return functions()->IsRegistered(ns, name);
  }

  // Returns the factory function registry singleton.
  static Functions* functions() {
    static auto* functions = new Functions();
    return functions;
  }

 private:
  GlobalFactoryRegistry() = delete;
};

// Two levels of macros are required to convert __LINE__ into a string
// containing the line number.
#define REGISTRY_STATIC_VAR_INNER(var_name, line) var_name##_##line##__
#define REGISTRY_STATIC_VAR(var_name, line) \
  REGISTRY_STATIC_VAR_INNER(var_name, line)

// Disables all static registration in MediaPipe accomplished using:
// - REGISTER_FACTORY_FUNCTION_QUALIFIED
// - MEDIAPIPE_REGISTER_FACTORY_FUNCTION
// - MEDIAPIPE_STATIC_REGISTRATOR_TEMPLATE
//
// Which includes:
// - calculators
// - input stream handlers
// - output stream handlers
// - generators
// - anything else registered using above macros
#if !defined(MEDIAPIPE_DISABLE_STATIC_REGISTRATION)
#define MEDIAPIPE_DISABLE_STATIC_REGISTRATION 0
#endif  // !defined(MEDIAPIPE_DISABLE_STATIC_REGISTRATION)

// Enables "Dry Run" for MediaPipe static registration: MediaPipe logs the
// registration code, instead of actual registration.
//
// The intended use: if you plan to disable static registration using
// MEDIAPIPE_DISABLE_STATIC_REGISTRATION, you may find it useful to build your
// MediaPipe dependency first with only:
//   MEDIAPIPE_ENABLE_STATIC_REGISTRATION_DRY_RUN
// and load it to see what manual registration will be required when you build
// with:
//   MEDIAPIPE_DISABLE_STATIC_REGISTRATION
#if !defined(MEDIAPIPE_ENABLE_STATIC_REGISTRATION_DRY_RUN)
#define MEDIAPIPE_ENABLE_STATIC_REGISTRATION_DRY_RUN 0
#endif  // !defined(MEDIAPIPE_ENABLE_STATIC_REGISTRATION_DRY_RUN)

#if MEDIAPIPE_DISABLE_STATIC_REGISTRATION && \
    MEDIAPIPE_ENABLE_STATIC_REGISTRATION_DRY_RUN
static_assert(false,
              "Cannot do static registration Dry Run as static registration is "
              "disabled.");
#endif  // MEDIAPIPE_DISABLE_STATIC_REGISTRATION &&
        // MEDIAPIPE_ENABLE_STATIC_REGISTRATION_DRY_RUN

#if MEDIAPIPE_DISABLE_STATIC_REGISTRATION
// When static registration is disabled, make sure corresponding macros don't do
// any registration.

#define MEDIAPIPE_REGISTER_FACTORY_FUNCTION_QUALIFIED(RegistryType, var_name, \
                                                      name, ...)
#define MEDIAPIPE_STATIC_REGISTRATOR_TEMPLATE(RegistratorName, RegistryType, \
                                              name, ...)                     \
  template <typename T>                                                      \
  class RegistratorName {};

#elif MEDIAPIPE_ENABLE_STATIC_REGISTRATION_DRY_RUN
// When static registration is enabled and running in Dry-Run mode, make sure
// corresponding macros print registration details instead of doing actual
// registration.

#define INTERNAL_MEDIAPIPE_REGISTER_FACTORY_STRINGIFY_HELPER(x) #x
#define INTERNAL_MEDIAPIPE_REGISTER_FACTORY_STRINGIFY(x) \
  INTERNAL_MEDIAPIPE_REGISTER_FACTORY_STRINGIFY_HELPER(x)

#define MEDIAPIPE_REGISTER_FACTORY_FUNCTION_QUALIFIED(RegistryType, var_name, \
                                                      name, ...)              \
  static mediapipe::RegistrationToken* REGISTRY_STATIC_VAR(var_name,          \
                                                           __LINE__) = []() { \
    ABSL_RAW_LOG(WARNING, "Registration Dry Run: %s",                         \
                 INTERNAL_MEDIAPIPE_REGISTER_FACTORY_STRINGIFY(               \
                     RegistryType::Register(name, __VA_ARGS__)));             \
    return nullptr;                                                           \
  }();

#define MEDIAPIPE_STATIC_REGISTRATOR_TEMPLATE(RegistratorName, RegistryType,  \
                                              names, ...)                     \
  template <typename T>                                                       \
  struct Internal##RegistratorName {                                          \
    static NoDestructor<mediapipe::RegistrationToken> registration;           \
                                                                              \
    static mediapipe::RegistrationToken Make() {                              \
      ABSL_RAW_LOG(WARNING, "Registration Dry Run: %s",                       \
                   INTERNAL_MEDIAPIPE_REGISTER_FACTORY_STRINGIFY(             \
                       RegistryType::Register(names, __VA_ARGS__)));          \
      ABSL_RAW_LOG(WARNING, "Where typeid(T).name() is: %s",                  \
                   typeid(T).name());                                         \
      return {};                                                              \
    }                                                                         \
                                                                              \
    using RequireStatics =                                                    \
        registration_internal::ForceStaticInstantiation<&registration>;       \
  };                                                                          \
  /* Static members of template classes can be defined in the header. */      \
  template <typename T>                                                       \
  NoDestructor<mediapipe::RegistrationToken>                                  \
      Internal##RegistratorName<T>::registration(                             \
          Internal##RegistratorName<T>::Make());                              \
                                                                              \
  template <typename T>                                                       \
  class RegistratorName {                                                     \
   private:                                                                   \
    /* The member below triggers instantiation of the registration static. */ \
    typename Internal##RegistratorName<T>::RequireStatics register_;          \
  };

#else
// When static registration is enabled and NOT running in Dry-Run mode, make
// sure corresponding macros do proper static registration.

#define MEDIAPIPE_REGISTER_FACTORY_FUNCTION_QUALIFIED(RegistryType, var_name, \
                                                      name, ...)              \
  static mediapipe::RegistrationToken* REGISTRY_STATIC_VAR(var_name,          \
                                                           __LINE__) =        \
      new mediapipe::RegistrationToken(                                       \
          RegistryType::Register(name, __VA_ARGS__));

// Defines a utility registrator class which can be used to automatically
// register factory functions.
//
// Example:
// === Defining a registry ================================================
//
//  class Component {};
//
//  using ComponentRegistry = GlobalFactoryRegistry<std::unique_ptr<Component>>;
//
// === Defining a registrator =============================================
//
//  MEDIAPIPE_STATIC_REGISTRATOR_TEMPLATE(ComponentRegistrator,
//                                        ComponentRegistry, T::kName,
//                                        absl::make_unique<T>);
//
// === Defining and registering a new component. ==========================
//
//  class MyComponent : public Component,
//                      private ComponentRegistrator<MyComponent> {
//   public:
//    static constexpr char kName[] = "MyComponent";
//    ...
//  };
//
// NOTE:
// - MyComponent is automatically registered in ComponentRegistry by
//   "MyComponent" name.
// - Every component is require to provide its name (T::kName here.)
#define MEDIAPIPE_STATIC_REGISTRATOR_TEMPLATE(RegistratorName, RegistryType,  \
                                              name, ...)                      \
  template <typename T>                                                       \
  struct Internal##RegistratorName {                                          \
    static NoDestructor<mediapipe::RegistrationToken> registration;           \
                                                                              \
    static mediapipe::RegistrationToken Make() {                              \
      return RegistryType::Register(name, __VA_ARGS__);                       \
    }                                                                         \
                                                                              \
    using RequireStatics =                                                    \
        registration_internal::ForceStaticInstantiation<&registration>;       \
  };                                                                          \
  /* Static members of template classes can be defined in the header. */      \
  template <typename T>                                                       \
  NoDestructor<mediapipe::RegistrationToken>                                  \
      Internal##RegistratorName<T>::registration(                             \
          Internal##RegistratorName<T>::Make());                              \
                                                                              \
  template <typename T>                                                       \
  class RegistratorName {                                                     \
   private:                                                                   \
    /* The member below triggers instantiation of the registration static. */ \
    typename Internal##RegistratorName<T>::RequireStatics register_;          \
  };

#endif  // MEDIAPIPE_DISABLE_STATIC_REGISTRATION

#define MEDIAPIPE_REGISTER_FACTORY_FUNCTION(RegistryType, name, ...) \
  MEDIAPIPE_REGISTER_FACTORY_FUNCTION_QUALIFIED(                     \
      RegistryType, registration_##name, #name, __VA_ARGS__)

// TODO: migrate usages to use
// MEDIAPIPE_REGISTER_FACTORY_FUNCTION_QUALIFIED.
#define REGISTER_FACTORY_FUNCTION_QUALIFIED(RegistryType, var_name, name, ...) \
  MEDIAPIPE_REGISTER_FACTORY_FUNCTION_QUALIFIED(RegistryType, var_name, #name, \
                                                __VA_ARGS__)

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_REGISTRATION_H_
