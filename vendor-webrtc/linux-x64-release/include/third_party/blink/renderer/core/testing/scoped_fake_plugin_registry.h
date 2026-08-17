// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SCOPED_FAKE_PLUGIN_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SCOPED_FAKE_PLUGIN_REGISTRY_H_

namespace blink {

// Simulates the browser process serving a list of plugins that includes a fake
// PDF plugin and a fake x-webkit-test-webplugin plugin. The PDF plugin doesn't
// create a PluginDocument, whereas the test plugin does.
class ScopedFakePluginRegistry {
 public:
  ScopedFakePluginRegistry();
  ~ScopedFakePluginRegistry();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SCOPED_FAKE_PLUGIN_REGISTRY_H_
