// Copyright (C) 2016 Alex Trotsenko <alex1973tr@gmail.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSCTPSERVER_H
#define QSCTPSERVER_H

#include <QtNetwork/qtcpserver.h>

QT_BEGIN_NAMESPACE


#if !defined(QT_NO_SCTP) || defined(Q_CLANG_QDOC)

class QSctpServerPrivate;
class QSctpSocket;

class Q_NETWORK_EXPORT QSctpServer : public QTcpServer
{
    Q_OBJECT
public:
    explicit QSctpServer(QObject *parent = nullptr);
    virtual ~QSctpServer();

    void setMaximumChannelCount(int count);
    int maximumChannelCount() const;

    QSctpSocket *nextPendingDatagramConnection();

protected:
    void incomingConnection(qintptr handle) override;

private:
    Q_DISABLE_COPY(QSctpServer)
    Q_DECLARE_PRIVATE(QSctpServer)
};

#endif // QT_NO_SCTP

QT_END_NAMESPACE

#endif // QSCTPSERVER_H
