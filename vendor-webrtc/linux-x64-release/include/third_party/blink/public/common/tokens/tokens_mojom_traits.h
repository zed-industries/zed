// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_TOKENS_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_TOKENS_MOJOM_TRAITS_H_

#include "base/immediate_crash.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/tokens/token_mojom_traits_helper.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/tokens/tokens.mojom-shared.h"

namespace mojo {

// Mojom traits for the various token types.
// See third_party/blink/public/common/tokens/tokens.h for more details.

////////////////////////////////////////////////////////////////////////////////
// DOCUMENT TOKENS
template <>
struct StructTraits<blink::mojom::DocumentTokenDataView, blink::DocumentToken>
    : public blink::TokenMojomTraitsHelper<blink::mojom::DocumentTokenDataView,
                                           blink::DocumentToken> {};

////////////////////////////////////////////////////////////////////////////////
// FRAME TOKENS

template <>
struct StructTraits<blink::mojom::LocalFrameTokenDataView,
                    blink::LocalFrameToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::LocalFrameTokenDataView,
          blink::LocalFrameToken> {};

template <>
struct StructTraits<blink::mojom::RemoteFrameTokenDataView,
                    blink::RemoteFrameToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::RemoteFrameTokenDataView,
          blink::RemoteFrameToken> {};

template <>
struct BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::FrameTokenDataView, blink::FrameToken> {
 private:
  using DataView = blink::mojom::FrameTokenDataView;

 public:
  static bool Read(DataView input, blink::FrameToken* output);

  static DataView::Tag GetTag(const blink::FrameToken& token) {
    switch (token.variant_index()) {
      case blink::FrameToken::IndexOf<blink::LocalFrameToken>():
        return DataView::Tag::kLocalFrameToken;
      case blink::FrameToken::IndexOf<blink::RemoteFrameToken>():
        return DataView::Tag::kRemoteFrameToken;
    }
    base::ImmediateCrash();
  }

  static const blink::LocalFrameToken& local_frame_token(
      const blink::FrameToken& token) {
    return token.GetAs<blink::LocalFrameToken>();
  }
  static const blink::RemoteFrameToken& remote_frame_token(
      const blink::FrameToken& token) {
    return token.GetAs<blink::RemoteFrameToken>();
  }
};

////////////////////////////////////////////////////////////////////////////////
// WORKER TOKENS

template <>
struct StructTraits<blink::mojom::DedicatedWorkerTokenDataView,
                    blink::DedicatedWorkerToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::DedicatedWorkerTokenDataView,
          blink::DedicatedWorkerToken> {};

template <>
struct StructTraits<blink::mojom::ServiceWorkerTokenDataView,
                    blink::ServiceWorkerToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::ServiceWorkerTokenDataView,
          blink::ServiceWorkerToken> {};

template <>
struct StructTraits<blink::mojom::SharedWorkerTokenDataView,
                    blink::SharedWorkerToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::SharedWorkerTokenDataView,
          blink::SharedWorkerToken> {};

template <>
struct BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::WorkerTokenDataView, blink::WorkerToken> {
 private:
  using DataView = blink::mojom::WorkerTokenDataView;

 public:
  static bool Read(DataView input, blink::WorkerToken* output);

  static blink::mojom::WorkerTokenDataView::Tag GetTag(
      const blink::WorkerToken& token) {
    switch (token.variant_index()) {
      case blink::WorkerToken::IndexOf<blink::DedicatedWorkerToken>():
        return DataView::Tag::kDedicatedWorkerToken;
      case blink::WorkerToken::IndexOf<blink::ServiceWorkerToken>():
        return DataView::Tag::kServiceWorkerToken;
      case blink::WorkerToken::IndexOf<blink::SharedWorkerToken>():
        return DataView::Tag::kSharedWorkerToken;
    }
    base::ImmediateCrash();
  }

  static const blink::DedicatedWorkerToken& dedicated_worker_token(
      const blink::WorkerToken& token) {
    return token.GetAs<blink::DedicatedWorkerToken>();
  }
  static const blink::ServiceWorkerToken& service_worker_token(
      const blink::WorkerToken& token) {
    return token.GetAs<blink::ServiceWorkerToken>();
  }
  static const blink::SharedWorkerToken& shared_worker_token(
      const blink::WorkerToken& token) {
    return token.GetAs<blink::SharedWorkerToken>();
  }
};

////////////////////////////////////////////////////////////////////////////////
// WORKLET TOKENS

template <>
struct StructTraits<blink::mojom::AnimationWorkletTokenDataView,
                    blink::AnimationWorkletToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::AnimationWorkletTokenDataView,
          blink::AnimationWorkletToken> {};

template <>
struct StructTraits<blink::mojom::AudioWorkletTokenDataView,
                    blink::AudioWorkletToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::AudioWorkletTokenDataView,
          blink::AudioWorkletToken> {};

template <>
struct StructTraits<blink::mojom::LayoutWorkletTokenDataView,
                    blink::LayoutWorkletToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::LayoutWorkletTokenDataView,
          blink::LayoutWorkletToken> {};

template <>
struct StructTraits<blink::mojom::PaintWorkletTokenDataView,
                    blink::PaintWorkletToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::PaintWorkletTokenDataView,
          blink::PaintWorkletToken> {};

template <>
struct StructTraits<blink::mojom::SharedStorageWorkletTokenDataView,
                    blink::SharedStorageWorkletToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::SharedStorageWorkletTokenDataView,
          blink::SharedStorageWorkletToken> {};

template <>
struct BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::WorkletTokenDataView, blink::WorkletToken> {
 private:
  using DataView = blink::mojom::WorkletTokenDataView;

