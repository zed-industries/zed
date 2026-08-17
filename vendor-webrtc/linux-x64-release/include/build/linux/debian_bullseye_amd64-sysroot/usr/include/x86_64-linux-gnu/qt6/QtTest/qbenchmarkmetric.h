// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2016 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBENCHMARKMETRIC_H
#define QBENCHMARKMETRIC_H

#include <QtTest/qttestglobal.h>

QT_BEGIN_NAMESPACE


namespace QTest {

enum QBenchmarkMetric {
    FramesPerSecond,
    BitsPerSecond,
    BytesPerSecond,
    WalltimeMilliseconds,
    CPUTicks,
    InstructionReads,
    Events,
    WalltimeNanoseconds,
    BytesAllocated,
    CPUMigrations,
    CPUCycles,
    BusCycles,
    StalledCycles,
    Instructions,
    BranchInstructions,
    BranchMisses,
    CacheReferences,
    CacheReads,
    CacheWrites,
    CachePrefetches,
    CacheMisses,
    CacheReadMisses,
    CacheWriteMisses,
    CachePrefetchMisses,
    ContextSwitches,
    PageFaults,
    MinorPageFaults,
    MajorPageFaults,
    AlignmentFaults,
    EmulationFaults,
    RefCPUCycles,
};

}

QT_END_NAMESPACE

#endif // QBENCHMARK_H
