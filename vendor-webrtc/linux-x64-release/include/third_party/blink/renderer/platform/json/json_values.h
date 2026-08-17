/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_JSON_JSON_VALUES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_JSON_JSON_VALUES_H_

#include <memory>
#include <utility>

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class StringBuilder;
}  // namespace WTF

namespace blink {

class JSONArray;
class JSONObject;
class JSONValue;

class PLATFORM_EXPORT JSONValue {
  USING_FAST_MALLOC(JSONValue);

 public:
  static const int kMaxDepth = 1000;

  JSONValue(const JSONValue&) = delete;
  JSONValue& operator=(const JSONValue&) = delete;
  virtual ~JSONValue() = default;

  static std::unique_ptr<JSONValue> Null() {
    return base::WrapUnique(new JSONValue());
  }

  enum ValueType {
    kTypeNull = 0,
    kTypeBoolean,
    kTypeInteger,
    kTypeDouble,
    kTypeString,
    kTypeObject,
    kTypeArray
  };

  ValueType GetType() const { return type_; }

  bool IsNull() const { return type_ == kTypeNull; }

  virtual bool AsBoolean(bool* output) const;
  virtual bool AsDouble(double* output) const;
  virtual bool AsInteger(int* output) const;
  virtual bool AsString(String* output) const;

  String ToJSONString() const;
  String ToPrettyJSONString() const;
  virtual void WriteJSON(WTF::StringBuilder* output) const;
  virtual void PrettyWriteJSON(WTF::StringBuilder* output) const;
  virtual std::unique_ptr<JSONValue> Clone() const;

  static String QuoteString(const String&);

 protected:
  JSONValue() : type_(kTypeNull) {}
  explicit JSONValue(ValueType type) : type_(type) {}
  virtual void PrettyWriteJSONInternal(WTF::StringBuilder* output,
                                       int depth) const;

 private:
  friend class JSONObject;
  friend class JSONArray;

  ValueType type_;
};

class PLATFORM_EXPORT JSONBasicValue : public JSONValue {
 public:
  explicit JSONBasicValue(bool value)
      : JSONValue(kTypeBoolean), bool_value_(value) {}
  explicit JSONBasicValue(int value)
      : JSONValue(kTypeInteger), integer_value_(value) {}
  explicit JSONBasicValue(double value)
      : JSONValue(kTypeDouble), double_value_(value) {}

  bool AsBoolean(bool* output) const override;
  bool AsDouble(double* output) const override;
  bool AsInteger(int* output) const override;
  void WriteJSON(WTF::StringBuilder* output) const override;
  std::unique_ptr<JSONValue> Clone() const override;

 private:
  union {
    bool bool_value_;
    double double_value_;
    int integer_value_;
  };
};

class PLATFORM_EXPORT JSONString : public JSONValue {
 public:
  explicit JSONString(const String& value)
      : JSONValue(kTypeString), string_value_(value) {}
  explicit JSONString(const char* value)
      : JSONValue(kTypeString), string_value_(value) {}

  bool AsString(String* output) const override;
  void WriteJSON(WTF::StringBuilder* output) const override;
  std::unique_ptr<JSONValue> Clone() const override;

 private:
  String string_value_;
};

class PLATFORM_EXPORT JSONObject : public JSONValue {
 public:
  using Entry = std::pair<String, JSONValue*>;

  JSONObject();

  static JSONObject* Cast(JSONValue* value) {
    if (!value || value->GetType() != kTypeObject)
      return nullptr;
    return static_cast<JSONObject*>(value);
  }

  static const JSONObject* Cast(const JSONValue* value) {
    if (!value || value->GetType() != kTypeObject)
      return nullptr;
    return static_cast<const JSONObject*>(value);
  }

  static std::unique_ptr<JSONObject> From(std::unique_ptr<JSONValue> value) {
    auto maybe_object = base::WrapUnique(JSONObject::Cast(value.get()));
    if (maybe_object)
      value.release();
    return maybe_object;
  }