 public:
  static bool Read(DataView input, blink::WorkletToken* output);

  static blink::mojom::WorkletTokenDataView::Tag GetTag(
      const blink::WorkletToken& token) {
    switch (token.variant_index()) {
      case blink::WorkletToken::IndexOf<blink::AnimationWorkletToken>():
        return DataView::Tag::kAnimationWorkletToken;
      case blink::WorkletToken::IndexOf<blink::AudioWorkletToken>():
        return DataView::Tag::kAudioWorkletToken;
      case blink::WorkletToken::IndexOf<blink::LayoutWorkletToken>():
        return DataView::Tag::kLayoutWorkletToken;
      case blink::WorkletToken::IndexOf<blink::PaintWorkletToken>():
        return DataView::Tag::kPaintWorkletToken;
      case blink::WorkletToken::IndexOf<blink::SharedStorageWorkletToken>():
        return DataView::Tag::kSharedStorageWorkletToken;
    }
    base::ImmediateCrash();
  }

  static const blink::AnimationWorkletToken& animation_worklet_token(
      const blink::WorkletToken& token) {
    return token.GetAs<blink::AnimationWorkletToken>();
  }
  static const blink::AudioWorkletToken& audio_worklet_token(
      const blink::WorkletToken& token) {
    return token.GetAs<blink::AudioWorkletToken>();
  }
  static const blink::LayoutWorkletToken& layout_worklet_token(
      const blink::WorkletToken& token) {
    return token.GetAs<blink::LayoutWorkletToken>();
  }
  static const blink::PaintWorkletToken& paint_worklet_token(
      const blink::WorkletToken& token) {
    return token.GetAs<blink::PaintWorkletToken>();
  }
  static const blink::SharedStorageWorkletToken& shared_storage_worklet_token(
      const blink::WorkletToken& token) {
    return token.GetAs<blink::SharedStorageWorkletToken>();
  }
};

////////////////////////////////////////////////////////////////////////////////
// SHADOW REALM TOKENS

template <>
struct StructTraits<blink::mojom::ShadowRealmTokenDataView,
                    blink::ShadowRealmToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::ShadowRealmTokenDataView,
          blink::ShadowRealmToken> {};

////////////////////////////////////////////////////////////////////////////////
// OTHER TOKENS
//
// Keep this section last.
//
// If you have multiple tokens that make a thematic group, please lift them to
// their own section, in alphabetical order. If adding a new token here, please
// keep the following list in alphabetic order.

template <>
struct StructTraits<blink::mojom::AttributionSrcTokenDataView,
                    blink::AttributionSrcToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::AttributionSrcTokenDataView,
          blink::AttributionSrcToken> {};

template <>
struct StructTraits<blink::mojom::ClipboardSequenceNumberTokenDataView,
                    blink::ClipboardSequenceNumberToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::ClipboardSequenceNumberTokenDataView,
          blink::ClipboardSequenceNumberToken> {};

template <>
struct BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::ExecutionContextTokenDataView,
                blink::ExecutionContextToken> {
 private:
  using DataView = blink::mojom::ExecutionContextTokenDataView;

 public:
  static bool Read(blink::mojom::ExecutionContextTokenDataView input,
                   blink::ExecutionContextToken* output);

  static DataView::Tag GetTag(const blink::ExecutionContextToken& token) {
    switch (token.variant_index()) {
      case blink::ExecutionContextToken::IndexOf<blink::LocalFrameToken>():
        return DataView::Tag::kLocalFrameToken;
      case blink::ExecutionContextToken::IndexOf<blink::DedicatedWorkerToken>():
        return DataView::Tag::kDedicatedWorkerToken;
      case blink::ExecutionContextToken::IndexOf<blink::ServiceWorkerToken>():
        return DataView::Tag::kServiceWorkerToken;
      case blink::ExecutionContextToken::IndexOf<blink::SharedWorkerToken>():
        return DataView::Tag::kSharedWorkerToken;
      case blink::ExecutionContextToken::IndexOf<
          blink::AnimationWorkletToken>():
        return DataView::Tag::kAnimationWorkletToken;
      case blink::ExecutionContextToken::IndexOf<blink::AudioWorkletToken>():
        return DataView::Tag::kAudioWorkletToken;
      case blink::ExecutionContextToken::IndexOf<blink::LayoutWorkletToken>():
        return DataView::Tag::kLayoutWorkletToken;
      case blink::ExecutionContextToken::IndexOf<blink::PaintWorkletToken>():
        return DataView::Tag::kPaintWorkletToken;
      case blink::ExecutionContextToken::IndexOf<
          blink::SharedStorageWorkletToken>():
        return DataView::Tag::kSharedStorageWorkletToken;
      case blink::ExecutionContextToken::IndexOf<blink::ShadowRealmToken>():
        return DataView::Tag::kShadowRealmToken;
    }
    base::ImmediateCrash();
  }

  static const blink::LocalFrameToken& local_frame_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::LocalFrameToken>();
  }
  static const blink::DedicatedWorkerToken& dedicated_worker_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::DedicatedWorkerToken>();
  }
  static const blink::ServiceWorkerToken& service_worker_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::ServiceWorkerToken>();
  }
  static const blink::SharedWorkerToken& shared_worker_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::SharedWorkerToken>();
  }
  static const blink::AnimationWorkletToken& animation_worklet_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::AnimationWorkletToken>();
  }
  static const blink::AudioWorkletToken& audio_worklet_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::AudioWorkletToken>();
  }
  static const blink::LayoutWorkletToken& layout_worklet_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::LayoutWorkletToken>();
  }
  static const blink::PaintWorkletToken& paint_worklet_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::PaintWorkletToken>();
  }
  static const blink::SharedStorageWorkletToken& shared_storage_worklet_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::SharedStorageWorkletToken>();
  }
  static const blink::ShadowRealmToken& shadow_realm_token(
      const blink::ExecutionContextToken& token) {
    return token.GetAs<blink::ShadowRealmToken>();
  }
};

