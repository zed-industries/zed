/****************************************************************************
**
** Copyright (C) 2016 Alex Trotsenko <alex1973tr@gmail.com>
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtNetwork module of the Qt Toolkit.
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

#ifndef QSCTPSOCKET_H
#define QSCTPSOCKET_H

#include <QtNetwork/qtcpsocket.h>
#include <QtNetwork/qnetworkdatagram.h>

QT_BEGIN_NAMESPACE

#if !defined(QT_NO_SCTP) || defined(Q_CLANG_QDOC)

class QSctpSocketPrivate;

class Q_NETWORK_EXPORT QSctpSocket : public QTcpSocket
{
    Q_OBJECT
public:
    explicit QSctpSocket(QObject *parent = nullptr);
    virtual ~QSctpSocket();

    void close() override;
    void disconnectFromHost() override;

    void setMaximumChannelCount(int count);
    int maximumChannelCount() const;
    bool isInDatagramMode() const;

    QNetworkDatagram readDatagram();
    bool writeDatagram(const QNetworkDatagram &datagram);

protected:
    qint64 readData(char *data, qint64 maxlen) override;
    qint64 readLineData(char *data, qint64 maxlen) override;

private:
    Q_DISABLE_COPY(QSctpSocket)
    Q_DECLARE_PRIVATE(QSctpSocket)
};

#endif // QT_NO_SCTP

QT_END_NAMESPACE

#endif // QSCTPSOCKET_H
