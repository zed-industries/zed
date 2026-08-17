/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QSESSIONMANAGER_H
#define QSESSIONMANAGER_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qobject.h>
#include <QtGui/qwindowdefs.h>
#include <QtCore/qstring.h>
#include <QtCore/qstringlist.h>

#ifndef QT_NO_SESSIONMANAGER

QT_BEGIN_NAMESPACE


class QGuiApplication;

class QSessionManagerPrivate;

class Q_GUI_EXPORT  QSessionManager : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QSessionManager)
    QSessionManager(QGuiApplication *app, QString &id, QString &key);
    ~QSessionManager();
public:
    QString sessionId() const;
    QString sessionKey() const;

    bool allowsInteraction();
    bool allowsErrorInteraction();
    void release();

    void cancel();

    enum RestartHint {
        RestartIfRunning,
        RestartAnyway,
        RestartImmediately,
        RestartNever
    };
    void setRestartHint(RestartHint);
    RestartHint restartHint() const;

    void setRestartCommand(const QStringList&);
    QStringList restartCommand() const;
    void setDiscardCommand(const QStringList&);
    QStringList discardCommand() const;

    void setManagerProperty(const QString& name, const QString& value);
    void setManagerProperty(const QString& name, const QStringList& value);

    bool isPhase2() const;
    void requestPhase2();

private:
    friend class QGuiApplication;
    friend class QGuiApplicationPrivate;
};

QT_END_NAMESPACE

#endif // QT_NO_SESSIONMANAGER

#endif // QSESSIONMANAGER_H
