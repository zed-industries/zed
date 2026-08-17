/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#ifndef QSOCKETNOTIFIER_H
#define QSOCKETNOTIFIER_H

#include <QtCore/qobject.h>

QT_BEGIN_NAMESPACE

class QSocketDescriptor;
class QSocketNotifierPrivate;
class Q_CORE_EXPORT QSocketNotifier : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QSocketNotifier)

public:
    enum Type { Read, Write, Exception };

    QSocketNotifier(qintptr socket, Type, QObject *parent = nullptr);
    ~QSocketNotifier();

    qintptr socket() const;
    Type type() const;

    bool isEnabled() const;

public Q_SLOTS:
    void setEnabled(bool);

Q_SIGNALS:
#if defined(Q_MOC_RUN)
    // Add default arguments during Q_MOC_RUN which makes moc generate "signals" which takes less
    // parameters, but we won't actually allow emitting without all 3. This lets users use the
    // string-based connect without specifying QSocketNotifier::Type as one of the parameters.
    void activated(QSocketDescriptor socket, QSocketNotifier::Type activationEvent = Read,
                   QPrivateSignal = {});
#else
    void activated(QSocketDescriptor socket, QSocketNotifier::Type activationEvent, QPrivateSignal);
#endif

    // ### Qt7: consider removing it.
    // The old signal is compiled internally, but hidden outside of this class.
    // This means the PMF-based connect(..) will automatically, on recompile, pick up the new
    // version while the old-style connect(..) can query the metaobject system for this version.
#if defined(Q_MOC_RUN) || defined(BUILDING_QSOCKETNOTIFIER) || defined(Q_QDOC)
    void activated(int socket, QPrivateSignal);
#endif

protected:
    bool event(QEvent *) override;

private:
    Q_DISABLE_COPY(QSocketNotifier)
};

class QSocketDescriptor
{
public:
#if defined(Q_OS_WIN) || defined(Q_QDOC)
    using DescriptorType = Qt::HANDLE;
#define Q_DECL_CONSTEXPR_NOT_WIN
#else
    using DescriptorType = int;
#define Q_DECL_CONSTEXPR_NOT_WIN Q_DECL_CONSTEXPR
#endif

    /* implicit */ Q_DECL_CONSTEXPR_NOT_WIN
    QSocketDescriptor(DescriptorType descriptor = DescriptorType(-1)) noexcept : sockfd(descriptor)
    {
    }

#if defined(Q_OS_WIN) || defined(Q_QDOC)
    /* implicit */ QSocketDescriptor(qintptr desc) noexcept : sockfd(DescriptorType(desc)) {}
    operator qintptr() const noexcept { return qintptr(sockfd); }
    Q_DECL_CONSTEXPR Qt::HANDLE winHandle() const noexcept { return sockfd; }
#endif
    Q_DECL_CONSTEXPR operator DescriptorType() const noexcept { return sockfd; }

    Q_DECL_CONSTEXPR_NOT_WIN bool isValid() const noexcept { return *this != QSocketDescriptor(); }

    friend Q_DECL_CONSTEXPR_NOT_WIN bool operator==(QSocketDescriptor lhs,
                                                    QSocketDescriptor rhs) noexcept
    {
        return lhs.sockfd == rhs.sockfd;
    }
    friend Q_DECL_CONSTEXPR_NOT_WIN bool operator!=(QSocketDescriptor lhs,
                                                    QSocketDescriptor rhs) noexcept
    {
        return lhs.sockfd != rhs.sockfd;
    }

#undef Q_DECL_CONSTEXPR_NOT_WIN

private:
    DescriptorType sockfd;
};

QT_END_NAMESPACE
Q_DECLARE_METATYPE(QSocketNotifier::Type)
Q_DECLARE_METATYPE(QSocketDescriptor)

#endif // QSOCKETNOTIFIER_H
