/*
 *  Copyright (C) 2006 Apple Computer, Inc.
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Library General Public
 *  License as published by the Free Software Foundation; either
 *  version 2 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Library General Public License for more details.
 *
 *  You should have received a copy of the GNU Library General Public License
 *  along with this library; see the file COPYING.LIB.  If not, write to
 *  the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 *  Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_GET_PTR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_GET_PTR_H_

template <typename>
class scoped_refptr;

namespace WTF {

template <typename T>
inline T* GetPtr(T* p) {
  return p;
}

template <typename T>
inline T* GetPtr(T& p) {
  return &p;
}

template <typename T>
inline T* GetPtr(const scoped_refptr<T>& p) {
  return p.get();
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_GET_PTR_H_
