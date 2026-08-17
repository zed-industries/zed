// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// This header defines utilities for converting between Mojo interface variant
// types. Any code that needs to convert interface endpoints between
// blink::mojom::MyInterface and blink::mojom::blink::MyInterface (such as the
// Blink public API) should use these helpers to eliminate boilerplate code and
// improve type safety.
//
// Background: Mojo generates two C++ interface classes for a given interface:
// one using STL types and another using Blink's WTF types. The two are not
// related to each other in any way. Converting between them previously meant
// decomposing an interface endpoint into an untyped ScopedMessagePipeHandle,
// with only comments to document the interface type.
//
// Example conversion from the Blink variant into a cross-variant handle:
//
// namespace blink {
//
// void WebLocalFrameImpl::PassGoatTeleporter() {
//   // The fully-qualified type of the Blink variant is
//   // blink::mojom::blink::GoatTeleporter.
//   mojo::PendingRemote<mojom::blink::GoatTeleporter> remote =
//       ProcureGoatTeleporter();
//
//   // `PassGoatTeleporter()`'s argument is a `CrossVariantMojoRemote<>`; see
//   // below example for the other part of this example.
//   web_local_frame_client->PassGoatTeleporter(std::move(remote)));
// }
//
// }  // namespace blink
//
// Example conversion from a cross-variant handle into the regular variant:
//
// namespace content {
//
//   // Note the use of the *InterfaceBase class as the cross-variant handle's
//   // template parameter. This is an empty helper class defined by the
//   // .mojom-shared.h header that is shared as a nested type alias by all
//   // generated C++ interface class variants. The cross-variant types key off
//   // this shared type to provide type safety.
// void RenderFrameImpl::PassGoatTeleporter(
//     blink::CrossVariantMojoRemote<GoatTeleporterInterfaceBase>
//     cross_variant_remote) {
//   // Non-Blink code uses the regular variant, so the `SetGoatTeleporter`
//   // argument has  type `blink::mojom::GoatTeleporter`.
//   frame_host_remote_->SetGoatTeleporter(std::move(cross_variant_remote));
// }
//
// }  // namespace content

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_CROSS_VARIANT_MOJO_UTIL_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_CROSS_VARIANT_MOJO_UTIL_H_

#include <type_traits>
#include <utility>

#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/scoped_interface_endpoint_handle.h"
#include "mojo/public/cpp/system/message_pipe.h"

namespace blink {

// Helpers for passing a variant-less non-associated interface across the Blink
// public API.

template <typename Interface>
class CrossVariantMojoReceiver {
 public:
  CrossVariantMojoReceiver() = default;
  ~CrossVariantMojoReceiver() = default;

  CrossVariantMojoReceiver(CrossVariantMojoReceiver&&) noexcept = default;
  CrossVariantMojoReceiver& operator=(CrossVariantMojoReceiver&&) noexcept =
      default;

  CrossVariantMojoReceiver(const CrossVariantMojoReceiver&) = delete;
  CrossVariantMojoReceiver& operator=(const CrossVariantMojoReceiver&) =
      default;

  template <typename VariantInterface,
            typename CrossVariantBase = typename VariantInterface::Base_,
            std::enable_if_t<
                std::is_same<CrossVariantBase, Interface>::value>* = nullptr>
  CrossVariantMojoReceiver(mojo::PendingReceiver<VariantInterface> receiver)
      : pipe_(receiver.PassPipe()) {}

  CrossVariantMojoReceiver(const mojo::NullReceiver&) {}

  explicit operator bool() const { return pipe_.is_valid(); }

 private:
  friend struct mojo::PendingReceiverConverter<CrossVariantMojoReceiver>;

  mojo::ScopedMessagePipeHandle pipe_;
};

template <typename Interface>
class CrossVariantMojoRemote {
 public:
  CrossVariantMojoRemote() = default;
  ~CrossVariantMojoRemote() = default;

