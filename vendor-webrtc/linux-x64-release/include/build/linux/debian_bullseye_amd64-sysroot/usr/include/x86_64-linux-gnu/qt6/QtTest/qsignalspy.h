// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSIGNALSPY_H
#define QSIGNALSPY_H

#include <QtCore/qbytearray.h>
#include <QtCore/qlist.h>
#include <QtCore/qobject.h>
#include <QtCore/qmetaobject.h>
#include <QtTest/qtesteventloop.h>
#include <QtCore/qvariant.h>

QT_BEGIN_NAMESPACE


class QVariant;

class QSignalSpy: public QObject, public QList<QList<QVariant> >
{
public:
    explicit QSignalSpy(const QObject *obj, const char *aSignal)
        : m_waiting(false)
    {
        if (!isObjectValid(obj))
            return;

        if (!aSignal) {
            qWarning("QSignalSpy: Null signal name is not valid");
            return;
        }

        if (((aSignal[0] - '0') & 0x03) != QSIGNAL_CODE) {
            qWarning("QSignalSpy: Not a valid signal, use the SIGNAL macro");
            return;
        }

        const QByteArray ba = QMetaObject::normalizedSignature(aSignal + 1);
        const QMetaObject * const mo = obj->metaObject();
        const int sigIndex = mo->indexOfMethod(ba.constData());
        if (sigIndex < 0) {
            qWarning("QSignalSpy: No such signal: '%s'", ba.constData());
            return;
        }

        if (!connectToSignal(obj, sigIndex))
            return;

        sig = ba;
        initArgs(mo->method(sigIndex), obj);
    }

#ifdef Q_CLANG_QDOC
    template <typename PointerToMemberFunction>
    QSignalSpy(const QObject *object, PointerToMemberFunction signal);
#else
    template <typename Func>
    QSignalSpy(const typename QtPrivate::FunctionPointer<Func>::Object *obj, Func signal0)
        : m_waiting(false)
    {
        if (!isObjectValid(obj))
            return;

        if (!signal0) {
            qWarning("QSignalSpy: Null signal name is not valid");
            return;
        }

        const QMetaObject * const mo = obj->metaObject();
        const QMetaMethod signalMetaMethod = QMetaMethod::fromSignal(signal0);
        const int sigIndex = signalMetaMethod.methodIndex();

        if (!isSignalMetaMethodValid(signalMetaMethod))
            return;

        if (!connectToSignal(obj, sigIndex))
            return;

        sig = signalMetaMethod.methodSignature();
        initArgs(mo->method(sigIndex), obj);
    }
#endif // Q_CLANG_QDOC

    QSignalSpy(const QObject *obj, const QMetaMethod &signal)
        : m_waiting(false)
    {
        if (isObjectValid(obj) && isSignalMetaMethodValid(signal) &&
            connectToSignal(obj, signal.methodIndex())) {
            sig = signal.methodSignature();
            initArgs(signal, obj);
        }
    }

    inline bool isValid() const { return !sig.isEmpty(); }
    inline QByteArray signal() const { return sig; }

    bool wait(int timeout = 5000)
    {
        Q_ASSERT(!m_waiting);
        const int origCount = size();
        m_waiting = true;
        m_loop.enterLoopMSecs(timeout);
        m_waiting = false;
        return size() > origCount;
    }

    int qt_metacall(QMetaObject::Call call, int methodId, void **a) override
    {
        methodId = QObject::qt_metacall(call, methodId, a);
        if (methodId < 0)
            return methodId;

        if (call == QMetaObject::InvokeMetaMethod) {
            if (methodId == 0) {
                appendArgs(a);
            }
            --methodId;
        }
        return methodId;
    }

private:
    bool connectToSignal(const QObject *sender, int sigIndex)
    {
        static const int memberOffset = QObject::staticMetaObject.methodCount();
        const bool connected = QMetaObject::connect(
            sender, sigIndex, this, memberOffset, Qt::DirectConnection, nullptr);

        if (!connected)
            qWarning("QSignalSpy: QMetaObject::connect returned false. Unable to connect.");

        return connected;
    }

    static bool isSignalMetaMethodValid(const QMetaMethod &signal)
    {
        const bool valid = signal.isValid() && signal.methodType() == QMetaMethod::Signal;

        if (!valid)
            qWarning("QSignalSpy: Not a valid signal: '%s'", signal.methodSignature().constData());

        return valid;
    }

    static bool isObjectValid(const QObject *object)
    {
        const bool valid = !!object;

        if (!valid)
            qWarning("QSignalSpy: Cannot spy on a null object");

        return valid;
    }

    void initArgs(const QMetaMethod &member, const QObject *obj)
    {
        args.reserve(member.parameterCount());
        for (int i = 0; i < member.parameterCount(); ++i) {
            QMetaType tp = member.parameterMetaType(i);
            if (!tp.isValid() && obj) {
                void *argv[] = { &tp, &i };
                QMetaObject::metacall(const_cast<QObject*>(obj),
                                      QMetaObject::RegisterMethodArgumentMetaType,
                                      member.methodIndex(), argv);
            }
            if (!tp.isValid()) {
                qWarning("QSignalSpy: Unable to handle parameter '%s' of type '%s' of method '%s',"
                         " use qRegisterMetaType to register it.",
                         member.parameterNames().at(i).constData(),
                         member.parameterTypes().at(i).constData(),
                         member.name().constData());
            }
            args << tp.id();
        }
    }

    void appendArgs(void **a)
    {
        QList<QVariant> list;
        list.reserve(args.size());
        for (int i = 0; i < args.size(); ++i) {
            const QMetaType::Type type = static_cast<QMetaType::Type>(args.at(i));
            if (type == QMetaType::QVariant)
                list << *reinterpret_cast<QVariant *>(a[i + 1]);
            else
                list << QVariant(QMetaType(type), a[i + 1]);
        }
        append(list);

        if (m_waiting)
            m_loop.exitLoop();
    }

    // the full, normalized signal name
    QByteArray sig;
    // holds the QMetaType types for the argument list of the signal
    QList<int> args;

    QTestEventLoop m_loop;
    bool m_waiting;
};

QT_END_NAMESPACE

#endif
