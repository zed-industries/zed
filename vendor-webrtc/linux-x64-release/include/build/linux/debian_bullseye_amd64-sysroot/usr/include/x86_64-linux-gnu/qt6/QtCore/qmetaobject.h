// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2014 Olivier Goffart <ogoffart@woboq.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QMETAOBJECT_H
#define QMETAOBJECT_H

#include <QtCore/qobjectdefs.h>
#include <QtCore/qvariant.h>

QT_BEGIN_NAMESPACE

class QUntypedBindable;

#define Q_METAMETHOD_INVOKE_MAX_ARGS 10

class Q_CORE_EXPORT QMetaMethod
{
public:
    constexpr inline QMetaMethod() : mobj(nullptr), data({ nullptr }) {}

    QByteArray methodSignature() const;
    QByteArray name() const;
    const char *typeName() const;
    int returnType() const;
    QMetaType returnMetaType() const;
    int parameterCount() const;
    int parameterType(int index) const;
    QMetaType parameterMetaType(int index) const;
    void getParameterTypes(int *types) const;
    QList<QByteArray> parameterTypes() const;
    QByteArray parameterTypeName(int index) const;
    QList<QByteArray> parameterNames() const;
    const char *tag() const;
    enum Access { Private, Protected, Public };
    Access access() const;
    enum MethodType { Method, Signal, Slot, Constructor };
    MethodType methodType() const;
    enum Attributes { Compatibility = 0x1, Cloned = 0x2, Scriptable = 0x4 };
    int attributes() const;
    int methodIndex() const;
    int relativeMethodIndex() const;
    int revision() const;
    bool isConst() const;

    inline const QMetaObject *enclosingMetaObject() const { return mobj; }

    bool invoke(QObject *object,
                Qt::ConnectionType connectionType,
                QGenericReturnArgument returnValue,
                QGenericArgument val0 = QGenericArgument(nullptr),
                QGenericArgument val1 = QGenericArgument(),
                QGenericArgument val2 = QGenericArgument(),
                QGenericArgument val3 = QGenericArgument(),
                QGenericArgument val4 = QGenericArgument(),
                QGenericArgument val5 = QGenericArgument(),
                QGenericArgument val6 = QGenericArgument(),
                QGenericArgument val7 = QGenericArgument(),
                QGenericArgument val8 = QGenericArgument(),
                QGenericArgument val9 = QGenericArgument()) const;
    inline bool invoke(QObject *object,
                       QGenericReturnArgument returnValue,
                       QGenericArgument val0 = QGenericArgument(nullptr),
                       QGenericArgument val1 = QGenericArgument(),
                       QGenericArgument val2 = QGenericArgument(),
                       QGenericArgument val3 = QGenericArgument(),
                       QGenericArgument val4 = QGenericArgument(),
                       QGenericArgument val5 = QGenericArgument(),
                       QGenericArgument val6 = QGenericArgument(),
                       QGenericArgument val7 = QGenericArgument(),
                       QGenericArgument val8 = QGenericArgument(),
                       QGenericArgument val9 = QGenericArgument()) const
    {
        return invoke(object, Qt::AutoConnection, returnValue,
                      val0, val1, val2, val3, val4, val5, val6, val7, val8, val9);
    }
    inline bool invoke(QObject *object,
                       Qt::ConnectionType connectionType,
                       QGenericArgument val0 = QGenericArgument(nullptr),
                       QGenericArgument val1 = QGenericArgument(),
                       QGenericArgument val2 = QGenericArgument(),
                       QGenericArgument val3 = QGenericArgument(),
                       QGenericArgument val4 = QGenericArgument(),
                       QGenericArgument val5 = QGenericArgument(),
                       QGenericArgument val6 = QGenericArgument(),
                       QGenericArgument val7 = QGenericArgument(),
                       QGenericArgument val8 = QGenericArgument(),
                       QGenericArgument val9 = QGenericArgument()) const
    {
        return invoke(object, connectionType, QGenericReturnArgument(),
                      val0, val1, val2, val3, val4, val5, val6, val7, val8, val9);
    }
    inline bool invoke(QObject *object,
                       QGenericArgument val0 = QGenericArgument(nullptr),
                       QGenericArgument val1 = QGenericArgument(),
                       QGenericArgument val2 = QGenericArgument(),
                       QGenericArgument val3 = QGenericArgument(),
                       QGenericArgument val4 = QGenericArgument(),
                       QGenericArgument val5 = QGenericArgument(),
                       QGenericArgument val6 = QGenericArgument(),
                       QGenericArgument val7 = QGenericArgument(),
                       QGenericArgument val8 = QGenericArgument(),
                       QGenericArgument val9 = QGenericArgument()) const
    {
        return invoke(object, Qt::AutoConnection, QGenericReturnArgument(),
                      val0, val1, val2, val3, val4, val5, val6, val7, val8, val9);
    }
    bool invokeOnGadget(void *gadget,
                        QGenericReturnArgument returnValue,
                        QGenericArgument val0 = QGenericArgument(nullptr),
                        QGenericArgument val1 = QGenericArgument(),
                        QGenericArgument val2 = QGenericArgument(),
                        QGenericArgument val3 = QGenericArgument(),
                        QGenericArgument val4 = QGenericArgument(),
                        QGenericArgument val5 = QGenericArgument(),
                        QGenericArgument val6 = QGenericArgument(),
                        QGenericArgument val7 = QGenericArgument(),
                        QGenericArgument val8 = QGenericArgument(),
                        QGenericArgument val9 = QGenericArgument()) const;
    inline bool invokeOnGadget(void *gadget,
                               QGenericArgument val0 = QGenericArgument(nullptr),
                               QGenericArgument val1 = QGenericArgument(),
                               QGenericArgument val2 = QGenericArgument(),
                               QGenericArgument val3 = QGenericArgument(),
                               QGenericArgument val4 = QGenericArgument(),
                               QGenericArgument val5 = QGenericArgument(),
                               QGenericArgument val6 = QGenericArgument(),
                               QGenericArgument val7 = QGenericArgument(),
                               QGenericArgument val8 = QGenericArgument(),
                               QGenericArgument val9 = QGenericArgument()) const
    {
        return invokeOnGadget(gadget, QGenericReturnArgument(),
                              val0, val1, val2, val3, val4, val5, val6, val7, val8, val9);
    }

