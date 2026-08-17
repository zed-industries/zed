/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2018 Intel Corporation.
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

#ifndef QMETATYPE_H
#define QMETATYPE_H

#include <QtCore/qglobal.h>
#include <QtCore/qatomic.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qvarlengtharray.h>
#ifndef QT_NO_QOBJECT
#include <QtCore/qobjectdefs.h>
#endif
#include <new>

#include <vector>
#include <list>
#include <map>

#ifdef Bool
#error qmetatype.h must be included before any header file that defines Bool
#endif

QT_BEGIN_NAMESPACE

// from qcborcommon.h
enum class QCborSimpleType : quint8;

template <typename T>
struct QMetaTypeId2;

template <typename T>
inline Q_DECL_CONSTEXPR int qMetaTypeId();

// F is a tuple: (QMetaType::TypeName, QMetaType::TypeNameID, RealType)
// ### Qt6: reorder the types to match the C++ integral type ranking
#define QT_FOR_EACH_STATIC_PRIMITIVE_TYPE(F)\
    F(Void, 43, void) \
    F(Bool, 1, bool) \
    F(Int, 2, int) \
    F(UInt, 3, uint) \
    F(LongLong, 4, qlonglong) \
    F(ULongLong, 5, qulonglong) \
    F(Double, 6, double) \
    F(Long, 32, long) \
    F(Short, 33, short) \
    F(Char, 34, char) \
    F(ULong, 35, ulong) \
    F(UShort, 36, ushort) \
    F(UChar, 37, uchar) \
    F(Float, 38, float) \
    F(SChar, 40, signed char) \
    F(Nullptr, 51, std::nullptr_t) \
    F(QCborSimpleType, 52, QCborSimpleType) \

#define QT_FOR_EACH_STATIC_PRIMITIVE_POINTER(F)\
    F(VoidStar, 31, void*) \

#if QT_CONFIG(easingcurve)
#define QT_FOR_EACH_STATIC_EASINGCURVE(F)\
    F(QEasingCurve, 29, QEasingCurve)
#else
#define QT_FOR_EACH_STATIC_EASINGCURVE(F)
#endif

#if QT_CONFIG(itemmodel)
#define QT_FOR_EACH_STATIC_ITEMMODEL_CLASS(F)\
    F(QModelIndex, 42, QModelIndex) \
    F(QPersistentModelIndex, 50, QPersistentModelIndex)
#else
#define QT_FOR_EACH_STATIC_ITEMMODEL_CLASS(F)
#endif

#define QT_FOR_EACH_STATIC_CORE_CLASS(F)\
    F(QChar, 7, QChar) \
    F(QString, 10, QString) \
    F(QStringList, 11, QStringList) \
    F(QByteArray, 12, QByteArray) \
    F(QBitArray, 13, QBitArray) \
    F(QDate, 14, QDate) \
    F(QTime, 15, QTime) \
    F(QDateTime, 16, QDateTime) \
    F(QUrl, 17, QUrl) \
    F(QLocale, 18, QLocale) \
    F(QRect, 19, QRect) \
    F(QRectF, 20, QRectF) \
    F(QSize, 21, QSize) \
    F(QSizeF, 22, QSizeF) \
    F(QLine, 23, QLine) \
    F(QLineF, 24, QLineF) \
    F(QPoint, 25, QPoint) \
    F(QPointF, 26, QPointF) \
    F(QRegExp, 27, QRegExp) \
    QT_FOR_EACH_STATIC_EASINGCURVE(F) \
    F(QUuid, 30, QUuid) \
    F(QVariant, 41, QVariant) \
    F(QRegularExpression, 44, QRegularExpression) \
    F(QJsonValue, 45, QJsonValue) \
    F(QJsonObject, 46, QJsonObject) \
    F(QJsonArray, 47, QJsonArray) \
    F(QJsonDocument, 48, QJsonDocument) \
    F(QCborValue, 53, QCborValue) \
    F(QCborArray, 54, QCborArray) \
    F(QCborMap, 55, QCborMap) \
    QT_FOR_EACH_STATIC_ITEMMODEL_CLASS(F)

#define QT_FOR_EACH_STATIC_CORE_POINTER(F)\
    F(QObjectStar, 39, QObject*)

#define QT_FOR_EACH_STATIC_CORE_TEMPLATE(F)\
    F(QVariantMap, 8, QVariantMap) \
    F(QVariantList, 9, QVariantList) \
    F(QVariantHash, 28, QVariantHash) \
    F(QByteArrayList, 49, QByteArrayList) \

#define QT_FOR_EACH_STATIC_GUI_CLASS(F)\
    F(QFont, 64, QFont) \
    F(QPixmap, 65, QPixmap) \
    F(QBrush, 66, QBrush) \
    F(QColor, 67, QColor) \
    F(QPalette, 68, QPalette) \
    F(QIcon, 69, QIcon) \
    F(QImage, 70, QImage) \
    F(QPolygon, 71, QPolygon) \
    F(QRegion, 72, QRegion) \
    F(QBitmap, 73, QBitmap) \
    F(QCursor, 74, QCursor) \
    F(QKeySequence, 75, QKeySequence) \
    F(QPen, 76, QPen) \
    F(QTextLength, 77, QTextLength) \
    F(QTextFormat, 78, QTextFormat) \
    F(QMatrix, 79, QMatrix) \
    F(QTransform, 80, QTransform) \
    F(QMatrix4x4, 81, QMatrix4x4) \
    F(QVector2D, 82, QVector2D) \
    F(QVector3D, 83, QVector3D) \
    F(QVector4D, 84, QVector4D) \
    F(QQuaternion, 85, QQuaternion) \
    F(QPolygonF, 86, QPolygonF) \
    F(QColorSpace, 87, QColorSpace) \


#define QT_FOR_EACH_STATIC_WIDGETS_CLASS(F)\
    F(QSizePolicy, 121, QSizePolicy) \

// ### FIXME kill that set
#define QT_FOR_EACH_STATIC_HACKS_TYPE(F)\
    F(QMetaTypeId2<qreal>::MetaType, -1, qreal)

// F is a tuple: (QMetaType::TypeName, QMetaType::TypeNameID, AliasingType, "RealType")
#define QT_FOR_EACH_STATIC_ALIAS_TYPE(F)\
    F(ULong, -1, ulong, "unsigned long") \
    F(UInt, -1, uint, "unsigned int") \
    F(UShort, -1, ushort, "unsigned short") \
    F(UChar, -1, uchar, "unsigned char") \
    F(LongLong, -1, qlonglong, "long long") \
    F(ULongLong, -1, qulonglong, "unsigned long long") \
    F(SChar, -1, signed char, "qint8") \
    F(UChar, -1, uchar, "quint8") \
    F(Short, -1, short, "qint16") \
    F(UShort, -1, ushort, "quint16") \
    F(Int, -1, int, "qint32") \
    F(UInt, -1, uint, "quint32") \
    F(LongLong, -1, qlonglong, "qint64") \
    F(ULongLong, -1, qulonglong, "quint64") \
    F(QVariantList, -1, QVariantList, "QList<QVariant>") \
    F(QVariantMap, -1, QVariantMap, "QMap<QString,QVariant>") \
    F(QVariantHash, -1, QVariantHash, "QHash<QString,QVariant>") \
    F(QByteArrayList, -1, QByteArrayList, "QList<QByteArray>") \

#define QT_FOR_EACH_STATIC_TYPE(F)\
    QT_FOR_EACH_STATIC_PRIMITIVE_TYPE(F)\
    QT_FOR_EACH_STATIC_PRIMITIVE_POINTER(F)\
    QT_FOR_EACH_STATIC_CORE_CLASS(F)\
    QT_FOR_EACH_STATIC_CORE_POINTER(F)\
    QT_FOR_EACH_STATIC_CORE_TEMPLATE(F)\
    QT_FOR_EACH_STATIC_GUI_CLASS(F)\
    QT_FOR_EACH_STATIC_WIDGETS_CLASS(F)\

#define QT_DEFINE_METATYPE_ID(TypeName, Id, Name) \
    TypeName = Id,

#define QT_FOR_EACH_AUTOMATIC_TEMPLATE_1ARG(F) \
    F(QList) \
    F(QVector) \
    F(QQueue) \
    F(QStack) \
    F(QSet) \
    /*end*/

#define QT_FOR_EACH_AUTOMATIC_TEMPLATE_2ARG(F) \
    F(QHash, class) \
    F(QMap, class) \
    F(QPair, struct)

#define QT_FOR_EACH_AUTOMATIC_TEMPLATE_SMART_POINTER(F) \
    F(QSharedPointer) \
    F(QWeakPointer) \
    F(QPointer)

class QDataStream;
class QMetaTypeInterface;
struct QMetaObject;

namespace QtPrivate
{
/*!
    This template is used for implicit conversion from type From to type To.
    \internal
*/
template<typename From, typename To>
To convertImplicit(const From& from)
{
    return from;
}

#ifndef QT_NO_DEBUG_STREAM
struct AbstractDebugStreamFunction
{
    typedef void (*Stream)(const AbstractDebugStreamFunction *, QDebug&, const void *);
    typedef void (*Destroy)(AbstractDebugStreamFunction *);
    explicit AbstractDebugStreamFunction(Stream s = nullptr, Destroy d = nullptr)
        : stream(s), destroy(d) {}
    Q_DISABLE_COPY(AbstractDebugStreamFunction)
    Stream stream;
    Destroy destroy;
};

template<typename T>
struct BuiltInDebugStreamFunction : public AbstractDebugStreamFunction
{
    BuiltInDebugStreamFunction()
        : AbstractDebugStreamFunction(stream, destroy) {}
    static void stream(const AbstractDebugStreamFunction *, QDebug& dbg, const void *r)
    {
        const T *rhs = static_cast<const T *>(r);
        operator<<(dbg, *rhs);
    }

    static void destroy(AbstractDebugStreamFunction *_this)
    {
        delete static_cast<BuiltInDebugStreamFunction *>(_this);
    }
};
#endif

struct AbstractComparatorFunction
{
    typedef bool (*LessThan)(const AbstractComparatorFunction *, const void *, const void *);
    typedef bool (*Equals)(const AbstractComparatorFunction *, const void *, const void *);
    typedef void (*Destroy)(AbstractComparatorFunction *);
    explicit AbstractComparatorFunction(LessThan lt = nullptr, Equals e = nullptr, Destroy d = nullptr)
        : lessThan(lt), equals(e), destroy(d) {}
    Q_DISABLE_COPY(AbstractComparatorFunction)
    LessThan lessThan;
    Equals equals;
    Destroy destroy;
};

template<typename T>
struct BuiltInComparatorFunction : public AbstractComparatorFunction
{
    BuiltInComparatorFunction()
        : AbstractComparatorFunction(lessThan, equals, destroy) {}
    static bool lessThan(const AbstractComparatorFunction *, const void *l, const void *r)
    {
        const T *lhs = static_cast<const T *>(l);
        const T *rhs = static_cast<const T *>(r);
        return *lhs < *rhs;
    }

    static bool equals(const AbstractComparatorFunction *, const void *l, const void *r)
    {
        const T *lhs = static_cast<const T *>(l);
        const T *rhs = static_cast<const T *>(r);
        return *lhs == *rhs;
    }

