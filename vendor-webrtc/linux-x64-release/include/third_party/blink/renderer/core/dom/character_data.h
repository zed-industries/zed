/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2008, 2009 Apple Inc. All rights
 * reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CHARACTER_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CHARACTER_DATA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class NodeCloningData;
struct TextDiffRange;

class CORE_EXPORT CharacterData : public Node {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CharacterData(const CharacterData&) = delete;
  CharacterData& operator=(const CharacterData&) = delete;

  // Makes the data Parkable. This enables de-duplication and compression.
  void MakeParkable();
  const String& data() const {
    return is_parkable_ ? parkable_data_.ToString() : data_;
  }
  void setData(const String&);
  unsigned length() const {
    return is_parkable_ ? parkable_data_.length() : data_.length();
  }
  String substringData(unsigned offset, unsigned count, ExceptionState&);
  void appendData(const String&);
  void replaceData(unsigned offset,
                   unsigned count,
                   const String&,
                   ExceptionState&);

  void insertData(unsigned offset, const String&, ExceptionState&);
  void deleteData(unsigned offset, unsigned count, ExceptionState&);

  bool ContainsOnlyWhitespaceOrEmpty() const;

  void ParserAppendData(const String&);

 protected:
  CharacterData(TreeScope& tree_scope,
                const String& text,
                ConstructionType type)
      : Node(&tree_scope, type),
        data_(!text.IsNull() ? text : g_empty_string),
        is_parkable_(false) {
    DCHECK(type == kCreateComment || type == kCreateText ||
           type == kCreateCdataSection ||
           type == kCreateProcessingInstruction || type == kCreateEditingText);
  }

  CharacterData(TreeScope& tree_scope, String&& text, ConstructionType type)
      : Node(&tree_scope, type), data_(std::move(text)), is_parkable_(false) {
    DCHECK(type == kCreateComment || type == kCreateText ||
           type == kCreateCdataSection ||
           type == kCreateProcessingInstruction || type == kCreateEditingText);
    DCHECK(!is_parkable_);
    if (data_.IsNull()) {
      data_ = g_empty_string;
    }
  }

  ~CharacterData() noexcept override {
    if (is_parkable_) {
      parkable_data_.~ParkableString();
    } else {
      data_.~String();
    }
  }

  void SetDataWithoutUpdate(const String& data) {
    DCHECK(!data.IsNull());
    if (!is_parkable_) {
      data_ = data;
      return;
    }
    is_parkable_ = false;
    parkable_data_.~ParkableString();
    new (&data_) String(data);
  }

  enum UpdateSource {
    kUpdateFromParser,
    kUpdateFromNonParser,
  };
  void DidModifyData(const String& old_value, UpdateSource);

  union {
    ParkableString parkable_data_;
    String data_;
  };
  bool is_parkable_;

 private:
  String nodeValue() const final;
  void setNodeValue(const String&, ExceptionState&) final;
  bool IsCharacterDataNode() const final { return true; }
  void SetDataAndUpdate(const String&,
                        const TextDiffRange&,
                        UpdateSource = kUpdateFromNonParser);
  Node* Clone(Document& factory,
              NodeCloningData& data,
              ContainerNode* append_to,
              ExceptionState& append_exception_state) const override;
  virtual CharacterData* CloneWithData(Document&, const String&) const = 0;

  bool IsContainerNode() const =
      delete;  // This will catch anyone doing an unnecessary check.
  bool IsElementNode() const =
      delete;  // This will catch anyone doing an unnecessary check.
};

template <>
struct DowncastTraits<CharacterData> {
  static bool AllowFrom(const Node& node) { return node.IsCharacterDataNode(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CHARACTER_DATA_H_
