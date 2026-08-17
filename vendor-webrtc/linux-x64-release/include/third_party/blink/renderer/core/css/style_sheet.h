/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2006, 2008, 2012 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SHEET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SHEET_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSRule;
class KURL;
class MediaList;
class Node;
class StyleSheet;

class CORE_EXPORT StyleSheet : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  StyleSheet() = default;
  ~StyleSheet() override;

  virtual bool disabled() const = 0;
  virtual void setDisabled(bool) = 0;
  virtual Node* ownerNode() const = 0;
  virtual StyleSheet* parentStyleSheet() const { return nullptr; }
  virtual WTF::String href() const = 0;
  virtual WTF::String title() const = 0;
  virtual MediaList* media() { return nullptr; }
  virtual WTF::String type() const = 0;

  virtual CSSRule* ownerRule() const { return nullptr; }
  virtual void ClearOwnerNode() = 0;
  virtual KURL BaseURL() const = 0;
  virtual bool IsLoading() const = 0;
  virtual bool IsCSSStyleSheet() const { return false; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SHEET_H_