    static void destroy(AbstractComparatorFunction *_this)
    {
        delete static_cast<BuiltInComparatorFunction *>(_this);
    }
};

template<typename T>
struct BuiltInEqualsComparatorFunction : public AbstractComparatorFunction
{
    BuiltInEqualsComparatorFunction()
        : AbstractComparatorFunction(nullptr, equals, destroy) {}
    static bool equals(const AbstractComparatorFunction *, const void *l, const void *r)
    {
        const T *lhs = static_cast<const T *>(l);
        const T *rhs = static_cast<const T *>(r);
        return *lhs == *rhs;
    }

    static void destroy(AbstractComparatorFunction *_this)
    {
        delete static_cast<BuiltInEqualsComparatorFunction *>(_this);
    }
};

struct AbstractConverterFunction
{
    typedef bool (*Converter)(const AbstractConverterFunction *, const void *, void*);
    explicit AbstractConverterFunction(Converter c = nullptr)
        : convert(c) {}
    Q_DISABLE_COPY(AbstractConverterFunction)
    Converter convert;
};

template<typename From, typename To>
struct ConverterMemberFunction : public AbstractConverterFunction
{
    explicit ConverterMemberFunction(To(From::*function)() const)
        : AbstractConverterFunction(convert),
          m_function(function) {}
    ~ConverterMemberFunction();
    static bool convert(const AbstractConverterFunction *_this, const void *in, void *out)
    {
        const From *f = static_cast<const From *>(in);
        To *t = static_cast<To *>(out);
        const ConverterMemberFunction *_typedThis =
            static_cast<const ConverterMemberFunction *>(_this);
        *t = (f->*_typedThis->m_function)();
        return true;
    }

    To(From::* const m_function)() const;
};

template<typename From, typename To>
struct ConverterMemberFunctionOk : public AbstractConverterFunction
{
    explicit ConverterMemberFunctionOk(To(From::*function)(bool *) const)
        : AbstractConverterFunction(convert),
          m_function(function) {}
    ~ConverterMemberFunctionOk();
    static bool convert(const AbstractConverterFunction *_this, const void *in, void *out)
    {
        const From *f = static_cast<const From *>(in);
        To *t = static_cast<To *>(out);
        bool ok = false;
        const ConverterMemberFunctionOk *_typedThis =
            static_cast<const ConverterMemberFunctionOk *>(_this);
        *t = (f->*_typedThis->m_function)(&ok);
        if (!ok)
            *t = To();
        return ok;
    }

    To(From::* const m_function)(bool*) const;
};

template<typename From, typename To, typename UnaryFunction>
struct ConverterFunctor : public AbstractConverterFunction
{
    explicit ConverterFunctor(UnaryFunction function)
        : AbstractConverterFunction(convert),
          m_function(function) {}
    ~ConverterFunctor();
    static bool convert(const AbstractConverterFunction *_this, const void *in, void *out)
    {
        const From *f = static_cast<const From *>(in);
        To *t = static_cast<To *>(out);
        const ConverterFunctor *_typedThis =
            static_cast<const ConverterFunctor *>(_this);
        *t = _typedThis->m_function(*f);
        return true;
    }

    UnaryFunction m_function;
};

    template<typename T, bool>
    struct ValueTypeIsMetaType;
    template<typename T, bool>
    struct AssociativeValueTypeIsMetaType;
    template<typename T, bool>
    struct IsMetaTypePair;
    template<typename, typename>
    struct MetaTypeSmartPointerHelper;
}

class Q_CORE_EXPORT QMetaType {
    enum ExtensionFlag { NoExtensionFlags,
                         CreateEx = 0x1, DestroyEx = 0x2,
                         ConstructEx = 0x4, DestructEx = 0x8,
                         NameEx = 0x10, SizeEx = 0x20,
                         CtorEx = 0x40, DtorEx = 0x80,
                         FlagsEx = 0x100, MetaObjectEx = 0x200
                       };
public:
#ifndef Q_CLANG_QDOC
    // The code that actually gets compiled.
    enum Type {
        // these are merged with QVariant
        QT_FOR_EACH_STATIC_TYPE(QT_DEFINE_METATYPE_ID)

        FirstCoreType = Bool,
        LastCoreType = QCborMap,
        FirstGuiType = QFont,
        LastGuiType = QColorSpace,
        FirstWidgetsType = QSizePolicy,
        LastWidgetsType = QSizePolicy,
        HighestInternalId = LastWidgetsType,

        QReal = sizeof(qreal) == sizeof(double) ? Double : Float,

        UnknownType = 0,
        User = 1024
    };
#else
    // If we are using QDoc it fakes the Type enum looks like this.
    enum Type {
        UnknownType = 0, Bool = 1, Int = 2, UInt = 3, LongLong = 4, ULongLong = 5,
        Double = 6, Long = 32, Short = 33, Char = 34, ULong = 35, UShort = 36,
        UChar = 37, Float = 38,
        VoidStar = 31,
        QChar = 7, QString = 10, QStringList = 11, QByteArray = 12,
        QBitArray = 13, QDate = 14, QTime = 15, QDateTime = 16, QUrl = 17,
        QLocale = 18, QRect = 19, QRectF = 20, QSize = 21, QSizeF = 22,
        QLine = 23, QLineF = 24, QPoint = 25, QPointF = 26, QRegExp = 27,
        QEasingCurve = 29, QUuid = 30, QVariant = 41, QModelIndex = 42,
        QPersistentModelIndex = 50, QRegularExpression = 44,
        QJsonValue = 45, QJsonObject = 46, QJsonArray = 47, QJsonDocument = 48,
        QByteArrayList = 49, QObjectStar = 39, SChar = 40,
        Void = 43,
        Nullptr = 51,
        QVariantMap = 8, QVariantList = 9, QVariantHash = 28,
        QCborSimpleType = 52, QCborValue = 53, QCborArray = 54, QCborMap = 55,

        // Gui types
        QFont = 64, QPixmap = 65, QBrush = 66, QColor = 67, QPalette = 68,
        QIcon = 69, QImage = 70, QPolygon = 71, QRegion = 72, QBitmap = 73,
        QCursor = 74, QKeySequence = 75, QPen = 76, QTextLength = 77, QTextFormat = 78,
        QMatrix = 79, QTransform = 80, QMatrix4x4 = 81, QVector2D = 82,
        QVector3D = 83, QVector4D = 84, QQuaternion = 85, QPolygonF = 86, QColorSpace = 87,

        // Widget types
        QSizePolicy = 121,
        LastCoreType = QCborMap,
        LastGuiType = QColorSpace,
        User = 1024
    };
#endif

    enum TypeFlag {
        NeedsConstruction = 0x1,
        NeedsDestruction = 0x2,
        MovableType = 0x4,
        PointerToQObject = 0x8,
        IsEnumeration = 0x10,
        SharedPointerToQObject = 0x20,
        WeakPointerToQObject = 0x40,
        TrackingPointerToQObject = 0x80,
        WasDeclaredAsMetaType = 0x100,
        IsGadget = 0x200,
        PointerToGadget = 0x400
    };
    Q_DECLARE_FLAGS(TypeFlags, TypeFlag)

    typedef void (*Deleter)(void *);
    typedef void *(*Creator)(const void *);

#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    typedef void (*Destructor)(void *);
    typedef void *(*Constructor)(void *, const void *); // TODO Qt6: remove me
#endif
    typedef void (*TypedDestructor)(int, void *);
    typedef void *(*TypedConstructor)(int, void *, const void *);

    typedef void (*SaveOperator)(QDataStream &, const void *);
    typedef void (*LoadOperator)(QDataStream &, void *);
#ifndef QT_NO_DATASTREAM
    static void registerStreamOperators(const char *typeName, SaveOperator saveOp,
                                        LoadOperator loadOp);
    static void registerStreamOperators(int type, SaveOperator saveOp,
                                        LoadOperator loadOp);
#endif
    static int registerType(const char *typeName, Deleter deleter,
                            Creator creator);
    static int registerType(const char *typeName, Deleter deleter,
                            Creator creator,
                            Destructor destructor,
                            Constructor constructor,
                            int size,
                            QMetaType::TypeFlags flags,
                            const QMetaObject *metaObject);
    static int registerType(const char *typeName,
                            TypedDestructor destructor,
                            TypedConstructor constructor,
                            int size,
                            QMetaType::TypeFlags flags,
                            const QMetaObject *metaObject);
    static bool unregisterType(int type);
    static int registerNormalizedType(const QT_PREPEND_NAMESPACE(QByteArray) &normalizedTypeName, Deleter deleter,
                            Creator creator,
                            Destructor destructor,
                            Constructor constructor,
                            int size,
                            QMetaType::TypeFlags flags,
                            const QMetaObject *metaObject);
    static int registerNormalizedType(const QT_PREPEND_NAMESPACE(QByteArray) &normalizedTypeName, Destructor destructor,
                            Constructor constructor,
                            int size,
                            QMetaType::TypeFlags flags,
                            const QMetaObject *metaObject);
    static int registerNormalizedType(const QT_PREPEND_NAMESPACE(QByteArray) &normalizedTypeName, TypedDestructor destructor,
                            TypedConstructor constructor,
                            int size,
                            QMetaType::TypeFlags flags,
                            const QMetaObject *metaObject);
    static int registerTypedef(const char *typeName, int aliasId);
    static int registerNormalizedTypedef(const QT_PREPEND_NAMESPACE(QByteArray) &normalizedTypeName, int aliasId);
    static int type(const char *typeName);

    static int type(const QT_PREPEND_NAMESPACE(QByteArray) &typeName);
    static const char *typeName(int type);
    static int sizeOf(int type);
    static TypeFlags typeFlags(int type);
    static const QMetaObject *metaObjectForType(int type);
    static bool isRegistered(int type);
    static void *create(int type, const void *copy = nullptr);
#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED static void *construct(int type, const void *copy = nullptr)
    { return create(type, copy); }
#endif
    static void destroy(int type, void *data);
    static void *construct(int type, void *where, const void *copy);
    static void destruct(int type, void *where);

#ifndef QT_NO_DATASTREAM
    static bool save(QDataStream &stream, int type, const void *data);
    static bool load(QDataStream &stream, int type, void *data);
#endif

    explicit QMetaType(const int type = QMetaType::UnknownType); // ### Qt6: drop const
    inline ~QMetaType();

    inline bool isValid() const;
    inline bool isRegistered() const;
    inline int id() const;
    inline int sizeOf() const;
    inline TypeFlags flags() const;
    inline const QMetaObject *metaObject() const;
    QT_PREPEND_NAMESPACE(QByteArray) name() const;

    inline void *create(const void *copy = nullptr) const;
    inline void destroy(void *data) const;
    inline void *construct(void *where, const void *copy = nullptr) const;
    inline void destruct(void *data) const;

    template<typename T>
    static QMetaType fromType()
    { return QMetaType(qMetaTypeId<T>()); }

    friend bool operator==(const QMetaType &a, const QMetaType &b)
    { return a.m_typeId == b.m_typeId; }

    friend bool operator!=(const QMetaType &a, const QMetaType &b)
    { return a.m_typeId != b.m_typeId; }


public:
    template<typename T>
    static bool registerComparators()
    {
        Q_STATIC_ASSERT_X((!QMetaTypeId2<T>::IsBuiltIn),
            "QMetaType::registerComparators: The type must be a custom type.");

        const int typeId = qMetaTypeId<T>();
        static const QtPrivate::BuiltInComparatorFunction<T> f;
        return registerComparatorFunction( &f, typeId);
    }
    template<typename T>
    static bool registerEqualsComparator()
    {
        Q_STATIC_ASSERT_X((!QMetaTypeId2<T>::IsBuiltIn),
            "QMetaType::registerEqualsComparator: The type must be a custom type.");
        const int typeId = qMetaTypeId<T>();
        static const QtPrivate::BuiltInEqualsComparatorFunction<T> f;
        return registerComparatorFunction( &f, typeId);
    }

