/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2014 Olivier Goffart <ogoffart@woboq.com>
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

#ifndef QMETAOBJECT_H
#define QMETAOBJECT_H

#include <QtCore/qobjectdefs.h>
#include <QtCore/qvariant.h>

QT_BEGIN_NAMESPACE


template <typename T> class QList;

#define Q_METAMETHOD_INVOKE_MAX_ARGS 10

class Q_CORE_EXPORT QMetaMethod
{
public:
    Q_DECL_CONSTEXPR inline QMetaMethod() : mobj(nullptr), handle(0) {}

    QByteArray methodSignature() const;
    QByteArray name() const;
    const char *typeName() const;
    int returnType() const;
    int parameterCount() const;
    int parameterType(int index) const;
    void getParameterTypes(int *types) const;
    QList<QByteArray> parameterTypes() const;
    QList<QByteArray> parameterNames() const;
    const char *tag() const;
    enum Access { Private, Protected, Public };
    Access access() const;
    enum MethodType { Method, Signal, Slot, Constructor };
    MethodType methodType() const;
    enum Attributes { Compatibility = 0x1, Cloned = 0x2, Scriptable = 0x4 };
    int attributes() const;
    int methodIndex() const;
    int revision() const;

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
        Q_STATIC_ASSERT_X(QtPrivate::HasQ_OBJECT_Macro<typename SignalType::Object>::Value,
                          "No Q_OBJECT in the class with the signal");
        return fromSignalImpl(&SignalType::Object::staticMetaObject,
                              reinterpret_cast<void **>(&signal));
    }

private:
#if QT_DEPRECATED_SINCE(5,0)
    // signature() has been renamed to methodSignature() in Qt 5.
    // Warning, that function returns a QByteArray; check the life time if
    // you convert to char*.
    char *signature(struct renamedInQt5_warning_checkTheLifeTime * = nullptr) = delete;
#endif
    static QMetaMethod fromSignalImpl(const QMetaObject *, void **);

    const QMetaObject *mobj;
    uint handle;
    friend class QMetaMethodPrivate;
    friend struct QMetaObject;
    friend struct QMetaObjectPrivate;
    friend class QObject;
    friend bool operator==(const QMetaMethod &m1, const QMetaMethod &m2);
    friend bool operator!=(const QMetaMethod &m1, const QMetaMethod &m2);
};
Q_DECLARE_TYPEINFO(QMetaMethod, Q_MOVABLE_TYPE);

inline bool operator==(const QMetaMethod &m1, const QMetaMethod &m2)
{ return m1.mobj == m2.mobj && m1.handle == m2.handle; }
inline bool operator!=(const QMetaMethod &m1, const QMetaMethod &m2)
{ return !(m1 == m2); }

class Q_CORE_EXPORT QMetaEnum
{
public:
    Q_DECL_CONSTEXPR inline QMetaEnum() : mobj(nullptr), handle(0) {}

    const char *name() const;
    const char *enumName() const;
    bool isFlag() const;
    bool isScoped() const;

    int keyCount() const;
    const char *key(int index) const;
    int value(int index) const;

    const char *scope() const;

    int keyToValue(const char *key, bool *ok = nullptr) const;
    const char* valueToKey(int value) const;
    int keysToValue(const char * keys, bool *ok = nullptr) const;
    QByteArray valueToKeys(int value) const;

    inline const QMetaObject *enclosingMetaObject() const { return mobj; }

    inline bool isValid() const { return name() != nullptr; }

    template<typename T> static QMetaEnum fromType() {
        Q_STATIC_ASSERT_X(QtPrivate::IsQEnumHelper<T>::Value,
                          "QMetaEnum::fromType only works with enums declared as "
                          "Q_ENUM, Q_ENUM_NS, Q_FLAG or Q_FLAG_NS");
        const QMetaObject *metaObject = qt_getEnumMetaObject(T());
        const char *name = qt_getEnumName(T());
        return metaObject->enumerator(metaObject->indexOfEnumerator(name));
    }

private:
    const QMetaObject *mobj;
    uint handle;
    friend struct QMetaObject;
};
Q_DECLARE_TYPEINFO(QMetaEnum, Q_MOVABLE_TYPE);

class Q_CORE_EXPORT QMetaProperty
{
public:
    QMetaProperty();

    const char *name() const;
    const char *typeName() const;
    QVariant::Type type() const;
    int userType() const;
    int propertyIndex() const;
    int relativePropertyIndex() const;

    bool isReadable() const;
    bool isWritable() const;
    bool isResettable() const;
    bool isDesignable(const QObject *obj = nullptr) const;
    bool isScriptable(const QObject *obj = nullptr) const;
    bool isStored(const QObject *obj = nullptr) const;
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_VERSION_5_15 bool isEditable(const QObject *obj = nullptr) const;
#endif
    bool isUser(const QObject *obj = nullptr) const;
    bool isConstant() const;
    bool isFinal() const;
    bool isRequired() const;

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

    QVariant readOnGadget(const void *gadget) const;
    bool writeOnGadget(void *gadget, const QVariant &value) const;
    bool resetOnGadget(void *gadget) const;

    bool hasStdCppSet() const;
    inline bool isValid() const { return isReadable(); }
    inline const QMetaObject *enclosingMetaObject() const { return mobj; }

private:
    int registerPropertyType() const;

    const QMetaObject *mobj;
    uint handle;
    int idx;
    QMetaEnum menum;
    friend struct QMetaObject;
    friend struct QMetaObjectPrivate;
};

class Q_CORE_EXPORT QMetaClassInfo
{
public:
    Q_DECL_CONSTEXPR inline QMetaClassInfo() : mobj(nullptr), handle(0) {}
    const char *name() const;
    const char *value() const;
    inline const QMetaObject *enclosingMetaObject() const { return mobj; }
private:
    const QMetaObject *mobj;
    uint handle;
    friend struct QMetaObject;
};
Q_DECLARE_TYPEINFO(QMetaClassInfo, Q_MOVABLE_TYPE);

QT_END_NAMESPACE

#endif // QMETAOBJECT_H
