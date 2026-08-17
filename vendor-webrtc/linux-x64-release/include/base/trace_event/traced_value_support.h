// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TRACED_VALUE_SUPPORT_H_
#define BASE_TRACE_EVENT_TRACED_VALUE_SUPPORT_H_

#include <optional>
#include <string_view>

#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ref.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/strings/utf_string_conversions.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_proto.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value.h"

// This file contains specialisations for trace serialisation for key
// widely-used //base classes. As these specialisations require full definition
// of perfetto::TracedValue and almost every source unit in Chromium requires
// one of these //base concepts, include specialiazations here and expose them
// to the users including trace_event.h, rather than adding a dependency from
// scoped_refptr.h et al on traced_value.h.

namespace perfetto {

// If T is serialisable into a trace, scoped_refptr<T> is serialisable as well.
template <class T>
struct TraceFormatTraits<scoped_refptr<T>,
                         perfetto::check_traced_value_support_t<T>> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const scoped_refptr<T>& value) {
    if (!value) {
      std::move(context).WritePointer(nullptr);
      return;
    }
    perfetto::WriteIntoTracedValue(std::move(context), *value);
  }

  template <class MessageType>
  static void WriteIntoTrace(perfetto::TracedProto<MessageType> context,
                             const scoped_refptr<T>& value) {
    if (value) {
      // Proto message without any fields is treated as nullptr.
      return;
    }
    perfetto::WriteIntoTracedProto(std::move(context), *value);
  }
};

// If T is serialisable into a trace, base::WeakPtr<T> is serialisable as well.
template <class T>
struct TraceFormatTraits<::base::WeakPtr<T>,
                         perfetto::check_traced_value_support_t<T>> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const ::base::WeakPtr<T>& value) {
    if (!value) {
      std::move(context).WritePointer(nullptr);
      return;
    }
    perfetto::WriteIntoTracedValue(std::move(context), *value);
  }
};

// If T is serialisable into a trace, std::optional<T> is serialisable as well.
// Note that we need definitions for both std::optional<T>& and
// const std::optional<T>& (unlike scoped_refptr and WeakPtr above), as
// dereferencing const scoped_refptr<T>& gives you T, while dereferencing const
// std::optional<T>& gives you const T&.
template <class T>
struct TraceFormatTraits<::std::optional<T>,
                         perfetto::check_traced_value_support_t<T>> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const ::std::optional<T>& value) {
    if (!value) {
      std::move(context).WritePointer(nullptr);
      return;
    }
    perfetto::WriteIntoTracedValue(std::move(context), *value);
  }

  static void WriteIntoTrace(perfetto::TracedValue context,
                             ::std::optional<T>& value) {
    if (!value) {
      std::move(context).WritePointer(nullptr);
      return;
    }
    perfetto::WriteIntoTracedValue(std::move(context), *value);
  }
};

// If T is serialisable into a trace, raw_ptr<T> is serialisable as well.
template <class T, ::base::RawPtrTraits Traits>
struct TraceFormatTraits<::base::raw_ptr<T, Traits>,
                         perfetto::check_traced_value_support_t<T>> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const ::base::raw_ptr<T, Traits>& value) {
    perfetto::WriteIntoTracedValue(std::move(context), value.get());
  }

  static void WriteIntoTrace(perfetto::TracedValue context,
                             ::base::raw_ptr<T, Traits>& value) {
    perfetto::WriteIntoTracedValue(std::move(context), value.get());
  }
};

// If T is serialisable into a trace, raw_ref<T> is serialisable as well.
template <class T, ::base::RawPtrTraits Traits>
struct TraceFormatTraits<::base::raw_ref<T, Traits>,
                         perfetto::check_traced_value_support_t<T>> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const ::base::raw_ref<T, Traits>& value) {
    perfetto::WriteIntoTracedValue(std::move(context), value.get());
  }

  static void WriteIntoTrace(perfetto::TracedValue context,
                             ::base::raw_ref<T, Traits>& value) {
    perfetto::WriteIntoTracedValue(std::move(context), value.get());
  }
};

// Time-related classes.
// TODO(altimin): Make them first-class primitives in TracedValue and Perfetto
// UI.
template <>
struct TraceFormatTraits<::base::TimeDelta> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const ::base::TimeDelta& value) {
    std::move(context).WriteInt64(value.InMicroseconds());
  }
};

template <>
struct TraceFormatTraits<::base::TimeTicks> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const ::base::TimeTicks& value) {
    perfetto::WriteIntoTracedValue(std::move(context), value.since_origin());
  }
};

template <>
struct TraceFormatTraits<::base::Time> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const ::base::Time& value) {
    perfetto::WriteIntoTracedValue(std::move(context), value.since_origin());
  }
};

// base::UnguessableToken.
// TODO(altimin): Add first-class primitive, which will allow to show a
// human-comprehensible alias for all unguessable tokens instead.
template <>
struct TraceFormatTraits<::base::UnguessableToken> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const ::base::UnguessableToken& value) {
    return std::move(context).WriteString(value.ToString());
  }
};

// UTF-16 string support.
template <>
struct TraceFormatTraits<std::u16string> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const std::u16string& value) {
    return std::move(context).WriteString(::base::UTF16ToUTF8(value));
  }
};

template <size_t N>
struct TraceFormatTraits<char16_t[N]> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const char16_t value[N]) {
    return std::move(context).WriteString(
        ::base::UTF16ToUTF8(::std::u16string_view(value)));
  }
};

template <>
struct TraceFormatTraits<const char16_t*> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const char16_t* value) {
    return std::move(context).WriteString(
        ::base::UTF16ToUTF8(::std::u16string_view(value)));
  }
};

// Wide string support.
template <>
struct TraceFormatTraits<std::wstring> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const std::wstring& value) {
    return std::move(context).WriteString(::base::WideToUTF8(value));
  }
};

template <size_t N>
struct TraceFormatTraits<wchar_t[N]> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const wchar_t value[N]) {
    return std::move(context).WriteString(
        ::base::WideToUTF8(::std::wstring_view(value)));
  }
};

template <>
struct TraceFormatTraits<const wchar_t*> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             const wchar_t* value) {
    return std::move(context).WriteString(
        ::base::WideToUTF8(::std::wstring_view(value)));
  }
};

// std::string_view support.
template <>
struct TraceFormatTraits<::std::u16string_view> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             ::std::u16string_view value) {
    return std::move(context).WriteString(::base::UTF16ToUTF8(value));
  }
};

template <>
struct TraceFormatTraits<::std::wstring_view> {
  static void WriteIntoTrace(perfetto::TracedValue context,
                             ::std::wstring_view value) {
    return std::move(context).WriteString(::base::WideToUTF8(value));
  }
};

}  // namespace perfetto

#endif  // BASE_TRACE_EVENT_TRACED_VALUE_SUPPORT_H_