    template<typename T>
    static bool hasRegisteredComparators()
    {
        return hasRegisteredComparators(qMetaTypeId<T>());
    }
    static bool hasRegisteredComparators(int typeId);


#ifndef QT_NO_DEBUG_STREAM
    template<typename T>
    static bool registerDebugStreamOperator()
    {
        Q_STATIC_ASSERT_X((!QMetaTypeId2<T>::IsBuiltIn),
            "QMetaType::registerDebugStreamOperator: The type must be a custom type.");

        const int typeId = qMetaTypeId<T>();
        static const QtPrivate::BuiltInDebugStreamFunction<T> f;
        return registerDebugStreamOperatorFunction(&f, typeId);
    }
    template<typename T>
    static bool hasRegisteredDebugStreamOperator()
    {
        return hasRegisteredDebugStreamOperator(qMetaTypeId<T>());
    }
    static bool hasRegisteredDebugStreamOperator(int typeId);
#endif

    // implicit conversion supported like double -> float
    template<typename From, typename To>
    static bool registerConverter()
    {
        return registerConverter<From, To>(QtPrivate::convertImplicit<From, To>);
    }

#ifdef Q_CLANG_QDOC
    template<typename MemberFunction, int>
    static bool registerConverter(MemberFunction function);
    template<typename MemberFunctionOk, char>
    static bool registerConverter(MemberFunctionOk function);
    template<typename UnaryFunction>
    static bool registerConverter(UnaryFunction function);
#else
    // member function as in "QString QFont::toString() const"
    template<typename From, typename To>
    static bool registerConverter(To(From::*function)() const)
    {
        Q_STATIC_ASSERT_X((!QMetaTypeId2<To>::IsBuiltIn || !QMetaTypeId2<From>::IsBuiltIn),
            "QMetaType::registerConverter: At least one of the types must be a custom type.");

        const int fromTypeId = qMetaTypeId<From>();
        const int toTypeId = qMetaTypeId<To>();
        static const QtPrivate::ConverterMemberFunction<From, To> f(function);
        return registerConverterFunction(&f, fromTypeId, toTypeId);
    }

    // member function as in "double QString::toDouble(bool *ok = nullptr) const"
    template<typename From, typename To>
    static bool registerConverter(To(From::*function)(bool*) const)
    {
        Q_STATIC_ASSERT_X((!QMetaTypeId2<To>::IsBuiltIn || !QMetaTypeId2<From>::IsBuiltIn),
            "QMetaType::registerConverter: At least one of the types must be a custom type.");

        const int fromTypeId = qMetaTypeId<From>();
        const int toTypeId = qMetaTypeId<To>();
        static const QtPrivate::ConverterMemberFunctionOk<From, To> f(function);
        return registerConverterFunction(&f, fromTypeId, toTypeId);
    }

    // functor or function pointer
    template<typename From, typename To, typename UnaryFunction>
    static bool registerConverter(UnaryFunction function)
    {
        Q_STATIC_ASSERT_X((!QMetaTypeId2<To>::IsBuiltIn || !QMetaTypeId2<From>::IsBuiltIn),
            "QMetaType::registerConverter: At least one of the types must be a custom type.");

        const int fromTypeId = qMetaTypeId<From>();
        const int toTypeId = qMetaTypeId<To>();
        static const QtPrivate::ConverterFunctor<From, To, UnaryFunction> f(function);
        return registerConverterFunction(&f, fromTypeId, toTypeId);
    }
#endif

    static bool convert(const void *from, int fromTypeId, void *to, int toTypeId);
    static bool compare(const void *lhs, const void *rhs, int typeId, int* result);
    static bool equals(const void *lhs, const void *rhs, int typeId, int* result);
    static bool debugStream(QDebug& dbg, const void *rhs, int typeId);

    template<typename From, typename To>
    static bool hasRegisteredConverterFunction()
    {
        return hasRegisteredConverterFunction(qMetaTypeId<From>(), qMetaTypeId<To>());
    }

    static bool hasRegisteredConverterFunction(int fromTypeId, int toTypeId);

private:
    static QMetaType typeInfo(const int type);
    inline QMetaType(const ExtensionFlag extensionFlags, const QMetaTypeInterface *info,
                     TypedConstructor creator,
                     TypedDestructor deleter,
                     SaveOperator saveOp,
                     LoadOperator loadOp,
                     Constructor constructor,
                     Destructor destructor,
                     uint sizeOf,
                     uint theTypeFlags,
                     int typeId,
                     const QMetaObject *metaObject);
    QMetaType(const QMetaType &other);
    QMetaType &operator =(const QMetaType &);
    inline bool isExtended(const ExtensionFlag flag) const { return m_extensionFlags & flag; }

    // Methods used for future binary compatible extensions
    void ctor(const QMetaTypeInterface *info);
    void dtor();
    uint sizeExtended() const;
    QMetaType::TypeFlags flagsExtended() const;
    const QMetaObject *metaObjectExtended() const;
    void *createExtended(const void *copy = nullptr) const;
    void destroyExtended(void *data) const;
    void *constructExtended(void *where, const void *copy = nullptr) const;
    void destructExtended(void *data) const;

    static bool registerComparatorFunction(const QtPrivate::AbstractComparatorFunction *f, int type);
#ifndef QT_NO_DEBUG_STREAM
    static bool registerDebugStreamOperatorFunction(const QtPrivate::AbstractDebugStreamFunction *f, int type);
#endif

// ### Qt6: FIXME: Remove the special Q_CC_MSVC handling, it was introduced to maintain BC.
#if !defined(Q_NO_TEMPLATE_FRIENDS) && !defined(Q_CC_MSVC)
#ifndef Q_CLANG_QDOC
    template<typename, bool> friend struct QtPrivate::ValueTypeIsMetaType;
    template<typename, typename> friend struct QtPrivate::ConverterMemberFunction;
    template<typename, typename> friend struct QtPrivate::ConverterMemberFunctionOk;
    template<typename, typename, typename> friend struct QtPrivate::ConverterFunctor;
    template<typename, bool> friend struct QtPrivate::AssociativeValueTypeIsMetaType;
    template<typename, bool> friend struct QtPrivate::IsMetaTypePair;
    template<typename, typename> friend struct QtPrivate::MetaTypeSmartPointerHelper;
#endif
#else
public:
#endif
    static bool registerConverterFunction(const QtPrivate::AbstractConverterFunction *f, int from, int to);
    static void unregisterConverterFunction(int from, int to);
private:

    TypedConstructor m_typedConstructor;
    TypedDestructor m_typedDestructor;
    SaveOperator m_saveOp;
    LoadOperator m_loadOp;
    Constructor m_constructor;
    Destructor m_destructor;
    void *m_extension; // space reserved for future use
    uint m_size;
    uint m_typeFlags;
    uint m_extensionFlags;
    int m_typeId;
    const QMetaObject *m_metaObject;
};

#undef QT_DEFINE_METATYPE_ID

Q_DECLARE_OPERATORS_FOR_FLAGS(QMetaType::TypeFlags)

namespace QtPrivate {

template<typename From, typename To>
ConverterMemberFunction<From, To>::~ConverterMemberFunction()
{
    QMetaType::unregisterConverterFunction(qMetaTypeId<From>(), qMetaTypeId<To>());
}
template<typename From, typename To>
ConverterMemberFunctionOk<From, To>::~ConverterMemberFunctionOk()
{
    QMetaType::unregisterConverterFunction(qMetaTypeId<From>(), qMetaTypeId<To>());
}
template<typename From, typename To, typename UnaryFunction>
ConverterFunctor<From, To, UnaryFunction>::~ConverterFunctor()
{
    QMetaType::unregisterConverterFunction(qMetaTypeId<From>(), qMetaTypeId<To>());
}

}