template <>
struct StructTraits<
    blink::mojom::SameDocNavigationScreenshotDestinationTokenDataView,
    blink::SameDocNavigationScreenshotDestinationToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::SameDocNavigationScreenshotDestinationTokenDataView,
          blink::SameDocNavigationScreenshotDestinationToken> {};

template <>
struct StructTraits<blink::mojom::V8ContextTokenDataView, blink::V8ContextToken>
    : public blink::TokenMojomTraitsHelper<blink::mojom::V8ContextTokenDataView,
                                           blink::V8ContextToken> {};

template <>
struct StructTraits<blink::mojom::ViewTransitionTokenDataView,
                    blink::ViewTransitionToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::ViewTransitionTokenDataView,
          blink::ViewTransitionToken> {};

template <>
struct BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::WebGPUExecutionContextTokenDataView,
                blink::WebGPUExecutionContextToken> {
 private:
  using DataView = blink::mojom::WebGPUExecutionContextTokenDataView;

 public:
  static bool Read(DataView input, blink::WebGPUExecutionContextToken* output);

  static DataView::Tag GetTag(const blink::WebGPUExecutionContextToken& token) {
    switch (token.variant_index()) {
      case blink::WebGPUExecutionContextToken::IndexOf<blink::DocumentToken>():
        return DataView::Tag::kDocumentToken;
      case blink::WebGPUExecutionContextToken::IndexOf<
          blink::DedicatedWorkerToken>():
        return DataView::Tag::kDedicatedWorkerToken;
      case blink::WebGPUExecutionContextToken::IndexOf<
          blink::SharedWorkerToken>():
        return DataView::Tag::kSharedWorkerToken;
      case blink::WebGPUExecutionContextToken::IndexOf<
          blink::ServiceWorkerToken>():
        return DataView::Tag::kServiceWorkerToken;
    }
    base::ImmediateCrash();
  }

  static const blink::DocumentToken& document_token(
      const blink::WebGPUExecutionContextToken& token) {
    return token.GetAs<blink::DocumentToken>();
  }
  static const blink::DedicatedWorkerToken& dedicated_worker_token(
      const blink::WebGPUExecutionContextToken& token) {
    return token.GetAs<blink::DedicatedWorkerToken>();
  }
  static const blink::SharedWorkerToken& shared_worker_token(
      const blink::WebGPUExecutionContextToken& token) {
    return token.GetAs<blink::SharedWorkerToken>();
  }
  static const blink::ServiceWorkerToken& service_worker_token(
      const blink::WebGPUExecutionContextToken& token) {
    return token.GetAs<blink::ServiceWorkerToken>();
  }
};

template <>
struct StructTraits<blink::mojom::WebNNContextTokenDataView,
                    blink::WebNNContextToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::WebNNContextTokenDataView,
          blink::WebNNContextToken> {};

template <>
struct StructTraits<blink::mojom::WebNNPendingConstantTokenDataView,
                    blink::WebNNPendingConstantToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::WebNNPendingConstantTokenDataView,
          blink::WebNNPendingConstantToken> {};

template <>
struct StructTraits<blink::mojom::WebNNTensorTokenDataView,
                    blink::WebNNTensorToken>
    : public blink::TokenMojomTraitsHelper<
          blink::mojom::WebNNTensorTokenDataView,
          blink::WebNNTensorToken> {};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_TOKENS_MOJOM_TRAITS_H_