    inline bool isValid() const { return mobj != nullptr; }

    template <typename PointerToMemberFunction>
    static inline QMetaMethod fromSignal(PointerToMemberFunction signal)
    {
        typedef QtPrivate::FunctionPointer<PointerToMemberFunction> SignalType;
        static_assert(QtPrivate::HasQ_OBJECT_Macro<typename SignalType::Object>::Value,
                      "No Q_OBJECT in the class with the signal");
        return fromSignalImpl(&SignalType::Object::staticMetaObject,
                              reinterpret_cast<void **>(&signal));
    }

private:
    static QMetaMethod fromSignalImpl(const QMetaObject *, void **);
    static QMetaMethod fromRelativeMethodIndex(const QMetaObject *mobj, int index);
    static QMetaMethod fromRelativeConstructorIndex(const QMetaObject *mobj, int index);

    struct Data {
        enum { Size = 6 };

        uint name() const { return d[0]; }
        uint argc() const { return d[1]; }
        uint parameters() const { return d[2]; }
        uint tag() const { return d[3]; }
        uint flags() const { return d[4]; }
        uint metaTypeOffset() const { return d[5]; }
        bool operator==(const Data &other) const { return d == other.d; }

        const uint *d;
    };
    constexpr QMetaMethod(const QMetaObject *metaObject, const Data &data_)
        : mobj(metaObject), data(data_)
    {}

    const QMetaObject *mobj;
    Data data;
    friend class QMetaMethodPrivate;
    friend struct QMetaObject;
    friend struct QMetaObjectPrivate;
    friend class QObject;
    friend bool operator==(const QMetaMethod &m1, const QMetaMethod &m2) noexcept
    { return m1.data == m2.data; }
    friend bool operator!=(const QMetaMethod &m1, const QMetaMethod &m2) noexcept
    { return !(m1 == m2); }
};
Q_DECLARE_TYPEINFO(QMetaMethod, Q_RELOCATABLE_TYPE);

class Q_CORE_EXPORT QMetaEnum
{
public:
    constexpr inline QMetaEnum() : mobj(nullptr), data({ nullptr }) {}

    const char *name() const;
    const char *enumName() const;
    bool isFlag() const;
    bool isScoped() const;

    int keyCount() const;
    const char *key(int index) const;
    int value(int index) const;

    const char *scope() const;

    int keyToValue(const char *key, bool *ok = nullptr) const;
    const char *valueToKey(int value) const;
    int keysToValue(const char *keys, bool *ok = nullptr) const;
    QByteArray valueToKeys(int value) const;

    inline const QMetaObject *enclosingMetaObject() const { return mobj; }

    inline bool isValid() const { return name() != nullptr; }

