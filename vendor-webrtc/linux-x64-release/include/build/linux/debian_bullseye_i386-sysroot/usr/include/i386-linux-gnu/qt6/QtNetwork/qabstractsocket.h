// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QABSTRACTSOCKET_H
#define QABSTRACTSOCKET_H

#include <QtNetwork/qtnetworkglobal.h>
#if QT_VERSION >= QT_VERSION_CHECK(7, 0, 0)
#include <QtNetwork/qabstractsocket.h>
#endif
#ifdef Q_CLANG_QDOC
#include <QtNetwork/qhostaddress.h>
#endif
#include <QtCore/qiodevice.h>
#include <QtCore/qobject.h>
#ifndef QT_NO_DEBUG_STREAM
#include <QtCore/qdebug.h>
#endif

QT_BEGIN_NAMESPACE


class QHostAddress;
#ifndef QT_NO_NETWORKPROXY
class QNetworkProxy;
#endif
class QAbstractSocketPrivate;
class QAuthenticator;

class Q_NETWORK_EXPORT QAbstractSocket : public QIODevice
{
    Q_OBJECT
    Q_MOC_INCLUDE(<QtNetwork/qauthenticator.h>)

public:
    enum SocketType {
        TcpSocket,
        UdpSocket,
        SctpSocket,
        UnknownSocketType = -1
    };
    Q_ENUM(SocketType)

#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0)
    enum NetworkLayerProtocol {
        IPv4Protocol,
        IPv6Protocol,
        AnyIPProtocol,
        UnknownNetworkLayerProtocol = -1
    };
    Q_ENUM(NetworkLayerProtocol)
#else
    // compatibility with Qt 4 to 6
    using NetworkLayerProtocol = QHostAddress::NetworkLayerProtocol;
    static constexpr auto IPv4Protocol = QHostAddress::IPv4Protocol;
    static constexpr auto IPv6Protocol = QHostAddress::IPv6Protocol;
    static constexpr auto AnyIPProtocol = QHostAddress::AnyIPProtocol;
    static constexpr auto UnknownNetworkLayerProtocol = QHostAddress::UnknownNetworkLayerProtocol;
#endif

    enum SocketError {
        ConnectionRefusedError,
        RemoteHostClosedError,
        HostNotFoundError,
        SocketAccessError,
        SocketResourceError,
        SocketTimeoutError,                     /* 5 */
        DatagramTooLargeError,
        NetworkError,
        AddressInUseError,
        SocketAddressNotAvailableError,
        UnsupportedSocketOperationError,        /* 10 */
        UnfinishedSocketOperationError,
        ProxyAuthenticationRequiredError,
        SslHandshakeFailedError,
        ProxyConnectionRefusedError,
        ProxyConnectionClosedError,             /* 15 */
        ProxyConnectionTimeoutError,
        ProxyNotFoundError,
        ProxyProtocolError,
        OperationError,
        SslInternalError,                       /* 20 */
        SslInvalidUserDataError,
        TemporaryError,

        UnknownSocketError = -1
    };
    Q_ENUM(SocketError)
    enum SocketState {
        UnconnectedState,
        HostLookupState,
        ConnectingState,
        ConnectedState,
        BoundState,
        ListeningState,
        ClosingState
    };
    Q_ENUM(SocketState)
    enum SocketOption {
        LowDelayOption, // TCP_NODELAY
        KeepAliveOption, // SO_KEEPALIVE
        MulticastTtlOption, // IP_MULTICAST_TTL
        MulticastLoopbackOption, // IP_MULTICAST_LOOPBACK
        TypeOfServiceOption, //IP_TOS
        SendBufferSizeSocketOption,    //SO_SNDBUF
        ReceiveBufferSizeSocketOption,  //SO_RCVBUF
        PathMtuSocketOption // IP_MTU
    };
    Q_ENUM(SocketOption)
    enum BindFlag {
        DefaultForPlatform = 0x0,
        ShareAddress = 0x1,
        DontShareAddress = 0x2,
        ReuseAddressHint = 0x4
    };
    Q_DECLARE_FLAGS(BindMode, BindFlag)
    enum PauseMode {
        PauseNever = 0x0,
        PauseOnSslErrors = 0x1
    };
    Q_DECLARE_FLAGS(PauseModes, PauseMode)

    QAbstractSocket(SocketType socketType, QObject *parent);
    virtual ~QAbstractSocket();

    virtual void resume(); // to continue after proxy authentication required, SSL errors etc.
    PauseModes pauseMode() const;
    void setPauseMode(PauseModes pauseMode);

