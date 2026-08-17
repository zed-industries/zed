// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_TESTING_FAKE_AREA_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_TESTING_FAKE_AREA_SOURCE_H_

#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/modules/storage/cached_storage_area.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class FakeAreaSource : public GarbageCollected<FakeAreaSource>,
                       public CachedStorageArea::Source {
 public:
  explicit FakeAreaSource(const KURL& page_url,
                          LocalDOMWindow* local_dom_window)
      : page_url_(page_url), local_dom_window_(local_dom_window) {}

  void Trace(Visitor* visitor) const override {
    visitor->Trace(local_dom_window_);
    CachedStorageArea::Source::Trace(visitor);
  }

  KURL GetPageUrl() const override { return page_url_; }
  bool EnqueueStorageEvent(const String& key,
                           const String& old_value,
                           const String& new_value,
                           const String& url) override {
    events.push_back(Event{key, old_value, new_value, url});
    return true;
  }

  blink::WebScopedVirtualTimePauser CreateWebScopedVirtualTimePauser(
      const char* name,
      WebScopedVirtualTimePauser::VirtualTaskDuration duration) override {
    return blink::WebScopedVirtualTimePauser();
  }

  LocalDOMWindow* GetDOMWindow() override { return local_dom_window_.Get(); }

  struct Event {
    String key, old_value, new_value, url;

    bool operator==(const Event& other) const {
      return std::tie(key, old_value, new_value, url) ==
             std::tie(other.key, other.old_value, other.new_value, other.url);
    }
  };

  Vector<Event> events;

 private:
  KURL page_url_;
  Member<LocalDOMWindow> local_dom_window_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_TESTING_FAKE_AREA_SOURCE_H_
