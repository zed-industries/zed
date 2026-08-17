// Copyright (C) 2021 The Qt Company Ltd.
// Copyright (C) 2018 Intel Corporation.
// Copyright (C) 2014 Olivier Goffart <ogoffart@woboq.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QMETATYPE_H
#define QMETATYPE_H

#include <QtCore/qglobal.h>
#include <QtCore/qatomic.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qcompare.h>
#include <QtCore/qscopeguard.h>
#include <QtCore/qdatastream.h>
#include <QtCore/qiterable.h>
#ifndef QT_NO_QOBJECT
#include <QtCore/qobjectdefs.h>
#endif
#include <QtCore/qhashfunctions.h>

#include <array>
#include <new>
#include <vector>
#include <list>
#include <map>
#include <functional>

#ifdef Bool
#error qmetatype.h must be included before any header file that defines Bool
#endif

QT_BEGIN_NAMESPACE

// from qcborcommon.h
enum class QCborSimpleType : quint8;

template <typename T>
struct QMetaTypeId2;

template <typename T>
inline constexpr int qMetaTypeId();

// F is a tuple: (QMetaType::TypeName, QMetaType::TypeNameID, RealType)
#define QT_FOR_EACH_STATIC_PRIMITIVE_NON_VOID_TYPE(F)\
    F(Bool, 1, bool) \
    F(Int, 2, int) \
    F(UInt, 3, uint) \
    F(LongLong, 4, qlonglong) \
    F(ULongLong, 5, qulonglong) \
    F(Double, 6, double) \
    F(Long, 32, long) \
    F(Short, 33, short) \
    F(Char, 34, char) \
    F(Char16, 56, char16_t) \
    F(Char32, 57, char32_t) \
    F(ULong, 35, ulong) \
    F(UShort, 36, ushort) \
    F(UChar, 37, uchar) \
    F(Float, 38, float) \
    F(SChar, 40, signed char) \
    F(Nullptr, 51, std::nullptr_t) \
    F(QCborSimpleType, 52, QCborSimpleType) \

#define QT_FOR_EACH_STATIC_PRIMITIVE_TYPE(F)        \
    QT_FOR_EACH_STATIC_PRIMITIVE_NON_VOID_TYPE(F)   \
    F(Void, 43, void) \

#define QT_FOR_EACH_STATIC_PRIMITIVE_POINTER(F)     \
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

#if QT_CONFIG(regularexpression)
#  define QT_FOR_EACH_STATIC_REGULAR_EXPRESSION(F) \
    F(QRegularExpression, 44, QRegularExpression)
#else
#  define QT_FOR_EACH_STATIC_REGULAR_EXPRESSION(F)
#endif

#define QT_FOR_EACH_STATIC_CORE_CLASS(F)\
    F(QChar, 7, QChar) \
    F(QString, 10, QString) \
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
    QT_FOR_EACH_STATIC_EASINGCURVE(F) \
    F(QUuid, 30, QUuid) \
    F(QVariant, 41, QVariant) \
    QT_FOR_EACH_STATIC_REGULAR_EXPRESSION(F) \
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
    F(QVariantPair, 58, QVariantPair) \
    F(QByteArrayList, 49, QByteArrayList) \
    F(QStringList, 11, QStringList) \

#if QT_CONFIG(shortcut)
#define QT_FOR_EACH_STATIC_KEYSEQUENCE_CLASS(F)\
    F(QKeySequence, 0x100b, QKeySequence)
#else
#define QT_FOR_EACH_STATIC_KEYSEQUENCE_CLASS(F)
#endif

#define QT_FOR_EACH_STATIC_GUI_CLASS(F)\
    F(QFont, 0x1000, QFont) \
    F(QPixmap, 0x1001, QPixmap) \
    F(QBrush, 0x1002, QBrush) \
    F(QColor, 0x1003, QColor) \
    F(QPalette, 0x1004, QPalette) \
    F(QIcon, 0x1005, QIcon) \
    F(QImage, 0x1006, QImage) \
    F(QPolygon, 0x1007, QPolygon) \
    F(QRegion, 0x1008, QRegion) \
    F(QBitmap, 0x1009, QBitmap) \
    F(QCursor, 0x100a, QCursor) \
    QT_FOR_EACH_STATIC_KEYSEQUENCE_CLASS(F) \
    F(QPen, 0x100c, QPen) \
    F(QTextLength, 0x100d, QTextLength) \
    F(QTextFormat, 0x100e, QTextFormat) \
    F(QTransform, 0x1010, QTransform) \
    F(QMatrix4x4, 0x1011, QMatrix4x4) \
    F(QVector2D, 0x1012, QVector2D) \
    F(QVector3D, 0x1013, QVector3D) \
    F(QVector4D, 0x1014, QVector4D) \
    F(QQuaternion, 0x1015, QQuaternion) \
    F(QPolygonF, 0x1016, QPolygonF) \
    F(QColorSpace, 0x1017, QColorSpace) \


#define QT_FOR_EACH_STATIC_WIDGETS_CLASS(F)\
    F(QSizePolicy, 0x2000, QSizePolicy) \

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
    F(QVariantPair, -1, QVariantPair, "QPair<QVariant,QVariant>") \
    F(QByteArrayList, -1, QByteArrayList, "QList<QByteArray>") \
    F(QStringList, -1, QStringList, "QList<QString>") \

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
    F(QQueue) \
    F(QStack) \
    F(QSet) \
    /*end*/

#define QT_FOR_EACH_AUTOMATIC_TEMPLATE_2ARG(F) \
    F(QHash, class) \
    F(QMap, class)

#define QT_FOR_EACH_AUTOMATIC_TEMPLATE_SMART_POINTER(F) \
    F(QSharedPointer) \
    F(QWeakPointer) \
    F(QPointer)

class QDataStream;
struct QMetaObject;

namespace QtPrivate
{

class QMetaTypeInterface
{
public:
    ushort revision; // 0 in Qt 6.0. Can increase if new field are added
    ushort alignment;
    uint size;
    uint flags;
    mutable QBasicAtomicInt typeId;

    using MetaObjectFn = const QMetaObject *(*)(const QMetaTypeInterface *);
    MetaObjectFn metaObjectFn;

    const char *name;

    using DefaultCtrFn = void (*)(const QMetaTypeInterface *, void *);
    DefaultCtrFn defaultCtr;
    using CopyCtrFn = void (*)(const QMetaTypeInterface *, void *, const void *);
    CopyCtrFn copyCtr;
    using MoveCtrFn = void (*)(const QMetaTypeInterface *, void *, void *);
    MoveCtrFn moveCtr;
    using DtorFn = void (*)(const QMetaTypeInterface *, void *);
    DtorFn dtor;
    using EqualsFn = bool (*)(const QMetaTypeInterface *, const void *, const void *);
    EqualsFn equals;
    using LessThanFn = bool (*)(const QMetaTypeInterface *, const void *, const void *);
    LessThanFn lessThan;
    using DebugStreamFn = void (*)(const QMetaTypeInterface *, QDebug &, const void *);
    DebugStreamFn debugStream;
    using DataStreamOutFn = void (*)(const QMetaTypeInterface *, QDataStream &, const void *);
    DataStreamOutFn dataStreamOut;
    using DataStreamInFn = void (*)(const QMetaTypeInterface *, QDataStream &, void *);
    DataStreamInFn dataStreamIn;

    using LegacyRegisterOp = void (*)();
    LegacyRegisterOp legacyRegisterOp;
};

/*!
    This template is used for implicit conversion from type From to type To.
    \internal
*/
template<typename From, typename To>
To convertImplicit(const From& from)
{
    return from;
}

    template<typename T, bool>
    struct SequentialValueTypeIsMetaType;
    template<typename T, bool>
    struct AssociativeValueTypeIsMetaType;
    template<typename T, bool>
    struct IsMetaTypePair;
    template<typename, typename>
    struct MetaTypeSmartPointerHelper;

    template<typename T>
    struct IsQFlags : std::false_type {};

    template<typename Enum>
    struct IsQFlags<QFlags<Enum>> : std::true_type {};

    template<typename T>
    struct IsEnumOrFlags : std::disjunction<std::is_enum<T>, IsQFlags<T>> {};
}  // namespace QtPrivate

class Q_CORE_EXPORT QMetaType {
public:
#ifndef Q_CLANG_QDOC
    // The code that actually gets compiled.
    enum Type {
        // these are merged with QVariant
        QT_FOR_EACH_STATIC_TYPE(QT_DEFINE_METATYPE_ID)

        FirstCoreType = Bool,
        LastCoreType = QVariantPair,
        FirstGuiType = QFont,
        LastGuiType = QColorSpace,
        FirstWidgetsType = QSizePolicy,
        LastWidgetsType = QSizePolicy,
        HighestInternalId = LastWidgetsType,

        QReal = sizeof(qreal) == sizeof(double) ? Double : Float,

        UnknownType = 0,
        User = 65536
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
        QLine = 23, QLineF = 24, QPoint = 25, QPointF = 26,
        QEasingCurve = 29, QUuid = 30, QVariant = 41, QModelIndex = 42,
        QPersistentModelIndex = 50, QRegularExpression = 44,
        QJsonValue = 45, QJsonObject = 46, QJsonArray = 47, QJsonDocument = 48,
        QByteArrayList = 49, QObjectStar = 39, SChar = 40,
        Void = 43,
        Nullptr = 51,
        QVariantMap = 8, QVariantList = 9, QVariantHash = 28,
        QCborSimpleType = 52, QCborValue = 53, QCborArray = 54, QCborMap = 55,
        Char16 = 56, Char32 = 57,

