// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_SESSION_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_SESSION_STATE_H_

#include <memory>
#include <type_traits>
#include <vector>

#include "third_party/blink/public/mojom/devtools/devtools_agent.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/inspector_protocol/crdtp/span.h"

namespace blink {
class InspectorAgentState;
// An abstraction for the inspector session state in the renderer.
// Makes the |reattach_state| sent from the browser available to the
// InspectorAgentState::Field instances, and queues the updates
// that are made to these fields.
class CORE_EXPORT InspectorSessionState {
 public:
  InspectorSessionState(mojom::blink::DevToolsSessionStatePtr reattach_state);

  // Make |reattach_state| available to InspectorAgentState::Field instances.
  const mojom::blink::DevToolsSessionState* ReattachState() const;

  // Registers a field update in the stored session state and in the updates
  // that are sent back to the browser.
  // A null string for |value| indicates a deletion.
  // TODO(johannes): Lower cost of repeated updates.
  void EnqueueUpdate(const WTF::String& key, const std::vector<uint8_t>* value);

  // Yields and consumes the field updates that have thus far accumulated.
  // These updates are sent back to DevToolsSession on the browser side.
  mojom::blink::DevToolsSessionStatePtr TakeUpdates();

 private:
  const mojom::blink::DevToolsSessionStatePtr reattach_state_;
  mojom::blink::DevToolsSessionStatePtr updates_;
};

// InspectorAgentState connects the fields of inspector agents
// with the renderer-side InspectorSessionState.
class CORE_EXPORT InspectorAgentState {
 private:
  // Trivial Helpers for converting between the value types used for the agent
  // state fields and CBOR byte arrays used for the wire protocol. The point of
  // these is to be able to call overloaded methods from the template
  // implementations below; they just delegate to protocol::Value parsing
  // and serialization.
  static void Serialize(bool v, std::vector<uint8_t>* out);
  static bool Deserialize(crdtp::span<uint8_t> in, bool* v);
  static void Serialize(int32_t v, std::vector<uint8_t>* out);
  static bool Deserialize(crdtp::span<uint8_t> in, int32_t* v);
  static void Serialize(double v, std::vector<uint8_t>* out);
  static bool Deserialize(crdtp::span<uint8_t> in, double* v);
  static void Serialize(const WTF::String& v, std::vector<uint8_t>* out);
  static bool Deserialize(crdtp::span<uint8_t> in, WTF::String* v);
  static void Serialize(const std::vector<uint8_t>& v,
                        std::vector<uint8_t>* out);
  static bool Deserialize(crdtp::span<uint8_t> in, std::vector<uint8_t>* v);

 public:
  // A field is connected to the |agent_state|, which initializes the field
  // via ::InitFrom / ::Decode when the agent is (re)attached.
  class Field {
   public:
    Field(InspectorAgentState* agent_state)
        : prefix_key_(agent_state->RegisterField(this)) {}
    virtual ~Field() = default;

    void InitFrom(InspectorSessionState* session_state) {
      session_state_ = session_state;
      Decode();
    }

    // Clears the field to its default or to the empty map if it's a map field.
    virtual void Clear() = 0;

   protected:
    virtual void Decode() = 0;

    // The field instance is allowed to use/allocate any entry in
    // the session state starting with this prefix. SimpleField instances
    // just use prefix_key_ directly, MapField instances append a suffix.
    const WTF::String prefix_key_;
    InspectorSessionState* session_state_;
  };

  // A simple field with a default value, providing Get, Set, and Clear
  // operations. E.g. an instantiation with WTF::String yields:
  // - const WTF::String& Get();
  // - Set(const WTF::String&);
  // - void Clear();
  template <class ValueType>
  class SimpleField : public Field {
    // Means in practice: const WTF::String& for WTF::String, otherwise same
    // as ValueType.
    using ConstRefType =
        typename std::conditional<std::is_fundamental<ValueType>::value,
                                  ValueType,
                                  const ValueType&>::type;

   public:
    // Constructs a new field registered with |agent_state| which
    // will use |default_value| when not set.
    SimpleField(InspectorAgentState* agent_state, ConstRefType default_value)
        : Field(agent_state),
          default_value_(default_value),
          value_(default_value) {}

    // Returns the value of the field, or the default if not set.
    ConstRefType Get() const { return value_; }

    // Sets the field to the value, or clears if |value| is the default.
    void Set(ConstRefType value) {
      if (value == value_)
        return;
      if (value == default_value_) {
        Clear();
        return;
      }
      value_ = value;
      std::vector<uint8_t> encoded_value;
      Serialize(value, &encoded_value);
      session_state_->EnqueueUpdate(prefix_key_, &encoded_value);
    }

    // Clears the field to its default.
    void Clear() override {
      if (default_value_ == value_)
        return;
      value_ = default_value_;
      session_state_->EnqueueUpdate(prefix_key_, nullptr);
    }

