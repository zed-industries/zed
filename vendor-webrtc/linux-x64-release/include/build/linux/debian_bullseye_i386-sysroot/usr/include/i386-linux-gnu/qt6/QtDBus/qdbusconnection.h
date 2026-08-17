// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2016 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSCONNECTION_H
#define QDBUSCONNECTION_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qobjectdefs.h>
#include <QtCore/qstring.h>

#ifndef QT_NO_DBUS

#ifdef interface
#  undef interface
#endif

QT_BEGIN_NAMESPACE


namespace QDBus
{
    enum CallMode {
        NoBlock,
        Block,
        BlockWithGui,
        AutoDetect
    };
}

class QDBusAbstractInterfacePrivate;
class QDBusInterface;
class QDBusError;
class QDBusMessage;
class QDBusPendingCall;
class QDBusConnectionInterface;
class QDBusVirtualObject;
class QObject;

class QDBusConnectionPrivate;
class Q_DBUS_EXPORT QDBusConnection
{
    Q_GADGET
    Q_MOC_INCLUDE(<QtDBus/qdbuspendingcall.h>)

public:
    enum BusType { SessionBus, SystemBus, ActivationBus };
    Q_ENUM(BusType)
    enum RegisterOption {
        ExportAdaptors = 0x01,

        ExportScriptableSlots = 0x10,
        ExportScriptableSignals = 0x20,
        ExportScriptableProperties = 0x40,
        ExportScriptableInvokables = 0x80,
        ExportScriptableContents = 0xf0,

        ExportNonScriptableSlots = 0x100,
        ExportNonScriptableSignals = 0x200,
        ExportNonScriptableProperties = 0x400,
        ExportNonScriptableInvokables = 0x800,
        ExportNonScriptableContents = 0xf00,

        ExportAllSlots = ExportScriptableSlots|ExportNonScriptableSlots,
        ExportAllSignals = ExportScriptableSignals|ExportNonScriptableSignals,
        ExportAllProperties = ExportScriptableProperties|ExportNonScriptableProperties,
        ExportAllInvokables = ExportScriptableInvokables|ExportNonScriptableInvokables,
        ExportAllContents = ExportScriptableContents|ExportNonScriptableContents,

#ifndef Q_QDOC
        // Qt 4.2 had a misspelling here
        ExportAllSignal = ExportAllSignals,
#endif
        ExportChildObjects = 0x1000
        // Reserved = 0xff000000
    };
    Q_DECLARE_FLAGS(RegisterOptions, RegisterOption)
    Q_FLAG(RegisterOptions)

    enum UnregisterMode {
        UnregisterNode,
        UnregisterTree
    };
    Q_ENUM(UnregisterMode)

    enum VirtualObjectRegisterOption {
        SingleNode = 0x0,
        SubPath = 0x1
        // Reserved = 0xff000000
    };
    Q_DECLARE_FLAGS(VirtualObjectRegisterOptions, VirtualObjectRegisterOption)

    enum ConnectionCapability {
        UnixFileDescriptorPassing = 0x0001
    };
    Q_DECLARE_FLAGS(ConnectionCapabilities, ConnectionCapability)

    explicit QDBusConnection(const QString &name);
    QDBusConnection(const QDBusConnection &other);
    QDBusConnection(QDBusConnection &&other) noexcept : d(other.d) { other.d = nullptr; }
    QDBusConnection &operator=(QDBusConnection &&other) noexcept { swap(other); return *this; }
    QDBusConnection &operator=(const QDBusConnection &other);
    ~QDBusConnection();

    void swap(QDBusConnection &other) noexcept { qt_ptr_swap(d, other.d); }

    bool isConnected() const;
    QString baseService() const;
    QDBusError lastError() const;
    QString name() const;
    ConnectionCapabilities connectionCapabilities() const;

    bool send(const QDBusMessage &message) const;
    bool callWithCallback(const QDBusMessage &message, QObject *receiver,
                          const char *returnMethod, const char *errorMethod,
                          int timeout = -1) const;
    bool callWithCallback(const QDBusMessage &message, QObject *receiver,
                          const char *slot, int timeout = -1) const;
    QDBusMessage call(const QDBusMessage &message, QDBus::CallMode mode = QDBus::Block,
                      int timeout = -1) const;
    QDBusPendingCall asyncCall(const QDBusMessage &message, int timeout = -1) const;

    bool connect(const QString &service, const QString &path, const QString &interface,
                 const QString &name, QObject *receiver, const char *slot);
    bool connect(const QString &service, const QString &path, const QString &interface,
                 const QString &name, const QString& signature,
                 QObject *receiver, const char *slot);
    bool connect(const QString &service, const QString &path, const QString &interface,
                 const QString &name, const QStringList &argumentMatch, const QString& signature,
                 QObject *receiver, const char *slot);

    bool disconnect(const QString &service, const QString &path, const QString &interface,
                    const QString &name, QObject *receiver, const char *slot);
    bool disconnect(const QString &service, const QString &path, const QString &interface,
                    const QString &name, const QString& signature,
                    QObject *receiver, const char *slot);
    bool disconnect(const QString &service, const QString &path, const QString &interface,
                    const QString &name, const QStringList &argumentMatch, const QString& signature,
                    QObject *receiver, const char *slot);

    bool registerObject(const QString &path, QObject *object,
                        RegisterOptions options = ExportAdaptors);
    bool registerObject(const QString &path, const QString &interface, QObject *object,
                        RegisterOptions options = ExportAdaptors);
    void unregisterObject(const QString &path, UnregisterMode mode = UnregisterNode);
    QObject *objectRegisteredAt(const QString &path) const;

    bool registerVirtualObject(const QString &path, QDBusVirtualObject *object,
                          VirtualObjectRegisterOption options = SingleNode);

    bool registerService(const QString &serviceName);
    bool unregisterService(const QString &serviceName);

    QDBusConnectionInterface *interface() const;

    void *internalPointer() const;

    static QDBusConnection connectToBus(BusType type, const QString &name);
    static QDBusConnection connectToBus(const QString &address, const QString &name);
    static QDBusConnection connectToPeer(const QString &address, const QString &name);
    static void disconnectFromBus(const QString &name);
    static void disconnectFromPeer(const QString &name);

    static QByteArray localMachineId();

    static QDBusConnection sessionBus();
    static QDBusConnection systemBus();

protected:
    explicit QDBusConnection(QDBusConnectionPrivate *dd);

private:
    friend class QDBusConnectionPrivate;
    QDBusConnectionPrivate *d;
};
Q_DECLARE_SHARED(QDBusConnection)

Q_DECLARE_OPERATORS_FOR_FLAGS(QDBusConnection::RegisterOptions)
Q_DECLARE_OPERATORS_FOR_FLAGS(QDBusConnection::VirtualObjectRegisterOptions)
Q_DECLARE_OPERATORS_FOR_FLAGS(QDBusConnection::ConnectionCapabilities)

QT_END_NAMESPACE

#endif // QT_NO_DBUS
#endif
