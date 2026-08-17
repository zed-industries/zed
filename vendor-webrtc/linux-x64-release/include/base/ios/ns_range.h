// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_IOS_NS_RANGE_H_
#define BASE_IOS_NS_RANGE_H_

#import <Foundation/Foundation.h>

// Returns whether the two NSRange are equal.
inline bool operator==(NSRange lhs, NSRange rhs) {
  return lhs.location == rhs.location && lhs.length == rhs.length;
}

// Returns whether the two NSRange are not equal.
inline bool operator!=(NSRange lhs, NSRange rhs) {
  return lhs.location != rhs.location || lhs.length != rhs.length;
}

#endif  // BASE_IOS_NS_RANGE_H_