#define QT_METATYPE_PRIVATE_DECLARE_TYPEINFO(C, F)  \
    }                                               \
    Q_DECLARE_TYPEINFO(QtMetaTypePrivate:: C, (F)); \
    namespace QtMetaTypePrivate {

namespace QtMetaTypePrivate {
template <typename T, bool Accepted = true>
struct QMetaTypeFunctionHelper {
    static void Destruct(void *t)
    {
        Q_UNUSED(t) // Silence MSVC that warns for POD types.
        static_cast<T*>(t)->~T();
    }

    static void *Construct(void *where, const void *t)
    {
        if (t)
            return new (where) T(*static_cast<const T*>(t));
        return new (where) T;
    }
#ifndef QT_NO_DATASTREAM
    static void Save(QDataStream &stream, const void *t)
    {
        stream << *static_cast<const T*>(t);
    }

    static void Load(QDataStream &stream, void *t)
    {
        stream >> *static_cast<T*>(t);
    }
#endif // QT_NO_DATASTREAM
};

template <typename T>
struct QMetaTypeFunctionHelper<T, /* Accepted */ false> {
    static void Destruct(void *) {}
    static void *Construct(void *, const void *) { return nullptr; }
#ifndef QT_NO_DATASTREAM
    static void Save(QDataStream &, const void *) {}
    static void Load(QDataStream &, void *) {}
#endif // QT_NO_DATASTREAM
};
template <>
struct QMetaTypeFunctionHelper<void, /* Accepted */ true>
        : public QMetaTypeFunctionHelper<void, /* Accepted */ false>
{};

struct VariantData
{
    VariantData(const int metaTypeId_,
                const void *data_,
                const uint flags_)
      : metaTypeId(metaTypeId_)
      , data(data_)
      , flags(flags_)
    {
    }
    VariantData(const VariantData &other)
        : metaTypeId(other.metaTypeId), data(other.data), flags(other.flags){}
    const int metaTypeId;
    const void *data;
    const uint flags;
private:
    // copy constructor allowed to be implicit to silence level 4 warning from MSVC
    VariantData &operator=(const VariantData &) = delete;
};

template<typename const_iterator>
struct IteratorOwnerCommon
{
    static void assign(void **ptr, const_iterator iterator)
    {
        *ptr = new const_iterator(iterator);
    }
    static void assign(void **ptr, void * const * src)
    {
        *ptr = new const_iterator(*static_cast<const_iterator*>(*src));
    }

    static void advance(void **iterator, int step)
    {
        const_iterator &it = *static_cast<const_iterator*>(*iterator);
        std::advance(it, step);
    }

    static void destroy(void **ptr)
    {
        delete static_cast<const_iterator*>(*ptr);
    }

    static bool equal(void * const *it, void * const *other)
    {
        return *static_cast<const_iterator*>(*it) == *static_cast<const_iterator*>(*other);
    }
};

template<typename const_iterator>
struct IteratorOwner : IteratorOwnerCommon<const_iterator>
{
    static const void *getData(void * const *iterator)
    {
        return &**static_cast<const_iterator*>(*iterator);
    }

    static const void *getData(const_iterator it)
    {
        return &*it;
    }
};

struct Q_CORE_EXPORT VectorBoolElements
{
  static const bool true_element;
  static const bool false_element;
};

template<>
struct IteratorOwner<std::vector<bool>::const_iterator> : IteratorOwnerCommon<std::vector<bool>::const_iterator>
{
public:
    static const void *getData(void * const *iterator)
    {
        return **static_cast<std::vector<bool>::const_iterator*>(*iterator) ?
            &VectorBoolElements::true_element : &VectorBoolElements::false_element;
    }

    static const void *getData(const std::vector<bool>::const_iterator& it)
    {
        return *it ? &VectorBoolElements::true_element : &VectorBoolElements::false_element;
    }
};

template<typename value_type>
struct IteratorOwner<const value_type*>
{
private:
    // We need to disable typed overloads of assign() and getData() if the value_type
    // is void* to avoid overloads conflicts. We do it by injecting unaccessible Dummy
    // type as part of the overload signature.
    struct Dummy {};
    typedef typename std::conditional<std::is_same<value_type, void*>::value, Dummy, value_type>::type value_type_OR_Dummy;
public:
    static void assign(void **ptr, const value_type_OR_Dummy *iterator )
    {
        *ptr = const_cast<value_type*>(iterator);
    }
    static void assign(void **ptr, void * const * src)
    {
        *ptr = static_cast<value_type*>(*src);
    }

    static void advance(void **iterator, int step)
    {
        value_type *it = static_cast<value_type*>(*iterator);
        std::advance(it, step);
        *iterator = it;
    }

    static void destroy(void **)
    {
    }

    static const void *getData(void * const *iterator)
    {
        return *iterator;
    }

    static const void *getData(const value_type_OR_Dummy *it)
    {
        return it;
    }

    static bool equal(void * const *it, void * const *other)
    {
        return static_cast<value_type*>(*it) == static_cast<value_type*>(*other);
    }
};

enum IteratorCapability
{
    ForwardCapability = 1,
    BiDirectionalCapability = 2,
    RandomAccessCapability = 4
};

enum ContainerCapability
{
    ContainerIsAppendable = 1
};

template<typename Container, typename T = void>
struct ContainerCapabilitiesImpl
{
    enum {ContainerCapabilities = 0};
    using appendFunction = void(*)(const void *container, const void *newElement);
    static constexpr const appendFunction appendImpl = nullptr;
};

template<typename Container>
struct ContainerCapabilitiesImpl<Container, decltype(std::declval<Container>().push_back(std::declval<typename Container::value_type>()))>
{
    enum {ContainerCapabilities = ContainerIsAppendable};

    // The code below invokes undefined behavior if and only if the pointer passed into QSequentialIterableImpl
    // pointed to a const object to begin with
    static void appendImpl(const void *container, const void *value)
    { static_cast<Container *>(const_cast<void *>(container))->push_back(*static_cast<const typename Container::value_type *>(value)); }
};

namespace QtPrivate {
namespace ContainerCapabilitiesMetaProgrammingHelper {
    template<typename... Ts>
    using void_t = void;
}
}

template<typename Container>
struct ContainerCapabilitiesImpl<Container, QtPrivate::ContainerCapabilitiesMetaProgrammingHelper::void_t<decltype(std::declval<Container>().insert(std::declval<typename Container::value_type>())), decltype(std::declval<typename Container::value_type>() == std::declval<typename Container::value_type>())>>
{
    enum {ContainerCapabilities = ContainerIsAppendable};

    // The code below invokes undefined behavior if and only if the pointer passed into QSequentialIterableImpl
    // pointed to a const object to begin with
    static void appendImpl(const void *container, const void *value)
    { static_cast<Container *>(const_cast<void *>(container))->insert(*static_cast<const typename Container::value_type *>(value)); }
};

template<typename T, typename Category = typename std::iterator_traits<typename T::const_iterator>::iterator_category>
struct CapabilitiesImpl;

template<typename T>
struct CapabilitiesImpl<T, std::forward_iterator_tag>
{ enum { IteratorCapabilities = ForwardCapability }; };
template<typename T>
struct CapabilitiesImpl<T, std::bidirectional_iterator_tag>
{ enum { IteratorCapabilities = BiDirectionalCapability | ForwardCapability }; };
template<typename T>
struct CapabilitiesImpl<T, std::random_access_iterator_tag>
{ enum { IteratorCapabilities = RandomAccessCapability | BiDirectionalCapability | ForwardCapability }; };

template<typename T>
struct ContainerAPI : CapabilitiesImpl<T>
{
    static int size(const T *t) { return int(std::distance(t->begin(), t->end())); }
};

template<typename T>
struct ContainerAPI<QList<T> > : CapabilitiesImpl<QList<T> >
{ static int size(const QList<T> *t) { return t->size(); } };

template<typename T>
struct ContainerAPI<QVector<T> > : CapabilitiesImpl<QVector<T> >
{ static int size(const QVector<T> *t) { return t->size(); } };

template<typename T>
struct ContainerAPI<std::vector<T> > : CapabilitiesImpl<std::vector<T> >
{ static int size(const std::vector<T> *t) { return int(t->size()); } };

template<typename T>
struct ContainerAPI<std::list<T> > : CapabilitiesImpl<std::list<T> >
{ static int size(const std::list<T> *t) { return int(t->size()); } };

/*
 revision 0: _iteratorCapabilities is simply a uint, where the bits at _revision were never set
 revision 1: _iteratorCapabilties is treated as a bitfield, the remaining bits are used to introduce
             _revision, _containerCapabilities and _unused. The latter contains 21 bits that are
             not used yet
*/
class QSequentialIterableImpl
{
public:
    const void * _iterable;
    void *_iterator;
    int _metaType_id;
    uint _metaType_flags;
    uint _iteratorCapabilities;
    // Iterator capabilities looks actually like
    // uint _iteratorCapabilities:4;
    // uint _revision:3;
    // uint _containerCapabilities:4;
    // uint _unused:21;*/
    typedef int(*sizeFunc)(const void *p);
    typedef const void * (*atFunc)(const void *p, int);
    typedef void (*moveIteratorFunc)(const void *p, void **);
    enum Position { ToBegin, ToEnd };
    typedef void (*moveIteratorFunc2)(const void *p, void **, Position position);
    typedef void (*advanceFunc)(void **p, int);
    typedef VariantData (*getFunc)( void * const *p, int metaTypeId, uint flags);
    typedef void (*destroyIterFunc)(void **p);
    typedef bool (*equalIterFunc)(void * const *p, void * const *other);
    typedef void (*copyIterFunc)(void **, void * const *);
    typedef void(*appendFunction)(const void *container, const void *newElement);

    IteratorCapability iteratorCapabilities() {return static_cast<IteratorCapability>(_iteratorCapabilities & 0xF);}
    uint revision() {return _iteratorCapabilities >> 4 & 0x7;}
    uint containerCapabilities() {return _iteratorCapabilities >> 7 & 0xF;}

    sizeFunc _size;
    atFunc _at;
    union {
        moveIteratorFunc _moveToBegin;
        moveIteratorFunc2 _moveTo;
    };
    union {
        moveIteratorFunc _moveToEnd;
        appendFunction _append;
    };
    advanceFunc _advance;
    getFunc _get;
    destroyIterFunc _destroyIter;
    equalIterFunc _equalIter;
    copyIterFunc _copyIter;

    template<class T>
    static int sizeImpl(const void *p)
    { return ContainerAPI<T>::size(static_cast<const T*>(p)); }

    template<class T>
    static const void* atImpl(const void *p, int idx)
    {
        typename T::const_iterator i = static_cast<const T*>(p)->begin();
        std::advance(i, idx);
        return IteratorOwner<typename T::const_iterator>::getData(i);
    }

    template<class T>
    static void moveToBeginImpl(const void *container, void **iterator)
    { IteratorOwner<typename T::const_iterator>::assign(iterator, static_cast<const T*>(container)->begin()); }

    template<class T>
    static void moveToEndImpl(const void *container, void **iterator)
    { IteratorOwner<typename T::const_iterator>::assign(iterator, static_cast<const T*>(container)->end()); }

    template<class Container>
    static void moveToImpl(const void *container, void **iterator, Position position)
    {
        if (position == ToBegin)
            moveToBeginImpl<Container>(container, iterator);
        else
            moveToEndImpl<Container>(container, iterator);
    }

    template<class T>
    static VariantData getImpl(void * const *iterator, int metaTypeId, uint flags)
    { return VariantData(metaTypeId, IteratorOwner<typename T::const_iterator>::getData(iterator), flags); }

public:
    template<class T> QSequentialIterableImpl(const T*p)
      : _iterable(p)
      , _iterator(nullptr)
      , _metaType_id(qMetaTypeId<typename T::value_type>())
      , _metaType_flags(QTypeInfo<typename T::value_type>::isPointer)
      , _iteratorCapabilities(ContainerAPI<T>::IteratorCapabilities | (1 << 4) | (ContainerCapabilitiesImpl<T>::ContainerCapabilities << (4+3)))
      , _size(sizeImpl<T>)
      , _at(atImpl<T>)
      , _moveTo(moveToImpl<T>)
      , _append(ContainerCapabilitiesImpl<T>::appendImpl)
      , _advance(IteratorOwner<typename T::const_iterator>::advance)
      , _get(getImpl<T>)
      , _destroyIter(IteratorOwner<typename T::const_iterator>::destroy)
      , _equalIter(IteratorOwner<typename T::const_iterator>::equal)
      , _copyIter(IteratorOwner<typename T::const_iterator>::assign)
    {
    }

    QSequentialIterableImpl()
      : _iterable(nullptr)
      , _iterator(nullptr)
      , _metaType_id(QMetaType::UnknownType)
      , _metaType_flags(0)
      , _iteratorCapabilities(0 | (1 << 4) ) // no iterator capabilities, revision 1
      , _size(nullptr)
      , _at(nullptr)
      , _moveToBegin(nullptr)
      , _moveToEnd(nullptr)
      , _advance(nullptr)
      , _get(nullptr)
      , _destroyIter(nullptr)
      , _equalIter(nullptr)
      , _copyIter(nullptr)
    {
    }

    inline void moveToBegin() {
        if (revision() == 0)
            _moveToBegin(_iterable, &_iterator);
        else
            _moveTo(_iterable, &_iterator, ToBegin);
    }
    inline void moveToEnd() {
        if (revision() == 0)
            _moveToEnd(_iterable, &_iterator);
        else
            _moveTo(_iterable, &_iterator, ToEnd);
    }
    inline bool equal(const QSequentialIterableImpl&other) const { return _equalIter(&_iterator, &other._iterator); }
    inline QSequentialIterableImpl &advance(int i) {
      Q_ASSERT(i > 0 || _iteratorCapabilities & BiDirectionalCapability);
      _advance(&_iterator, i);
      return *this;
    }

    inline void append(const void *newElement) {
        if (containerCapabilities() & ContainerIsAppendable)
            _append(_iterable, newElement);
    }

    inline VariantData getCurrent() const { return _get(&_iterator, _metaType_id, _metaType_flags); }

    VariantData at(int idx) const
    { return VariantData(_metaType_id, _at(_iterable, idx), _metaType_flags); }

    int size() const { Q_ASSERT(_iterable); return _size(_iterable); }

    inline void destroyIter() { _destroyIter(&_iterator); }

    void copy(const QSequentialIterableImpl &other)
    {
      *this = other;
      _copyIter(&_iterator, &other._iterator);
    }
};
QT_METATYPE_PRIVATE_DECLARE_TYPEINFO(QSequentialIterableImpl, Q_MOVABLE_TYPE)

template<typename From>
struct QSequentialIterableConvertFunctor
{
    QSequentialIterableImpl operator()(const From &f) const
    {
        return QSequentialIterableImpl(&f);
    }
};
}

namespace QtMetaTypePrivate {
template<typename T, bool = std::is_same<typename T::const_iterator::value_type, typename T::mapped_type>::value>
struct AssociativeContainerAccessor
{
    static const typename T::key_type& getKey(const typename T::const_iterator &it)
    {
        return it.key();
    }

    static const typename T::mapped_type& getValue(const typename T::const_iterator &it)
    {
        return it.value();
    }
};

template<typename T, bool = std::is_same<typename T::const_iterator::value_type, std::pair<const typename T::key_type, typename T::mapped_type> >::value>
struct StlStyleAssociativeContainerAccessor;

template<typename T>
struct StlStyleAssociativeContainerAccessor<T, true>
{
    static const typename T::key_type& getKey(const typename T::const_iterator &it)
    {
        return it->first;
    }

    static const typename T::mapped_type& getValue(const typename T::const_iterator &it)
    {
        return it->second;
    }
};

template<typename T>
struct AssociativeContainerAccessor<T, false> : public StlStyleAssociativeContainerAccessor<T>
{
};

class QAssociativeIterableImpl
{
public:
    const void *_iterable;
    void *_iterator;
    int _metaType_id_key;
    uint _metaType_flags_key;
    int _metaType_id_value;
    uint _metaType_flags_value;
    typedef int(*sizeFunc)(const void *p);
    typedef void (*findFunc)(const void *container, const void *p, void **iterator);
    typedef void (*beginFunc)(const void *p, void **);
    typedef void (*advanceFunc)(void **p, int);
    typedef VariantData (*getFunc)(void * const *p, int metaTypeId, uint flags);
    typedef void (*destroyIterFunc)(void **p);
    typedef bool (*equalIterFunc)(void * const *p, void * const *other);
    typedef void (*copyIterFunc)(void **, void * const *);

    sizeFunc _size;
    findFunc _find;
    beginFunc _begin;
    beginFunc _end;
    advanceFunc _advance;
    getFunc _getKey;
    getFunc _getValue;
    destroyIterFunc _destroyIter;
    equalIterFunc _equalIter;
    copyIterFunc _copyIter;

    template<class T>
    static int sizeImpl(const void *p)
    { return int(std::distance(static_cast<const T*>(p)->begin(),
                               static_cast<const T*>(p)->end())); }

    template<class T>
    static void findImpl(const void *container, const void *p, void **iterator)
    { IteratorOwner<typename T::const_iterator>::assign(iterator,
                                                        static_cast<const T*>(container)->find(*static_cast<const typename T::key_type*>(p))); }

    QT_WARNING_PUSH
    QT_WARNING_DISABLE_DEPRECATED // Hits on the deprecated QHash::iterator::operator--()
    template<class T>
    static void advanceImpl(void **p, int step)
    { std::advance(*static_cast<typename T::const_iterator*>(*p), step); }
    QT_WARNING_POP

    template<class T>
    static void beginImpl(const void *container, void **iterator)
    { IteratorOwner<typename T::const_iterator>::assign(iterator, static_cast<const T*>(container)->begin()); }

    template<class T>
    static void endImpl(const void *container, void **iterator)
    { IteratorOwner<typename T::const_iterator>::assign(iterator, static_cast<const T*>(container)->end()); }

    template<class T>
    static VariantData getKeyImpl(void * const *iterator, int metaTypeId, uint flags)
    { return VariantData(metaTypeId, &AssociativeContainerAccessor<T>::getKey(*static_cast<typename T::const_iterator*>(*iterator)), flags); }

    template<class T>
    static VariantData getValueImpl(void * const *iterator, int metaTypeId, uint flags)
    { return VariantData(metaTypeId, &AssociativeContainerAccessor<T>::getValue(*static_cast<typename T::const_iterator*>(*iterator)), flags); }

public:
    template<class T> QAssociativeIterableImpl(const T*p)
      : _iterable(p)
      , _iterator(nullptr)
      , _metaType_id_key(qMetaTypeId<typename T::key_type>())
      , _metaType_flags_key(QTypeInfo<typename T::key_type>::isPointer)
      , _metaType_id_value(qMetaTypeId<typename T::mapped_type>())
      , _metaType_flags_value(QTypeInfo<typename T::mapped_type>::isPointer)
      , _size(sizeImpl<T>)
      , _find(findImpl<T>)
      , _begin(beginImpl<T>)
      , _end(endImpl<T>)
      , _advance(advanceImpl<T>)
      , _getKey(getKeyImpl<T>)
      , _getValue(getValueImpl<T>)
      , _destroyIter(IteratorOwner<typename T::const_iterator>::destroy)
      , _equalIter(IteratorOwner<typename T::const_iterator>::equal)
      , _copyIter(IteratorOwner<typename T::const_iterator>::assign)
    {
    }

    QAssociativeIterableImpl()
      : _iterable(nullptr)
      , _iterator(nullptr)
      , _metaType_id_key(QMetaType::UnknownType)
      , _metaType_flags_key(0)
      , _metaType_id_value(QMetaType::UnknownType)
      , _metaType_flags_value(0)
      , _size(nullptr)
      , _find(nullptr)
      , _begin(nullptr)
      , _end(nullptr)
      , _advance(nullptr)
      , _getKey(nullptr)
      , _getValue(nullptr)
      , _destroyIter(nullptr)
      , _equalIter(nullptr)
      , _copyIter(nullptr)
    {
    }

    inline void begin() { _begin(_iterable, &_iterator); }
    inline void end() { _end(_iterable, &_iterator); }
    inline bool equal(const QAssociativeIterableImpl&other) const { return _equalIter(&_iterator, &other._iterator); }
    inline QAssociativeIterableImpl &advance(int i) { _advance(&_iterator, i); return *this; }

    inline void destroyIter() { _destroyIter(&_iterator); }

    inline VariantData getCurrentKey() const { return _getKey(&_iterator, _metaType_id_key, _metaType_flags_key); }
    inline VariantData getCurrentValue() const { return _getValue(&_iterator, _metaType_id_value, _metaType_flags_value); }

    inline void find(const VariantData &key)
    { _find(_iterable, key.data, &_iterator); }

    int size() const { Q_ASSERT(_iterable); return _size(_iterable); }

    void copy(const QAssociativeIterableImpl &other)
    {
      *this = other;
      _copyIter(&_iterator, &other._iterator);
    }
};
QT_METATYPE_PRIVATE_DECLARE_TYPEINFO(QAssociativeIterableImpl, Q_MOVABLE_TYPE)

template<typename From>
struct QAssociativeIterableConvertFunctor
{
    QAssociativeIterableImpl operator()(const From& f) const
    {
        return QAssociativeIterableImpl(&f);
    }
};

class QPairVariantInterfaceImpl
{
    const void *_pair;
    int _metaType_id_first;
    uint _metaType_flags_first;
    int _metaType_id_second;
    uint _metaType_flags_second;

    typedef VariantData (*getFunc)(const void * const *p, int metaTypeId, uint flags);

    getFunc _getFirst;
    getFunc _getSecond;

    template<class T>
    static VariantData getFirstImpl(const void * const *pair, int metaTypeId, uint flags)
    { return VariantData(metaTypeId, &static_cast<const T*>(*pair)->first, flags); }
    template<class T>
    static VariantData getSecondImpl(const void * const *pair, int metaTypeId, uint flags)
    { return VariantData(metaTypeId, &static_cast<const T*>(*pair)->second, flags); }

public:
    template<class T> QPairVariantInterfaceImpl(const T*p)
      : _pair(p)
      , _metaType_id_first(qMetaTypeId<typename T::first_type>())
      , _metaType_flags_first(QTypeInfo<typename T::first_type>::isPointer)
      , _metaType_id_second(qMetaTypeId<typename T::second_type>())
      , _metaType_flags_second(QTypeInfo<typename T::second_type>::isPointer)
      , _getFirst(getFirstImpl<T>)
      , _getSecond(getSecondImpl<T>)
    {
    }

    QPairVariantInterfaceImpl()
      : _pair(nullptr)
      , _metaType_id_first(QMetaType::UnknownType)
      , _metaType_flags_first(0)
      , _metaType_id_second(QMetaType::UnknownType)
      , _metaType_flags_second(0)
      , _getFirst(nullptr)
      , _getSecond(nullptr)
    {
    }

    inline VariantData first() const { return _getFirst(&_pair, _metaType_id_first, _metaType_flags_first); }
    inline VariantData second() const { return _getSecond(&_pair, _metaType_id_second, _metaType_flags_second); }
};
QT_METATYPE_PRIVATE_DECLARE_TYPEINFO(QPairVariantInterfaceImpl, Q_MOVABLE_TYPE)

template<typename From>
struct QPairVariantInterfaceConvertFunctor;

template<typename T, typename U>
struct QPairVariantInterfaceConvertFunctor<QPair<T, U> >
{
    QPairVariantInterfaceImpl operator()(const QPair<T, U>& f) const
    {
        return QPairVariantInterfaceImpl(&f);
    }
};

template<typename T, typename U>
struct QPairVariantInterfaceConvertFunctor<std::pair<T, U> >
{
    QPairVariantInterfaceImpl operator()(const std::pair<T, U>& f) const
    {
        return QPairVariantInterfaceImpl(&f);
    }
};

}

class QObject;
class QWidget;

#define QT_FORWARD_DECLARE_SHARED_POINTER_TYPES_ITER(Name) \
    template <class T> class Name; \

QT_FOR_EACH_AUTOMATIC_TEMPLATE_SMART_POINTER(QT_FORWARD_DECLARE_SHARED_POINTER_TYPES_ITER)

namespace QtPrivate
{
    template<typename T>
    struct IsPointerToTypeDerivedFromQObject
    {
        enum { Value = false };
    };

    // Specialize to avoid sizeof(void) warning
    template<>
    struct IsPointerToTypeDerivedFromQObject<void*>
    {
        enum { Value = false };
    };
    template<>
    struct IsPointerToTypeDerivedFromQObject<const void*>
    {
        enum { Value = false };
    };
    template<>
    struct IsPointerToTypeDerivedFromQObject<QObject*>
    {
        enum { Value = true };
    };

    template<typename T>
    struct IsPointerToTypeDerivedFromQObject<T*>
    {
        typedef qint8 yes_type;
        typedef qint64 no_type;

#ifndef QT_NO_QOBJECT
        static yes_type checkType(QObject* );
#endif
        static no_type checkType(...);
        Q_STATIC_ASSERT_X(sizeof(T), "Type argument of Q_DECLARE_METATYPE(T*) must be fully defined");
        enum { Value = sizeof(checkType(static_cast<T*>(nullptr))) == sizeof(yes_type) };
    };

    template<typename T, typename Enable = void>
    struct IsGadgetHelper { enum { IsRealGadget = false, IsGadgetOrDerivedFrom = false }; };

    template<typename T>
    struct IsGadgetHelper<T, typename T::QtGadgetHelper>
    {
        template <typename X>
        static char checkType(void (X::*)());
        static void *checkType(void (T::*)());
        enum {
            IsRealGadget = sizeof(checkType(&T::qt_check_for_QGADGET_macro)) == sizeof(void *),
            IsGadgetOrDerivedFrom = true
        };
    };

    template<typename T, typename Enable = void>
    struct IsPointerToGadgetHelper { enum { IsRealGadget = false, IsGadgetOrDerivedFrom = false }; };

    template<typename T>
    struct IsPointerToGadgetHelper<T*, typename T::QtGadgetHelper>
    {
        using BaseType = T;
        template <typename X>
        static char checkType(void (X::*)());
        static void *checkType(void (T::*)());
        enum {
            IsRealGadget = !IsPointerToTypeDerivedFromQObject<T*>::Value && sizeof(checkType(&T::qt_check_for_QGADGET_macro)) == sizeof(void *),
            IsGadgetOrDerivedFrom = !IsPointerToTypeDerivedFromQObject<T*>::Value
        };
    };


    template<typename T> char qt_getEnumMetaObject(const T&);

    template<typename T>
    struct IsQEnumHelper {
        static const T &declval();
        // If the type was declared with Q_ENUM, the friend qt_getEnumMetaObject() declared in the
        // Q_ENUM macro will be chosen by ADL, and the return type will be QMetaObject*.
        // Otherwise the chosen overload will be the catch all template function
        // qt_getEnumMetaObject(T) which returns 'char'
        enum { Value = sizeof(qt_getEnumMetaObject(declval())) == sizeof(QMetaObject*) };
    };
    template<> struct IsQEnumHelper<void> { enum { Value = false }; };

    template<typename T, typename Enable = void>
    struct MetaObjectForType
    {
        static inline const QMetaObject *value() { return nullptr; }
    };
    template<>
    struct MetaObjectForType<void>
    {
        static inline const QMetaObject *value() { return nullptr; }
    };
    template<typename T>
    struct MetaObjectForType<T*, typename std::enable_if<IsPointerToTypeDerivedFromQObject<T*>::Value>::type>
    {
        static inline const QMetaObject *value() { return &T::staticMetaObject; }
    };
    template<typename T>
    struct MetaObjectForType<T, typename std::enable_if<IsGadgetHelper<T>::IsGadgetOrDerivedFrom>::type>
    {
        static inline const QMetaObject *value() { return &T::staticMetaObject; }
    };
    template<typename T>
    struct MetaObjectForType<T, typename std::enable_if<IsPointerToGadgetHelper<T>::IsGadgetOrDerivedFrom>::type>
    {
        static inline const QMetaObject *value() { return &IsPointerToGadgetHelper<T>::BaseType::staticMetaObject; }
    };
    template<typename T>
    struct MetaObjectForType<T, typename std::enable_if<IsQEnumHelper<T>::Value>::type >
    {
        static inline const QMetaObject *value() { return qt_getEnumMetaObject(T()); }
    };

    template<typename T>
    struct IsSharedPointerToTypeDerivedFromQObject
    {
        enum { Value = false };
    };

    template<typename T>
    struct IsSharedPointerToTypeDerivedFromQObject<QSharedPointer<T> > : IsPointerToTypeDerivedFromQObject<T*>
    {
    };

    template<typename T>
    struct IsWeakPointerToTypeDerivedFromQObject
    {
        enum { Value = false };
    };

    template<typename T>
    struct IsWeakPointerToTypeDerivedFromQObject<QWeakPointer<T> > : IsPointerToTypeDerivedFromQObject<T*>
    {
    };

    template<typename T>
    struct IsTrackingPointerToTypeDerivedFromQObject
    {
        enum { Value = false };
    };

    template<typename T>
    struct IsTrackingPointerToTypeDerivedFromQObject<QPointer<T> >
    {
        enum { Value = true };
    };

    template<typename T>
    struct IsSequentialContainer
    {
        enum { Value = false };
    };

    template<typename T>
    struct IsAssociativeContainer
    {
        enum { Value = false };
    };

    template<typename T, bool = QtPrivate::IsSequentialContainer<T>::Value>
    struct SequentialContainerConverterHelper
    {
        static bool registerConverter(int)
        {
            return false;
        }
    };

    template<typename T, bool = QMetaTypeId2<typename T::value_type>::Defined>
    struct ValueTypeIsMetaType
    {
        static bool registerConverter(int)
        {
            return false;
        }
    };

    template<typename T>
    struct SequentialContainerConverterHelper<T, true> : ValueTypeIsMetaType<T>
    {
    };

    template<typename T, bool = QtPrivate::IsAssociativeContainer<T>::Value>
    struct AssociativeContainerConverterHelper
    {
        static bool registerConverter(int)
        {
            return false;
        }
    };

    template<typename T, bool = QMetaTypeId2<typename T::mapped_type>::Defined>
    struct AssociativeValueTypeIsMetaType
    {
        static bool registerConverter(int)
        {
            return false;
        }
    };

    template<typename T, bool = QMetaTypeId2<typename T::key_type>::Defined>
    struct KeyAndValueTypeIsMetaType
    {
        static bool registerConverter(int)
        {
            return false;
        }
    };

    template<typename T>
    struct KeyAndValueTypeIsMetaType<T, true> : AssociativeValueTypeIsMetaType<T>
    {
    };

    template<typename T>
    struct AssociativeContainerConverterHelper<T, true> : KeyAndValueTypeIsMetaType<T>
    {
    };

    template<typename T, bool = QMetaTypeId2<typename T::first_type>::Defined
                                && QMetaTypeId2<typename T::second_type>::Defined>
    struct IsMetaTypePair
    {
        static bool registerConverter(int)
        {
            return false;
        }
    };

    template<typename T>
    struct IsMetaTypePair<T, true>
    {
        inline static bool registerConverter(int id);
    };

    template<typename T>
    struct IsPair
    {
        static bool registerConverter(int)
        {
            return false;
        }
    };
    template<typename T, typename U>
    struct IsPair<QPair<T, U> > : IsMetaTypePair<QPair<T, U> > {};
    template<typename T, typename U>
    struct IsPair<std::pair<T, U> > : IsMetaTypePair<std::pair<T, U> > {};

    template<typename T>
    struct MetaTypePairHelper : IsPair<T> {};

    template<typename T, typename = void>
    struct MetaTypeSmartPointerHelper
    {
        static bool registerConverter(int) { return false; }
    };

    Q_CORE_EXPORT bool isBuiltinType(const QByteArray &type);
} // namespace QtPrivate

template <typename T, int =
    QtPrivate::IsPointerToTypeDerivedFromQObject<T>::Value ? QMetaType::PointerToQObject :
    QtPrivate::IsGadgetHelper<T>::IsRealGadget             ? QMetaType::IsGadget :
    QtPrivate::IsPointerToGadgetHelper<T>::IsRealGadget    ? QMetaType::PointerToGadget :
    QtPrivate::IsQEnumHelper<T>::Value                     ? QMetaType::IsEnumeration : 0>
struct QMetaTypeIdQObject
{
    enum {
        Defined = 0
    };
};

template <typename T>
struct QMetaTypeId : public QMetaTypeIdQObject<T>
{
};

template <typename T>
struct QMetaTypeId2
{
    enum { Defined = QMetaTypeId<T>::Defined, IsBuiltIn=false };
    static inline Q_DECL_CONSTEXPR int qt_metatype_id() { return QMetaTypeId<T>::qt_metatype_id(); }
};

template <typename T>
struct QMetaTypeId2<const T&> : QMetaTypeId2<T> {};

template <typename T>
struct QMetaTypeId2<T&> { enum {Defined = false }; };

namespace QtPrivate {
    template <typename T, bool Defined = QMetaTypeId2<T>::Defined>
    struct QMetaTypeIdHelper {
        static inline Q_DECL_CONSTEXPR int qt_metatype_id()
        { return QMetaTypeId2<T>::qt_metatype_id(); }
    };
    template <typename T> struct QMetaTypeIdHelper<T, false> {
        static inline Q_DECL_CONSTEXPR int qt_metatype_id()
        { return -1; }
    };

    // Function pointers don't derive from QObject
    template <typename Result, typename... Args>
    struct IsPointerToTypeDerivedFromQObject<Result(*)(Args...)> { enum { Value = false }; };

    template<typename T>
    struct QMetaTypeTypeFlags
    {
        enum { Flags = (QTypeInfoQuery<T>::isRelocatable ? QMetaType::MovableType : 0)
                     | (QTypeInfo<T>::isComplex ? QMetaType::NeedsConstruction : 0)
                     | (QTypeInfo<T>::isComplex ? QMetaType::NeedsDestruction : 0)
                     | (IsPointerToTypeDerivedFromQObject<T>::Value ? QMetaType::PointerToQObject : 0)
                     | (IsSharedPointerToTypeDerivedFromQObject<T>::Value ? QMetaType::SharedPointerToQObject : 0)
                     | (IsWeakPointerToTypeDerivedFromQObject<T>::Value ? QMetaType::WeakPointerToQObject : 0)
                     | (IsTrackingPointerToTypeDerivedFromQObject<T>::Value ? QMetaType::TrackingPointerToQObject : 0)
                     | (std::is_enum<T>::value ? QMetaType::IsEnumeration : 0)
                     | (IsGadgetHelper<T>::IsGadgetOrDerivedFrom ? QMetaType::IsGadget : 0)
                     | (IsPointerToGadgetHelper<T>::IsGadgetOrDerivedFrom ? QMetaType::PointerToGadget : 0)
             };
    };

    template<typename T, bool defined>
    struct MetaTypeDefinedHelper
    {
        enum DefinedType { Defined = defined };
    };

    template<typename SmartPointer>
    struct QSmartPointerConvertFunctor
    {
        QObject* operator()(const SmartPointer &p) const
        {
            return p.operator->();
        }
    };

    // hack to delay name lookup to instantiation time by making
    // EnableInternalData a dependent name:
    template <typename T>
    struct EnableInternalDataWrap;

    template<typename T>
    struct QSmartPointerConvertFunctor<QWeakPointer<T> >
    {
        QObject* operator()(const QWeakPointer<T> &p) const
        {
            return QtPrivate::EnableInternalDataWrap<T>::internalData(p);
        }
    };
}

template <typename T>
int qRegisterNormalizedMetaType(const QT_PREPEND_NAMESPACE(QByteArray) &normalizedTypeName
#ifndef Q_CLANG_QDOC
    , T * dummy = 0
    , typename QtPrivate::MetaTypeDefinedHelper<T, QMetaTypeId2<T>::Defined && !QMetaTypeId2<T>::IsBuiltIn>::DefinedType defined = QtPrivate::MetaTypeDefinedHelper<T, QMetaTypeId2<T>::Defined && !QMetaTypeId2<T>::IsBuiltIn>::Defined
#endif
)
{
#ifndef QT_NO_QOBJECT
    Q_ASSERT_X(normalizedTypeName == QMetaObject::normalizedType(normalizedTypeName.constData()), "qRegisterNormalizedMetaType", "qRegisterNormalizedMetaType was called with a not normalized type name, please call qRegisterMetaType instead.");
#endif
    const int typedefOf = dummy ? -1 : QtPrivate::QMetaTypeIdHelper<T>::qt_metatype_id();
    if (typedefOf != -1)
        return QMetaType::registerNormalizedTypedef(normalizedTypeName, typedefOf);

    QMetaType::TypeFlags flags(QtPrivate::QMetaTypeTypeFlags<T>::Flags);

    if (defined)
        flags |= QMetaType::WasDeclaredAsMetaType;

    const int id = QMetaType::registerNormalizedType(normalizedTypeName,
                                   QtMetaTypePrivate::QMetaTypeFunctionHelper<T>::Destruct,
                                   QtMetaTypePrivate::QMetaTypeFunctionHelper<T>::Construct,
                                   int(sizeof(T)),
                                   flags,
                                   QtPrivate::MetaObjectForType<T>::value());

    if (id > 0) {
        QtPrivate::SequentialContainerConverterHelper<T>::registerConverter(id);
        QtPrivate::AssociativeContainerConverterHelper<T>::registerConverter(id);
        QtPrivate::MetaTypePairHelper<T>::registerConverter(id);
        QtPrivate::MetaTypeSmartPointerHelper<T>::registerConverter(id);
    }

    return id;
}

template <typename T>
int qRegisterMetaType(const char *typeName
#ifndef Q_CLANG_QDOC
    , T * dummy = nullptr
    , typename QtPrivate::MetaTypeDefinedHelper<T, QMetaTypeId2<T>::Defined && !QMetaTypeId2<T>::IsBuiltIn>::DefinedType defined = QtPrivate::MetaTypeDefinedHelper<T, QMetaTypeId2<T>::Defined && !QMetaTypeId2<T>::IsBuiltIn>::Defined
#endif
)
{
#ifdef QT_NO_QOBJECT
    QT_PREPEND_NAMESPACE(QByteArray) normalizedTypeName = typeName;
#else
    QT_PREPEND_NAMESPACE(QByteArray) normalizedTypeName = QMetaObject::normalizedType(typeName);
#endif
    return qRegisterNormalizedMetaType<T>(normalizedTypeName, dummy, defined);
}

#ifndef QT_NO_DATASTREAM
template <typename T>
void qRegisterMetaTypeStreamOperators(const char *typeName
#ifndef Q_CLANG_QDOC
    , T * /* dummy */ = nullptr
#endif
)
{
    qRegisterMetaType<T>(typeName);
    QMetaType::registerStreamOperators(typeName, QtMetaTypePrivate::QMetaTypeFunctionHelper<T>::Save,
                                                 QtMetaTypePrivate::QMetaTypeFunctionHelper<T>::Load);
}
#endif // QT_NO_DATASTREAM

template <typename T>
inline Q_DECL_CONSTEXPR int qMetaTypeId()
{
    Q_STATIC_ASSERT_X(QMetaTypeId2<T>::Defined, "Type is not registered, please use the Q_DECLARE_METATYPE macro to make it known to Qt's meta-object system");
    return QMetaTypeId2<T>::qt_metatype_id();
}

template <typename T>
inline Q_DECL_CONSTEXPR int qRegisterMetaType()
{
    return qMetaTypeId<T>();
}

#if QT_DEPRECATED_SINCE(5, 1) && !defined(Q_CLANG_QDOC)
// There used to be a T *dummy = 0 argument in Qt 4.0 to support MSVC6
template <typename T>
QT_DEPRECATED inline Q_DECL_CONSTEXPR int qMetaTypeId(T *)
{ return qMetaTypeId<T>(); }
#ifndef Q_CC_SUN
template <typename T>
QT_DEPRECATED inline Q_DECL_CONSTEXPR int qRegisterMetaType(T *)
{ return qRegisterMetaType<T>(); }
#endif
#endif

#ifndef QT_NO_QOBJECT
template <typename T>
struct QMetaTypeIdQObject<T*, QMetaType::PointerToQObject>
{
    enum {
        Defined = 1
    };

    static int qt_metatype_id()
    {
        static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0);
        if (const int id = metatype_id.loadAcquire())
            return id;
        const char * const cName = T::staticMetaObject.className();
        QByteArray typeName;
        typeName.reserve(int(strlen(cName)) + 1);
        typeName.append(cName).append('*');
        const int newId = qRegisterNormalizedMetaType<T*>(
                        typeName,
                        reinterpret_cast<T**>(quintptr(-1)));
        metatype_id.storeRelease(newId);
        return newId;
    }
};