    template<typename T>
    static QMetaEnum fromType()
    {
        static_assert(QtPrivate::IsQEnumHelper<T>::Value,
                      "QMetaEnum::fromType only works with enums declared as "
                      "Q_ENUM, Q_ENUM_NS, Q_FLAG or Q_FLAG_NS");
        const QMetaObject *metaObject = qt_getEnumMetaObject(T());
        const char *name = qt_getEnumName(T());
        return metaObject->enumerator(metaObject->indexOfEnumerator(name));
    }

private:
    struct Data {
        enum { Size = 5 };
        quint32 name() const { return d[0]; }
        quint32 alias() const { return d[1]; }
        quint32 flags() const { return d[2]; }
        qint32 keyCount() const { return static_cast<qint32>(d[3]); }
        quint32 data() const { return d[4]; }

        const uint *d;
    };

    QMetaEnum(const QMetaObject *mobj, int index);

    const QMetaObject *mobj;
    Data data;
    friend struct QMetaObject;
    friend struct QMetaObjectPrivate;
};
Q_DECLARE_TYPEINFO(QMetaEnum, Q_RELOCATABLE_TYPE);

class Q_CORE_EXPORT QMetaProperty
{
public:
    constexpr QMetaProperty() : mobj(nullptr), data({ nullptr }) {}

    const char *name() const;
    const char *typeName() const;
#if QT_DEPRECATED_SINCE(6, 0)
    QT_WARNING_PUSH
    QT_WARNING_DISABLE_DEPRECATED
    QT_DEPRECATED_VERSION_6_0
    QVariant::Type type() const
    { int t = userType(); return t >= QMetaType::User ? QVariant::UserType : QVariant::Type(t); }
    QT_WARNING_POP
#endif
    int userType() const { return typeId(); }
    int typeId() const { return metaType().id(); }
    QMetaType metaType() const;
    int propertyIndex() const;
    int relativePropertyIndex() const;

    bool isReadable() const;
    bool isWritable() const;
    bool isResettable() const;
    bool isDesignable() const;
    bool isScriptable() const;
    bool isStored() const;
    bool isUser() const;
    bool isConstant() const;
    bool isFinal() const;
    bool isRequired() const;
    bool isBindable() const;

    bool isFlagType() const;
    bool isEnumType() const;
    QMetaEnum enumerator() const;

    bool hasNotifySignal() const;
    QMetaMethod notifySignal() const;
    int notifySignalIndex() const;

    int revision() const;

    QVariant read(const QObject *obj) const;
    bool write(QObject *obj, const QVariant &value) const;
    bool reset(QObject *obj) const;

    QUntypedBindable bindable(QObject *object) const;

    QVariant readOnGadget(const void *gadget) const;
    bool writeOnGadget(void *gadget, const QVariant &value) const;
    bool resetOnGadget(void *gadget) const;

    bool hasStdCppSet() const;
    bool isAlias() const;
    inline bool isValid() const { return isReadable(); }
    inline const QMetaObject *enclosingMetaObject() const { return mobj; }

private:
#if QT_DEPRECATED_SINCE(6, 4)
    QT_DEPRECATED_VERSION_X_6_4("obsolete, simply returns typeId()")
    int registerPropertyType() const;
#endif

    struct Data {
        enum { Size = 5 };

        uint name() const { return d[0]; }
        uint type() const { return d[1]; }
        uint flags() const { return d[2]; }
        uint notifyIndex() const { return d[3]; }
        uint revision() const { return d[4]; }

        int index(const QMetaObject *mobj) const;

        const uint *d;
    };

    QMetaProperty(const QMetaObject *mobj, int index);
    static Data getMetaPropertyData(const QMetaObject *mobj, int index);

    const QMetaObject *mobj;
    Data data;
    QMetaEnum menum;
    friend struct QMetaObject;
    friend struct QMetaObjectPrivate;
};

class Q_CORE_EXPORT QMetaClassInfo
{
public:
    constexpr inline QMetaClassInfo() : mobj(nullptr), data({ nullptr }) {}
    const char *name() const;
    const char *value() const;
    inline const QMetaObject *enclosingMetaObject() const { return mobj; }

private:
    struct Data {
        enum { Size = 2 };

        uint name() const { return d[0]; }
        uint value() const { return d[1]; }

        const uint *d;
    };

    const QMetaObject *mobj;
    Data data;
    friend struct QMetaObject;
};
Q_DECLARE_TYPEINFO(QMetaClassInfo, Q_RELOCATABLE_TYPE);

QT_END_NAMESPACE

#endif // QMETAOBJECT_H
