/*
 * Copyright (C) 2009, 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_MOJO_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_MOJO_H_

#include "mojo/public/cpp/bindings/deprecated_interface_types_forward.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace mojo {
template <typename Interface>
class PendingReceiver;
template <typename Interface>
class PendingRemote;
template <typename Interface>
class PendingAssociatedRemote;
template <typename Interface>
class PendingAssociatedReceiver;
template <typename Interface>
class ScopedHandleBase;
class DataPipeProducerHandle;
typedef ScopedHandleBase<DataPipeProducerHandle> ScopedDataPipeProducerHandle;
class DataPipeConsumerHandle;
typedef ScopedHandleBase<DataPipeConsumerHandle> ScopedDataPipeConsumerHandle;
}  // namespace mojo

namespace WTF {

template <typename Interface>
struct CrossThreadCopier<mojo::PendingReceiver<Interface>>
    : public CrossThreadCopierByValuePassThrough<
          mojo::PendingReceiver<Interface>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <typename Interface>
struct CrossThreadCopier<mojo::PendingRemote<Interface>>
    : public CrossThreadCopierByValuePassThrough<
          mojo::PendingRemote<Interface>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <typename Interface>
struct CrossThreadCopier<mojo::PendingAssociatedRemote<Interface>>
    : public CrossThreadCopierByValuePassThrough<
          mojo::PendingAssociatedRemote<Interface>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <typename Interface>
struct CrossThreadCopier<mojo::PendingAssociatedReceiver<Interface>>
    : public CrossThreadCopierByValuePassThrough<
          mojo::PendingAssociatedReceiver<Interface>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<mojo::ScopedDataPipeProducerHandle>
    : public CrossThreadCopierByValuePassThrough<
          mojo::ScopedDataPipeProducerHandle> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<mojo::ScopedDataPipeConsumerHandle>
    : public CrossThreadCopierByValuePassThrough<
          mojo::ScopedDataPipeConsumerHandle> {
  STATIC_ONLY(CrossThreadCopier);
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_MOJO_H_
