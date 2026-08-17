// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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
    void setUseFetchMore(bool value);

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
