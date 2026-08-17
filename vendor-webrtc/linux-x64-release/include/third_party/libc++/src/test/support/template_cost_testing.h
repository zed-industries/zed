//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_TEMPLATE_COST_TESTING_H
#define TEST_SUPPORT_TEMPLATE_COST_TESTING_H

// This file contains macros used to repeat an expression many times.
// This is useful for testing the compile time and memory usage
// of templates.

#define REPEAT_10(DO_IT) \
    DO_IT() DO_IT() DO_IT() DO_IT() DO_IT() \
    DO_IT() DO_IT() DO_IT() DO_IT() DO_IT()
#define REPEAT_100(DO_IT) \
    REPEAT_10(DO_IT) REPEAT_10(DO_IT) REPEAT_10(DO_IT) REPEAT_10(DO_IT) REPEAT_10(DO_IT) \
    REPEAT_10(DO_IT) REPEAT_10(DO_IT) REPEAT_10(DO_IT) REPEAT_10(DO_IT) REPEAT_10(DO_IT)
#define REPEAT_200(DO_IT) \
    REPEAT_100(DO_IT) REPEAT_100(DO_IT)
#define REPEAT_300(DO_IT) \
    REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT)
#define REPEAT_500(DO_IT) \
    REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT)
#define REPEAT_1000(DO_IT) \
    REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT) \
    REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT) REPEAT_100(DO_IT)
#define REPEAT_5000(DO_IT) \
    REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) REPEAT_1000(DO_IT)
#define REPEAT_10000(DO_IT) \
    REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) \
    REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) REPEAT_1000(DO_IT) REPEAT_1000(DO_IT)

#endif // TEST_SUPPORT_TEMPLATE_COST_TESTING_H
