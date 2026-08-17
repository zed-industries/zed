// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFACTORYINTERFACE_H
#define QFACTORYINTERFACE_H

#include <QtCore/qobject.h>
#include <QtCore/qstringlist.h>

QT_BEGIN_NAMESPACE

struct Q_CORE_EXPORT QFactoryInterface
{
    virtual ~QFactoryInterface();
    virtual QStringList keys() const = 0;
};

#ifndef Q_CLANG_QDOC
Q_DECLARE_INTERFACE(QFactoryInterface, "org.qt-project.Qt.QFactoryInterface")
#endif

QT_END_NAMESPACE

#endif // QFACTORYINTERFACE_H
