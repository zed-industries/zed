// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_LENS_H_
#define TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_LENS_H_

#include <string_view>

namespace caspian {
class BaseSymbol;

class BaseLens {
 public:
  virtual ~BaseLens() = default;
  virtual std::string_view ParentName(const BaseSymbol& symbol) = 0;
};

class IdPathLens : public BaseLens {
 public:
  std::string_view ParentName(const BaseSymbol& symbol) override;
};

class ContainerLens : public BaseLens {
 public:
  std::string_view ParentName(const BaseSymbol& symbol) override;
};

class ComponentLens : public BaseLens {
 public:
  std::string_view ParentName(const BaseSymbol& symbol) override;
};

class TemplateLens : public BaseLens {
 public:
  std::string_view ParentName(const BaseSymbol& symbol) override;
};

class GeneratedLens : public BaseLens {
 public:
  std::string_view ParentName(const BaseSymbol& symbol) override;
};
}  // namespace caspian

#endif  // TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_LENS_H_