template <typename T>
struct QMetaTypeIdQObject<T, QMetaType::IsGadget>
{
    enum {
        Defined = std::is_default_constructible<T>::value
    };

    static int qt_metatype_id()
    {
        static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0);
        if (const int id = metatype_id.loadAcquire())
            return id;
        const char * const cName = T::staticMetaObject.className();
        const int newId = qRegisterNormalizedMetaType<T>(
            cName,
            reinterpret_cast<T*>(quintptr(-1)));
        metatype_id.storeRelease(newId);
        return newId;
    }
};

template <typename T>
struct QMetaTypeIdQObject<T*, QMetaType::PointerToGadget>
{
    enum {
        Defined = 1
    };

    static int qt_metatype_id()
    {
        static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0);
        if (const int id = metatype_id.loadAcquire())
            return id;
        const char * const cName = T::staticMetaObject.className();
        QByteArray typeName;
        typeName.reserve(int(strlen(cName)) + 1);
        typeName.append(cName).append('*');
        const int newId = qRegisterNormalizedMetaType<T*>(
            typeName,
            reinterpret_cast<T**>(quintptr(-1)));
        metatype_id.storeRelease(newId);
        return newId;
    }
};

template <typename T>
struct QMetaTypeIdQObject<T, QMetaType::IsEnumeration>
{
    enum {
        Defined = 1
    };

