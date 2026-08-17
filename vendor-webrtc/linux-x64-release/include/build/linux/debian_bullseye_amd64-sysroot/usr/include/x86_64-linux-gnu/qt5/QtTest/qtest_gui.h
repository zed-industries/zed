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

#ifndef QTEST_GUI_H
#define QTEST_GUI_H

// enable GUI features
#ifndef QT_GUI_LIB
#define QT_GUI_LIB
#endif
#if 0
#pragma qt_class(QtTestGui)
#endif

#include <QtTest/qtestassert.h>
#include <QtTest/qtest.h>
#include <QtTest/qtestevent.h>
#include <QtTest/qtestmouse.h>
#include <QtTest/qtesttouch.h>
#include <QtTest/qtestkeyboard.h>

#include <QtGui/qcolor.h>
#include <QtGui/qpixmap.h>
#include <QtGui/qimage.h>
#include <QtGui/qregion.h>
#include <QtGui/qvector2d.h>
#include <QtGui/qvector3d.h>
#include <QtGui/qvector4d.h>
#include <QtGui/qicon.h>

#if 0
// inform syncqt
#pragma qt_no_master_include
#endif

QT_BEGIN_NAMESPACE


namespace QTest
{

template<> inline char *toString(const QColor &color)
{
    return qstrdup(color.name(QColor::HexArgb).toLocal8Bit().constData());
}

template<> inline char *toString(const QRegion &region)
{
    QByteArray result = "QRegion(";
    if (region.isNull()) {
        result += "null";
    } else if (region.isEmpty()) {
        result += "empty";
    } else {
        const auto rects = region.begin();
        const int rectCount = region.rectCount();
        if (rectCount > 1) {
            result += QByteArray::number(rectCount);
            result += " rectangles, ";
        }
        for (int i = 0; i < rectCount; ++i) {
            if (i)
                result += ", ";
            const QRect &r = rects[i];
            result += QByteArray::number(r.width());
            result += 'x';
            result += QByteArray::number(r.height());
            if (r.x() >= 0)
                result += '+';
            result += QByteArray::number(r.x());
            if (r.y() >= 0)
                result += '+';
            result += QByteArray::number(r.y());
        }
    }
    result += ')';
    return qstrdup(result.constData());
}

#if !defined(QT_NO_VECTOR2D) || defined(Q_CLANG_QDOC)
template<> inline char *toString(const QVector2D &v)
{
    QByteArray result = "QVector2D(" + QByteArray::number(double(v.x())) + ", "
        + QByteArray::number(double(v.y())) + ')';
    return qstrdup(result.constData());
}
#endif // !QT_NO_VECTOR2D
#if !defined(QT_NO_VECTOR3D) || defined(Q_CLANG_QDOC)
template<> inline char *toString(const QVector3D &v)
{
    QByteArray result = "QVector3D(" + QByteArray::number(double(v.x())) + ", "
        + QByteArray::number(double(v.y())) + ", " + QByteArray::number(double(v.z())) + ')';
    return qstrdup(result.constData());
}
#endif // !QT_NO_VECTOR3D
#if !defined(QT_NO_VECTOR4D) || defined(Q_CLANG_QDOC)
template<> inline char *toString(const QVector4D &v)
{
    QByteArray result = "QVector4D(" + QByteArray::number(double(v.x())) + ", "
        + QByteArray::number(double(v.y())) + ", " + QByteArray::number(double(v.z()))
        + ", " + QByteArray::number(double(v.w())) + ')';
    return qstrdup(result.constData());
}
#endif // !QT_NO_VECTOR4D

inline bool qCompare(QIcon const &t1, QIcon const &t2, const char *actual, const char *expected,
                    const char *file, int line)
{
    QTEST_ASSERT(sizeof(QIcon) == sizeof(void *));
    return qCompare(*reinterpret_cast<void * const *>(&t1),
                   *reinterpret_cast<void * const *>(&t2), actual, expected, file, line);
}

inline bool qCompare(QImage const &t1, QImage const &t2,
                     const char *actual, const char *expected, const char *file, int line)
{
    char msg[1024];
    msg[0] = '\0';
    const bool t1Null = t1.isNull();
    const bool t2Null = t2.isNull();
    if (t1Null != t2Null) {
        qsnprintf(msg, 1024, "Compared QImages differ.\n"
                  "   Actual   (%s).isNull(): %d\n"
                  "   Expected (%s).isNull(): %d", actual, t1Null, expected, t2Null);
        return compare_helper(false, msg, nullptr, nullptr, actual, expected, file, line);
    }
    if (t1Null && t2Null)
        return compare_helper(true, nullptr, nullptr, nullptr, actual, expected, file, line);
    if (!qFuzzyCompare(t1.devicePixelRatioF(), t2.devicePixelRatioF())) {
        qsnprintf(msg, 1024, "Compared QImages differ in device pixel ratio.\n"
                  "   Actual   (%s): %g\n"
                  "   Expected (%s): %g",
                  actual, t1.devicePixelRatioF(),
                  expected, t2.devicePixelRatioF());
        return compare_helper(false, msg, nullptr, nullptr, actual, expected, file, line);
    }
    if (t1.width() != t2.width() || t1.height() != t2.height()) {
        qsnprintf(msg, 1024, "Compared QImages differ in size.\n"
                  "   Actual   (%s): %dx%d\n"
                  "   Expected (%s): %dx%d",
                  actual, t1.width(), t1.height(),
                  expected, t2.width(), t2.height());
        return compare_helper(false, msg, nullptr, nullptr, actual, expected, file, line);
    }
    if (t1.format() != t2.format()) {
        qsnprintf(msg, 1024, "Compared QImages differ in format.\n"
                  "   Actual   (%s): %d\n"
                  "   Expected (%s): %d",
                  actual, t1.format(), expected, t2.format());
        return compare_helper(false, msg, nullptr, nullptr, actual, expected, file, line);
    }
    return compare_helper(t1 == t2, "Compared values are not the same",
                          toString(t1), toString(t2), actual, expected, file, line);
}

inline bool qCompare(QPixmap const &t1, QPixmap const &t2, const char *actual, const char *expected,
                    const char *file, int line)
{
    char msg[1024];
    msg[0] = '\0';
    const bool t1Null = t1.isNull();
    const bool t2Null = t2.isNull();
    if (t1Null != t2Null) {
        qsnprintf(msg, 1024, "Compared QPixmaps differ.\n"
                  "   Actual   (%s).isNull(): %d\n"
                  "   Expected (%s).isNull(): %d", actual, t1Null, expected, t2Null);
        return compare_helper(false, msg, nullptr, nullptr, actual, expected, file, line);
    }
    if (t1Null && t2Null)
        return compare_helper(true, nullptr, nullptr, nullptr, actual, expected, file, line);
    if (!qFuzzyCompare(t1.devicePixelRatioF(), t2.devicePixelRatioF())) {
        qsnprintf(msg, 1024, "Compared QPixmaps differ in device pixel ratio.\n"
                  "   Actual   (%s): %g\n"
                  "   Expected (%s): %g",
                  actual, t1.devicePixelRatioF(),
                  expected, t2.devicePixelRatioF());
        return compare_helper(false, msg, nullptr, nullptr, actual, expected, file, line);
    }
    if (t1.width() != t2.width() || t1.height() != t2.height()) {
        qsnprintf(msg, 1024, "Compared QPixmaps differ in size.\n"
                  "   Actual   (%s): %dx%d\n"
                  "   Expected (%s): %dx%d",
                  actual, t1.width(), t1.height(),
                  expected, t2.width(), t2.height());
        return compare_helper(false, msg, nullptr, nullptr, actual, expected, file, line);
    }
    return qCompare(t1.toImage(), t2.toImage(), actual, expected, file, line);
}

}

QT_END_NAMESPACE

#endif
