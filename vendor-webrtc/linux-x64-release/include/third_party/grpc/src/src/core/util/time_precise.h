//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_UTIL_TIME_PRECISE_H
#define GRPC_SRC_CORE_UTIL_TIME_PRECISE_H

#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>

// Depending on the platform gpr_get_cycle_counter() can have a resolution as
// low as a usec. Use other clock sources or gpr_precise_clock_now(),
// where you need high resolution clocks.
//
// Using gpr_get_cycle_counter() is preferred to using Timestamp::Now()
// whenever possible.

#if GPR_CYCLE_COUNTER_CUSTOM
typedef int64_t gpr_cycle_counter;
gpr_cycle_counter gpr_get_cycle_counter();
#elif GPR_CYCLE_COUNTER_RDTSC_32
typedef int64_t gpr_cycle_counter;
inline gpr_cycle_counter gpr_get_cycle_counter() {
  int64_t ret;
  __asm__ volatile("rdtsc" : "=A"(ret));
  return ret;
}
#elif GPR_CYCLE_COUNTER_RDTSC_64
typedef int64_t gpr_cycle_counter;
inline gpr_cycle_counter gpr_get_cycle_counter() {
  uint64_t low, high;
  __asm__ volatile("rdtsc" : "=a"(low), "=d"(high));
  return (high << 32) | low;
}
#elif GPR_CYCLE_COUNTER_FALLBACK
// TODO(soheil): add support for mrs on Arm.

// Real time in micros.
typedef double gpr_cycle_counter;
gpr_cycle_counter gpr_get_cycle_counter();
#else
#error Must define exactly one of \
    GPR_CYCLE_COUNTER_RDTSC_32, \
    GPR_CYCLE_COUNTER_RDTSC_64, \
    GPR_CYCLE_COUNTER_CUSTOM, or \
    GPR_CYCLE_COUNTER_FALLBACK
#endif

void gpr_precise_clock_init(void);
void gpr_precise_clock_now(gpr_timespec* clk);
gpr_timespec gpr_cycle_counter_to_time(gpr_cycle_counter cycles);
gpr_timespec gpr_cycle_counter_sub(gpr_cycle_counter a, gpr_cycle_counter b);

#endif  // GRPC_SRC_CORE_UTIL_TIME_PRECISE_H
