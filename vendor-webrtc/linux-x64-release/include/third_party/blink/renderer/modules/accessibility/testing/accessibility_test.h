// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_TESTING_ACCESSIBILITY_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_TESTING_ACCESSIBILITY_TEST_H_

#include <ostream>
#include <sstream>
#include <string>

#include "third_party/blink/renderer/core/accessibility/ax_context.h"
#include "third_party/blink/renderer/core/testing/core_unit_test_helper.h"
#include "third_party/blink/renderer/platform/testing/runtime_enabled_features_test_helpers.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class AXObject;
class AXObjectCacheImpl;
class LocalFrameClient;
class Node;

class AccessibilityTest : public RenderingTest {
  USING_FAST_MALLOC(AccessibilityTest);

 public:
  AccessibilityTest(LocalFrameClient* local_frame_client = nullptr);

 protected:
  void SetUp() override;

  AXObjectCacheImpl& GetAXObjectCache() const;

  AXObject* GetAXObject(LayoutObject* layout_object) const;

  AXObject* GetAXObject(const Node& node) const;

  AXObject* GetAXRootObject() const;

  AXObject* GetAXBodyObject() const;

  // Returns the object with the accessibility focus.
  AXObject* GetAXFocusedObject() const;

  AXObject* GetAXObjectByElementId(const char* id,
                                   PseudoId = kPseudoIdNone) const;

  std::string PrintAXTree() const;

 protected:
  std::unique_ptr<AXContext> ax_context_;

 private:
  std::ostringstream& PrintAXTreeHelper(std::ostringstream&,
                                        const AXObject* root,
                                        size_t level) const;

  ScopedAccessibilityUseAXPositionForDocumentMarkersForTest use_ax_position{
      true};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_TESTING_ACCESSIBILITY_TEST_H_