        // Gui types
        QFont = 0x1000, QPixmap = 0x1001, QBrush = 0x1002, QColor = 0x1003, QPalette = 0x1004,
        QIcon = 0x1005, QImage = 0x1006, QPolygon = 0x1007, QRegion = 0x1008, QBitmap = 0x1009,
        QCursor = 0x100a, QKeySequence = 0x100b, QPen = 0x100c, QTextLength = 0x100d, QTextFormat = 0x100e,
        QTransform = 0x1010, QMatrix4x4 = 0x1011, QVector2D = 0x1012,
        QVector3D = 0x1013, QVector4D = 0x1014, QQuaternion = 0x1015, QPolygonF = 0x1016, QColorSpace = 0x1017,

        // Widget types
        QSizePolicy = 0x2000,
        LastCoreType = Char32,
        LastGuiType = QColorSpace,
        User = 65536
    };
#endif

    enum TypeFlag {
        NeedsConstruction = 0x1,
        NeedsDestruction = 0x2,
        RelocatableType = 0x4,
#if QT_DEPRECATED_SINCE(6, 0)
        MovableType Q_DECL_ENUMERATOR_DEPRECATED_X("Use RelocatableType instead.") = RelocatableType,
#endif
        PointerToQObject = 0x8,
        IsEnumeration = 0x10,
        SharedPointerToQObject = 0x20,
        WeakPointerToQObject = 0x40,
        TrackingPointerToQObject = 0x80,
        IsUnsignedEnumeration = 0x100,
        IsGadget = 0x200,
        PointerToGadget = 0x400,
        IsPointer = 0x800,
        IsQmlList =0x1000, // used in the QML engine to recognize QQmlListProperty<T> and list<T>
        IsConst = 0x2000,
    };
    Q_DECLARE_FLAGS(TypeFlags, TypeFlag)

    static void registerNormalizedTypedef(const QT_PREPEND_NAMESPACE(QByteArray) &normalizedTypeName, QMetaType type);

#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_6_0
    static int type(const char *typeName)
    { return QMetaType::fromName(typeName).id(); }
    QT_DEPRECATED_VERSION_6_0
    static int type(const QT_PREPEND_NAMESPACE(QByteArray) &typeName)
    { return QMetaType::fromName(typeName).id(); }
    QT_DEPRECATED_VERSION_6_0
    static const char *typeName(int type)
    { return QMetaType(type).name(); }
    QT_DEPRECATED_VERSION_6_0
    static int sizeOf(int type)
    { return int(QMetaType(type).sizeOf()); }
    QT_DEPRECATED_VERSION_6_0
    static TypeFlags typeFlags(int type)
    { return QMetaType(type).flags(); }
    QT_DEPRECATED_VERSION_6_0
    static const QMetaObject *metaObjectForType(int type)
    { return QMetaType(type).metaObject(); }
    QT_DEPRECATED_VERSION_6_0
    static void *create(int type, const void *copy = nullptr)
    { return QMetaType(type).create(copy); }
    QT_DEPRECATED_VERSION_6_0
    static void destroy(int type, void *data)
    { return QMetaType(type).destroy(data); }
    QT_DEPRECATED_VERSION_6_0
    static void *construct(int type, void *where, const void *copy)
    { return QMetaType(type).construct(where, copy); }
    QT_DEPRECATED_VERSION_6_0
    static void destruct(int type, void *where)
    { return QMetaType(type).destruct(where); }
#endif
    static bool isRegistered(int type);

    explicit QMetaType(int type);
    explicit constexpr QMetaType(const QtPrivate::QMetaTypeInterface *d) : d_ptr(d) {}
    constexpr QMetaType() = default;

    bool isValid() const;
    bool isRegistered() const;
#if QT_CORE_REMOVED_SINCE(6, 1) || defined(Q_QDOC)
    int id() const;
#else
    // ### Qt 7: Remove traces of out of line version
    // unused int parameter is used to avoid ODR violation
    int id(int = 0) const
    {
        // keep in sync with the version in removed_api.cpp
        if (d_ptr) {
            if (int id = d_ptr->typeId.loadRelaxed())
                return id;
            return idHelper();
        }
        return 0;
    }
#endif
    constexpr qsizetype sizeOf() const;
    constexpr qsizetype alignOf() const;
    constexpr TypeFlags flags() const;
    constexpr const QMetaObject *metaObject() const;
    constexpr const char *name() const;

    void *create(const void *copy = nullptr) const;
    void destroy(void *data) const;
    void *construct(void *where, const void *copy = nullptr) const;
    void destruct(void *data) const;
    QPartialOrdering compare(const void *lhs, const void *rhs) const;
    bool equals(const void *lhs, const void *rhs) const;

    bool isEqualityComparable() const;
    bool isOrdered() const;

#ifndef QT_NO_DATASTREAM
    bool save(QDataStream &stream, const void *data) const;
    bool load(QDataStream &stream, void *data) const;
    bool hasRegisteredDataStreamOperators() const;

#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_6_0
    static bool save(QDataStream &stream, int type, const void *data)
    { return QMetaType(type).save(stream, data); }
    QT_DEPRECATED_VERSION_6_0
    static bool load(QDataStream &stream, int type, void *data)
    { return QMetaType(type).load(stream, data); }
#endif
#endif

    template<typename T>
    constexpr static QMetaType fromType();
    static QMetaType fromName(QByteArrayView name);

    friend bool operator==(QMetaType a, QMetaType b)
    {
        if (a.d_ptr == b.d_ptr)
            return true;
        if (!a.d_ptr || !b.d_ptr)
            return false; // one type is undefined, the other is defined
        // avoid id call if we already have the id
        const int aId = a.id();
        const int bId = b.id();
        return aId == bId;
    }
    friend bool operator!=(QMetaType a, QMetaType b) { return !(a == b); }

public:

#ifndef QT_NO_DEBUG_STREAM
    bool debugStream(QDebug& dbg, const void *rhs);
    bool hasRegisteredDebugStreamOperator() const;

#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_6_0
    static bool debugStream(QDebug& dbg, const void *rhs, int typeId)
    { return QMetaType(typeId).debugStream(dbg, rhs); }
    template<typename T>
    QT_DEPRECATED_VERSION_6_0
    static bool hasRegisteredDebugStreamOperator()
    { return QMetaType::fromType<T>().hasRegisteredDebugStreamOperator(); }
    QT_DEPRECATED_VERSION_6_0
    static bool hasRegisteredDebugStreamOperator(int typeId)
    { return QMetaType(typeId).hasRegisteredDebugStreamOperator(); }
#endif
#endif

    // type erased converter function
    using ConverterFunction = std::function<bool(const void *src, void *target)>;

    // type erased mutable view, primarily for containers
    using MutableViewFunction = std::function<bool(void *src, void *target)>;

    // implicit conversion supported like double -> float
    template<typename From, typename To>
    static bool registerConverter()
    {
        return registerConverter<From, To>(QtPrivate::convertImplicit<From, To>);
    }

    // member function as in "QString QFont::toString() const"
    template<typename From, typename To>
    static bool registerConverter(To(From::*function)() const)
    {
        static_assert((!QMetaTypeId2<To>::IsBuiltIn || !QMetaTypeId2<From>::IsBuiltIn),
            "QMetaType::registerConverter: At least one of the types must be a custom type.");

        const QMetaType fromType = QMetaType::fromType<From>();
        const QMetaType toType = QMetaType::fromType<To>();
        auto converter = [function](const void *from, void *to) -> bool {
            const From *f = static_cast<const From *>(from);
            To *t = static_cast<To *>(to);
            *t = (f->*function)();
            return true;
        };
        return registerConverterImpl<From, To>(converter, fromType, toType);
    }

    // member function
    template<typename From, typename To>
    static bool registerMutableView(To(From::*function)())
    {
        static_assert((!QMetaTypeId2<To>::IsBuiltIn || !QMetaTypeId2<From>::IsBuiltIn),
            "QMetaType::registerMutableView: At least one of the types must be a custom type.");

        const QMetaType fromType = QMetaType::fromType<From>();
        const QMetaType toType = QMetaType::fromType<To>();
        auto view = [function](void *from, void *to) -> bool {
            From *f = static_cast<From *>(from);
            To *t = static_cast<To *>(to);
            *t = (f->*function)();
            return true;
        };
        return registerMutableViewImpl<From, To>(view, fromType, toType);
    }

    // member function as in "double QString::toDouble(bool *ok = nullptr) const"
    template<typename From, typename To>
    static bool registerConverter(To(From::*function)(bool*) const)
    {
        static_assert((!QMetaTypeId2<To>::IsBuiltIn || !QMetaTypeId2<From>::IsBuiltIn),
            "QMetaType::registerConverter: At least one of the types must be a custom type.");

        const QMetaType fromType = QMetaType::fromType<From>();
        const QMetaType toType = QMetaType::fromType<To>();
        auto converter = [function](const void *from, void *to) -> bool {
            const From *f = static_cast<const From *>(from);
            To *t = static_cast<To *>(to);
            bool result = true;
            *t = (f->*function)(&result);
            if (!result)
                *t = To();
            return result;
        };
        return registerConverterImpl<From, To>(converter, fromType, toType);
    }

    // functor or function pointer
    template<typename From, typename To, typename UnaryFunction>
    static bool registerConverter(UnaryFunction function)
    {
        static_assert((!QMetaTypeId2<To>::IsBuiltIn || !QMetaTypeId2<From>::IsBuiltIn),
            "QMetaType::registerConverter: At least one of the types must be a custom type.");

        const QMetaType fromType = QMetaType::fromType<From>();
        const QMetaType toType = QMetaType::fromType<To>();
        auto converter = [function = std::move(function)](const void *from, void *to) -> bool {
            const From *f = static_cast<const From *>(from);
            To *t = static_cast<To *>(to);
            *t = function(*f);
            return true;
        };
        return registerConverterImpl<From, To>(std::move(converter), fromType, toType);
    }

