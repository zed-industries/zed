/*
 * Copyright (C) 2006, 2007, 2008, 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCOPED_PAGE_PAUSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCOPED_PAGE_PAUSER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/main_thread_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Page;

class CORE_EXPORT ScopedPagePauser final {
  USING_FAST_MALLOC(ScopedPagePauser);

 public:
  explicit ScopedPagePauser(Page* primary_page);
  ScopedPagePauser();
  ScopedPagePauser(const ScopedPagePauser&) = delete;
  ScopedPagePauser& operator=(const ScopedPagePauser&) = delete;
  ~ScopedPagePauser();

 private:
  friend class Page;

  static void SetPaused(Page* primary_page, bool);
  static bool IsActive();

  std::unique_ptr<MainThreadScheduler::RendererPauseHandle> pause_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCOPED_PAGE_PAUSER_H_
