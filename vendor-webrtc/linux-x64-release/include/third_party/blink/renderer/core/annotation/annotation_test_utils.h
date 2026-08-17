// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_TEST_UTILS_H_

#include <optional>

#include "base/check_op.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/annotation/annotation.mojom-blink.h"
#include "third_party/blink/renderer/core/annotation/annotation_agent_impl.h"
#include "third_party/blink/renderer/core/annotation/annotation_selector.h"
#include "third_party/blink/renderer/core/editing/position.h"
#include "third_party/blink/renderer/core/editing/range_in_flat_tree.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

// A selector type for returning a predetermined range in tests. The empty
// constructor will simply return the entire Document tree as a range.
// Otherwise, the caller can specify a range in the constructor that will be
// returned. Specifying a null Range will simulate a "not found" result.
class MockAnnotationSelector : public AnnotationSelector {
 public:
  MockAnnotationSelector() = default;
  explicit MockAnnotationSelector(RangeInFlatTree& mock_result)
      : mock_result_(&mock_result) {}

  String Serialize() const override { return ""; }

  void FindRange(Range& search_range,
                 SearchType type,
                 FinishedCallback finished_cb) override {
    RangeInFlatTree* range;

    if (mock_result_) {
      if (mock_result_->IsNull()) {
        range = nullptr;
      } else {
        range = mock_result_;

        DCHECK_EQ(mock_result_->StartPosition().GetDocument(),
                  &search_range.OwnerDocument());
      }
    } else {
      // Just select the whole document.
      auto range_start =
          PositionInFlatTree::FirstPositionInNode(search_range.OwnerDocument());
      auto range_end =
          PositionInFlatTree::LastPositionInNode(search_range.OwnerDocument());
      range = MakeGarbageCollected<RangeInFlatTree>(range_start, range_end);
    }

    std::move(finished_cb).Run(range);
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(mock_result_);
    AnnotationSelector::Trace(visitor);
  }

 private:
  Member<RangeInFlatTree> mock_result_;
};

class ScopedUseMockAnnotationSelector {
 public:
  ScopedUseMockAnnotationSelector() {
    AnnotationSelector::SetGeneratorForTesting(WTF::BindRepeating(
        &ScopedUseMockAnnotationSelector::Generate, WTF::Unretained(this)));
  }

  ~ScopedUseMockAnnotationSelector() {
    AnnotationSelector::UnsetGeneratorForTesting();
  }

 private:
  AnnotationSelector* Generate(const String& serialized) {
    return MakeGarbageCollected<MockAnnotationSelector>();
  }
};

class MockAnnotationAgentHost : public mojom::blink::AnnotationAgentHost {
 public:
  MockAnnotationAgentHost() : receiver_(this) {}
  ~MockAnnotationAgentHost() override = default;
  void DidFinishAttachment(const gfx::Rect& rect,
                           mojom::blink::AttachmentResult result) override {
    did_finish_attachment_rect_ = rect;
  }

  void BindToAgent(AnnotationAgentImpl& agent) {
    // Receivers have no way to query is_connected so we use a disconnect
    // handler.
    auto pending_remote = receiver_.BindNewPipeAndPassRemote();
    DCHECK(receiver_.is_bound());
    receiver_.set_disconnect_handler(
        WTF::BindRepeating([](bool* did_disconnect) { *did_disconnect = true; },
                           WTF::Unretained(&did_disconnect_)));

    agent.Bind(std::move(pending_remote), agent_.BindNewPipeAndPassReceiver());
  }

  void Bind(
      mojo::PendingReceiver<mojom::blink::AnnotationAgentHost> host_receiver,
      mojo::PendingRemote<mojom::blink::AnnotationAgent> agent_remote) {
    agent_.Bind(std::move(agent_remote));
    receiver_.Bind(std::move(host_receiver));
  }

  using RemoteHostReceiverAgentPair =
      std::pair<mojo::PendingRemote<mojom::blink::AnnotationAgentHost>,
                mojo::PendingReceiver<mojom::blink::AnnotationAgent>>;

  RemoteHostReceiverAgentPair BindForCreateAgent() {
    auto pending_remote = receiver_.BindNewPipeAndPassRemote();
    receiver_.set_disconnect_handler(
        WTF::BindRepeating([](bool* did_disconnect) { *did_disconnect = true; },
                           WTF::Unretained(&did_disconnect_)));

    return std::make_pair(std::move(pending_remote),
                          agent_.BindNewPipeAndPassReceiver());
  }

  // Flushes both the receiver and remote mojo connections.
  void FlushForTesting() {
    receiver_.FlushForTesting();
    if (agent_)
      agent_.FlushForTesting();
  }

  // Public to allow inspection.
  mojo::Receiver<mojom::blink::AnnotationAgentHost> receiver_;
  mojo::Remote<mojom::blink::AnnotationAgent> agent_;
  std::optional<gfx::Rect> did_finish_attachment_rect_;
  bool did_disconnect_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_TEST_UTILS_H_