    static int qt_metatype_id()
    {
        static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0);
        if (const int id = metatype_id.loadAcquire())
            return id;
        const char *eName = qt_getEnumName(T());
        const char *cName = qt_getEnumMetaObject(T())->className();
        QByteArray typeName;
        typeName.reserve(int(strlen(cName) + 2 + strlen(eName)));
        typeName.append(cName).append("::").append(eName);
        const int newId = qRegisterNormalizedMetaType<T>(
            typeName,
            reinterpret_cast<T*>(quintptr(-1)));
        metatype_id.storeRelease(newId);
        return newId;
    }
};
#endif

#ifndef QT_NO_DATASTREAM
template <typename T>
inline int qRegisterMetaTypeStreamOperators()
{
    int id = qMetaTypeId<T>();
    QMetaType::registerStreamOperators(id, QtMetaTypePrivate::QMetaTypeFunctionHelper<T>::Save,
                                           QtMetaTypePrivate::QMetaTypeFunctionHelper<T>::Load);
    return id;
}
#endif

#define Q_DECLARE_OPAQUE_POINTER(POINTER)                               \
    QT_BEGIN_NAMESPACE namespace QtPrivate {                            \
        template <>                                                     \
        struct IsPointerToTypeDerivedFromQObject<POINTER >              \
        {                                                               \
            enum { Value = false };                                     \
        };                                                              \
    } QT_END_NAMESPACE                                                  \
    /**/

