// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_NIX_SCOPED_XDG_ACTIVATION_TOKEN_INJECTOR_H_
#define BASE_NIX_SCOPED_XDG_ACTIVATION_TOKEN_INJECTOR_H_

#include "base/base_export.h"
#include "base/memory/raw_ref.h"

namespace base {

class CommandLine;
class Environment;

namespace nix {

// Sets the global xdg-activation token after reading from the launching app and
// injects it temporarily into the command line if it needs to be sent to
// another browser process.
// The token switch is removed from the command line on destruction.
class BASE_EXPORT ScopedXdgActivationTokenInjector {
 public:
  ScopedXdgActivationTokenInjector(base::CommandLine& command_line,
                                   base::Environment& env);

  ScopedXdgActivationTokenInjector(const ScopedXdgActivationTokenInjector&) =
      delete;
  ScopedXdgActivationTokenInjector& operator=(
      const ScopedXdgActivationTokenInjector&) = delete;

  ~ScopedXdgActivationTokenInjector();

 private:
  const raw_ref<CommandLine> command_line_;
  bool token_injected_ = false;
};

}  // namespace nix

}  // namespace base

#endif  // BASE_NIX_SCOPED_XDG_ACTIVATION_TOKEN_INJECTOR_H_
