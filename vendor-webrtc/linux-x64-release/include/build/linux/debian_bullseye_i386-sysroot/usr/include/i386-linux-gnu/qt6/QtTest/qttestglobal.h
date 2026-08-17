// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTTESTGLOBAL_H
#define QTTESTGLOBAL_H

#include <QtCore/qglobal.h>
#include <QtTest/qttestlib-config.h>
#include <QtTest/qttestexports.h>

QT_BEGIN_NAMESPACE

#if (defined Q_CC_HPACC) && (defined __ia64)
# ifdef Q_TESTLIB_EXPORT
#  undef Q_TESTLIB_EXPORT
# endif
# define Q_TESTLIB_EXPORT
#endif

#define QTEST_VERSION     QT_VERSION
#define QTEST_VERSION_STR QT_VERSION_STR

namespace QTest
{
    enum TestFailMode { Abort = 1, Continue = 2 };
    enum class ComparisonOperation {
        CustomCompare, /* Used for QCOMPARE() */
        Equal,
        NotEqual,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
    };
}

QT_END_NAMESPACE

#endif
