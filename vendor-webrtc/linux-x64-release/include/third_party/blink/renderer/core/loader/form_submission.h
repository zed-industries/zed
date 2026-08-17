/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FORM_SUBMISSION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FORM_SUBMISSION_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/frame/remote_frame.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/triggering_event_info.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/core/loader/navigation_policy.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class EncodedFormData;
class Event;
class Frame;
class HTMLFormControlElement;
class HTMLFormElement;
class LocalDOMWindow;
class ResourceRequest;
class SourceLocation;

class FormSubmission final : public GarbageCollected<FormSubmission> {
 public:
  enum SubmitMethod { kGetMethod, kPostMethod, kDialogMethod };

  class Attributes {
    DISALLOW_NEW();

   public:
    Attributes()
        : method_(kGetMethod),
          is_multi_part_form_(false),
          encoding_type_("application/x-www-form-urlencoded") {}
    Attributes(const Attributes&) = delete;
    Attributes& operator=(const Attributes&) = delete;

    SubmitMethod Method() const { return method_; }
    static SubmitMethod ParseMethodType(const String&);
    void UpdateMethodType(const String&);
    static String MethodString(SubmitMethod);

    const String& Action() const { return action_; }
    void ParseAction(const String&);

    const AtomicString& Target() const { return target_; }
    void SetTarget(const AtomicString& target) { target_ = target; }

    const AtomicString& EncodingType() const { return encoding_type_; }
    static AtomicString ParseEncodingType(const String&);
    void UpdateEncodingType(const String&);
    bool IsMultiPartForm() const { return is_multi_part_form_; }

    const String& AcceptCharset() const { return accept_charset_; }
    void SetAcceptCharset(const String& value) { accept_charset_ = value; }

    void CopyFrom(const Attributes&);

   private:
    SubmitMethod method_;
    bool is_multi_part_form_;

    String action_;
    AtomicString target_;
    AtomicString encoding_type_;
    String accept_charset_;
  };

  // Create FormSubmission
  //
  // This returns nullptr if form submission is not allowed for the given
  // arguments. For example, if navigation policy for the event is
  // `kNavigationPolicyLinkPreview`.
  static FormSubmission* Create(HTMLFormElement*,
                                const Attributes&,
                                const Event*,
                                HTMLFormControlElement* submit_button);

  FormSubmission(
      SubmitMethod,
      const KURL& action,
      const AtomicString& target,
      const AtomicString& content_type,
      Element* submitter,
      scoped_refptr<EncodedFormData>,
      const Event*,
      NavigationPolicy navigation_policy,
      mojom::blink::TriggeringEventInfo triggering_event_info,
      ClientNavigationReason reason,
      std::unique_ptr<ResourceRequest> resource_request,
      Frame* target_frame,
      WebFrameLoadType load_type,
      LocalDOMWindow* origin_window,
      const LocalFrameToken& initiator_frame_token,
      bool has_rel_opener,
      std::unique_ptr<SourceLocation> source_location,
      mojo::PendingRemote<mojom::blink::NavigationStateKeepAliveHandle>
          initiator_navigation_state_keep_alive_handle);
  // FormSubmission for DialogMethod
  explicit FormSubmission(const String& result);

  void Trace(Visitor*) const;

  void NotifyInspector();
  void Navigate();

  KURL RequestURL() const;

  SubmitMethod Method() const { return method_; }
  const KURL& Action() const { return action_; }
  EncodedFormData* Data() const { return form_data_.get(); }

  const String& Result() const { return result_; }

  Frame* TargetFrame() const { return target_frame_.Get(); }

 private:
  // FIXME: Hold an instance of Attributes instead of individual members.
  SubmitMethod method_;
  KURL action_;
  AtomicString target_;
  AtomicString content_type_;
  Member<Element> submitter_;
  scoped_refptr<EncodedFormData> form_data_;
  NavigationPolicy navigation_policy_;
  mojom::blink::TriggeringEventInfo triggering_event_info_;
  String result_;
  ClientNavigationReason reason_;
  std::unique_ptr<ResourceRequest> resource_request_;
  Member<Frame> target_frame_;
  WebFrameLoadType load_type_;
  Member<LocalDOMWindow> origin_window_;
  LocalFrameToken initiator_frame_token_;
  bool has_rel_opener_ = false;

  // Since form submissions are scheduled asynchronously, we need to store the
  // source location when we create the form submission and then pass it over to
  // the `FrameLoadRequest`. Capturing the source location later when creating
  // the `FrameLoadRequest` will not return the correct location.
  std::unique_ptr<SourceLocation> source_location_;

  // Since form submissions are scheduled asynchronously, we need to keep a
  // handle to the initiator NavigationStateKeepAliveHandle. This ensures that
  // it remains available in the browser until we create the NavigationRequest.
  mojo::PendingRemote<mojom::blink::NavigationStateKeepAliveHandle>
      initiator_navigation_state_keep_alive_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FORM_SUBMISSION_H_