    virtual bool bind(const QHostAddress &address, quint16 port = 0,
                      BindMode mode = DefaultForPlatform);
#if QT_VERSION >= QT_VERSION_CHECK(7,0,0) || defined(Q_CLANG_QDOC)
    bool bind(QHostAddress::SpecialAddress addr, quint16 port = 0, BindMode mode = DefaultForPlatform)
    { return bind(QHostAddress(addr), port, mode); }
    bool bind(quint16 port = 0, BindMode mode = DefaultForPlatform)
    { return bind(QHostAddress::Any, port, mode); }
#else
    bool bind(quint16 port = 0, BindMode mode = DefaultForPlatform);
#endif

    virtual void connectToHost(const QString &hostName, quint16 port, OpenMode mode = ReadWrite, NetworkLayerProtocol protocol = AnyIPProtocol);
    void connectToHost(const QHostAddress &address, quint16 port, OpenMode mode = ReadWrite);
    virtual void disconnectFromHost();

    bool isValid() const;

    qint64 bytesAvailable() const override;
    qint64 bytesToWrite() const override;

    quint16 localPort() const;
    QHostAddress localAddress() const;
    quint16 peerPort() const;
    QHostAddress peerAddress() const;
    QString peerName() const;

    qint64 readBufferSize() const;
    virtual void setReadBufferSize(qint64 size);

    void abort();

    virtual qintptr socketDescriptor() const;
    virtual bool setSocketDescriptor(qintptr socketDescriptor, SocketState state = ConnectedState,
                             OpenMode openMode = ReadWrite);

    virtual void setSocketOption(QAbstractSocket::SocketOption option, const QVariant &value);
    virtual QVariant socketOption(QAbstractSocket::SocketOption option);

    SocketType socketType() const;
    SocketState state() const;
    SocketError error() const;

    // from QIODevice
    void close() override;
    bool isSequential() const override;
    bool flush();

    // for synchronous access
    virtual bool waitForConnected(int msecs = 30000);
    bool waitForReadyRead(int msecs = 30000) override;
    bool waitForBytesWritten(int msecs = 30000) override;
    virtual bool waitForDisconnected(int msecs = 30000);

#ifndef QT_NO_NETWORKPROXY
    void setProxy(const QNetworkProxy &networkProxy);
    QNetworkProxy proxy() const;
    QString protocolTag() const;
    void setProtocolTag(const QString &tag);
#endif

Q_SIGNALS:
    void hostFound();
    void connected();
    void disconnected();
    void stateChanged(QAbstractSocket::SocketState);
    void errorOccurred(QAbstractSocket::SocketError);
#ifndef QT_NO_NETWORKPROXY
    void proxyAuthenticationRequired(const QNetworkProxy &proxy, QAuthenticator *authenticator);
#endif

protected:
    qint64 readData(char *data, qint64 maxlen) override;
    qint64 readLineData(char *data, qint64 maxlen) override;
    qint64 skipData(qint64 maxSize) override;
    qint64 writeData(const char *data, qint64 len) override;

    void setSocketState(SocketState state);
    void setSocketError(SocketError socketError);
    void setLocalPort(quint16 port);
    void setLocalAddress(const QHostAddress &address);
    void setPeerPort(quint16 port);
    void setPeerAddress(const QHostAddress &address);
    void setPeerName(const QString &name);

    QAbstractSocket(SocketType socketType, QAbstractSocketPrivate &dd, QObject *parent = nullptr);

private:
    Q_DECLARE_PRIVATE(QAbstractSocket)
    Q_DISABLE_COPY_MOVE(QAbstractSocket)

    Q_PRIVATE_SLOT(d_func(), void _q_connectToNextAddress())
    Q_PRIVATE_SLOT(d_func(), void _q_startConnecting(const QHostInfo &))
    Q_PRIVATE_SLOT(d_func(), void _q_abortConnectionAttempt())
    Q_PRIVATE_SLOT(d_func(), void _q_testConnection())
};


Q_DECLARE_OPERATORS_FOR_FLAGS(QAbstractSocket::BindMode)
Q_DECLARE_OPERATORS_FOR_FLAGS(QAbstractSocket::PauseModes)

#ifndef QT_NO_DEBUG_STREAM
Q_NETWORK_EXPORT QDebug operator<<(QDebug, QAbstractSocket::SocketError);
Q_NETWORK_EXPORT QDebug operator<<(QDebug, QAbstractSocket::SocketState);
#endif

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN_TAGGED(QAbstractSocket::SocketState,
                               QAbstractSocket__SocketState, Q_NETWORK_EXPORT)
QT_DECL_METATYPE_EXTERN_TAGGED(QAbstractSocket::SocketError,
                               QAbstractSocket__SocketError, Q_NETWORK_EXPORT)

#endif // QABSTRACTSOCKET_H
