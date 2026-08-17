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

#ifndef QABSTRACTITEMMODELTESTER_H
#define QABSTRACTITEMMODELTESTER_H

#include <QtCore/QObject>
#include <QtTest/qttestglobal.h>
#include <QtCore/QAbstractItemModel>
#include <QtCore/QVariant>

#ifdef QT_GUI_LIB
#include <QtGui/QFont>
#include <QtGui/QColor>
#include <QtGui/QBrush>
#include <QtGui/QPixmap>
#include <QtGui/QImage>
#include <QtGui/QIcon>
#endif

QT_REQUIRE_CONFIG(itemmodeltester);

QT_BEGIN_NAMESPACE

class QAbstractItemModel;
class QAbstractItemModelTester;
class QAbstractItemModelTesterPrivate;

namespace QTestPrivate {
inline bool testDataGuiRoles(QAbstractItemModelTester *tester);
}

class Q_TESTLIB_EXPORT QAbstractItemModelTester : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QAbstractItemModelTester)

public:
    enum class FailureReportingMode {
        QtTest,
        Warning,
        Fatal
    };

    QAbstractItemModelTester(QAbstractItemModel *model, QObject *parent = nullptr);
    QAbstractItemModelTester(QAbstractItemModel *model, FailureReportingMode mode, QObject *parent = nullptr);

    QAbstractItemModel *model() const;
    FailureReportingMode failureReportingMode() const;

private:
    friend inline bool QTestPrivate::testDataGuiRoles(QAbstractItemModelTester *tester);
    bool verify(bool statement, const char *statementStr, const char *description, const char *file, int line);
};

namespace QTestPrivate {
inline bool testDataGuiRoles(QAbstractItemModelTester *tester)
{
#ifdef QT_GUI_LIB

#define MODELTESTER_VERIFY(statement) \
do { \
    if (!tester->verify(static_cast<bool>(statement), #statement, "", __FILE__, __LINE__)) \
        return false; \
} while (false)

    const auto model = tester->model();
    Q_ASSERT(model);

    if (!model->hasChildren())
        return true;

    QVariant variant;

    variant = model->data(model->index(0, 0), Qt::DecorationRole);
    if (variant.isValid()) {
        MODELTESTER_VERIFY(variant.canConvert<QPixmap>()
                           || variant.canConvert<QImage>()
                           || variant.canConvert<QIcon>()
                           || variant.canConvert<QColor>()
                           || variant.canConvert<QBrush>());
    }

    // General Purpose roles that should return a QFont
    variant = model->data(model->index(0, 0), Qt::FontRole);
    if (variant.isValid())
        MODELTESTER_VERIFY(variant.canConvert<QFont>());

    // General Purpose roles that should return a QColor or a QBrush
    variant = model->data(model->index(0, 0), Qt::BackgroundRole);
    if (variant.isValid())
        MODELTESTER_VERIFY(variant.canConvert<QColor>() || variant.canConvert<QBrush>());

    variant = model->data(model->index(0, 0), Qt::ForegroundRole);
    if (variant.isValid())
        MODELTESTER_VERIFY(variant.canConvert<QColor>() || variant.canConvert<QBrush>());

#undef MODELTESTER_VERIFY

#else
    Q_UNUSED(tester);
#endif // QT_GUI_LIB

    return true;
}
} // namespaceQTestPrivate

QT_END_NAMESPACE

#endif // QABSTRACTITEMMODELTESTER_H