  CrossVariantMojoRemote(CrossVariantMojoRemote&&) noexcept = default;
  CrossVariantMojoRemote& operator=(CrossVariantMojoRemote&&) noexcept =
      default;

  CrossVariantMojoRemote(const CrossVariantMojoRemote&) = delete;
  CrossVariantMojoRemote& operator=(const CrossVariantMojoRemote&) = default;

  template <typename VariantInterface,
            typename CrossVariantBase = typename VariantInterface::Base_,
            std::enable_if_t<
                std::is_same<CrossVariantBase, Interface>::value>* = nullptr>
  CrossVariantMojoRemote(mojo::PendingRemote<VariantInterface> remote)
      : version_(remote.version()), pipe_(remote.PassPipe()) {}

  CrossVariantMojoRemote(const mojo::NullRemote&) {}

  explicit operator bool() const { return pipe_.is_valid(); }

 private:
  friend struct mojo::PendingRemoteConverter<CrossVariantMojoRemote>;

  // Subtle: |version_| is ordered before |pipe_| so it can be initialized first
  // in the move conversion constructor. |PendingRemote::PassPipe()| invalidates
  // all other state on PendingRemote so it must be called last.
  uint32_t version_;
  mojo::ScopedMessagePipeHandle pipe_;
};

// Helpers for passing a variant-less associated interface across the Blink
// public API.

template <typename Interface>
class CrossVariantMojoAssociatedReceiver {
 public:
  CrossVariantMojoAssociatedReceiver() = default;
  ~CrossVariantMojoAssociatedReceiver() = default;

  CrossVariantMojoAssociatedReceiver(CrossVariantMojoAssociatedReceiver&&) =
      default;
  CrossVariantMojoAssociatedReceiver& operator=(
      CrossVariantMojoAssociatedReceiver&&) = default;

  CrossVariantMojoAssociatedReceiver(
      const CrossVariantMojoAssociatedReceiver&) = delete;
  CrossVariantMojoAssociatedReceiver& operator=(
      const CrossVariantMojoAssociatedReceiver&) = default;

  template <typename VariantInterface,
            typename CrossVariantBase = typename VariantInterface::Base_,
            std::enable_if_t<
                std::is_same<CrossVariantBase, Interface>::value>* = nullptr>
  CrossVariantMojoAssociatedReceiver(
      mojo::PendingAssociatedReceiver<VariantInterface> receiver)
      : handle_(receiver.PassHandle()) {}

  CrossVariantMojoAssociatedReceiver(const mojo::NullAssociatedReceiver&) {}

  explicit operator bool() const { return handle_.is_valid(); }

 private:
  friend struct mojo::PendingAssociatedReceiverConverter<
      CrossVariantMojoAssociatedReceiver>;

  mojo::ScopedInterfaceEndpointHandle handle_;
};

template <typename Interface>
class CrossVariantMojoAssociatedRemote {
 public:
  CrossVariantMojoAssociatedRemote() = default;
  ~CrossVariantMojoAssociatedRemote() = default;

  CrossVariantMojoAssociatedRemote(CrossVariantMojoAssociatedRemote&&) =
      default;
  CrossVariantMojoAssociatedRemote& operator=(
      CrossVariantMojoAssociatedRemote&&) = default;

  CrossVariantMojoAssociatedRemote(const CrossVariantMojoAssociatedRemote&) =
      delete;
  CrossVariantMojoAssociatedRemote& operator=(
      const CrossVariantMojoAssociatedRemote&) = default;

  template <typename VariantInterface,
            typename CrossVariantBase = typename VariantInterface::Base_,
            std::enable_if_t<
                std::is_same<CrossVariantBase, Interface>::value>* = nullptr>
  CrossVariantMojoAssociatedRemote(
      mojo::PendingAssociatedRemote<VariantInterface> remote)
      : version_(remote.version()), handle_(remote.PassHandle()) {}

  CrossVariantMojoAssociatedRemote(const mojo::NullAssociatedRemote&) {}

