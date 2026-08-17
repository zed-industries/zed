/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MEDIA_QUERY_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MEDIA_QUERY_RESULT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/media_list.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"

namespace blink {

class MediaQuerySetResult {
  DISALLOW_NEW();

 public:
  MediaQuerySetResult(const MediaQuerySet& media_queries, bool result)
      : media_queries_(&media_queries), result_(result) {}
  void Trace(Visitor* visitor) const { visitor->Trace(media_queries_); }

  const MediaQuerySet& MediaQueries() const { return *media_queries_; }

  bool Result() const { return result_; }

 private:
  Member<const MediaQuerySet> media_queries_;
  bool result_;
};

struct MediaQueryResultFlags {
  DISALLOW_NEW();

 public:
  bool operator==(const MediaQueryResultFlags& o) const {
    return (unit_flags == o.unit_flags) &&
           (is_viewport_dependent == o.is_viewport_dependent) &&
           (is_device_dependent == o.is_device_dependent);
  }
  bool operator!=(const MediaQueryResultFlags& o) const {
    return !(*this == o);
  }

  void Add(const MediaQueryResultFlags& o) {
    unit_flags |= o.unit_flags;
    is_viewport_dependent |= o.is_viewport_dependent;
    is_device_dependent |= o.is_device_dependent;
  }

  void Clear() {
    unit_flags = 0;
    is_viewport_dependent = false;
    is_device_dependent = false;
  }

  // Or'ed MediaQueryExpValue::UnitFlags.
  unsigned unit_flags = 0;
  // True if the result is viewport dependent, for example if the 'width'
  // media feature was used in the evaluation.
  bool is_viewport_dependent = false;
  // True if the result is device dependent, for example if the 'device-width'
  // media feature was used in the evaluation.
  bool is_device_dependent = false;
};

}  // namespace blink

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(blink::MediaQuerySetResult)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MEDIA_QUERY_RESULT_H_
