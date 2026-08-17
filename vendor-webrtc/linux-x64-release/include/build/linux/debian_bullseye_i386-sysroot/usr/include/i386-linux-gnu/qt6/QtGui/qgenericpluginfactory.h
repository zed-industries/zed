// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QGENERICPLUGINFACTORY_H
#define QGENERICPLUGINFACTORY_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qstringlist.h>

QT_BEGIN_NAMESPACE


class QString;
class QObject;

class Q_GUI_EXPORT QGenericPluginFactory
{
public:
    static QStringList keys();
    static QObject *create(const QString&, const QString &);
};

QT_END_NAMESPACE

#endif // QGENERICPLUGINFACTORY_H