  explicit operator bool() const { return handle_.is_valid(); }

 private:
  friend struct mojo::PendingAssociatedRemoteConverter<
      CrossVariantMojoAssociatedRemote>;

  // Note: unlike CrossVariantMojoRemote, there's no initialization ordering
  // dependency here but keep the same ordering anyway to be consistent.
  uint32_t version_;
  mojo::ScopedInterfaceEndpointHandle handle_;
};

// The `ToCrossVariantMojoType` helpers are more convenient to use when there
// isn't already an explicit CrossVariant{Associated,}{Receiver,Remote} type,
// e.g. Blink code already has the Blink interface variant but wants to share
// common code that requires the regular interface variant.
template <typename VariantBase>
auto ToCrossVariantMojoType(mojo::PendingReceiver<VariantBase>&& in) {
  return blink::CrossVariantMojoReceiver<typename VariantBase::Base_>(
      std::move(in));
}

template <typename VariantBase>
auto ToCrossVariantMojoType(mojo::PendingRemote<VariantBase>&& in) {
  return blink::CrossVariantMojoRemote<typename VariantBase::Base_>(
      std::move(in));
}

template <typename VariantBase>
auto ToCrossVariantMojoType(mojo::PendingAssociatedReceiver<VariantBase>&& in) {
  return blink::CrossVariantMojoAssociatedReceiver<typename VariantBase::Base_>(
      std::move(in));
}

template <typename VariantBase>
auto ToCrossVariantMojoType(mojo::PendingAssociatedRemote<VariantBase>&& in) {
  return blink::CrossVariantMojoAssociatedRemote<typename VariantBase::Base_>(
      std::move(in));
}

}  // namespace blink

namespace mojo {

// Template specializations so //mojo understands how to convert between
// Pending{Associated,}{Receiver,Remote} and the cross-variant types.
template <typename CrossVariantBase>
struct PendingReceiverConverter<
    blink::CrossVariantMojoReceiver<CrossVariantBase>> {
  template <typename VariantBase>
  static PendingReceiver<VariantBase> To(
      blink::CrossVariantMojoReceiver<CrossVariantBase>&& in) {
    return in.pipe_.is_valid()
               ? PendingReceiver<VariantBase>(std::move(in.pipe_))
               : PendingReceiver<VariantBase>();
  }
};

template <typename CrossVariantBase>
struct PendingRemoteConverter<blink::CrossVariantMojoRemote<CrossVariantBase>> {
  template <typename VariantBase>
  static PendingRemote<VariantBase> To(
      blink::CrossVariantMojoRemote<CrossVariantBase>&& in) {
    return in.pipe_.is_valid()
               ? PendingRemote<VariantBase>(std::move(in.pipe_), in.version_)
               : PendingRemote<VariantBase>();
  }
};

template <typename CrossVariantBase>
struct PendingAssociatedReceiverConverter<
    blink::CrossVariantMojoAssociatedReceiver<CrossVariantBase>> {
  template <typename VariantBase>
  static PendingAssociatedReceiver<VariantBase> To(
      blink::CrossVariantMojoAssociatedReceiver<CrossVariantBase>&& in) {
    return in.handle_.is_valid()
               ? PendingAssociatedReceiver<VariantBase>(std::move(in.handle_))
               : PendingAssociatedReceiver<VariantBase>();
  }
};

template <typename CrossVariantBase>
struct PendingAssociatedRemoteConverter<
    blink::CrossVariantMojoAssociatedRemote<CrossVariantBase>> {
  template <typename VariantBase>
  static PendingAssociatedRemote<VariantBase> To(
      blink::CrossVariantMojoAssociatedRemote<CrossVariantBase>&& in) {
    return in.handle_.is_valid() ? PendingAssociatedRemote<VariantBase>(
                                       std::move(in.handle_), in.version_)
                                 : PendingAssociatedRemote<VariantBase>();
  }
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_CROSS_VARIANT_MOJO_UTIL_H_
