// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_IDL_MEMBER_INSTALLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_IDL_MEMBER_INSTALLER_H_

#include "base/containers/span.h"
#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8-fast-api-calls.h"
#include "v8/include/v8.h"

namespace blink {

class DOMWrapperWorld;

namespace bindings {

// IDLMemberInstaller is a set of utility functions to define IDL members as
// ES properties.
class PLATFORM_EXPORT IDLMemberInstaller final {
  STATIC_ONLY(IDLMemberInstaller);

 public:
  // On which object the property is defined
  enum class FlagLocation {
    kInstance,
    kPrototype,
    kInterface,
  };
  // In which world the property is defined
  enum class FlagWorld {
    kMainWorld = 1 << 0,
    kNonMainWorlds = 1 << 1,
    kAllWorlds = kMainWorld | kNonMainWorlds,
  };
  // v8::Signature check against the receiver object
  enum class FlagReceiverCheck {
    kCheck,
    kDoNotCheck,
  };
  // Cross origin access check
  enum class FlagCrossOriginCheck {
    kCheck,
    kDoNotCheck,
  };

  // Web IDL attribute
  struct AttributeConfig {
    AttributeConfig& operator=(const AttributeConfig&) = delete;

    const char* property_name;
    v8::FunctionCallback callback_for_get;
    v8::FunctionCallback callback_for_set;
    unsigned v8_property_attribute : 3;       // v8::PropertyAttribute
    unsigned location : 2;                    // FlagLocation
    unsigned world : 2;                       // FlagWorld
    unsigned receiver_check : 1;              // FlagReceiverCheck
    unsigned cross_origin_check_for_get : 1;  // FlagCrossOriginCheck
    unsigned cross_origin_check_for_set : 1;  // FlagCrossOriginCheck
    unsigned v8_side_effect : 2;              // v8::SideEffectType
    unsigned v8_cached_accessor : 2;  // V8PrivateProperty::CachedAccessor
  };
  static void InstallAttributes(v8::Isolate* isolate,
                                const DOMWrapperWorld& world,
                                v8::Local<v8::Template> instance_template,
                                v8::Local<v8::Template> prototype_template,
                                v8::Local<v8::Template> interface_template,
                                v8::Local<v8::Signature> signature,
                                const char* interface_name,
                                base::span<const AttributeConfig> configs);
  static void InstallAttributes(v8::Isolate* isolate,
                                const DOMWrapperWorld& world,
                                v8::Local<v8::Object> instance_object,
                                v8::Local<v8::Object> prototype_object,
                                v8::Local<v8::Object> interface_object,
                                v8::Local<v8::Signature> signature,
                                const char* interface_name,
                                base::span<const AttributeConfig> configs);

  struct NoAllocDirectCallAttributeConfig {
    NoAllocDirectCallAttributeConfig& operator=(
        const NoAllocDirectCallAttributeConfig&) = delete;

    AttributeConfig attribute_config;
    raw_ptr<const v8::CFunction> v8_cfunction_for_set;
  };

  static void InstallAttributes(
      v8::Isolate* isolate,
      const DOMWrapperWorld& world,
      v8::Local<v8::Template> instance_template,
      v8::Local<v8::Template> prototype_template,
      v8::Local<v8::Template> interface_template,
      v8::Local<v8::Signature> signature,
      const char* interface_name,
      base::span<const NoAllocDirectCallAttributeConfig> configs);
  static void InstallAttributes(
      v8::Isolate* isolate,
      const DOMWrapperWorld& world,
      v8::Local<v8::Object> instance_object,
      v8::Local<v8::Object> prototype_object,
      v8::Local<v8::Object> interface_object,
      v8::Local<v8::Signature> signature,
      const char* interface_name,
      base::span<const NoAllocDirectCallAttributeConfig> configs);

  struct ConstantValueConfig {
    ConstantValueConfig& operator=(const ConstantValueConfig&) = delete;

