#ifndef MEDIAPIPE_FRAMEWORK_TOOL_OPTIONS_MAP_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_OPTIONS_MAP_H_

#include <map>
#include <memory>
#include <type_traits>

#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/port/any_proto.h"
#include "mediapipe/framework/tool/type_util.h"

namespace mediapipe {

namespace tool {

extern absl::Mutex option_extension_lock;

// A compile-time detector for the constant |T::ext|.
template <typename T>
struct IsExtension {
 private:
  template <typename U>
  static char test(decltype(&U::ext));

  template <typename>
  static int test(...);

 public:
  static constexpr bool value = (sizeof(test<T>(0)) == sizeof(char));
};

template <class T,
          typename std::enable_if<IsExtension<T>::value, int>::type = 0>
bool HasExtension(const CalculatorOptions& options) {
  return options.HasExtension(T::ext);
}

template <class T,
          typename std::enable_if<!IsExtension<T>::value, int>::type = 0>
bool HasExtension(const CalculatorOptions& options) {
  return false;
}

template <class T,
          typename std::enable_if<IsExtension<T>::value, int>::type = 0>
T* GetExtension(CalculatorOptions& options) {
  absl::MutexLock lock(&option_extension_lock);
  if (options.HasExtension(T::ext)) {
    return options.MutableExtension(T::ext);
  }
  return nullptr;
}

template <class T,
          typename std::enable_if<!IsExtension<T>::value, int>::type = 0>
T* GetExtension(const CalculatorOptions& options) {
  return nullptr;
}

template <class T>
void GetExtension(const CalculatorOptions& options, T* result) {
  T* r = GetExtension<T>(*const_cast<CalculatorOptions*>(&options));
  if (r) {
    *result = *r;
  }
}

template <class T>
void GetNodeOptions(const CalculatorGraphConfig::Node& node_config, T* result) {
#if defined(MEDIAPIPE_PROTO_LITE) && defined(MEDIAPIPE_PROTO_THIRD_PARTY)
  // protobuf::Any is unavailable with third_party/protobuf:protobuf-lite.
#else
  for (const mediapipe::protobuf::Any& options : node_config.node_options()) {
    if (options.Is<T>()) {
      options.UnpackTo(result);
    }
  }
#endif
}

template <class T>
void SetNodeOptions(CalculatorGraphConfig::Node& node_config, const T& value) {
#if defined(MEDIAPIPE_PROTO_LITE) && defined(MEDIAPIPE_PROTO_THIRD_PARTY)
  // protobuf::Any is unavailable with third_party/protobuf:protobuf-lite.
#else
  for (mediapipe::protobuf::Any& options :
       *node_config.mutable_node_options()) {
    if (options.Is<T>()) {
      options.PackFrom(value);
      return;
    }
  }
  node_config.add_node_options()->PackFrom(value);
#endif
}

// A map from object type to object.
class TypeMap {
 public:
  template <class T>
  bool Has() const {
    return content_.count(kTypeId<T>) > 0;
  }
  template <class T>
  T* Get() const {
    if (!Has<T>()) {
      content_[kTypeId<T>] = std::make_shared<T>();
    }
    return static_cast<T*>(content_[kTypeId<T>].get());
  }

 private:
  mutable std::map<TypeId, std::shared_ptr<void>> content_;
};

// Extracts the options message of a specified type from a
// CalculatorGraphConfig::Node.
class OptionsMap {
 public:
  OptionsMap& Initialize(const CalculatorGraphConfig::Node& node_config) {
    node_config_ = const_cast<CalculatorGraphConfig::Node*>(&node_config);
    return *this;
  }

  // Returns the options data for a CalculatorGraphConfig::Node, from
  // either "options" or "node_options" using either GetExtension or UnpackTo.
  template <class T>
  const T& Get() const {
    if (options_.Has<T>()) {
      return *options_.Get<T>();
    }
    T* result = options_.Get<T>();
    if (node_config_->has_options() &&
        HasExtension<T>(node_config_->options())) {
      GetExtension(node_config_->options(), result);
    } else {
      GetNodeOptions(*node_config_, result);
    }
    return *result;
  }

  template <class T>
  bool Has() const {
    if (options_.Has<T>()) {
      return true;
    }
    if (node_config_->has_options() &&
        HasExtension<T>(node_config_->options())) {
      return true;
    }
#if defined(MEDIAPIPE_PROTO_LITE) && defined(MEDIAPIPE_PROTO_THIRD_PARTY)
    // protobuf::Any is unavailable with third_party/protobuf:protobuf-lite.
#else
    for (const mediapipe::protobuf::Any& options :
         node_config_->node_options()) {
      if (options.Is<T>()) {
        return true;
      }
    }
#endif
    return false;
  }

  CalculatorGraphConfig::Node* node_config_;
  TypeMap options_;
};

class MutableOptionsMap : public OptionsMap {
 public:
  MutableOptionsMap& Initialize(CalculatorGraphConfig::Node& node_config) {
    node_config_ = &node_config;
    return *this;
  }
  template <class T>
  void Set(const T& value) const {
    *options_.Get<T>() = value;
    if (node_config_->has_options() &&
        HasExtension<T>(node_config_->options())) {
      *GetExtension<T>(*node_config_->mutable_options()) = value;
    } else {
      SetNodeOptions(*node_config_, value);
    }
  }

  template <class T>
  T* GetMutable() const {
    if (options_.Has<T>()) {
      return options_.Get<T>();
    }
    if (node_config_->has_options() &&
        HasExtension<T>(node_config_->options())) {
      return GetExtension<T>(*node_config_->mutable_options());
    }
    T* result = options_.Get<T>();
    GetNodeOptions(*node_config_, result);
    return result;
  }
};

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_OPTIONS_MAP_H_