    // functor or function pointer
    template<typename From, typename To, typename UnaryFunction>
    static bool registerMutableView(UnaryFunction function)
    {
        static_assert((!QMetaTypeId2<To>::IsBuiltIn || !QMetaTypeId2<From>::IsBuiltIn),
            "QMetaType::registerMutableView: At least one of the types must be a custom type.");

        const QMetaType fromType = QMetaType::fromType<From>();
        const QMetaType toType = QMetaType::fromType<To>();
        auto view = [function = std::move(function)](void *from, void *to) -> bool {
            From *f = static_cast<From *>(from);
            To *t = static_cast<To *>(to);
            *t = function(*f);
            return true;
        };
        return registerMutableViewImpl<From, To>(std::move(view), fromType, toType);
    }

private:
    template<typename From, typename To>
    static bool registerConverterImpl(ConverterFunction converter, QMetaType fromType, QMetaType toType)
    {
        if (registerConverterFunction(std::move(converter), fromType, toType)) {
            static const auto unregister = qScopeGuard([=] {
                unregisterConverterFunction(fromType, toType);
            });
            return true;
        } else {
            return false;
        }
    }

    template<typename From, typename To>
    static bool registerMutableViewImpl(MutableViewFunction view, QMetaType fromType, QMetaType toType)
    {
        if (registerMutableViewFunction(std::move(view), fromType, toType)) {
            static const auto unregister = qScopeGuard([=] {
               unregisterMutableViewFunction(fromType, toType);
            });
            return true;
        } else {
            return false;
        }
    }
public:

    static bool convert(QMetaType fromType, const void *from, QMetaType toType, void *to);
    static bool canConvert(QMetaType fromType, QMetaType toType);

    static bool view(QMetaType fromType, void *from, QMetaType toType, void *to);
    static bool canView(QMetaType fromType, QMetaType toType);
#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_6_0
    static bool convert(const void *from, int fromTypeId, void *to, int toTypeId)
    { return convert(QMetaType(fromTypeId), from, QMetaType(toTypeId), to); }
    QT_DEPRECATED_VERSION_6_0
    static bool compare(const void *lhs, const void *rhs, int typeId, int *result)
    {
        QMetaType t(typeId);
        auto c = t.compare(lhs, rhs);
        if (c == QPartialOrdering::Unordered) {
            *result = 0;
            return false;
        } else if (c == QPartialOrdering::Less) {
            *result = -1;
            return true;
        } else if (c == QPartialOrdering::Equivalent) {
            *result = 0;
            return true;
        } else {
            *result = 1;
            return true;
        }
    }
    QT_DEPRECATED_VERSION_6_0
    static bool equals(const void *lhs, const void *rhs, int typeId, int *result)
    {
        QMetaType t(typeId);
        if (!t.isEqualityComparable())
            return false;
        *result = t.equals(lhs, rhs) ? 0 : -1;
        return true;
    }
#endif

    template<typename From, typename To>
    static bool hasRegisteredConverterFunction()
    {
        return hasRegisteredConverterFunction(
                    QMetaType::fromType<From>(), QMetaType::fromType<To>());
    }

    static bool hasRegisteredConverterFunction(QMetaType fromType, QMetaType toType);

    template<typename From, typename To>
    static bool hasRegisteredMutableViewFunction()
    {
        return hasRegisteredMutableViewFunction(
                    QMetaType::fromType<From>(), QMetaType::fromType<To>());
    }

    static bool hasRegisteredMutableViewFunction(QMetaType fromType, QMetaType toType);

#ifndef Q_CLANG_QDOC
    template<typename, bool> friend struct QtPrivate::SequentialValueTypeIsMetaType;
    template<typename, bool> friend struct QtPrivate::AssociativeValueTypeIsMetaType;
    template<typename, bool> friend struct QtPrivate::IsMetaTypePair;
    template<typename, typename> friend struct QtPrivate::MetaTypeSmartPointerHelper;
#endif
    static bool registerConverterFunction(const ConverterFunction &f, QMetaType from, QMetaType to);
    static void unregisterConverterFunction(QMetaType from, QMetaType to);

    static bool registerMutableViewFunction(const MutableViewFunction &f, QMetaType from, QMetaType to);
    static void unregisterMutableViewFunction(QMetaType from, QMetaType to);

    static void unregisterMetaType(QMetaType type);

#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0)
    const QtPrivate::QMetaTypeInterface *iface() { return d_ptr; }
#endif
    const QtPrivate::QMetaTypeInterface *iface() const { return d_ptr; }

private:
    int idHelper() const;
    friend class QVariant;
    const QtPrivate::QMetaTypeInterface *d_ptr = nullptr;
};

#undef QT_DEFINE_METATYPE_ID

Q_DECLARE_OPERATORS_FOR_FLAGS(QMetaType::TypeFlags)

