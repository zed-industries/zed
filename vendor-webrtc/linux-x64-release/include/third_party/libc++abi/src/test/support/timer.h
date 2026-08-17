//===----------------------------------------------------------------------===////
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===////

#ifndef TIMER_H
#define TIMER_H

// Define LIBCXXABI_USE_TIMER to enable testing with a timer.
#if defined(LIBCXXABI_USE_TIMER)

#include <chrono>
#include <cstdio>

class timer
{
    typedef std::chrono::high_resolution_clock Clock;
    typedef Clock::time_point TimePoint;
    typedef std::chrono::microseconds MicroSeconds;
public:
    timer() : m_start(Clock::now()) {}

    timer(timer const &) = delete;
    timer & operator=(timer const &) = delete;

    ~timer()
    {
        using std::chrono::duration_cast;
        TimePoint end = Clock::now();
        MicroSeconds us = duration_cast<MicroSeconds>(end - m_start);
        std::printf("%d microseconds\n", us.count());
    }

private:
    TimePoint m_start;
};

#else /* LIBCXXABI_USE_TIMER */

class timer
{
public:
    timer() {}
    timer(timer const &) = delete;
    timer & operator=(timer const &) = delete;
    ~timer() {}
};

#endif /* LIBCXXABI_USE_TIMER */

#endif /* TIMER_H */