#ifndef Q_MOC_RUN
#define Q_DECLARE_METATYPE(TYPE) Q_DECLARE_METATYPE_IMPL(TYPE)
#define Q_DECLARE_METATYPE_IMPL(TYPE)                                   \
    QT_BEGIN_NAMESPACE                                                  \
    template <>                                                         \
    struct QMetaTypeId< TYPE >                                          \
    {                                                                   \
        enum { Defined = 1 };                                           \
        static int qt_metatype_id()                                     \
            {                                                           \
                static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0); \
                if (const int id = metatype_id.loadAcquire())           \
                    return id;                                          \
                const int newId = qRegisterMetaType< TYPE >(#TYPE,      \
                              reinterpret_cast< TYPE *>(quintptr(-1))); \
                metatype_id.storeRelease(newId);                        \
                return newId;                                           \
            }                                                           \
    };                                                                  \
    QT_END_NAMESPACE
#endif // Q_MOC_RUN

#define Q_DECLARE_BUILTIN_METATYPE(TYPE, METATYPEID, NAME) \
    QT_BEGIN_NAMESPACE \
    template<> struct QMetaTypeId2<NAME> \
    { \
        enum { Defined = 1, IsBuiltIn = true, MetaType = METATYPEID };   \
        static inline Q_DECL_CONSTEXPR int qt_metatype_id() { return METATYPEID; } \
    }; \
    QT_END_NAMESPACE

#define QT_FORWARD_DECLARE_STATIC_TYPES_ITER(TypeName, TypeId, Name) \
    class Name;

QT_FOR_EACH_STATIC_CORE_CLASS(QT_FORWARD_DECLARE_STATIC_TYPES_ITER)
QT_FOR_EACH_STATIC_GUI_CLASS(QT_FORWARD_DECLARE_STATIC_TYPES_ITER)
QT_FOR_EACH_STATIC_WIDGETS_CLASS(QT_FORWARD_DECLARE_STATIC_TYPES_ITER)

#undef QT_FORWARD_DECLARE_STATIC_TYPES_ITER

typedef QList<QVariant> QVariantList;
typedef QMap<QString, QVariant> QVariantMap;
typedef QHash<QString, QVariant> QVariantHash;
#ifdef Q_CLANG_QDOC
class QByteArrayList;
#else
typedef QList<QByteArray> QByteArrayList;
#endif

