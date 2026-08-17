// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_ADD_EVENT_LISTENER_OPTIONS_RESOLVED_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_ADD_EVENT_LISTENER_OPTIONS_RESOLVED_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_add_event_listener_options.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

// AddEventListenerOptionsResolved class represents resolved event listener
// options. An application requests AddEventListenerOptions and the user
// agent may change ('resolve') these settings (based on settings or policies)
// and the result and the reasons why changes occurred are stored in this class.
class CORE_EXPORT AddEventListenerOptionsResolved
    : public AddEventListenerOptions {
 public:
  AddEventListenerOptionsResolved();
  AddEventListenerOptionsResolved(const AddEventListenerOptions*);
  ~AddEventListenerOptionsResolved() override;

  void SetPassiveForcedForDocumentTarget(bool forced) {
    passive_forced_for_document_target_ = forced;
  }
  bool PassiveForcedForDocumentTarget() const {
    return passive_forced_for_document_target_;
  }

  // Set whether passive was specified when the options were
  // created by callee.
  void SetPassiveSpecified(bool specified) { passive_specified_ = specified; }
  bool PassiveSpecified() const { return passive_specified_; }

  void Trace(Visitor*) const override;

 private:
  bool passive_forced_for_document_target_;
  bool passive_specified_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_ADD_EVENT_LISTENER_OPTIONS_RESOLVED_H_
