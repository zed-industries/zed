// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_MEDIAN_H
#define QTCONCURRENT_MEDIAN_H

#include <QtConcurrent/qtconcurrent_global.h>

#if !defined(QT_NO_CONCURRENT) || defined(Q_CLANG_QDOC)

#include <algorithm>
#include <cstring>

QT_BEGIN_NAMESPACE

namespace QtConcurrent {

class Median
{
public:
    enum { BufferSize = 7 };

    Median()
        : currentMedian(), currentIndex(0), valid(false), dirty(true)
    {
        std::fill_n(values, static_cast<int>(BufferSize), 0.0);
    }

    void reset()
    {
        std::fill_n(values, static_cast<int>(BufferSize), 0.0);
        currentIndex = 0;
        valid = false;
        dirty = true;
    }

    void addValue(double value)
    {
        ++currentIndex;
        if (currentIndex == BufferSize) {
            currentIndex = 0;
            valid = true;
        }

        // Only update the cached median value when we have to, that
        // is when the new value is on then other side of the median
        // compared to the current value at the index.
        const double currentIndexValue = values[currentIndex];
        if ((currentIndexValue > currentMedian && currentMedian > value)
            || (currentMedian > currentIndexValue && value > currentMedian)) {
            dirty = true;
        }

        values[currentIndex] = value;
    }

    bool isMedianValid() const
    {
        return valid;
    }

    double median()
    {
        if (dirty) {
            dirty = false;

            double sorted[BufferSize];
            ::memcpy(&sorted, &values, sizeof(sorted));
            std::sort(sorted, sorted + static_cast<int>(BufferSize));
            currentMedian = sorted[BufferSize / 2];
        }

        return currentMedian;
    }

private:
    double values[BufferSize];
    double currentMedian;
    int currentIndex;
    bool valid;
    bool dirty;
};

} // namespace QtConcurrent

QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
