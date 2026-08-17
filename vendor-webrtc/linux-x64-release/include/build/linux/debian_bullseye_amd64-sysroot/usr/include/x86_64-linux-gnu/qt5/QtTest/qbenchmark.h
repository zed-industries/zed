/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtTest module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

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
    for (QTest::QBenchmarkIterationController __iteration_controller; \
            __iteration_controller.isDone() == false; __iteration_controller.next())

#define QBENCHMARK_ONCE \
    for (QTest::QBenchmarkIterationController __iteration_controller(QTest::QBenchmarkIterationController::RunOnce); \
            __iteration_controller.isDone() == false; __iteration_controller.next())

namespace QTest
{
    void Q_TESTLIB_EXPORT setBenchmarkResult(qreal result, QBenchmarkMetric metric);
}

// --- END public API ---

QT_END_NAMESPACE

#endif // QBENCHMARK_H