    const char* name;
    int64_t value;
  };
  static void InstallConstants(v8::Isolate* isolate,
                               const DOMWrapperWorld& world,
                               v8::Local<v8::Template> instance_template,
                               v8::Local<v8::Template> prototype_template,
                               v8::Local<v8::Template> interface_template,
                               v8::Local<v8::Signature> signature,
                               base::span<const ConstantValueConfig> configs);

  // Web IDL operation
  struct OperationConfig {
    OperationConfig& operator=(const OperationConfig&) = delete;

    const char* property_name;
    v8::FunctionCallback callback;
    unsigned length : 8;
    unsigned v8_property_attribute : 3;  // v8::PropertyAttribute
    unsigned location : 2;               // FlagLocation
    unsigned world : 2;                  // FlagWorld
    unsigned receiver_check : 1;         // FlagReceiverCheck
    unsigned cross_origin_check : 1;     // FlagCrossOriginCheck
    unsigned v8_side_effect : 2;         // v8::SideEffectType
  };
  static void InstallOperations(v8::Isolate* isolate,
                                const DOMWrapperWorld& world,
                                v8::Local<v8::Template> instance_template,
                                v8::Local<v8::Template> prototype_template,
                                v8::Local<v8::Template> interface_template,
                                v8::Local<v8::Signature> signature,
                                const char* interface_name,
                                base::span<const OperationConfig> configs);
  static void InstallOperations(v8::Isolate* isolate,
                                const DOMWrapperWorld& world,
                                v8::Local<v8::Object> instance_object,
                                v8::Local<v8::Object> prototype_object,
                                v8::Local<v8::Object> interface_object,
                                v8::Local<v8::Signature> signature,
                                const char* interface_name,
                                base::span<const OperationConfig> configs);

  struct NoAllocDirectCallOperationConfig {
    OperationConfig operation_config;
    raw_ptr<const v8::CFunction> v8_cfunction_table_data;
    uint32_t v8_cfunction_table_size;
  };
  static void InstallOperations(
      v8::Isolate* isolate,
      const DOMWrapperWorld& world,
      v8::Local<v8::Template> instance_template,
      v8::Local<v8::Template> prototype_template,
      v8::Local<v8::Template> interface_template,
      v8::Local<v8::Signature> signature,
      const char* interface_name,
      base::span<const NoAllocDirectCallOperationConfig> configs);
  static void InstallOperations(
      v8::Isolate* isolate,
      const DOMWrapperWorld& world,
      v8::Local<v8::Object> instance_object,
      v8::Local<v8::Object> prototype_object,
      v8::Local<v8::Object> interface_object,
      v8::Local<v8::Signature> signature,
      const char* interface_name,
      base::span<const NoAllocDirectCallOperationConfig> configs);

  // Global property reference
  // https://webidl.spec.whatwg.org/#define-the-global-property-references
  // [LegacyNamespace]
  // https://webidl.spec.whatwg.org/#LegacyNamespace
  struct ExposedConstructConfig {
    ExposedConstructConfig& operator=(const ExposedConstructConfig&) = delete;

    const char* name;
    v8::AccessorNameGetterCallback callback;
  };
  static void InstallExposedConstructs(
      v8::Isolate* isolate,
      const DOMWrapperWorld& world,
      v8::Local<v8::Template> instance_template,
      v8::Local<v8::Template> prototype_template,
      v8::Local<v8::Template> interface_template,
      v8::Local<v8::Signature> signature,
      base::span<const ExposedConstructConfig> configs);
  static void InstallExposedConstructs(
      v8::Isolate* isolate,
      const DOMWrapperWorld& world,
      v8::Local<v8::Object> instance_object,
      v8::Local<v8::Object> prototype_object,
      v8::Local<v8::Object> interface_object,
      v8::Local<v8::Signature> signature,
      base::span<const ExposedConstructConfig> configs);
};

}  // namespace bindings

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_IDL_MEMBER_INSTALLER_H_
