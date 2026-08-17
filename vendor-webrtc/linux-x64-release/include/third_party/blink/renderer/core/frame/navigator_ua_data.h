// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_UA_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_UA_DATA_H_

#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_navigator_ua_brand_version.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class NavigatorUABrandVersion;
class ScriptState;
class UADataValues;

class NavigatorUAData : public ScriptWrappable, ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static NavigatorUAData* Create(ExecutionContext* context) {
    return MakeGarbageCollected<NavigatorUAData>(context);
  }

  explicit NavigatorUAData(ExecutionContext* context);

  void SetBrandVersionList(const UserAgentBrandList& brand_version_list);
  void SetFullVersionList(const UserAgentBrandList& full_version_list);
  void SetMobile(bool mobile);
  void SetPlatform(const String& brand, const String& version);
  void SetArchitecture(const String& architecture);
  void SetModel(const String& model);
  void SetUAFullVersion(const String& uaFullVersion);
  void SetBitness(const String& bitness);
  void SetWoW64(bool wow64);
  void SetFormFactors(Vector<String> form_factors);

  // IDL implementation
  const HeapVector<Member<NavigatorUABrandVersion>>& brands() const;
  bool mobile() const;
  const String& platform() const;
  ScriptPromise<UADataValues> getHighEntropyValues(ScriptState*,
                                                   const Vector<String>&) const;
  ScriptObject toJSON(ScriptState*) const;

  void Trace(Visitor* visitor) const final;

 private:
  HeapVector<Member<NavigatorUABrandVersion>> brand_set_;
  HeapVector<Member<NavigatorUABrandVersion>> empty_brand_set_;
  HeapVector<Member<NavigatorUABrandVersion>> full_version_list_;
  bool is_mobile_ = false;
  String platform_;
  String platform_version_;
  String architecture_;
  String model_;
  String ua_full_version_;
  String bitness_;
  bool is_wow64_ = false;
  Vector<String> form_factors_;

  void AddBrandVersion(const String& brand, const String& version);
  void AddBrandFullVersion(const String& brand, const String& version);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_UA_DATA_H_
