// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFILEICONPROVIDER_H
#define QFILEICONPROVIDER_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qfileinfo.h>
#include <QtCore/qscopedpointer.h>
#include <QtGui/qicon.h>
#include <QtGui/qabstractfileiconprovider.h>

QT_BEGIN_NAMESPACE


class QFileIconProviderPrivate;

class Q_WIDGETS_EXPORT QFileIconProvider : public QAbstractFileIconProvider
{
public:
    QFileIconProvider();
    ~QFileIconProvider();

    QIcon icon(IconType type) const override;
    QIcon icon(const QFileInfo &info) const override;

private:
    Q_DECLARE_PRIVATE(QFileIconProvider)
    Q_DISABLE_COPY(QFileIconProvider)
};

QT_END_NAMESPACE

#endif // QFILEICONPROVIDER_H
