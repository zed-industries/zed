//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef NOTCONSTRUCTIBLE_H
#define NOTCONSTRUCTIBLE_H

#include <functional>

class NotConstructible {
  NotConstructible(const NotConstructible&);
  NotConstructible& operator=(const NotConstructible&);

public:
};

inline bool operator==(const NotConstructible&, const NotConstructible&) { return true; }

template <>
struct std::hash<NotConstructible> {
  typedef NotConstructible argument_type;
  typedef std::size_t result_type;

  std::size_t operator()(const NotConstructible&) const { return 0; }
};

#endif // NOTCONSTRUCTIBLE_H