#define Q_DECLARE_METATYPE_TEMPLATE_1ARG(SINGLE_ARG_TEMPLATE) \
QT_BEGIN_NAMESPACE \
template <typename T> \
struct QMetaTypeId< SINGLE_ARG_TEMPLATE<T> > \
{ \
    enum { \
        Defined = QMetaTypeId2<T>::Defined \
    }; \
    static int qt_metatype_id() \
    { \
        static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0); \
        if (const int id = metatype_id.loadRelaxed()) \
            return id; \
        const char *tName = QMetaType::typeName(qMetaTypeId<T>()); \
        Q_ASSERT(tName); \
        const int tNameLen = int(qstrlen(tName)); \
        QByteArray typeName; \
        typeName.reserve(int(sizeof(#SINGLE_ARG_TEMPLATE)) + 1 + tNameLen + 1 + 1); \
        typeName.append(#SINGLE_ARG_TEMPLATE, int(sizeof(#SINGLE_ARG_TEMPLATE)) - 1) \
            .append('<').append(tName, tNameLen); \
        if (typeName.endsWith('>')) \
            typeName.append(' '); \
        typeName.append('>'); \
        const int newId = qRegisterNormalizedMetaType< SINGLE_ARG_TEMPLATE<T> >( \
                        typeName, \
                        reinterpret_cast< SINGLE_ARG_TEMPLATE<T> *>(quintptr(-1))); \
        metatype_id.storeRelease(newId); \
        return newId; \
    } \
}; \
namespace QtPrivate { \
template<typename T> \
struct IsSequentialContainer<SINGLE_ARG_TEMPLATE<T> > \
{ \
    enum { Value = true }; \
}; \
} \
QT_END_NAMESPACE

#define Q_DECLARE_METATYPE_TEMPLATE_2ARG(DOUBLE_ARG_TEMPLATE) \
QT_BEGIN_NAMESPACE \
template<typename T, typename U> \
struct QMetaTypeId< DOUBLE_ARG_TEMPLATE<T, U> > \
{ \
    enum { \
        Defined = QMetaTypeId2<T>::Defined && QMetaTypeId2<U>::Defined \
    }; \
    static int qt_metatype_id() \
    { \
        static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0); \
        if (const int id = metatype_id.loadAcquire()) \
            return id; \
        const char *tName = QMetaType::typeName(qMetaTypeId<T>()); \
        const char *uName = QMetaType::typeName(qMetaTypeId<U>()); \
        Q_ASSERT(tName); \
        Q_ASSERT(uName); \
        const int tNameLen = int(qstrlen(tName)); \
        const int uNameLen = int(qstrlen(uName)); \
        QByteArray typeName; \
        typeName.reserve(int(sizeof(#DOUBLE_ARG_TEMPLATE)) + 1 + tNameLen + 1 + uNameLen + 1 + 1); \
        typeName.append(#DOUBLE_ARG_TEMPLATE, int(sizeof(#DOUBLE_ARG_TEMPLATE)) - 1) \
            .append('<').append(tName, tNameLen).append(',').append(uName, uNameLen); \
        if (typeName.endsWith('>')) \
            typeName.append(' '); \
        typeName.append('>'); \
        const int newId = qRegisterNormalizedMetaType< DOUBLE_ARG_TEMPLATE<T, U> >(\
                        typeName, \
                        reinterpret_cast< DOUBLE_ARG_TEMPLATE<T, U> *>(quintptr(-1))); \
        metatype_id.storeRelease(newId); \
        return newId; \
    } \
}; \
QT_END_NAMESPACE

namespace QtPrivate {

template<typename T, bool /* isSharedPointerToQObjectDerived */ = false>
struct SharedPointerMetaTypeIdHelper
{
    enum {
        Defined = 0
    };
    static int qt_metatype_id()
    {
        return -1;
    }
};

}

#define Q_DECLARE_SMART_POINTER_METATYPE(SMART_POINTER) \
QT_BEGIN_NAMESPACE \
namespace QtPrivate { \
template<typename T> \
struct SharedPointerMetaTypeIdHelper<SMART_POINTER<T>, true> \
{ \
    enum { \
        Defined = 1 \
    }; \
    static int qt_metatype_id() \
    { \
        static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0); \
        if (const int id = metatype_id.loadAcquire()) \
            return id; \
        const char * const cName = T::staticMetaObject.className(); \
        QByteArray typeName; \
        typeName.reserve(int(sizeof(#SMART_POINTER) + 1 + strlen(cName) + 1)); \
        typeName.append(#SMART_POINTER, int(sizeof(#SMART_POINTER)) - 1) \
            .append('<').append(cName).append('>'); \
        const int newId = qRegisterNormalizedMetaType< SMART_POINTER<T> >( \
                        typeName, \
                        reinterpret_cast< SMART_POINTER<T> *>(quintptr(-1))); \
        metatype_id.storeRelease(newId); \
        return newId; \
    } \
}; \
template<typename T> \
struct MetaTypeSmartPointerHelper<SMART_POINTER<T> , \
        typename std::enable_if<IsPointerToTypeDerivedFromQObject<T*>::Value>::type> \
{ \
    static bool registerConverter(int id) \
    { \
        const int toId = QMetaType::QObjectStar; \
        if (!QMetaType::hasRegisteredConverterFunction(id, toId)) { \
            QtPrivate::QSmartPointerConvertFunctor<SMART_POINTER<T> > o; \
            static const QtPrivate::ConverterFunctor<SMART_POINTER<T>, \
                                    QObject*, \
                                    QSmartPointerConvertFunctor<SMART_POINTER<T> > > f(o); \
            return QMetaType::registerConverterFunction(&f, id, toId); \
        } \
        return true; \
    } \
}; \
} \
template <typename T> \
struct QMetaTypeId< SMART_POINTER<T> > \
    : QtPrivate::SharedPointerMetaTypeIdHelper< SMART_POINTER<T>, \
                                                QtPrivate::IsPointerToTypeDerivedFromQObject<T*>::Value> \
{ \
};\
QT_END_NAMESPACE

#define Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE_ITER(TEMPLATENAME) \
    QT_BEGIN_NAMESPACE \
    template <class T> class TEMPLATENAME; \
    QT_END_NAMESPACE \
    Q_DECLARE_METATYPE_TEMPLATE_1ARG(TEMPLATENAME)

QT_END_NAMESPACE

QT_FOR_EACH_AUTOMATIC_TEMPLATE_1ARG(Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE_ITER)

#undef Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE_ITER

#define Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE Q_DECLARE_METATYPE_TEMPLATE_1ARG

Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE(std::vector)
Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE(std::list)

#define Q_FORWARD_DECLARE_METATYPE_TEMPLATE_2ARG_ITER(TEMPLATENAME, CPPTYPE) \
    QT_BEGIN_NAMESPACE \
    template <class T1, class T2> CPPTYPE TEMPLATENAME; \
    QT_END_NAMESPACE \

QT_FOR_EACH_AUTOMATIC_TEMPLATE_2ARG(Q_FORWARD_DECLARE_METATYPE_TEMPLATE_2ARG_ITER)

#undef Q_DECLARE_METATYPE_TEMPLATE_2ARG_ITER

#define Q_DECLARE_ASSOCIATIVE_CONTAINER_METATYPE(TEMPLATENAME) \
    QT_BEGIN_NAMESPACE \
    namespace QtPrivate { \
    template<typename T, typename U> \
    struct IsAssociativeContainer<TEMPLATENAME<T, U> > \
    { \
        enum { Value = true }; \
    }; \
    } \
    QT_END_NAMESPACE \
    Q_DECLARE_METATYPE_TEMPLATE_2ARG(TEMPLATENAME)

Q_DECLARE_ASSOCIATIVE_CONTAINER_METATYPE(QHash)
Q_DECLARE_ASSOCIATIVE_CONTAINER_METATYPE(QMap)
Q_DECLARE_ASSOCIATIVE_CONTAINER_METATYPE(std::map)

Q_DECLARE_METATYPE_TEMPLATE_2ARG(QPair)
Q_DECLARE_METATYPE_TEMPLATE_2ARG(std::pair)

#define Q_DECLARE_METATYPE_TEMPLATE_SMART_POINTER_ITER(TEMPLATENAME) \
    Q_DECLARE_SMART_POINTER_METATYPE(TEMPLATENAME)


QT_FOR_EACH_AUTOMATIC_TEMPLATE_SMART_POINTER(Q_DECLARE_METATYPE_TEMPLATE_SMART_POINTER_ITER)

QT_BEGIN_NAMESPACE

#undef Q_DECLARE_METATYPE_TEMPLATE_SMART_POINTER_ITER

inline QMetaType::QMetaType(const ExtensionFlag extensionFlags, const QMetaTypeInterface *info,
                            TypedConstructor creator,
                            TypedDestructor deleter,
                            SaveOperator saveOp,
                            LoadOperator loadOp,
                            Constructor constructor,
                            Destructor destructor,
                            uint size,
                            uint theTypeFlags,
                            int typeId,
                            const QMetaObject *_metaObject)
    : m_typedConstructor(creator)
    , m_typedDestructor(deleter)
    , m_saveOp(saveOp)
    , m_loadOp(loadOp)
    , m_constructor(constructor)
    , m_destructor(destructor)
    , m_extension(nullptr)
    , m_size(size)
    , m_typeFlags(theTypeFlags)
    , m_extensionFlags(extensionFlags)
    , m_typeId(typeId)
    , m_metaObject(_metaObject)
{
    if (Q_UNLIKELY(isExtended(CtorEx) || typeId == QMetaType::Void))
        ctor(info);
}

inline QMetaType::~QMetaType()
{
    if (Q_UNLIKELY(isExtended(DtorEx)))
        dtor();
}

inline bool QMetaType::isValid() const
{
    return m_typeId != UnknownType;
}

inline bool QMetaType::isRegistered() const
{
    return isValid();
}

inline int QMetaType::id() const
{
    return m_typeId;
}

inline void *QMetaType::create(const void *copy) const
{
    // ### TODO Qt6 remove the extension
    return createExtended(copy);
}

inline void QMetaType::destroy(void *data) const
{
    // ### TODO Qt6 remove the extension
    destroyExtended(data);
}

inline void *QMetaType::construct(void *where, const void *copy) const
{
    if (Q_UNLIKELY(isExtended(ConstructEx)))
        return constructExtended(where, copy);
    return m_constructor(where, copy);
}

inline void QMetaType::destruct(void *data) const
{
    if (Q_UNLIKELY(isExtended(DestructEx)))
        return destructExtended(data);
    if (Q_UNLIKELY(!data))
        return;
    m_destructor(data);
}

inline int QMetaType::sizeOf() const
{
    if (Q_UNLIKELY(isExtended(SizeEx)))
        return sizeExtended();
    return m_size;
}

inline QMetaType::TypeFlags QMetaType::flags() const
{
    if (Q_UNLIKELY(isExtended(FlagsEx)))
        return flagsExtended();
    return QMetaType::TypeFlags(m_typeFlags);
}

inline const QMetaObject *QMetaType::metaObject() const
{
    if (Q_UNLIKELY(isExtended(MetaObjectEx)))
        return metaObjectExtended();
    return m_metaObject;
}

QT_END_NAMESPACE


QT_FOR_EACH_STATIC_TYPE(Q_DECLARE_BUILTIN_METATYPE)

Q_DECLARE_METATYPE(QtMetaTypePrivate::QSequentialIterableImpl)
Q_DECLARE_METATYPE(QtMetaTypePrivate::QAssociativeIterableImpl)
Q_DECLARE_METATYPE(QtMetaTypePrivate::QPairVariantInterfaceImpl)

QT_BEGIN_NAMESPACE

template <typename T>
inline bool QtPrivate::IsMetaTypePair<T, true>::registerConverter(int id)
{
    const int toId = qMetaTypeId<QtMetaTypePrivate::QPairVariantInterfaceImpl>();
    if (!QMetaType::hasRegisteredConverterFunction(id, toId)) {
        QtMetaTypePrivate::QPairVariantInterfaceConvertFunctor<T> o;
        static const QtPrivate::ConverterFunctor<T,
                                    QtMetaTypePrivate::QPairVariantInterfaceImpl,
                                    QtMetaTypePrivate::QPairVariantInterfaceConvertFunctor<T> > f(o);
        return QMetaType::registerConverterFunction(&f, id, toId);
    }
    return true;
}

namespace QtPrivate {
    template<typename T>
    struct ValueTypeIsMetaType<T, true>
    {
        static bool registerConverter(int id)
        {
            const int toId = qMetaTypeId<QtMetaTypePrivate::QSequentialIterableImpl>();
            if (!QMetaType::hasRegisteredConverterFunction(id, toId)) {
                QtMetaTypePrivate::QSequentialIterableConvertFunctor<T> o;
                static const QtPrivate::ConverterFunctor<T,
                        QtMetaTypePrivate::QSequentialIterableImpl,
                QtMetaTypePrivate::QSequentialIterableConvertFunctor<T> > f(o);
                return QMetaType::registerConverterFunction(&f, id, toId);
            }
            return true;
        }
    };

    template<typename T>
    struct AssociativeValueTypeIsMetaType<T, true>
    {
        static bool registerConverter(int id)
        {
            const int toId = qMetaTypeId<QtMetaTypePrivate::QAssociativeIterableImpl>();
            if (!QMetaType::hasRegisteredConverterFunction(id, toId)) {
                QtMetaTypePrivate::QAssociativeIterableConvertFunctor<T> o;
                static const QtPrivate::ConverterFunctor<T,
                                            QtMetaTypePrivate::QAssociativeIterableImpl,
                                            QtMetaTypePrivate::QAssociativeIterableConvertFunctor<T> > f(o);
                return QMetaType::registerConverterFunction(&f, id, toId);
            }
            return true;
        }
    };
}

QT_END_NAMESPACE

#endif // QMETATYPE_H
