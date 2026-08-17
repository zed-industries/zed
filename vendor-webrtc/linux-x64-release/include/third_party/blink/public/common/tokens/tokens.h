// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_TOKENS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_TOKENS_H_

#include "base/types/token_type.h"
#include "third_party/blink/public/common/tokens/multi_token.h"
#include "ui/base/clipboard/clipboard_sequence_number_token.h"

namespace blink {

// Various token types. These are used as cross-layer and cross-process
// identifiers for objects that exist in blink, but which have representations
// in the browser process. They should not be used to identify objects in
// browser-to-renderer control messages; rather, such messages should exist as
// methods on the interface bound to the object itself. They are fine to use
// for informational messages that cross over other interfaces, in both
// directions.
//
// See README.md for more details.

////////////////////////////////////////////////////////////////////////////////
// DOCUMENT TOKENS
using DocumentToken = base::TokenType<class DocumentTokenTypeMarker>;

////////////////////////////////////////////////////////////////////////////////
// FRAME TOKENS

// Uniquely identifies a blink::LocalFrame / blink::WebLocalFrame /
// content::RenderFrame in a renderer process, and its content::RenderFrameHost
// counterpart in the browser.
using LocalFrameToken = base::TokenType<class LocalFrameTokenTypeMarker>;

// Uniquely identifies a blink::RemoteFrame / blink::WebRemoteFrame /
// content::RenderFrameProxy in a renderer process, and its
// content::RenderFrameProxyHost counterpart in the browser. There can be
// multiple RemoteFrames corresponding to a single LocalFrame, and each token
// will be distinct.
using RemoteFrameToken = base::TokenType<class RemoteFrameTokenTypeMarker>;

// Can represent either type of FrameToken.
using FrameToken = MultiToken<LocalFrameToken, RemoteFrameToken>;

////////////////////////////////////////////////////////////////////////////////
// WORKER TOKENS

// Identifies a blink::DedicatedWorkerGlobalScope in the renderer and a
// content::DedicatedWorkerHost in the browser.
using DedicatedWorkerToken =
    base::TokenType<class DedicatedWorkerTokenTypeMarker>;

// Identifies a blink::ServiceWorkerGlobalScope in the renderer and a
// content::ServiceWorkerVersion in the browser.
using ServiceWorkerToken = base::TokenType<class ServiceWorkerTokenTypeMarker>;

// Identifies a blink::SharedWorkerGlobalScope in the renderer and a
// content::SharedWorkerHost in the browser.
using SharedWorkerToken = base::TokenType<class SharedWorkerTokenTypeMarker>;

// Can represent any type of WorkerToken.
using WorkerToken =
    MultiToken<DedicatedWorkerToken, ServiceWorkerToken, SharedWorkerToken>;

////////////////////////////////////////////////////////////////////////////////
// WORKLET TOKENS

// Identifies an animation worklet.
using AnimationWorkletToken =
    base::TokenType<class AnimationWorkletTokenTypeMarker>;

// Identifies an audio worklet.
using AudioWorkletToken = base::TokenType<class AudioWorkletTokenTypeMarker>;

// Identifies a layout worklet.
using LayoutWorkletToken = base::TokenType<class LayoutWorkletTokenTypeMarker>;

// Identifies a paint worklet.
using PaintWorkletToken = base::TokenType<class PaintWorkletTokenTypeMarker>;

// Identifies a shared storage worklet.
using SharedStorageWorkletToken =
    base::TokenType<class SharedStorageWorkletTokenTypeMarker>;

// Can represent any type of WorkletToken.
using WorkletToken = MultiToken<AnimationWorkletToken,
                                AudioWorkletToken,
                                LayoutWorkletToken,
                                PaintWorkletToken,
                                SharedStorageWorkletToken>;

////////////////////////////////////////////////////////////////////////////////
// SHADOW REALM TOKENS
using ShadowRealmToken = base::TokenType<class ShadowRealmTokenTypeMarker>;

////////////////////////////////////////////////////////////////////////////////
// OTHER TOKENS
//
// Keep this section last.
//
// If you have multiple tokens that make a thematic group, please lift them to
// their own section, in alphabetical order. If adding a new token here, please
// keep the following list in alphabetic order.

// Identifies an attributionsrc request made by the Attribution Reporting API.
using AttributionSrcToken =
    base::TokenType<class AttributionSrcTokenTypeMarker>;

// Identifies a unique clipboard state.
using ClipboardSequenceNumberToken = ui::ClipboardSequenceNumberToken;

// Identifies an arbitrary ExecutionContext. Each concrete implementation of an
// ExecutionContext has a distinct token type that can be represented here.
using ExecutionContextToken = MultiToken<LocalFrameToken,
                                         DedicatedWorkerToken,
                                         ServiceWorkerToken,
                                         SharedWorkerToken,
                                         AnimationWorkletToken,
                                         AudioWorkletToken,
                                         LayoutWorkletToken,
                                         PaintWorkletToken,
                                         SharedStorageWorkletToken,
                                         ShadowRealmToken>;

// Identifies the destination of a screenshot for a same-document navigation.
using SameDocNavigationScreenshotDestinationToken = base::TokenType<
    class SameDocNavigationScreenshotDestinationTokenTypeMarker>;

// Identifies a v8::Context / blink::ScriptState.
using V8ContextToken = base::TokenType<class V8ContextTokenTypeMarker>;

using ViewTransitionToken =
    base::TokenType<class ViewTransitionTokenTypeMarker>;

// Identifies possible contexts used for WebGPU. Used in cross-process mojo
// interfaces for isolation key coordination.
// TODO(dawn:549) Might be able to eventually swap this out to use
//     ExecutionContextToken from above with DocumentToken gets encapsulated
//     there later on.
using WebGPUExecutionContextToken = MultiToken<DocumentToken,
                                               DedicatedWorkerToken,
                                               SharedWorkerToken,
                                               ServiceWorkerToken>;

// Identify various WebNN types in a renderer process and the WebNN service.
using WebNNContextToken = base::TokenType<class WebNNContextTokenTypeMarker>;
using WebNNPendingConstantToken =
    base::TokenType<class WebNNPendingConstantTokenTypeMarker>;
using WebNNTensorToken = base::TokenType<class WebNNTensorTokenTypeMarker>;
using WebNNGraphToken = base::TokenType<class WebNNGraphTokenTypeMarker>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_TOKENS_H_