  static void Cast(JSONObject*) = delete;
  static void Cast(std::unique_ptr<JSONObject>) = delete;

  void WriteJSON(WTF::StringBuilder* output) const override;
  std::unique_ptr<JSONValue> Clone() const override;

  wtf_size_t size() const { return data_.size(); }

  // Return true if the key was newly added (it did not already exist).
  bool SetBoolean(const String& name, bool);
  bool SetInteger(const String& name, int);
  bool SetDouble(const String& name, double);
  bool SetString(const String& name, const String&);
  bool SetValue(const String& name, std::unique_ptr<JSONValue>);
  bool SetObject(const String& name, std::unique_ptr<JSONObject>);
  bool SetArray(const String& name, std::unique_ptr<JSONArray>);

  bool GetBoolean(const String& name, bool* output) const;
  bool GetInteger(const String& name, int* output) const;
  bool GetDouble(const String& name, double* output) const;
  bool GetString(const String& name, String* output) const;

  // This function is not "GetObject" in order to avoid replacement by
  // windows.h.
  JSONObject* GetJSONObject(const String& name) const;
  JSONArray* GetArray(const String& name) const;
  JSONValue* Get(const String& name) const;
  Entry at(wtf_size_t index) const;

  bool BooleanProperty(const String& name, bool default_value) const;
  int IntegerProperty(const String& name, int default_value) const;
  double DoubleProperty(const String& name, double default_value) const;
  void Remove(const String& name);

  ~JSONObject() override;

 protected:
  void PrettyWriteJSONInternal(WTF::StringBuilder* output,
                               int depth) const override;

 private:
  template <typename T>
  bool Set(const String& key, std::unique_ptr<T>& value) {
    DCHECK(value);
    if (data_.Set(key, std::move(value)).is_new_entry) {
      order_.push_back(key);
      return true;
    }
    return false;
  }

  using Dictionary = HashMap<String, std::unique_ptr<JSONValue>>;
  Dictionary data_;
  Vector<String> order_;
};

class PLATFORM_EXPORT JSONArray : public JSONValue {
 public:

  static JSONArray* Cast(JSONValue* value) {
    if (!value || value->GetType() != kTypeArray)
      return nullptr;
    return static_cast<JSONArray*>(value);
  }

  static const JSONArray* Cast(const JSONValue* value) {
    if (!value || value->GetType() != kTypeArray)
      return nullptr;
    return static_cast<const JSONArray*>(value);
  }

  static std::unique_ptr<JSONArray> From(std::unique_ptr<JSONValue> value) {
    auto maybe_array = base::WrapUnique(JSONArray::Cast(value.get()));
    if (maybe_array)
      value.release();
    return maybe_array;
  }

  static void Cast(JSONArray*) = delete;
  static void Cast(std::unique_ptr<JSONArray>) = delete;

  JSONArray();
  ~JSONArray() override;

  void WriteJSON(WTF::StringBuilder* output) const override;
  std::unique_ptr<JSONValue> Clone() const override;

  void PushBoolean(bool);
  void PushInteger(int);
  void PushDouble(double);
  void PushString(const String&);
  void PushValue(std::unique_ptr<JSONValue>);
  void PushObject(std::unique_ptr<JSONObject>);
  void PushArray(std::unique_ptr<JSONArray>);

  JSONValue* at(wtf_size_t index) const;
  wtf_size_t size() const { return data_.size(); }

 protected:
  void PrettyWriteJSONInternal(WTF::StringBuilder* output,
                               int depth) const override;

 private:
  Vector<std::unique_ptr<JSONValue>> data_;
};

inline constexpr char kJSONNullString[] = "null";
inline constexpr char kJSONTrueString[] = "true";
inline constexpr char kJSONFalseString[] = "false";

PLATFORM_EXPORT void EscapeStringForJSON(const String&, WTF::StringBuilder*);
void DoubleQuoteStringForJSON(const String&, WTF::StringBuilder*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_JSON_JSON_VALUES_H_
