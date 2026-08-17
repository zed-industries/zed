// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBENCHMARK_H
#define QBENCHMARK_H

#include <QtTest/qttestglobal.h>
#include <QtTest/qbenchmarkmetric.h>

QT_BEGIN_NAMESPACE


namespace QTest
{

//
//  W A R N I N G
//  -------------
//
// The QBenchmarkIterationController class is not a part of the
// Qt Test API. It exists purely as an implementation detail.
//
//
class Q_TESTLIB_EXPORT QBenchmarkIterationController
{
public:
    enum RunMode { RepeatUntilValidMeasurement, RunOnce };
    QBenchmarkIterationController();
    QBenchmarkIterationController(RunMode runMode);
    ~QBenchmarkIterationController();
    bool isDone();
    void next();
    int i;
};

}

// --- BEGIN public API ---

#define QBENCHMARK \
    for (QTest::QBenchmarkIterationController _q_iteration_controller; \
            _q_iteration_controller.isDone() == false; _q_iteration_controller.next())

#define QBENCHMARK_ONCE \
    for (QTest::QBenchmarkIterationController _q_iteration_controller(QTest::QBenchmarkIterationController::RunOnce); \
            _q_iteration_controller.isDone() == false; _q_iteration_controller.next())

namespace QTest
{
    void Q_TESTLIB_EXPORT setBenchmarkResult(qreal result, QBenchmarkMetric metric);
}

// --- END public API ---

QT_END_NAMESPACE

#endif // QBENCHMARK_H
