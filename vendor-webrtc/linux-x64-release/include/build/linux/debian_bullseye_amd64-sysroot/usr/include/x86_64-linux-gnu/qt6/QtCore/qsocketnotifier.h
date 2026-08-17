// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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

    explicit QSocketNotifier(Type, QObject *parent = nullptr);
    QSocketNotifier(qintptr socket, Type, QObject *parent = nullptr);
    ~QSocketNotifier();

    void setSocket(qintptr socket);
    qintptr socket() const;
    Type type() const;

    bool isValid() const;
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
    QT_MOC_COMPAT void activated(int socket, QPrivateSignal);
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

    Q_DECL_CONSTEXPR_NOT_WIN Q_IMPLICIT
    QSocketDescriptor(DescriptorType descriptor = DescriptorType(-1)) noexcept : sockfd(descriptor)
    {
    }

#if defined(Q_OS_WIN) || defined(Q_QDOC)
    Q_IMPLICIT QSocketDescriptor(qintptr desc) noexcept : sockfd(DescriptorType(desc)) {}
    Q_IMPLICIT operator qintptr() const noexcept { return qintptr(sockfd); }
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

QT_DECL_METATYPE_EXTERN_TAGGED(QSocketNotifier::Type, QSocketNotifier_Type, Q_CORE_EXPORT)
QT_DECL_METATYPE_EXTERN(QSocketDescriptor, Q_CORE_EXPORT)

#endif // QSOCKETNOTIFIER_H
