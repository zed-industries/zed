// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_TRACKED_CHILD_URL_LOADER_FACTORY_BUNDLE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_TRACKED_CHILD_URL_LOADER_FACTORY_BUNDLE_H_

#include <stdint.h>

#include <memory>
#include <unordered_map>
#include <utility>

#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/loader/local_resource_loader_config.mojom.h"
#include "third_party/blink/public/platform/child_url_loader_factory_bundle.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

// Identifier for a `TrackedChildURLLoaderFactoryBundle` to key entries in the
// list of observers. `ObserverKey` is derived from
// `TrackedChildURLLoaderFactoryBundle*`, used in comparison only, and are
// never deferenced.
using ObserverKey = std::uintptr_t;

class HostChildURLLoaderFactoryBundle;

// Holds the internal state of a |TrackedChildURLLoaderFactoryBundle| in a form
// that is safe to pass across sequences.
class BLINK_PLATFORM_EXPORT TrackedChildPendingURLLoaderFactoryBundle
    : public ChildPendingURLLoaderFactoryBundle {
 public:
  using HostPtrAndTaskRunner =
      std::pair<base::WeakPtr<HostChildURLLoaderFactoryBundle>,
                scoped_refptr<base::SequencedTaskRunner>>;

  TrackedChildPendingURLLoaderFactoryBundle();
  TrackedChildPendingURLLoaderFactoryBundle(
      mojo::PendingRemote<network::mojom::URLLoaderFactory>
          pending_default_factory,
      SchemeMap pending_scheme_specific_factories,
      OriginMap pending_isolated_world_factories,
      mojo::PendingRemote<network::mojom::URLLoaderFactory>
          pending_subresource_proxying_loader_factory,
      mojo::PendingRemote<network::mojom::URLLoaderFactory>
          pending_keep_alive_loader_factory,
      mojo::PendingAssociatedRemote<blink::mojom::FetchLaterLoaderFactory>
          pending_fetch_later_loader_factory,
      mojom::LocalResourceLoaderConfigPtr local_resource_loader_config,
      std::unique_ptr<HostPtrAndTaskRunner> main_thread_host_bundle,
      bool bypass_redirect_checks);
  TrackedChildPendingURLLoaderFactoryBundle(
      const TrackedChildPendingURLLoaderFactoryBundle&) = delete;
  TrackedChildPendingURLLoaderFactoryBundle& operator=(
      const TrackedChildPendingURLLoaderFactoryBundle&) = delete;
  ~TrackedChildPendingURLLoaderFactoryBundle() override;

  std::unique_ptr<HostPtrAndTaskRunner>& main_thread_host_bundle() {
    return main_thread_host_bundle_;
  }

  bool IsTrackedChildPendingURLLoaderFactoryBundle() const override;

 protected:
  // ChildPendingURLLoaderFactoryBundle overrides.
  scoped_refptr<network::SharedURLLoaderFactory> CreateFactory() override;

  std::unique_ptr<HostPtrAndTaskRunner> main_thread_host_bundle_;
};

