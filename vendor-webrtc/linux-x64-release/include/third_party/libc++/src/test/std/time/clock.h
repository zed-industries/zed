//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef CLOCK_H
#define CLOCK_H

#include <chrono>

class Clock
{
    typedef std::chrono::nanoseconds                 duration;
    typedef duration::rep                            rep;
    typedef duration::period                         period;
    typedef std::chrono::time_point<Clock, duration> time_point;
    static const bool is_steady =                    false;

    static time_point now();
};

#endif // CLOCK_H