   private:
    // Decodes the key from the encoded session state. This is used
    // during initialization when an agent is reattached.
    void Decode() override {
      const mojom::blink::DevToolsSessionState* reattach_state =
          session_state_->ReattachState();
      if (!reattach_state)
        return;
      auto it = reattach_state->entries.find(prefix_key_);
      if (it != reattach_state->entries.end()) {
        Deserialize(crdtp::span<uint8_t>(it->value->data(), it->value->size()),
                    &value_);
      }
    }

    const ValueType default_value_;
    ValueType value_;
  };

  // A map field provides a map from WTF::String to its value type,
  // and Keys, Get, Set, Clear operations.
  template <class ValueType>
  class MapField : public Field {
    // Means in practice: const WTF::String& for WTF::String, otherwise same
    // as ValueType.
    using ConstRefType =
        typename std::conditional<std::is_fundamental<ValueType>::value,
                                  ValueType,
                                  const ValueType&>::type;

   public:
    // Constructs a new field registered with |agent_state| which
    // will use |default_value| for keys that are not set.
    MapField(InspectorAgentState* agent_state, ConstRefType default_value)
        : Field(agent_state), default_value_(default_value) {}

    // Enumerates the keys for which values are stored in this field.
    // The order of the keys is undefined.
    const auto& Keys() const { return map_.Keys(); }

    // O(1) shortcut for Keys().empty().
    bool IsEmpty() const { return map_.empty(); }

    // Returns the value for a given |key|, or the default value if
    // the key wasn't set.
    ConstRefType Get(const WTF::String& key) const {
      auto it = map_.find(key);
      return it == map_.end() ? default_value_ : it->value;
    }

    // Sets the |value| for |key| as provided, except if |value| is the
    // default value in which case |key| is cleared.
    void Set(const WTF::String& key, ConstRefType value) {
      if (value == default_value_) {
        Clear(key);
        return;
      }
      auto it = map_.find(key);
      if (it != map_.end() && it->value == value)
        return;
      map_.Set(key, value);
      std::vector<uint8_t> encoded_value;
      Serialize(value, &encoded_value);
      session_state_->EnqueueUpdate(prefix_key_ + key, &encoded_value);
    }

    // Clears the entry for |key|.
    void Clear(const WTF::String& key) {
      auto it = map_.find(key);
      if (it == map_.end())
        return;
      map_.erase(it);
      session_state_->EnqueueUpdate(prefix_key_ + key, nullptr);
    }

    // Clears the entire field.
    void Clear() override {
      // TODO(johannes): Handle this in a single update.
      for (const WTF::String& key : map_.Keys()) {
        session_state_->EnqueueUpdate(prefix_key_ + key, nullptr);
      }
      map_.clear();
    }

   private:
    // Decodes the key from the encoded session state. This is used
    // during initialization when an agent is reattached.
    void Decode() override {
      const mojom::blink::DevToolsSessionState* reattach_state =
          session_state_->ReattachState();
      if (!reattach_state)
        return;
      // TODO(johannes): Avoid scanning all keys, let session_state_ provide
      // the keys that match a prefix.
      for (const auto& entry : reattach_state->entries) {
        if (!entry.key.StartsWith(prefix_key_))
          continue;
        WTF::String suffix_key = entry.key.Substring(prefix_key_.length());
        ValueType v;
        if (Deserialize(
                crdtp::span<uint8_t>(entry.value->data(), entry.value->size()),
                &v)) {
          map_.Set(suffix_key, v);
        }
      }
    }

    const ValueType default_value_;
    WTF::HashMap<WTF::String, ValueType> map_;
  };

  using Boolean = SimpleField<bool>;
  using Integer = SimpleField<int32_t>;
  using Double = SimpleField<double>;
  using String = SimpleField<WTF::String>;
  using Bytes = SimpleField<std::vector<uint8_t>>;
  using BooleanMap = MapField<bool>;
  using IntegerMap = MapField<int32_t>;
  using DoubleMap = MapField<double>;
  using StringMap = MapField<WTF::String>;

  InspectorAgentState(const WTF::String& domain_name);

  // Registers |field| and returns the prefix key for it.
  // The prefix key is domain_name + "." + index in fields_ + "/",
  // e.g. "network.0/".
  WTF::String RegisterField(Field* field);

  // Init must be called *after* all fields are registered with the
  // InspectorAgentState. Usually, the fact that fields are registered in
  // the constructors / initializers of agents takes care of it.
  void InitFrom(InspectorSessionState* session_state);

  // Clears all fields registered with this InspectorAgentState instance.
  void ClearAllFields();

 private:
  const WTF::String domain_name_;
  Vector<Field*> fields_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_SESSION_STATE_H_