#define QT_METATYPE_PRIVATE_DECLARE_TYPEINFO(C, F)  \
    }                                               \
    Q_DECLARE_TYPEINFO(QtMetaTypePrivate:: C, (F)); \
    namespace QtMetaTypePrivate {


namespace QtMetaTypePrivate {

class QPairVariantInterfaceImpl
{
public:
    const void *_pair;
    QMetaType _metaType_first;
    QMetaType _metaType_second;

    typedef void (*getFunc)(const void * const *p, void *);

    getFunc _getFirst;
    getFunc _getSecond;

    template<class T>
    static void getFirstImpl(const void * const *pair, void *dataPtr)
    { *static_cast<typename T::first_type *>(dataPtr) = static_cast<const T*>(*pair)->first; }
    template<class T>
    static void getSecondImpl(const void * const *pair, void *dataPtr)
    { *static_cast<typename T::second_type *>(dataPtr) = static_cast<const T*>(*pair)->second; }

public:
    template<class T> QPairVariantInterfaceImpl(const T*p)
      : _pair(p)
      , _metaType_first(QMetaType::fromType<typename T::first_type>())
      , _metaType_second(QMetaType::fromType<typename T::second_type>())
      , _getFirst(getFirstImpl<T>)
      , _getSecond(getSecondImpl<T>)
    {
    }

    constexpr QPairVariantInterfaceImpl()
      : _pair(nullptr)
      , _getFirst(nullptr)
      , _getSecond(nullptr)
    {
    }

    inline void first(void *dataPtr) const { _getFirst(&_pair, dataPtr); }
    inline void second(void *dataPtr) const { _getSecond(&_pair, dataPtr); }
};
QT_METATYPE_PRIVATE_DECLARE_TYPEINFO(QPairVariantInterfaceImpl, Q_RELOCATABLE_TYPE)

template<typename From>
struct QPairVariantInterfaceConvertFunctor;

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

#define QT_FORWARD_DECLARE_SHARED_POINTER_TYPES_ITER(Name) \
    template <class T> class Name; \

QT_FOR_EACH_AUTOMATIC_TEMPLATE_SMART_POINTER(QT_FORWARD_DECLARE_SHARED_POINTER_TYPES_ITER)

namespace QtPrivate
{
    namespace detail {
    template<typename T, typename ODR_VIOLATION_PREVENTER>
    struct is_complete_helper
    {
        template<typename U>
        static auto check(U *) -> std::integral_constant<bool, sizeof(U) != 0>;
        static auto check(...) -> std::false_type;
        using type = decltype(check(static_cast<T *>(nullptr)));
    };
    } // namespace detail

    template <typename T, typename ODR_VIOLATION_PREVENTER>
    struct is_complete : detail::is_complete_helper<std::remove_reference_t<T>, ODR_VIOLATION_PREVENTER>::type {};

    template <typename T> struct MetatypeDecay              { using type = T; };
    template <typename T> struct MetatypeDecay<const T>     { using type = T; };
    template <typename T> struct MetatypeDecay<const T &>   { using type = T; };

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
        static yes_type checkType(const QObject* );
#endif
        static no_type checkType(...);
        static_assert(sizeof(T), "Type argument of Q_PROPERTY or Q_DECLARE_METATYPE(T*) must be fully defined");
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
        static constexpr const QMetaObject *value() { return nullptr; }
        using MetaObjectFn = const QMetaObject *(*)(const QMetaTypeInterface *);
        static constexpr MetaObjectFn metaObjectFunction = nullptr;
    };
#ifndef QT_NO_QOBJECT
    template<typename T>
    struct MetaObjectForType<T*, typename std::enable_if<IsPointerToTypeDerivedFromQObject<T*>::Value>::type>
    {
        static constexpr const QMetaObject *value() { return &T::staticMetaObject; }
        static constexpr const QMetaObject *metaObjectFunction(const QMetaTypeInterface *) { return &T::staticMetaObject; }
    };
    template<typename T>
    struct MetaObjectForType<T, typename std::enable_if<IsGadgetHelper<T>::IsGadgetOrDerivedFrom>::type>
    {
        static constexpr const QMetaObject *value() { return &T::staticMetaObject; }
        static constexpr const QMetaObject *metaObjectFunction(const QMetaTypeInterface *) { return &T::staticMetaObject; }
    };
    template<typename T>
    struct MetaObjectForType<T, typename std::enable_if<IsPointerToGadgetHelper<T>::IsGadgetOrDerivedFrom>::type>
    {
        static constexpr const QMetaObject *value()
        {
            return &IsPointerToGadgetHelper<T>::BaseType::staticMetaObject;
        }
        static constexpr const QMetaObject *metaObjectFunction(const QMetaTypeInterface *) { return value(); }
    };
    template<typename T>
    struct MetaObjectForType<T, typename std::enable_if<IsQEnumHelper<T>::Value>::type >
    {
        static constexpr const QMetaObject *value() { return qt_getEnumMetaObject(T()); }
        static constexpr const QMetaObject *metaObjectFunction(const QMetaTypeInterface *) { return value(); }
    };
#endif

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
    struct SequentialContainerTransformationHelper
    {
        static bool registerConverter()
        {
            return false;
        }

        static bool registerMutableView()
        {
            return false;
        }
    };

    template<typename T, bool = QMetaTypeId2<typename T::value_type>::Defined>
    struct SequentialValueTypeIsMetaType
    {
        static bool registerConverter()
        {
            return false;
        }

        static bool registerMutableView()
        {
            return false;
        }
    };

    template<typename T>
    struct SequentialContainerTransformationHelper<T, true> : SequentialValueTypeIsMetaType<T>
    {
    };

    template<typename T, bool = QtPrivate::IsAssociativeContainer<T>::Value>
    struct AssociativeContainerTransformationHelper
    {
        static bool registerConverter()
        {
            return false;
        }

        static bool registerMutableView()
        {
            return false;
        }
    };

    template<typename T, bool = QMetaTypeId2<typename T::key_type>::Defined>
    struct AssociativeKeyTypeIsMetaType
    {
        static bool registerConverter()
        {
            return false;
        }

        static bool registerMutableView()
        {
            return false;
        }
    };

    template<typename T, bool = QMetaTypeId2<typename T::mapped_type>::Defined>
    struct AssociativeMappedTypeIsMetaType
    {
        static bool registerConverter()
        {
            return false;
        }

        static bool registerMutableView()
        {
            return false;
        }
    };

    template<typename T>
    struct AssociativeContainerTransformationHelper<T, true> : AssociativeKeyTypeIsMetaType<T>
    {
    };

    template<typename T, bool = QMetaTypeId2<typename T::first_type>::Defined
                                && QMetaTypeId2<typename T::second_type>::Defined>
    struct IsMetaTypePair
    {
        static bool registerConverter()
        {
            return false;
        }
    };

    template<typename T>
    struct IsMetaTypePair<T, true>
    {
        inline static bool registerConverter();
    };

    template<typename T>
    struct IsPair
    {
        static bool registerConverter()
        {
            return false;
        }
    };
    template<typename T, typename U>
    struct IsPair<std::pair<T, U> > : IsMetaTypePair<std::pair<T, U> > {};

    template<typename T>
    struct MetaTypePairHelper : IsPair<T> {};

    template<typename T, typename = void>
    struct MetaTypeSmartPointerHelper
    {
        static bool registerConverter() { return false; }
    };

#if QT_CONFIG(future)
    template<typename T>
    struct MetaTypeQFutureHelper
    {
        static bool registerConverter() { return false; }
    };
#endif

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
    using NameAsArrayType = void;
    enum { Defined = QMetaTypeId<T>::Defined, IsBuiltIn=false };
    static inline constexpr int qt_metatype_id() { return QMetaTypeId<T>::qt_metatype_id(); }
};

template <typename T>
struct QMetaTypeId2<const T&> : QMetaTypeId2<T> {};

template <typename T>
struct QMetaTypeId2<T&>
{
    using NameAsArrayType = void;
    enum { Defined = false, IsBuiltIn = false };
    static inline constexpr int qt_metatype_id() { return 0; }
};

namespace QtPrivate {
    template <typename T, bool Defined = QMetaTypeId2<T>::Defined>
    struct QMetaTypeIdHelper {
        static inline constexpr int qt_metatype_id()
        { return QMetaTypeId2<T>::qt_metatype_id(); }
    };
    template <typename T> struct QMetaTypeIdHelper<T, false> {
        static inline constexpr int qt_metatype_id()
        { return -1; }
    };

    // Function pointers don't derive from QObject
    template <typename Result, typename... Args>
    struct IsPointerToTypeDerivedFromQObject<Result(*)(Args...)> { enum { Value = false }; };

    template<typename T>
    inline constexpr bool IsQmlListType = false;

    template<typename T, bool = std::is_enum<T>::value>
    constexpr bool IsUnsignedEnum = false;
    template<typename T>
    constexpr bool IsUnsignedEnum<T, true> = !std::is_signed_v<std::underlying_type_t<T>>;

    template<typename T>
    struct QMetaTypeTypeFlags
    {
        enum { Flags = (QTypeInfo<T>::isRelocatable ? QMetaType::RelocatableType : 0)
                     | (QTypeInfo<T>::isComplex ? QMetaType::NeedsConstruction : 0)
                     | (QTypeInfo<T>::isComplex ? QMetaType::NeedsDestruction : 0)
                     | (IsPointerToTypeDerivedFromQObject<T>::Value ? QMetaType::PointerToQObject : 0)
                     | (IsSharedPointerToTypeDerivedFromQObject<T>::Value ? QMetaType::SharedPointerToQObject : 0)
                     | (IsWeakPointerToTypeDerivedFromQObject<T>::Value ? QMetaType::WeakPointerToQObject : 0)
                     | (IsTrackingPointerToTypeDerivedFromQObject<T>::Value ? QMetaType::TrackingPointerToQObject : 0)
                     | (IsEnumOrFlags<T>::value ? QMetaType::IsEnumeration : 0)
                     | (IsGadgetHelper<T>::IsGadgetOrDerivedFrom ? QMetaType::IsGadget : 0)
                     | (IsPointerToGadgetHelper<T>::IsGadgetOrDerivedFrom ? QMetaType::PointerToGadget : 0)
                     | (QTypeInfo<T>::isPointer ? QMetaType::IsPointer : 0)
                     | (IsUnsignedEnum<T> ? QMetaType::IsUnsignedEnumeration : 0)
                     | (IsQmlListType<T> ? QMetaType::IsQmlList : 0)
                     | (std::is_const_v<std::remove_pointer_t<T>> ? QMetaType::IsConst : 0)
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
int qRegisterNormalizedMetaTypeImplementation(const QT_PREPEND_NAMESPACE(QByteArray) &normalizedTypeName)
{
#ifndef QT_NO_QOBJECT
    Q_ASSERT_X(normalizedTypeName == QMetaObject::normalizedType(normalizedTypeName.constData()),
               "qRegisterNormalizedMetaType",
               "qRegisterNormalizedMetaType was called with a not normalized type name, "
               "please call qRegisterMetaType instead.");
#endif

    const QMetaType metaType = QMetaType::fromType<T>();
    const int id = metaType.id();

    QtPrivate::SequentialContainerTransformationHelper<T>::registerConverter();
    QtPrivate::SequentialContainerTransformationHelper<T>::registerMutableView();
    QtPrivate::AssociativeContainerTransformationHelper<T>::registerConverter();
    QtPrivate::AssociativeContainerTransformationHelper<T>::registerMutableView();
    QtPrivate::MetaTypePairHelper<T>::registerConverter();
    QtPrivate::MetaTypeSmartPointerHelper<T>::registerConverter();
#if QT_CONFIG(future)
    QtPrivate::MetaTypeQFutureHelper<T>::registerConverter();
#endif

    if (normalizedTypeName != metaType.name())
        QMetaType::registerNormalizedTypedef(normalizedTypeName, metaType);

    return id;
}

// This primary template calls the -Implementation, like all other specialisations should.
// But the split allows to
// - in a header:
//   - define a specialization of this template calling an out-of-line function
//     (QT_DECL_METATYPE_EXTERN{,_TAGGED})
// - in the .cpp file:
//   - define the out-of-line wrapper to call the -Implementation
//     (QT_IMPL_METATYPE_EXTERN{,_TAGGED})
// The _TAGGED variants let you choose a tag (must be a C identifier) to disambiguate
// the out-of-line function; the non-_TAGGED variants use the passed class name as tag.
template <typename T>
int qRegisterNormalizedMetaType(const QT_PREPEND_NAMESPACE(QByteArray) &normalizedTypeName)
{
    return qRegisterNormalizedMetaTypeImplementation<T>(normalizedTypeName);
}

#define QT_DECL_METATYPE_EXTERN_TAGGED(TYPE, TAG, EXPORT) \
    QT_BEGIN_NAMESPACE \
    EXPORT int qRegisterNormalizedMetaType_ ## TAG (const QByteArray &); \
    template <> inline int qRegisterNormalizedMetaType< TYPE >(const QByteArray &name) \
    { return qRegisterNormalizedMetaType_ ## TAG (name); } \
    QT_END_NAMESPACE \
    Q_DECLARE_METATYPE(TYPE) \
    /* end */
#define QT_IMPL_METATYPE_EXTERN_TAGGED(TYPE, TAG) \
    int qRegisterNormalizedMetaType_ ## TAG (const QByteArray &name) \
    { return qRegisterNormalizedMetaTypeImplementation< TYPE >(name); } \
    /* end */
#define QT_DECL_METATYPE_EXTERN(TYPE, EXPORT) \
    QT_DECL_METATYPE_EXTERN_TAGGED(TYPE, TYPE, EXPORT)
#define QT_IMPL_METATYPE_EXTERN(TYPE) \
    QT_IMPL_METATYPE_EXTERN_TAGGED(TYPE, TYPE)

template <typename T>
int qRegisterMetaType(const char *typeName)
{
#ifdef QT_NO_QOBJECT
    QT_PREPEND_NAMESPACE(QByteArray) normalizedTypeName = typeName;
#else
    QT_PREPEND_NAMESPACE(QByteArray) normalizedTypeName = QMetaObject::normalizedType(typeName);
#endif
    return qRegisterNormalizedMetaType<T>(normalizedTypeName);
}

template <typename T>
inline constexpr int qMetaTypeId()
{
    if constexpr (bool(QMetaTypeId2<T>::IsBuiltIn)) {
        return QMetaTypeId2<T>::MetaType;
    } else {
        return QMetaType::fromType<T>().id();
    }
}

template <typename T>
inline constexpr int qRegisterMetaType()
{
    int id = qMetaTypeId<T>();
    return id;
}

#ifndef QT_NO_QOBJECT
template <typename T>
struct QMetaTypeIdQObject<T*, QMetaType::PointerToQObject>
{
    enum {
        Defined = 1
    };

    static int qt_metatype_id()
    {
        Q_CONSTINIT static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0);
        if (const int id = metatype_id.loadAcquire())
            return id;
        const char *const cName = T::staticMetaObject.className();
        QByteArray typeName;
        typeName.reserve(strlen(cName) + 1);
        typeName.append(cName).append('*');
        const int newId = qRegisterNormalizedMetaType<T *>(typeName);
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
        Q_CONSTINIT static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0);
        if (const int id = metatype_id.loadAcquire())
            return id;
        const char *const cName = T::staticMetaObject.className();
        const int newId = qRegisterNormalizedMetaType<T>(cName);
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
        Q_CONSTINIT static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0);
        if (const int id = metatype_id.loadAcquire())
            return id;
        const char *const cName = T::staticMetaObject.className();
        QByteArray typeName;
        typeName.reserve(strlen(cName) + 1);
        typeName.append(cName).append('*');
        const int newId = qRegisterNormalizedMetaType<T *>(typeName);
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
        Q_CONSTINIT static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0);
        if (const int id = metatype_id.loadAcquire())
            return id;
        const char *eName = qt_getEnumName(T());
        const char *cName = qt_getEnumMetaObject(T())->className();
        QByteArray typeName;
        typeName.reserve(strlen(cName) + 2 + strlen(eName));
        typeName.append(cName).append("::").append(eName);
        const int newId = qRegisterNormalizedMetaType<T>(typeName);
        metatype_id.storeRelease(newId);
        return newId;
    }
};
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
                Q_CONSTINIT static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0); \
                if (const int id = metatype_id.loadAcquire())           \
                    return id;                                          \
                constexpr auto arr = QtPrivate::typenameHelper<TYPE>(); \
                auto name = arr.data();                                 \
                if (QByteArrayView(name) == (#TYPE)) {                  \
                    const int id = qRegisterNormalizedMetaType<TYPE>(name); \
                    metatype_id.storeRelease(id);                       \
                    return id;                                          \
                }                                                       \
                const int newId = qRegisterMetaType< TYPE >(#TYPE);     \
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
        using NameAsArrayType = std::array<char, sizeof(#NAME)>; \
        enum { Defined = 1, IsBuiltIn = true, MetaType = METATYPEID };   \
        static inline constexpr int qt_metatype_id() { return METATYPEID; } \
        static constexpr NameAsArrayType nameAsArray = { #NAME }; \
    }; \
    QT_END_NAMESPACE

#define QT_FORWARD_DECLARE_STATIC_TYPES_ITER(TypeName, TypeId, Name) \
    class Name;

QT_FOR_EACH_STATIC_CORE_CLASS(QT_FORWARD_DECLARE_STATIC_TYPES_ITER)
QT_FOR_EACH_STATIC_GUI_CLASS(QT_FORWARD_DECLARE_STATIC_TYPES_ITER)
QT_FOR_EACH_STATIC_WIDGETS_CLASS(QT_FORWARD_DECLARE_STATIC_TYPES_ITER)

#undef QT_FORWARD_DECLARE_STATIC_TYPES_ITER

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
        Q_CONSTINIT static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0); \
        if (const int id = metatype_id.loadRelaxed()) \
            return id; \
        const char *tName = QMetaType::fromType<T>().name(); \
        Q_ASSERT(tName); \
        const size_t tNameLen = qstrlen(tName); \
        QByteArray typeName; \
        typeName.reserve(sizeof(#SINGLE_ARG_TEMPLATE) + 1 + tNameLen + 1 + 1); \
        typeName.append(#SINGLE_ARG_TEMPLATE, int(sizeof(#SINGLE_ARG_TEMPLATE)) - 1) \
            .append('<').append(tName, tNameLen); \
        typeName.append('>'); \
        const int newId = qRegisterNormalizedMetaType< SINGLE_ARG_TEMPLATE<T> >(typeName); \
        metatype_id.storeRelease(newId); \
        return newId; \
    } \
}; \
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
        Q_CONSTINIT static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0); \
        if (const int id = metatype_id.loadAcquire()) \
            return id; \
        const char *tName = QMetaType::fromType<T>().name(); \
        const char *uName = QMetaType::fromType<U>().name(); \
        Q_ASSERT(tName); \
        Q_ASSERT(uName); \
        const size_t tNameLen = qstrlen(tName); \
        const size_t uNameLen = qstrlen(uName); \
        QByteArray typeName; \
        typeName.reserve(sizeof(#DOUBLE_ARG_TEMPLATE) + 1 + tNameLen + 1 + uNameLen + 1 + 1); \
        typeName.append(#DOUBLE_ARG_TEMPLATE, int(sizeof(#DOUBLE_ARG_TEMPLATE)) - 1) \
            .append('<').append(tName, tNameLen).append(',').append(uName, uNameLen); \
        typeName.append('>'); \
        const int newId = qRegisterNormalizedMetaType< DOUBLE_ARG_TEMPLATE<T, U> >(typeName); \
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
        Q_CONSTINIT static QBasicAtomicInt metatype_id = Q_BASIC_ATOMIC_INITIALIZER(0); \
        if (const int id = metatype_id.loadAcquire()) \
            return id; \
        const char * const cName = T::staticMetaObject.className(); \
        QByteArray typeName; \
        typeName.reserve(sizeof(#SMART_POINTER) + 1 + strlen(cName) + 1); \
        typeName.append(#SMART_POINTER, int(sizeof(#SMART_POINTER)) - 1) \
            .append('<').append(cName).append('>'); \
        const int newId = qRegisterNormalizedMetaType< SMART_POINTER<T> >(typeName); \
        metatype_id.storeRelease(newId); \
        return newId; \
    } \
}; \
template<typename T> \
struct MetaTypeSmartPointerHelper<SMART_POINTER<T> , \
        typename std::enable_if<IsPointerToTypeDerivedFromQObject<T*>::Value && !std::is_const_v<T>>::type> \
{ \
    static bool registerConverter() \
    { \
        const QMetaType to = QMetaType(QMetaType::QObjectStar); \
        if (!QMetaType::hasRegisteredConverterFunction(QMetaType::fromType<SMART_POINTER<T>>(), to)) { \
            QtPrivate::QSmartPointerConvertFunctor<SMART_POINTER<T> > o; \
            return QMetaType::registerConverter<SMART_POINTER<T>, QObject*>(o); \
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

#define Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE(SINGLE_ARG_TEMPLATE) \
    QT_BEGIN_NAMESPACE \
    namespace QtPrivate { \
    template<typename T> \
    struct IsSequentialContainer<SINGLE_ARG_TEMPLATE<T> > \
    { \
        enum { Value = true }; \
    }; \
    } \
    QT_END_NAMESPACE \
    Q_DECLARE_METATYPE_TEMPLATE_1ARG(SINGLE_ARG_TEMPLATE)

#define Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE_ITER(TEMPLATENAME) \
    Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE(TEMPLATENAME)

QT_END_NAMESPACE

QT_FOR_EACH_AUTOMATIC_TEMPLATE_1ARG(Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE_ITER)

#undef Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE_ITER

Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE(std::vector)
Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE(std::list)

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

Q_DECLARE_METATYPE_TEMPLATE_2ARG(std::pair)

#define Q_DECLARE_METATYPE_TEMPLATE_SMART_POINTER_ITER(TEMPLATENAME) \
    Q_DECLARE_SMART_POINTER_METATYPE(TEMPLATENAME)

QT_FOR_EACH_AUTOMATIC_TEMPLATE_SMART_POINTER(Q_DECLARE_METATYPE_TEMPLATE_SMART_POINTER_ITER)

QT_BEGIN_NAMESPACE

#undef Q_DECLARE_METATYPE_TEMPLATE_SMART_POINTER_ITER

QT_END_NAMESPACE

QT_FOR_EACH_STATIC_TYPE(Q_DECLARE_BUILTIN_METATYPE)


QT_BEGIN_NAMESPACE

template <typename T>
inline bool QtPrivate::IsMetaTypePair<T, true>::registerConverter()
{
    const QMetaType to = QMetaType::fromType<QtMetaTypePrivate::QPairVariantInterfaceImpl>();
    if (!QMetaType::hasRegisteredConverterFunction(QMetaType::fromType<T>(), to)) {
        QtMetaTypePrivate::QPairVariantInterfaceConvertFunctor<T> o;
        return QMetaType::registerConverter<T, QtMetaTypePrivate::QPairVariantInterfaceImpl>(o);
    }
    return true;
}

namespace QtPrivate {

template<typename From>
struct QSequentialIterableConvertFunctor
{
    QIterable<QMetaSequence> operator()(const From &f) const
    {
        return QIterable<QMetaSequence>(QMetaSequence::fromContainer<From>(), &f);
    }
};

template<typename From>
struct QSequentialIterableMutableViewFunctor
{
    QIterable<QMetaSequence> operator()(From &f) const
    {
        return QIterable<QMetaSequence>(QMetaSequence::fromContainer<From>(), &f);
    }
};

template<typename T>
struct SequentialValueTypeIsMetaType<T, true>
{
    static bool registerConverter()
    {
        const QMetaType to = QMetaType::fromType<QIterable<QMetaSequence>>();
        if (!QMetaType::hasRegisteredConverterFunction(QMetaType::fromType<T>(), to)) {
            QSequentialIterableConvertFunctor<T> o;
            return QMetaType::registerConverter<T, QIterable<QMetaSequence>>(o);
        }
        return true;
    }

    static bool registerMutableView()
    {
        const QMetaType to = QMetaType::fromType<QIterable<QMetaSequence>>();
        if (!QMetaType::hasRegisteredMutableViewFunction(QMetaType::fromType<T>(), to)) {
            QSequentialIterableMutableViewFunctor<T> o;
            return QMetaType::registerMutableView<T, QIterable<QMetaSequence>>(o);
        }
        return true;
    }
};

template<typename From>
struct QAssociativeIterableConvertFunctor
{
    QIterable<QMetaAssociation> operator()(const From &f) const
    {
        return QIterable<QMetaAssociation>(QMetaAssociation::fromContainer<From>(), &f);
    }
};

template<typename From>
struct QAssociativeIterableMutableViewFunctor
{
    QIterable<QMetaAssociation> operator()(From &f) const
    {
        return QIterable<QMetaAssociation>(QMetaAssociation::fromContainer<From>(), &f);
    }
};

// Mapped type can be omitted, for example in case of a set.
// However, if it is available, we want to instantiate the metatype here.
template<typename T>
struct AssociativeKeyTypeIsMetaType<T, true> : AssociativeMappedTypeIsMetaType<T>
{
    static bool registerConverter()
    {
        const QMetaType to = QMetaType::fromType<QIterable<QMetaAssociation>>();
        if (!QMetaType::hasRegisteredConverterFunction(QMetaType::fromType<T>(), to)) {
            QAssociativeIterableConvertFunctor<T> o;
            return QMetaType::registerConverter<T, QIterable<QMetaAssociation>>(o);
        }
        return true;
    }

    static bool registerMutableView()
    {
        const QMetaType to = QMetaType::fromType<QIterable<QMetaAssociation>>();
        if (!QMetaType::hasRegisteredMutableViewFunction(QMetaType::fromType<T>(), to)) {
            QAssociativeIterableMutableViewFunctor<T> o;
            return QMetaType::registerMutableView<T, QIterable<QMetaAssociation>>(o);
        }
        return true;
    }
};

struct QTypeNormalizer
{
    char *output;
    int len = 0;
    char last = 0;

private:
    static constexpr bool is_ident_char(char s)
    {
        return ((s >= 'a' && s <= 'z') || (s >= 'A' && s <= 'Z') || (s >= '0' && s <= '9')
                || s == '_');
    }
    static constexpr bool is_space(char s) { return (s == ' ' || s == '\t' || s == '\n'); }
    static constexpr bool is_number(char s) { return s >= '0' && s <= '9'; }
    static constexpr bool starts_with_token(const char *b, const char *e, const char *token,
                                            bool msvcKw = false)
    {
        while (b != e && *token && *b == *token) {
            b++;
            token++;
        }
        if (*token)
            return false;
#ifdef Q_CC_MSVC
        /// On MSVC, keywords like class or struct are not separated with spaces in constexpr
        /// context
        if (msvcKw && !is_ident_char(*b))
            return true;
#endif
        Q_UNUSED(msvcKw);
        return b == e || !is_ident_char(*b);
    }
    static constexpr bool skipToken(const char *&x, const char *e, const char *token,
                                    bool msvcKw = false)
    {
        if (!starts_with_token(x, e, token, msvcKw))
            return false;
        while (*token++)
            x++;
        while (x != e && is_space(*x))
            x++;
        return true;
    }
    static constexpr const char *skipString(const char *x, const char *e)
    {
        char delim = *x;
        x++;
        while (x != e && *x != delim) {
            if (*x == '\\') {
                x++;
                if (x == e)
                    return e;
            }
            x++;
        }
        if (x != e)
            x++;
        return x;
    }
    static constexpr const char *skipTemplate(const char *x, const char *e, bool stopAtComa = false)
    {
        int scopeDepth = 0;
        int templateDepth = 0;
        while (x != e) {
            switch (*x) {
            case '<':
                if (!scopeDepth)
                    templateDepth++;
                break;
            case ',':
                if (stopAtComa && !scopeDepth && !templateDepth)
                    return x;
                break;
            case '>':
                if (!scopeDepth)
                    if (--templateDepth < 0)
                        return x;
                break;
            case '(':
            case '[':
            case '{':
                scopeDepth++;
                break;
            case '}':
            case ']':
            case ')':
                scopeDepth--;
                break;
            case '\'':
                if (is_number(x[-1]))
                    break;
                Q_FALLTHROUGH();
            case '\"':
                x = skipString(x, e);
                continue;
            }
            x++;
        }
        return x;
    }

    constexpr void append(char x)
    {
        last = x;
        len++;
        if (output)
            *output++ = x;
    }

    constexpr void replaceLast(char x)
    {
        last = x;
        if (output)
            *(output - 1) = x;
    }

    constexpr void appendStr(const char *x)
    {
        while (*x)
            append(*x++);
    }

    constexpr void normalizeIntegerTypes(const char *&begin, const char *end)
    {
        int numLong = 0;
        int numSigned = 0;
        int numUnsigned = 0;
        int numInt = 0;
        int numShort = 0;
        int numChar = 0;
        while (begin < end) {
            if (skipToken(begin, end, "long")) {
                numLong++;
                continue;
            }
            if (skipToken(begin, end, "int")) {
                numInt++;
                continue;
            }
            if (skipToken(begin, end, "short")) {
                numShort++;
                continue;
            }
            if (skipToken(begin, end, "unsigned")) {
                numUnsigned++;
                continue;
            }
            if (skipToken(begin, end, "signed")) {
                numSigned++;
                continue;
            }
            if (skipToken(begin, end, "char")) {
                numChar++;
                continue;
            }
#ifdef Q_CC_MSVC
            if (skipToken(begin, end, "__int64")) {
                numLong = 2;
                continue;
            }
#endif
            break;
        }
        if (numLong == 2)
            append('q'); // q(u)longlong
        if (numSigned && numChar)
            appendStr("signed ");
        else if (numUnsigned)
            appendStr("u");
        if (numChar)
            appendStr("char");
        else if (numShort)
            appendStr("short");
        else if (numLong == 1)
            appendStr("long");
        else if (numLong == 2)
            appendStr("longlong");
        else if (numUnsigned || numSigned || numInt)
            appendStr("int");
    }

    constexpr void skipStructClassOrEnum(const char *&begin, const char *end)
    {
        // discard 'struct', 'class', and 'enum'; they are optional
        // and we don't want them in the normalized signature
        skipToken(begin, end, "struct", true) || skipToken(begin, end, "class", true)
                || skipToken(begin, end, "enum", true);
    }

    constexpr void skipQtNamespace(const char *&begin, const char *end)
    {
#ifdef QT_NAMESPACE
        const char *nsbeg = begin;
        if (skipToken(nsbeg, end, QT_STRINGIFY(QT_NAMESPACE)) && nsbeg + 2 < end && nsbeg[0] == ':'
            && nsbeg[1] == ':') {
            begin = nsbeg + 2;
            while (begin != end && is_space(*begin))
                begin++;
        }
#else
        Q_UNUSED(begin);
        Q_UNUSED(end);
#endif
    }

public:
#if defined(Q_CC_CLANG) || defined (Q_CC_GNU)
    // this is much simpler than the full type normalization below
    // the reason is that the signature returned by Q_FUNC_INFO is already
    // normalized to the largest degree, and we need to do only small adjustments
    constexpr int normalizeTypeFromSignature(const char *begin, const char *end)
    {
        // bail out if there is an anonymous struct
        std::string_view name(begin, end-begin);
#if defined (Q_CC_CLANG)
        if (name.find("anonymous ") != std::string_view::npos)
            return normalizeType(begin, end);
#endif
        if (name.find("unnamed ") != std::string_view::npos)
            return normalizeType(begin, end);
        while (begin < end) {
            if (*begin == ' ') {
                if (last == ',' || last == '>' || last == '<' || last == '*' || last == '&') {
                    ++begin;
                    continue;
                }
            }
            if (last == ' ') {
                if (*begin == '*' || *begin == '&' || *begin == '(') {
                    replaceLast(*begin);
                    ++begin;
                    continue;
                }
            }
            if (!is_ident_char(last)) {
                skipStructClassOrEnum(begin, end);
                if (begin == end)
                    break;

                skipQtNamespace(begin, end);
                if (begin == end)
                    break;

                normalizeIntegerTypes(begin, end);
                if (begin == end)
                    break;
            }
            append(*begin);
            ++begin;
        }
        return len;
    }
#else
    // MSVC needs the full normalization, as it puts the const in a different
    // place than we expect
    constexpr int normalizeTypeFromSignature(const char *begin, const char *end)
    { return normalizeType(begin, end); }
#endif

    constexpr int normalizeType(const char *begin, const char *end, bool adjustConst = true)
    {
        // Trim spaces
        while (begin != end && is_space(*begin))
            begin++;
        while (begin != end && is_space(*(end - 1)))
            end--;

        // Convert 'char const *' into 'const char *'. Start at index 1,
        // not 0, because 'const char *' is already OK.
        const char *cst = begin + 1;
        if (*begin == '\'' || *begin == '"')
            cst = skipString(begin, end);
        bool seenStar = false;
        bool hasMiddleConst = false;
        while (cst < end) {
            if (*cst == '\"' || (*cst == '\'' && !is_number(cst[-1]))) {
                cst = skipString(cst, end);
                if (cst == end)
                    break;
            }

            // We mustn't convert 'char * const *' into 'const char **'
            // and we must beware of 'Bar<const Bla>'.
            if (*cst == '&' || *cst == '*' || *cst == '[') {
                seenStar = *cst != '&' || cst != (end - 1);
                break;
            }
            if (*cst == '<') {
                cst = skipTemplate(cst + 1, end);
                if (cst == end)
                    break;
            }
            cst++;
            const char *skipedCst = cst;
            if (!is_ident_char(*(cst - 1)) && skipToken(skipedCst, end, "const")) {
                const char *testEnd = end;
                while (skipedCst < testEnd--) {
                    if (*testEnd == '*' || *testEnd == '['
                        || (*testEnd == '&' && testEnd != (end - 1))) {
                        seenStar = true;
                        break;
                    }
                    if (*testEnd == '>')
                        break;
                }
                if (adjustConst && !seenStar) {
                    if (*(end - 1) == '&')
                        end--;
                } else {
                    appendStr("const ");
                }
                normalizeType(begin, cst, false);
                begin = skipedCst;
                hasMiddleConst = true;
                break;
            }
        }
        if (skipToken(begin, end, "const")) {
            if (adjustConst && !seenStar) {
                if (*(end - 1) == '&')
                    end--;
            } else {
                appendStr("const ");
            }
        }
        if (seenStar && adjustConst) {
            const char *e = end;
            if (*(end - 1) == '&' && *(end - 2) != '&')
                e--;
            while (begin != e && is_space(*(e - 1)))
                e--;
            const char *token = "tsnoc"; // 'const' reverse, to check if it ends with const
            while (*token && begin != e && *(--e) == *token++)
                ;
            if (!*token && begin != e && !is_ident_char(*(e - 1))) {
                while (begin != e && is_space(*(e - 1)))
                    e--;
                end = e;
            }
        }

        skipStructClassOrEnum(begin, end);
        skipQtNamespace(begin, end);

        if (skipToken(begin, end, "QVector")) {
            // Replace QVector by QList
            appendStr("QList");
        }

        if (skipToken(begin, end, "QPair")) {
            // replace QPair by std::pair
            appendStr("std::pair");
        }

        if (!hasMiddleConst)
            // Normalize the integer types
            normalizeIntegerTypes(begin, end);

        bool spaceSkiped = true;
        while (begin != end) {
            char c = *begin++;
            if (is_space(c)) {
                spaceSkiped = true;
            } else if ((c == '\'' && !is_number(last)) || c == '\"') {
                begin--;
                auto x = skipString(begin, end);
                while (begin < x)
                    append(*begin++);
            } else {
                if (spaceSkiped && is_ident_char(last) && is_ident_char(c))
                    append(' ');
                append(c);
                spaceSkiped = false;
                if (c == '<') {
                    do {
                        // template recursion
                        const char *tpl = skipTemplate(begin, end, true);
                        normalizeType(begin, tpl, false);
                        if (tpl == end)
                            return len;
                        append(*tpl);
                        begin = tpl;
                    } while (*begin++ == ',');
                }
            }
        }
        return len;
    }
};

// Normalize the type between begin and end, and store the data in the output. Returns the length.
// The idea is to first run this function with nullptr as output to allocate the output with the
// size
constexpr int qNormalizeType(const char *begin, const char *end, char *output)
{
    return QTypeNormalizer { output }.normalizeType(begin, end);
}

template<typename T>
struct is_std_pair : std::false_type {};

template <typename T1_, typename T2_>
struct is_std_pair<std::pair<T1_, T2_>> : std::true_type {
    using T1 = T1_;
    using T2 = T2_;
};

template<typename T>
constexpr auto typenameHelper()
{
    if constexpr (is_std_pair<T>::value) {
        using T1 = typename is_std_pair<T>::T1;
        using T2 = typename is_std_pair<T>::T2;
        std::remove_const_t<std::conditional_t<bool (QMetaTypeId2<T1>::IsBuiltIn), typename QMetaTypeId2<T1>::NameAsArrayType, decltype(typenameHelper<T1>())>> t1Name {};
        std::remove_const_t<std::conditional_t<bool (QMetaTypeId2<T2>::IsBuiltIn), typename QMetaTypeId2<T2>::NameAsArrayType, decltype(typenameHelper<T2>())>> t2Name {};
        if constexpr (bool (QMetaTypeId2<T1>::IsBuiltIn) ) {
            t1Name = QMetaTypeId2<T1>::nameAsArray;
        } else {
            t1Name = typenameHelper<T1>();
        }
        if constexpr (bool(QMetaTypeId2<T2>::IsBuiltIn)) {
            t2Name = QMetaTypeId2<T2>::nameAsArray;
        } else {
            t2Name = typenameHelper<T2>();
        }
        constexpr auto nonTypeDependentLen = sizeof("std::pair<,>");
        constexpr auto t1Len = t1Name.size() - 1;
        constexpr auto t2Len = t2Name.size() - 1;
        constexpr auto length = nonTypeDependentLen + t1Len + t2Len;
        std::array<char, length + 1> result {};
        constexpr auto prefix = "std::pair<";
        int currentLength = 0;
        for (; currentLength < int(sizeof("std::pair<") - 1); ++currentLength)
            result[currentLength] = prefix[currentLength];
        for (int i = 0; i < int(t1Len); ++currentLength, ++i)
            result[currentLength] = t1Name[i];
        result[currentLength++] = ',';
        for (int i = 0; i < int(t2Len); ++currentLength, ++i)
            result[currentLength] = t2Name[i];
        result[currentLength++] = '>';
        result[currentLength++] = '\0';
        return result;
    } else {
        constexpr auto prefix = sizeof(
#ifdef QT_NAMESPACE
            QT_STRINGIFY(QT_NAMESPACE) "::"
#endif
#if defined(Q_CC_MSVC) && defined(Q_CC_CLANG)
            "auto __cdecl QtPrivate::typenameHelper(void) [T = "
#elif defined(Q_CC_MSVC)
            "auto __cdecl QtPrivate::typenameHelper<"
#elif defined(Q_CC_CLANG)
            "auto QtPrivate::typenameHelper() [T = "
#elif defined(Q_CC_GHS)
            "auto QtPrivate::typenameHelper<T>()[with T="
#else
            "constexpr auto QtPrivate::typenameHelper() [with T = "
#endif
            ) - 1;
#if defined(Q_CC_MSVC) && !defined(Q_CC_CLANG)
        constexpr int suffix = sizeof(">(void)");
#else
        constexpr int suffix = sizeof("]");
#endif

#if defined(Q_CC_GNU_ONLY) && Q_CC_GNU_ONLY < 804
        auto func = Q_FUNC_INFO;
        const char *begin = func + prefix;
        const char *end = func + sizeof(Q_FUNC_INFO) - suffix;
        // This is an upper bound of the size since the normalized signature should always be smaller
        constexpr int len = sizeof(Q_FUNC_INFO) - suffix - prefix;
#else
        constexpr auto func = Q_FUNC_INFO;
        constexpr const char *begin = func + prefix;
        constexpr const char *end = func + sizeof(Q_FUNC_INFO) - suffix;
        constexpr int len = QTypeNormalizer{ nullptr }.normalizeTypeFromSignature(begin, end);
#endif
        std::array<char, len + 1> result {};
        QTypeNormalizer{ result.data() }.normalizeTypeFromSignature(begin, end);
        return result;
    }
}

template<typename T, typename = void>
struct BuiltinMetaType : std::integral_constant<int, 0>
{
};
template<typename T>
struct BuiltinMetaType<T, std::enable_if_t<QMetaTypeId2<T>::IsBuiltIn>>
    : std::integral_constant<int, QMetaTypeId2<T>::MetaType>
{
};

template<typename T, bool = (QTypeTraits::has_operator_equal_v<T> && !std::is_pointer_v<T>)>
struct QEqualityOperatorForType
{
QT_WARNING_PUSH
QT_WARNING_DISABLE_FLOAT_COMPARE
    static bool equals(const QMetaTypeInterface *, const void *a, const void *b)
    { return *reinterpret_cast<const T *>(a) == *reinterpret_cast<const T *>(b); }
QT_WARNING_POP
};

template<typename T>
struct QEqualityOperatorForType <T, false>
{
    static constexpr QMetaTypeInterface::EqualsFn equals = nullptr;
};

template<typename T, bool = (QTypeTraits::has_operator_less_than_v<T> && !std::is_pointer_v<T>)>
struct QLessThanOperatorForType
{
    static bool lessThan(const QMetaTypeInterface *, const void *a, const void *b)
    { return *reinterpret_cast<const T *>(a) < *reinterpret_cast<const T *>(b); }
};

template<typename T>
struct QLessThanOperatorForType <T, false>
{
    static constexpr QMetaTypeInterface::LessThanFn lessThan = nullptr;
};

template<typename T, bool = (QTypeTraits::has_ostream_operator_v<QDebug, T> && !std::is_pointer_v<T>)>
struct QDebugStreamOperatorForType
{
    static void debugStream(const QMetaTypeInterface *, QDebug &dbg, const void *a)
    { dbg << *reinterpret_cast<const T *>(a); }
};

template<typename T>
struct QDebugStreamOperatorForType <T, false>
{
    static constexpr QMetaTypeInterface::DebugStreamFn debugStream = nullptr;
};

template<typename T, bool = QTypeTraits::has_stream_operator_v<QDataStream, T>>
struct QDataStreamOperatorForType
{
    static void dataStreamOut(const QMetaTypeInterface *, QDataStream &ds, const void *a)
    { ds << *reinterpret_cast<const T *>(a); }
    static void dataStreamIn(const QMetaTypeInterface *, QDataStream &ds, void *a)
    { ds >> *reinterpret_cast<T *>(a); }
};

template<typename T>
struct QDataStreamOperatorForType <T, false>
{
    static constexpr QMetaTypeInterface::DataStreamOutFn dataStreamOut = nullptr;
    static constexpr QMetaTypeInterface::DataStreamInFn dataStreamIn = nullptr;
};

// Performance optimization:
//
// Don't add all these symbols to the dynamic symbol tables on ELF systems and
// on Darwin. Each library is going to have a copy anyway and QMetaType already
// copes with some of these being "hidden" (see QMetaType::idHelper()). We may
// as well let the linker know it can always use the local copy.
//
// This is currently not enabled for GCC due to
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=106023

#if !defined(Q_OS_WIN) && defined(Q_CC_CLANG)
#  pragma GCC visibility push(hidden)
#endif

template<typename S>
class QMetaTypeForType
{
public:
    static constexpr decltype(typenameHelper<S>()) name = typenameHelper<S>();

    static constexpr QMetaTypeInterface::DefaultCtrFn getDefaultCtr()
    {
        if constexpr (std::is_default_constructible_v<S>) {
            return [](const QMetaTypeInterface *, void *addr) { new (addr) S(); };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaTypeInterface::CopyCtrFn getCopyCtr()
    {
        if constexpr (std::is_copy_constructible_v<S>) {
            return [](const QMetaTypeInterface *, void *addr, const void *other) {
                new (addr) S(*reinterpret_cast<const S *>(other));
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaTypeInterface::MoveCtrFn getMoveCtr()
    {
        if constexpr (std::is_move_constructible_v<S>) {
            return [](const QMetaTypeInterface *, void *addr, void *other) {
                new (addr) S(std::move(*reinterpret_cast<S *>(other)));
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaTypeInterface::DtorFn getDtor()
    {
        if constexpr (std::is_destructible_v<S> && !std::is_trivially_destructible_v<S>)
            return [](const QMetaTypeInterface *, void *addr) {
                reinterpret_cast<S *>(addr)->~S();
            };
        else
            return nullptr;
    }

    static constexpr QMetaTypeInterface::LegacyRegisterOp getLegacyRegister()
    {
        if constexpr (QMetaTypeId2<S>::Defined && !QMetaTypeId2<S>::IsBuiltIn) {
            return []() { QMetaTypeId2<S>::qt_metatype_id(); };
        } else {
            return nullptr;
        }
    }

    static constexpr const char *getName()
    {
        if constexpr (bool(QMetaTypeId2<S>::IsBuiltIn)) {
            return QMetaTypeId2<S>::nameAsArray.data();
        } else {
            return name.data();
        }
    }
};

template<typename T>
struct QMetaTypeInterfaceWrapper
{
    static inline constexpr const QMetaTypeInterface metaType = {
        /*.revision=*/ 0,
        /*.alignment=*/ alignof(T),
        /*.size=*/ sizeof(T),
        /*.flags=*/ QMetaTypeTypeFlags<T>::Flags,
        /*.typeId=*/ BuiltinMetaType<T>::value,
        /*.metaObjectFn=*/ MetaObjectForType<T>::metaObjectFunction,
        /*.name=*/ QMetaTypeForType<T>::getName(),
        /*.defaultCtr=*/ QMetaTypeForType<T>::getDefaultCtr(),
        /*.copyCtr=*/ QMetaTypeForType<T>::getCopyCtr(),
        /*.moveCtr=*/ QMetaTypeForType<T>::getMoveCtr(),
        /*.dtor=*/ QMetaTypeForType<T>::getDtor(),
        /*.equals=*/ QEqualityOperatorForType<T>::equals,
        /*.lessThan=*/ QLessThanOperatorForType<T>::lessThan,
        /*.debugStream=*/ QDebugStreamOperatorForType<T>::debugStream,
        /*.dataStreamOut=*/ QDataStreamOperatorForType<T>::dataStreamOut,
        /*.dataStreamIn=*/ QDataStreamOperatorForType<T>::dataStreamIn,
        /*.legacyRegisterOp=*/ QMetaTypeForType<T>::getLegacyRegister()
    };
};

#if !defined(Q_OS_WIN) && defined(Q_CC_CLANG)
#  pragma GCC visibility pop
#endif

template<>
class QMetaTypeInterfaceWrapper<void>
{
public:
    static constexpr QMetaTypeInterface metaType =
    {
        /*.revision=*/ 0,
        /*.alignment=*/ 0,
        /*.size=*/ 0,
        /*.flags=*/ 0,
        /*.typeId=*/ BuiltinMetaType<void>::value,
        /*.metaObjectFn=*/ nullptr,
        /*.name=*/ "void",
        /*.defaultCtr=*/ nullptr,
        /*.copyCtr=*/ nullptr,
        /*.moveCtr=*/ nullptr,
        /*.dtor=*/ nullptr,
        /*.equals=*/ nullptr,
        /*.lessThan=*/ nullptr,
        /*.debugStream=*/ nullptr,
        /*.dataStreamOut=*/ nullptr,
        /*.dataStreamIn=*/ nullptr,
        /*.legacyRegisterOp=*/ nullptr
    };
};

/*
 MSVC instantiates extern templates
(https://developercommunity.visualstudio.com/t/c11-extern-templates-doesnt-work-for-class-templat/157868)

 The INTEGRITY compiler apparently does too.

 On Windows (with other compilers or whenever MSVC is fixed), we can't declare
 QMetaTypeInterfaceWrapper with __declspec(dllimport) because taking its
 address is not a core constant expression.
 */
#if !defined(QT_BOOTSTRAPPED) && !defined(Q_CC_MSVC) && !defined(Q_OS_INTEGRITY)

#ifdef QT_NO_DATA_RELOCATION
#  define QT_METATYPE_DECLARE_EXTERN_TEMPLATE_ITER(TypeName, Id, Name)          \
    extern template class Q_CORE_EXPORT QMetaTypeForType<Name>;
#else
#  define QT_METATYPE_DECLARE_EXTERN_TEMPLATE_ITER(TypeName, Id, Name)          \
    extern template class Q_CORE_EXPORT QMetaTypeForType<Name>;                 \
    extern template struct Q_CORE_EXPORT QMetaTypeInterfaceWrapper<Name>;
#endif

QT_FOR_EACH_STATIC_PRIMITIVE_NON_VOID_TYPE(QT_METATYPE_DECLARE_EXTERN_TEMPLATE_ITER)
QT_FOR_EACH_STATIC_PRIMITIVE_POINTER(QT_METATYPE_DECLARE_EXTERN_TEMPLATE_ITER)
QT_FOR_EACH_STATIC_CORE_CLASS(QT_METATYPE_DECLARE_EXTERN_TEMPLATE_ITER)
QT_FOR_EACH_STATIC_CORE_POINTER(QT_METATYPE_DECLARE_EXTERN_TEMPLATE_ITER)
QT_FOR_EACH_STATIC_CORE_TEMPLATE(QT_METATYPE_DECLARE_EXTERN_TEMPLATE_ITER)
#undef QT_METATYPE_DECLARE_EXTERN_TEMPLATE_ITER
#endif

template<typename T>
struct qRemovePointerLike
{
    using type = std::remove_pointer_t<T>;
};

#define Q_REMOVE_POINTER_LIKE_IMPL(Pointer) \
template <typename T> \
struct qRemovePointerLike<Pointer<T>> \
{ \
    using type = T; \
};

QT_FOR_EACH_AUTOMATIC_TEMPLATE_SMART_POINTER(Q_REMOVE_POINTER_LIKE_IMPL)
template<typename T>
using qRemovePointerLike_t = typename qRemovePointerLike<T>::type;
#undef Q_REMOVE_POINTER_LIKE_IMPL

template<typename T, typename ForceComplete_>
struct TypeAndForceComplete
{
    using type = T;
    using ForceComplete = ForceComplete_;
};

template<typename T>
constexpr const QMetaTypeInterface *qMetaTypeInterfaceForType()
{
    using Ty = typename MetatypeDecay<T>::type;
    return &QMetaTypeInterfaceWrapper<Ty>::metaType;
}

template<typename Unique, typename TypeCompletePair>
constexpr const QMetaTypeInterface *qTryMetaTypeInterfaceForType()
{
    using T = typename TypeCompletePair::type;
    using ForceComplete = typename TypeCompletePair::ForceComplete;
    using Ty = typename MetatypeDecay<T>::type;
    using Tz = qRemovePointerLike_t<Ty>;

    if constexpr (ForceComplete::value) {
        return &QMetaTypeInterfaceWrapper<Ty>::metaType;
    } else if constexpr (std::is_reference_v<Tz>) {
        return nullptr;
    } else if constexpr (!is_complete<Tz, Unique>::value) {
        return nullptr;
    } else {
        return &QMetaTypeInterfaceWrapper<Ty>::metaType;
    }
}

} // namespace QtPrivate

template<typename T>
constexpr QMetaType QMetaType::fromType()
{
    return QMetaType(QtPrivate::qMetaTypeInterfaceForType<T>());
}

constexpr qsizetype QMetaType::sizeOf() const
{
    return d_ptr ? d_ptr->size : 0;
}

constexpr qsizetype QMetaType::alignOf() const
{
    return d_ptr ? d_ptr->alignment : 0;
}

constexpr QMetaType::TypeFlags QMetaType::flags() const
{
    return d_ptr ? TypeFlags(d_ptr->flags) : TypeFlags{};
}

constexpr const QMetaObject *QMetaType::metaObject() const
{
    return d_ptr && d_ptr->metaObjectFn ? d_ptr->metaObjectFn(d_ptr) : nullptr;
}

template<typename... T>
constexpr const QtPrivate::QMetaTypeInterface *const qt_metaTypeArray[] = {
    /*
       Unique in qTryMetaTypeInterfaceForType does not have to be unique here
       as we require _all_ types here to be actually complete.
       We just want to have the additional type processing that exist in
       QtPrivate::qTryMetaTypeInterfaceForType as opposed to the normal
       QtPrivate::qMetaTypeInterfaceForType used in QMetaType::fromType
    */
    QtPrivate::qTryMetaTypeInterfaceForType<void, QtPrivate::TypeAndForceComplete<T, std::true_type>>()...
};

constexpr const char *QMetaType::name() const
{
    return d_ptr ? d_ptr->name : nullptr;
}

template<typename Unique,typename... T>
constexpr const QtPrivate::QMetaTypeInterface *const qt_incomplete_metaTypeArray[] = {
    QtPrivate::qTryMetaTypeInterfaceForType<Unique, T>()...
};

inline size_t qHash(QMetaType type, size_t seed = 0)
{
    // We cannot use d_ptr here since the same type in different DLLs
    // might result in different pointers!
    return qHash(type.id(), seed);
}

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN_TAGGED(QtMetaTypePrivate::QPairVariantInterfaceImpl,
                               QPairVariantInterfaceImpl, Q_CORE_EXPORT)

#endif // QMETATYPE_H