// This class extends |ChildURLLoaderFactoryBundle| to support a
// host/observer tracking logic. There will be a single
// |HostChildURLLoaderFactoryBundle| owned by |RenderFrameImpl| which lives on
// the main thread, and multiple |TrackedChildURLLoaderFactoryBundle| on the
// worker thread (for Workers) or the main thread (for frames from
// 'window.open()'). Both |Host/TrackedChildURLLoaderFactoryBundle::Clone()| can
// be used to create a tracked bundle to the original host bundle. These two
// classes are required to bring bundles back online in the event of Network
// Service crash.
class BLINK_PLATFORM_EXPORT TrackedChildURLLoaderFactoryBundle final
    : public ChildURLLoaderFactoryBundle {
 public:
  using HostPtrAndTaskRunner =
      std::pair<base::WeakPtr<HostChildURLLoaderFactoryBundle>,
                scoped_refptr<base::SequencedTaskRunner>>;

  // Posts a task to the host bundle on main thread to start tracking |this|.
  explicit TrackedChildURLLoaderFactoryBundle(
      std::unique_ptr<TrackedChildPendingURLLoaderFactoryBundle>
          pending_factories);
  TrackedChildURLLoaderFactoryBundle(
      const TrackedChildURLLoaderFactoryBundle&) = delete;
  TrackedChildURLLoaderFactoryBundle& operator=(
      const TrackedChildURLLoaderFactoryBundle&) = delete;

  // ChildURLLoaderFactoryBundle overrides.
  // Returns |std::unique_ptr<TrackedChildPendingURLLoaderFactoryBundle>|.
  std::unique_ptr<network::PendingSharedURLLoaderFactory> Clone() override;

 private:
  friend class HostChildURLLoaderFactoryBundle;

  // Posts a task to the host bundle on main thread to stop tracking |this|.
  ~TrackedChildURLLoaderFactoryBundle() override;

  // Helper method to post a task to the host bundle on main thread to start
  // tracking |this|.
  void AddObserverOnMainThread();

  // Helper method to post a task to the host bundle on main thread to start
  // tracking |this|.
  void RemoveObserverOnMainThread();

  // Callback method to receive updates from the host bundle.
  void OnUpdate(std::unique_ptr<network::PendingSharedURLLoaderFactory>
                    pending_factories);

  // |WeakPtr| and |TaskRunner| of the host bundle. Can be copied and passed
  // across sequences.
  std::unique_ptr<HostPtrAndTaskRunner> main_thread_host_bundle_;
  base::WeakPtrFactory<TrackedChildURLLoaderFactoryBundle> weak_ptr_factory_{
      this};
};

// |HostChildURLLoaderFactoryBundle| lives entirely on the main thread, and all
// methods should be invoked on the main thread or through PostTask. See
// comments in |TrackedChildURLLoaderFactoryBundle| for details about the
// tracking logic.
class BLINK_PLATFORM_EXPORT HostChildURLLoaderFactoryBundle final
    : public ChildURLLoaderFactoryBundle {
 public:
  HostChildURLLoaderFactoryBundle(const HostChildURLLoaderFactoryBundle&) =
      delete;
  HostChildURLLoaderFactoryBundle& operator=(
      const HostChildURLLoaderFactoryBundle&) = delete;
  using ObserverPtrAndTaskRunner =
      std::pair<base::WeakPtr<TrackedChildURLLoaderFactoryBundle>,
                scoped_refptr<base::SequencedTaskRunner>>;
  using ObserverList =
      std::unordered_map<ObserverKey,
                         std::unique_ptr<ObserverPtrAndTaskRunner>>;

  explicit HostChildURLLoaderFactoryBundle(
      scoped_refptr<base::SequencedTaskRunner> task_runner);

  // ChildURLLoaderFactoryBundle overrides.
  // Returns |std::unique_ptr<TrackedChildPendingURLLoaderFactoryBundle>|.
  std::unique_ptr<network::PendingSharedURLLoaderFactory> Clone() override;
  bool IsHostChildURLLoaderFactoryBundle() const override;

  // Update this bundle with `pending_factories`, and post cloned
  // `pending_factories` to tracked bundles.
  void UpdateThisAndAllClones(
      std::unique_ptr<blink::PendingURLLoaderFactoryBundle> pending_factories);

 private:
  friend class TrackedChildURLLoaderFactoryBundle;

  ~HostChildURLLoaderFactoryBundle() override;

  // Must be called by the newly created |TrackedChildURLLoaderFactoryBundle|.
  void AddObserver(ObserverKey observer_key,
                   std::unique_ptr<ObserverPtrAndTaskRunner> observer_info);

  // Must be called by the observer before it was destroyed.
  void RemoveObserver(ObserverKey observer_key);

  // Post an update to the tracked bundle on the worker thread (for Workers) or
  // the main thread (for frames from 'window.open()'). Safe to use after the
  // tracked bundle has been destroyed.
  void NotifyUpdateOnMainOrWorkerThread(
      ObserverPtrAndTaskRunner* observer_bundle,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_factories);

  // Contains |WeakPtr| and |TaskRunner| to tracked bundles.
  std::unique_ptr<ObserverList> observer_list_;

  scoped_refptr<base::SequencedTaskRunner> task_runner_;
  base::WeakPtrFactory<HostChildURLLoaderFactoryBundle> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_TRACKED_CHILD_URL_LOADER_FACTORY_BUNDLE_H_
