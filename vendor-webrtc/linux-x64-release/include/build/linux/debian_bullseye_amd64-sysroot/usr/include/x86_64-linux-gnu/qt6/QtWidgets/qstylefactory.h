// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTYLEFACTORY_H
#define QSTYLEFACTORY_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qstringlist.h>

QT_BEGIN_NAMESPACE


class QStyle;

class Q_WIDGETS_EXPORT QStyleFactory
{
public:
    static QStringList keys();
    static QStyle *create(const QString&);
};

QT_END_NAMESPACE

#endif // QSTYLEFACTORY_H
