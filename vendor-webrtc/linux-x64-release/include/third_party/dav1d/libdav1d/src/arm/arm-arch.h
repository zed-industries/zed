/*
 * Copyright © 2024, VideoLAN and dav1d authors
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef ARM_ARM_ARCH_H
#define ARM_ARM_ARCH_H

/* Compatibility header to define __ARM_ARCH with older compilers */
#ifndef __ARM_ARCH

#ifdef _M_ARM
#define __ARM_ARCH _M_ARM

#elif defined(__ARM_ARCH_8A__) || defined(_M_ARM64)
#define __ARM_ARCH 8

#elif defined(__ARM_ARCH_7__) || defined(__ARM_ARCH_7A__) || \
      defined(__ARM_ARCH_7EM__) || defined(__ARM_ARCH_7R__) || \
      defined(__ARM_ARCH_7M__) || defined(__ARM_ARCH_7S__)
#define __ARM_ARCH 7

#elif defined(__ARM_ARCH_6__) || defined(__ARM_ARCH_6J__) || \
      defined(__ARM_ARCH_6K__) || defined(__ARM_ARCH_6T2__) || \
      defined(__ARM_ARCH_6Z__) || defined(__ARM_ARCH_6ZK__)
#define __ARM_ARCH 6

#elif defined(__ARM_ARCH_5__) || defined(__ARM_ARCH_5T__) || \
      defined(__ARM_ARCH_5E__) || defined(__ARM_ARCH_5TE__)
#define __ARM_ARCH 5

#elif defined(__ARM_ARCH_4__) || defined(__ARM_ARCH_4T__)
#define __ARM_ARCH 4

#elif defined(__ARM_ARCH_3__) || defined(__ARM_ARCH_3M__)
#define __ARM_ARCH 3

#elif defined(__ARM_ARCH_2__)
#define __ARM_ARCH 2

#else
#error Unknown ARM architecture version
#endif

#endif /* !__ARM_ARCH */

#endif /* ARM_ARM_ARCH_H */
