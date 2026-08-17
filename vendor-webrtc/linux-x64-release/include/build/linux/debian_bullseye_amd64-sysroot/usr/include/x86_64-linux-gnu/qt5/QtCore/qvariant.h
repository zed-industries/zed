/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
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

#ifndef QVARIANT_H
#define QVARIANT_H

#include <QtCore/qatomic.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qlist.h>
#include <QtCore/qmetatype.h>
#include <QtCore/qmap.h>
#include <QtCore/qhash.h>
#include <QtCore/qstring.h>
#include <QtCore/qstringlist.h>
#include <QtCore/qobject.h>
#ifndef QT_BOOTSTRAPPED
#include <QtCore/qbytearraylist.h>
#endif

#if __has_include(<variant>) && __cplusplus >= 201703L
#include <variant>
#elif defined(Q_CLANG_QDOC)
namespace std { template<typename...> struct variant; }
#endif

QT_BEGIN_NAMESPACE


class QBitArray;
class QDataStream;
class QDate;
class QDateTime;
#if QT_CONFIG(easingcurve)
class QEasingCurve;
#endif
class QLine;
class QLineF;
class QLocale;
class QMatrix;
class QTransform;
class QStringList;
class QTime;
class QPoint;
class QPointF;
class QSize;
class QSizeF;
class QRect;
class QRectF;
#ifndef QT_NO_REGEXP
class QRegExp;
#endif // QT_NO_REGEXP
#if QT_CONFIG(regularexpression)
class QRegularExpression;
#endif // QT_CONFIG(regularexpression)
class QTextFormat;
class QTextLength;
class QUrl;
class QVariant;
class QVariantComparisonHelper;

template<typename T>
inline T qvariant_cast(const QVariant &);

namespace QtPrivate {

    template <typename Derived, typename Argument, typename ReturnType>
    struct ObjectInvoker
    {
        static ReturnType invoke(Argument a)
        {
            return Derived::object(a);
        }
    };

    template <typename Derived, typename Argument, typename ReturnType>
    struct MetaTypeInvoker
    {
        static ReturnType invoke(Argument a)
        {
            return Derived::metaType(a);
        }
    };

    template <typename Derived, typename T, typename Argument, typename ReturnType, bool = IsPointerToTypeDerivedFromQObject<T>::Value>
    struct TreatAsQObjectBeforeMetaType : ObjectInvoker<Derived, Argument, ReturnType>
    {
    };

    template <typename Derived, typename T, typename Argument, typename ReturnType>
    struct TreatAsQObjectBeforeMetaType<Derived, T, Argument, ReturnType, false> : MetaTypeInvoker<Derived, Argument, ReturnType>
    {
    };

    template<typename T> struct QVariantValueHelper;
}

class Q_CORE_EXPORT QVariant
{
 public:
    enum Type {
        Invalid = QMetaType::UnknownType,
        Bool = QMetaType::Bool,
        Int = QMetaType::Int,
        UInt = QMetaType::UInt,
        LongLong = QMetaType::LongLong,
        ULongLong = QMetaType::ULongLong,
        Double = QMetaType::Double,
        Char = QMetaType::QChar,
        Map = QMetaType::QVariantMap,
        List = QMetaType::QVariantList,
        String = QMetaType::QString,
        StringList = QMetaType::QStringList,
        ByteArray = QMetaType::QByteArray,
        BitArray = QMetaType::QBitArray,
        Date = QMetaType::QDate,
        Time = QMetaType::QTime,
        DateTime = QMetaType::QDateTime,
        Url = QMetaType::QUrl,
        Locale = QMetaType::QLocale,
        Rect = QMetaType::QRect,
        RectF = QMetaType::QRectF,
        Size = QMetaType::QSize,
        SizeF = QMetaType::QSizeF,
        Line = QMetaType::QLine,
        LineF = QMetaType::QLineF,
        Point = QMetaType::QPoint,
        PointF = QMetaType::QPointF,
        RegExp = QMetaType::QRegExp,
        RegularExpression = QMetaType::QRegularExpression,
        Hash = QMetaType::QVariantHash,
#if QT_CONFIG(easingcurve)
        EasingCurve = QMetaType::QEasingCurve,
#endif
        Uuid = QMetaType::QUuid,
#if QT_CONFIG(itemmodel)
        ModelIndex = QMetaType::QModelIndex,
        PersistentModelIndex = QMetaType::QPersistentModelIndex,
#endif
        LastCoreType = QMetaType::LastCoreType,

        Font = QMetaType::QFont,
        Pixmap = QMetaType::QPixmap,
        Brush = QMetaType::QBrush,
        Color = QMetaType::QColor,
        Palette = QMetaType::QPalette,
        Image = QMetaType::QImage,
        Polygon = QMetaType::QPolygon,
        Region = QMetaType::QRegion,
        Bitmap = QMetaType::QBitmap,
        Cursor = QMetaType::QCursor,
        KeySequence = QMetaType::QKeySequence,
        Pen = QMetaType::QPen,
        TextLength = QMetaType::QTextLength,
        TextFormat = QMetaType::QTextFormat,
        Matrix = QMetaType::QMatrix,
        Transform = QMetaType::QTransform,
        Matrix4x4 = QMetaType::QMatrix4x4,
        Vector2D = QMetaType::QVector2D,
        Vector3D = QMetaType::QVector3D,
        Vector4D = QMetaType::QVector4D,
        Quaternion = QMetaType::QQuaternion,
        PolygonF = QMetaType::QPolygonF,
        Icon = QMetaType::QIcon,
        LastGuiType = QMetaType::LastGuiType,

        SizePolicy = QMetaType::QSizePolicy,

        UserType = QMetaType::User,
        LastType = 0xffffffff // need this so that gcc >= 3.4 allocates 32 bits for Type
    };

    QVariant() noexcept : d() {}
    ~QVariant();
    QVariant(Type type);
    QVariant(int typeId, const void *copy);
    QVariant(int typeId, const void *copy, uint flags);
    QVariant(const QVariant &other);

#ifndef QT_NO_DATASTREAM
    QVariant(QDataStream &s);
#endif

    QVariant(int i);
    QVariant(uint ui);
    QVariant(qlonglong ll);
    QVariant(qulonglong ull);
    QVariant(bool b);
    QVariant(double d);
    QVariant(float f);
#ifndef QT_NO_CAST_FROM_ASCII
    QT_ASCII_CAST_WARN QVariant(const char *str);
#endif

    QVariant(const QByteArray &bytearray);
    QVariant(const QBitArray &bitarray);
    QVariant(const QString &string);
    QVariant(QLatin1String string);
    QVariant(const QStringList &stringlist);
    QVariant(QChar qchar);
    QVariant(const QDate &date);
    QVariant(const QTime &time);
    QVariant(const QDateTime &datetime);
    QVariant(const QList<QVariant> &list);
    QVariant(const QMap<QString,QVariant> &map);
    QVariant(const QHash<QString,QVariant> &hash);
#ifndef QT_NO_GEOM_VARIANT
    QVariant(const QSize &size);
    QVariant(const QSizeF &size);
    QVariant(const QPoint &pt);
    QVariant(const QPointF &pt);
    QVariant(const QLine &line);
    QVariant(const QLineF &line);
    QVariant(const QRect &rect);
    QVariant(const QRectF &rect);
#endif
    QVariant(const QLocale &locale);
#ifndef QT_NO_REGEXP
    QVariant(const QRegExp &regExp);
#endif // QT_NO_REGEXP
#if QT_CONFIG(regularexpression)
    QVariant(const QRegularExpression &re);
#endif // QT_CONFIG(regularexpression)
#if QT_CONFIG(easingcurve)
    QVariant(const QEasingCurve &easing);
#endif
    QVariant(const QUuid &uuid);
#ifndef QT_BOOTSTRAPPED
    QVariant(const QUrl &url);
    QVariant(const QJsonValue &jsonValue);
    QVariant(const QJsonObject &jsonObject);
    QVariant(const QJsonArray &jsonArray);
    QVariant(const QJsonDocument &jsonDocument);
#endif // QT_BOOTSTRAPPED
#if QT_CONFIG(itemmodel)
    QVariant(const QModelIndex &modelIndex);
    QVariant(const QPersistentModelIndex &modelIndex);
#endif

    QVariant& operator=(const QVariant &other);
    inline QVariant(QVariant &&other) noexcept : d(other.d)
    { other.d = Private(); }
    inline QVariant &operator=(QVariant &&other) noexcept
    { qSwap(d, other.d); return *this; }

    inline void swap(QVariant &other) noexcept { qSwap(d, other.d); }

    Type type() const;
    int userType() const;
    const char *typeName() const;

    bool canConvert(int targetTypeId) const;
    bool convert(int targetTypeId);

    inline bool isValid() const;
    bool isNull() const;

    void clear();

    void detach();
    inline bool isDetached() const;

    int toInt(bool *ok = nullptr) const;
    uint toUInt(bool *ok = nullptr) const;
    qlonglong toLongLong(bool *ok = nullptr) const;
    qulonglong toULongLong(bool *ok = nullptr) const;
    bool toBool() const;
    double toDouble(bool *ok = nullptr) const;
    float toFloat(bool *ok = nullptr) const;
    qreal toReal(bool *ok = nullptr) const;
    QByteArray toByteArray() const;
    QBitArray toBitArray() const;
    QString toString() const;
    QStringList toStringList() const;
    QChar toChar() const;
    QDate toDate() const;
    QTime toTime() const;
    QDateTime toDateTime() const;
    QList<QVariant> toList() const;
    QMap<QString, QVariant> toMap() const;
    QHash<QString, QVariant> toHash() const;

#ifndef QT_NO_GEOM_VARIANT
    QPoint toPoint() const;
    QPointF toPointF() const;
    QRect toRect() const;
    QSize toSize() const;
    QSizeF toSizeF() const;
    QLine toLine() const;
    QLineF toLineF() const;
    QRectF toRectF() const;
#endif
    QLocale toLocale() const;
#ifndef QT_NO_REGEXP
    QRegExp toRegExp() const;
#endif // QT_NO_REGEXP
#if QT_CONFIG(regularexpression)
    QRegularExpression toRegularExpression() const;
#endif // QT_CONFIG(regularexpression)
#if QT_CONFIG(easingcurve)
    QEasingCurve toEasingCurve() const;
#endif
    QUuid toUuid() const;
#ifndef QT_BOOTSTRAPPED
    QUrl toUrl() const;
    QJsonValue toJsonValue() const;
    QJsonObject toJsonObject() const;
    QJsonArray toJsonArray() const;
    QJsonDocument toJsonDocument() const;
#endif // QT_BOOTSTRAPPED
#if QT_CONFIG(itemmodel)
    QModelIndex toModelIndex() const;
    QPersistentModelIndex toPersistentModelIndex() const;
#endif

#ifndef QT_NO_DATASTREAM
    void load(QDataStream &ds);
    void save(QDataStream &ds) const;
#endif
    static const char *typeToName(int typeId);
    static Type nameToType(const char *name);

    void *data();
    const void *constData() const;
    inline const void *data() const { return constData(); }

    template<typename T>
    inline void setValue(const T &value);

    template<typename T>
    inline T value() const
    { return qvariant_cast<T>(*this); }

    template<typename T>
    static inline QVariant fromValue(const T &value)
    { return QVariant(qMetaTypeId<T>(), &value, QTypeInfo<T>::isPointer); }

#if (__has_include(<variant>) && __cplusplus >= 201703L) || defined(Q_CLANG_QDOC)
    template<typename... Types>
    static inline QVariant fromStdVariant(const std::variant<Types...> &value)
    {
        if (value.valueless_by_exception())
            return QVariant();
        return std::visit([](const auto &arg) { return fromValue(arg); }, value);
    }
#endif

    template<typename T>
    bool canConvert() const
    { return canConvert(qMetaTypeId<T>()); }

 public:
    struct PrivateShared
    {
        inline PrivateShared(void *v) : ptr(v), ref(1) { }
        void *ptr;
        QAtomicInt ref;
    };
    struct Private
    {
        inline Private() noexcept : type(Invalid), is_shared(false), is_null(true)
        { data.ptr = nullptr; }

        // Internal constructor for initialized variants.
        explicit inline Private(uint variantType) noexcept
            : type(variantType), is_shared(false), is_null(false)
        {}

#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
        Private(const Private &other) noexcept
            : data(other.data), type(other.type),
              is_shared(other.is_shared), is_null(other.is_null)
        {}
        Private &operator=(const Private &other) noexcept = default;
#endif
        union Data
        {
            char c;
            uchar uc;
            short s;
            signed char sc;
            ushort us;
            int i;
            uint u;
            long l;
            ulong ul;
            bool b;
            double d;
            float f;
            qreal real;
            qlonglong ll;
            qulonglong ull;
            QObject *o;
            void *ptr;
            PrivateShared *shared;
        } data;
        uint type : 30;
        uint is_shared : 1;
        uint is_null : 1;
    };
 public:
    typedef void (*f_construct)(Private *, const void *);
    typedef void (*f_clear)(Private *);
    typedef bool (*f_null)(const Private *);
#ifndef QT_NO_DATASTREAM
    typedef void (*f_load)(Private *, QDataStream &);
    typedef void (*f_save)(const Private *, QDataStream &);
#endif
    typedef bool (*f_compare)(const Private *, const Private *);
    typedef bool (*f_convert)(const QVariant::Private *d, int t, void *, bool *);
    typedef bool (*f_canConvert)(const QVariant::Private *d, int t);
    typedef void (*f_debugStream)(QDebug, const QVariant &);
    struct Handler {
        f_construct construct;
        f_clear clear;
        f_null isNull;
#ifndef QT_NO_DATASTREAM
        f_load load;
        f_save save;
#endif
        f_compare compare;
        f_convert convert;
        f_canConvert canConvert;
        f_debugStream debugStream;
    };

    inline bool operator==(const QVariant &v) const
    { return cmp(v); }
    inline bool operator!=(const QVariant &v) const
    { return !cmp(v); }
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED inline bool operator<(const QVariant &v) const
    { return compare(v) < 0; }
    QT_DEPRECATED inline bool operator<=(const QVariant &v) const
    { return compare(v) <= 0; }
    QT_DEPRECATED inline bool operator>(const QVariant &v) const
    { return compare(v) > 0; }
    QT_DEPRECATED inline bool operator>=(const QVariant &v) const
    { return compare(v) >= 0; }
#endif

protected:
    friend inline bool operator==(const QVariant &, const QVariantComparisonHelper &);
#ifndef QT_NO_DEBUG_STREAM
    friend Q_CORE_EXPORT QDebug operator<<(QDebug, const QVariant &);
#endif
// ### Qt6: FIXME: Remove the special Q_CC_MSVC handling, it was introduced to maintain BC for QTBUG-41810 .
#if !defined(Q_NO_TEMPLATE_FRIENDS) && !defined(Q_CC_MSVC)
    template<typename T>
    friend inline T qvariant_cast(const QVariant &);
    template<typename T> friend struct QtPrivate::QVariantValueHelper;
protected:
#else
public:
#endif
    Private d;
    void create(int type, const void *copy);
    bool cmp(const QVariant &other) const;
    int compare(const QVariant &other) const;
    bool convert(const int t, void *ptr) const; // ### Qt6: drop const

private:
    // force compile error, prevent QVariant(bool) to be called
    inline QVariant(void *) = delete;
    // QVariant::Type is marked as \obsolete, but we don't want to
    // provide a constructor from its intended replacement,
    // QMetaType::Type, instead, because the idea behind these
    // constructors is flawed in the first place. But we also don't
    // want QVariant(QMetaType::String) to compile and falsely be an
    // int variant, so delete this constructor:
    QVariant(QMetaType::Type) = delete;

    // These constructors don't create QVariants of the type associcated
    // with the enum, as expected, but they would create a QVariant of
    // type int with the value of the enum value.
    // Use QVariant v = QColor(Qt::red) instead of QVariant v = Qt::red for
    // example.
    QVariant(Qt::GlobalColor) = delete;
    QVariant(Qt::BrushStyle) = delete;
    QVariant(Qt::PenStyle) = delete;
    QVariant(Qt::CursorShape) = delete;
#ifdef QT_NO_CAST_FROM_ASCII
    // force compile error when implicit conversion is not wanted
    inline QVariant(const char *) = delete;
#endif
public:
    typedef Private DataPtr;
    inline DataPtr &data_ptr() { return d; }
    inline const DataPtr &data_ptr() const { return d; }
};

#if QT_DEPRECATED_SINCE(5, 14)
template <typename T>
QT_DEPRECATED_X("Use QVariant::fromValue() instead.")
inline QVariant qVariantFromValue(const T &t)
{
    return QVariant::fromValue(t);
}

template <typename T>
QT_DEPRECATED_X("Use QVariant::setValue() instead.")
inline void qVariantSetValue(QVariant &v, const T &t)
{
    v.setValue(t);
}
#endif

template<>
inline QVariant QVariant::fromValue(const QVariant &value)
{
    return value;
}

#if __has_include(<variant>) && __cplusplus >= 201703L
template<>
inline QVariant QVariant::fromValue(const std::monostate &)
{
    return QVariant();
}
#endif

inline bool QVariant::isValid() const { return d.type != Invalid; }

template<typename T>
inline void QVariant::setValue(const T &avalue)
{
    // If possible we reuse the current QVariant private.
    const uint type = qMetaTypeId<T>();
    if (isDetached() && (type == d.type || (type <= uint(QVariant::Char) && d.type <= uint(QVariant::Char)))) {
        d.type = type;
        d.is_null = false;
        T *old = reinterpret_cast<T*>(d.is_shared ? d.data.shared->ptr : &d.data.ptr);
        if (QTypeInfo<T>::isComplex)
            old->~T();
        new (old) T(avalue); // call the copy constructor
    } else {
        *this = QVariant(type, &avalue, QTypeInfo<T>::isPointer);
    }
}

template<>
inline void QVariant::setValue(const QVariant &avalue)
{
    *this = avalue;
}

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream& operator>> (QDataStream& s, QVariant& p);
Q_CORE_EXPORT QDataStream& operator<< (QDataStream& s, const QVariant& p);
Q_CORE_EXPORT QDataStream& operator>> (QDataStream& s, QVariant::Type& p);
Q_CORE_EXPORT QDataStream& operator<< (QDataStream& s, const QVariant::Type p);
#endif

inline bool QVariant::isDetached() const
{ return !d.is_shared || d.data.shared->ref.loadRelaxed() == 1; }


#ifdef Q_QDOC
    inline bool operator==(const QVariant &v1, const QVariant &v2);
    inline bool operator!=(const QVariant &v1, const QVariant &v2);
#else

/* Helper class to add one more level of indirection to prevent
   implicit casts.
*/
class QVariantComparisonHelper
{
public:
    inline QVariantComparisonHelper(const QVariant &var)
        : v(&var) {}
private:
    friend inline bool operator==(const QVariant &, const QVariantComparisonHelper &);
    const QVariant *v;
};

inline bool operator==(const QVariant &v1, const QVariantComparisonHelper &v2)
{
    return v1.cmp(*v2.v);
}

inline bool operator!=(const QVariant &v1, const QVariantComparisonHelper &v2)
{
    return !operator==(v1, v2);
}
#endif
Q_DECLARE_SHARED(QVariant)

class Q_CORE_EXPORT QSequentialIterable
{
    QtMetaTypePrivate::QSequentialIterableImpl m_impl;
public:
    struct Q_CORE_EXPORT const_iterator
    {
    private:
        QtMetaTypePrivate::QSequentialIterableImpl m_impl;
        QAtomicInt *ref;
        friend class QSequentialIterable;
        explicit const_iterator(const QSequentialIterable &iter, QAtomicInt *ref_);

        explicit const_iterator(const QtMetaTypePrivate::QSequentialIterableImpl &impl, QAtomicInt *ref_);

        void begin();
        void end();
    public:
        ~const_iterator();

        const_iterator(const const_iterator &other);

        const_iterator& operator=(const const_iterator &other);

        const QVariant operator*() const;
        bool operator==(const const_iterator &o) const;
        bool operator!=(const const_iterator &o) const;
        const_iterator &operator++();
        const_iterator operator++(int);
        const_iterator &operator--();
        const_iterator operator--(int);
        const_iterator &operator+=(int j);
        const_iterator &operator-=(int j);
        const_iterator operator+(int j) const;
        const_iterator operator-(int j) const;
        friend inline const_iterator operator+(int j, const_iterator k) { return k + j; }
    };

    friend struct const_iterator;

#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    explicit QSequentialIterable(QtMetaTypePrivate::QSequentialIterableImpl impl);
#else
    explicit QSequentialIterable(const QtMetaTypePrivate::QSequentialIterableImpl &impl);
#endif

    const_iterator begin() const;
    const_iterator end() const;

    QVariant at(int idx) const;
    int size() const;

    bool canReverseIterate() const;
};

class Q_CORE_EXPORT QAssociativeIterable
{
    QtMetaTypePrivate::QAssociativeIterableImpl m_impl;
public:
    struct Q_CORE_EXPORT const_iterator
    {
    private:
        QtMetaTypePrivate::QAssociativeIterableImpl m_impl;
        QAtomicInt *ref;
        friend class QAssociativeIterable;
        explicit const_iterator(const QAssociativeIterable &iter, QAtomicInt *ref_);

        explicit const_iterator(const QtMetaTypePrivate::QAssociativeIterableImpl &impl, QAtomicInt *ref_);

        void begin();
        void end();
        void find(const QVariant &key);
    public:
        ~const_iterator();
        const_iterator(const const_iterator &other);

        const_iterator& operator=(const const_iterator &other);

        const QVariant key() const;

        const QVariant value() const;

        const QVariant operator*() const;
        bool operator==(const const_iterator &o) const;
        bool operator!=(const const_iterator &o) const;
        const_iterator &operator++();
        const_iterator operator++(int);
        const_iterator &operator--();
        const_iterator operator--(int);
        const_iterator &operator+=(int j);
        const_iterator &operator-=(int j);
        const_iterator operator+(int j) const;
        const_iterator operator-(int j) const;
        friend inline const_iterator operator+(int j, const_iterator k) { return k + j; }
    };

    friend struct const_iterator;

#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    explicit QAssociativeIterable(QtMetaTypePrivate::QAssociativeIterableImpl impl);
#else
    explicit QAssociativeIterable(const QtMetaTypePrivate::QAssociativeIterableImpl &impl);
#endif

    const_iterator begin() const;
    const_iterator end() const;
    const_iterator find(const QVariant &key) const;

    QVariant value(const QVariant &key) const;

    int size() const;
};

#ifndef QT_MOC
namespace QtPrivate {
    template<typename T>
    struct QVariantValueHelper : TreatAsQObjectBeforeMetaType<QVariantValueHelper<T>, T, const QVariant &, T>
    {
        static T metaType(const QVariant &v)
        {
            const int vid = qMetaTypeId<T>();
            if (vid == v.userType())
                return *reinterpret_cast<const T *>(v.constData());
            T t;
            if (v.convert(vid, &t))
                return t;
            return T();
        }
#ifndef QT_NO_QOBJECT
        static T object(const QVariant &v)
        {
            return qobject_cast<T>(QMetaType::typeFlags(v.userType()) & QMetaType::PointerToQObject
                ? v.d.data.o
                : QVariantValueHelper::metaType(v));
        }
#endif
    };

    template<typename T>
    struct QVariantValueHelperInterface : QVariantValueHelper<T>
    {
    };

    template<>
    struct QVariantValueHelperInterface<QSequentialIterable>
    {
        static QSequentialIterable invoke(const QVariant &v)
        {
            const int typeId = v.userType();
            if (typeId == qMetaTypeId<QVariantList>()) {
                return QSequentialIterable(QtMetaTypePrivate::QSequentialIterableImpl(reinterpret_cast<const QVariantList*>(v.constData())));
            }
            if (typeId == qMetaTypeId<QStringList>()) {
                return QSequentialIterable(QtMetaTypePrivate::QSequentialIterableImpl(reinterpret_cast<const QStringList*>(v.constData())));
            }
#ifndef QT_BOOTSTRAPPED
            if (typeId == qMetaTypeId<QByteArrayList>()) {
                return QSequentialIterable(QtMetaTypePrivate::QSequentialIterableImpl(reinterpret_cast<const QByteArrayList*>(v.constData())));
            }
#endif
            return QSequentialIterable(qvariant_cast<QtMetaTypePrivate::QSequentialIterableImpl>(v));
        }
    };
    template<>
    struct QVariantValueHelperInterface<QAssociativeIterable>
    {
        static QAssociativeIterable invoke(const QVariant &v)
        {
            const int typeId = v.userType();
            if (typeId == qMetaTypeId<QVariantMap>()) {
                return QAssociativeIterable(QtMetaTypePrivate::QAssociativeIterableImpl(reinterpret_cast<const QVariantMap*>(v.constData())));
            }
            if (typeId == qMetaTypeId<QVariantHash>()) {
                return QAssociativeIterable(QtMetaTypePrivate::QAssociativeIterableImpl(reinterpret_cast<const QVariantHash*>(v.constData())));
            }
            return QAssociativeIterable(qvariant_cast<QtMetaTypePrivate::QAssociativeIterableImpl>(v));
        }
    };
    template<>
    struct QVariantValueHelperInterface<QVariantList>
    {
        static QVariantList invoke(const QVariant &v)
        {
            const int typeId = v.userType();
            if (typeId == qMetaTypeId<QStringList>() || typeId == qMetaTypeId<QByteArrayList>() ||
                (QMetaType::hasRegisteredConverterFunction(typeId, qMetaTypeId<QtMetaTypePrivate::QSequentialIterableImpl>()) && !QMetaType::hasRegisteredConverterFunction(typeId, qMetaTypeId<QVariantList>()))) {
                QSequentialIterable iter = QVariantValueHelperInterface<QSequentialIterable>::invoke(v);
                QVariantList l;
                l.reserve(iter.size());
                for (QSequentialIterable::const_iterator it = iter.begin(), end = iter.end(); it != end; ++it)
                    l << *it;
                return l;
            }
            return QVariantValueHelper<QVariantList>::invoke(v);
        }
    };
    template<>
    struct QVariantValueHelperInterface<QVariantHash>
    {
        static QVariantHash invoke(const QVariant &v)
        {
            const int typeId = v.userType();
            if (typeId == qMetaTypeId<QVariantMap>() || ((QMetaType::hasRegisteredConverterFunction(typeId, qMetaTypeId<QtMetaTypePrivate::QAssociativeIterableImpl>())) && !QMetaType::hasRegisteredConverterFunction(typeId, qMetaTypeId<QVariantHash>()))) {
                QAssociativeIterable iter = QVariantValueHelperInterface<QAssociativeIterable>::invoke(v);
                QVariantHash l;
                l.reserve(iter.size());
                for (QAssociativeIterable::const_iterator it = iter.begin(), end = iter.end(); it != end; ++it)
                    static_cast<QMultiHash<QString, QVariant> &>(l).insert(it.key().toString(), it.value());
                return l;
            }
            return QVariantValueHelper<QVariantHash>::invoke(v);
        }
    };
    template<>
    struct QVariantValueHelperInterface<QVariantMap>
    {
        static QVariantMap invoke(const QVariant &v)
        {
            const int typeId = v.userType();
            if (typeId == qMetaTypeId<QVariantHash>() || (QMetaType::hasRegisteredConverterFunction(typeId, qMetaTypeId<QtMetaTypePrivate::QAssociativeIterableImpl>()) && !QMetaType::hasRegisteredConverterFunction(typeId, qMetaTypeId<QVariantMap>()))) {
                QAssociativeIterable iter = QVariantValueHelperInterface<QAssociativeIterable>::invoke(v);
                QVariantMap l;
                for (QAssociativeIterable::const_iterator it = iter.begin(), end = iter.end(); it != end; ++it)
                    static_cast<QMultiMap<QString, QVariant> &>(l).insert(it.key().toString(), it.value());
                return l;
            }
            return QVariantValueHelper<QVariantMap>::invoke(v);
        }
    };
    template<>
    struct QVariantValueHelperInterface<QPair<QVariant, QVariant> >
    {
        static QPair<QVariant, QVariant> invoke(const QVariant &v)
        {
            const int typeId = v.userType();

            if (QMetaType::hasRegisteredConverterFunction(typeId, qMetaTypeId<QtMetaTypePrivate::QPairVariantInterfaceImpl>()) && !(typeId == qMetaTypeId<QPair<QVariant, QVariant> >())) {
                QtMetaTypePrivate::QPairVariantInterfaceImpl pi = v.value<QtMetaTypePrivate::QPairVariantInterfaceImpl>();
                const QtMetaTypePrivate::VariantData d1 = pi.first();
                QVariant v1(d1.metaTypeId, d1.data, d1.flags);
                if (d1.metaTypeId == qMetaTypeId<QVariant>())
                    v1 = *reinterpret_cast<const QVariant*>(d1.data);

                const QtMetaTypePrivate::VariantData d2 = pi.second();
                QVariant v2(d2.metaTypeId, d2.data, d2.flags);
                if (d2.metaTypeId == qMetaTypeId<QVariant>())
                    v2 = *reinterpret_cast<const QVariant*>(d2.data);

                return QPair<QVariant, QVariant>(v1, v2);
            }
            return QVariantValueHelper<QPair<QVariant, QVariant> >::invoke(v);
        }
    };
}

template<typename T> inline T qvariant_cast(const QVariant &v)
{
    return QtPrivate::QVariantValueHelperInterface<T>::invoke(v);
}

template<> inline QVariant qvariant_cast<QVariant>(const QVariant &v)
{
    if (v.userType() == QMetaType::QVariant)
        return *reinterpret_cast<const QVariant *>(v.constData());
    return v;
}

#if QT_DEPRECATED_SINCE(5, 0)
template<typename T>
inline QT_DEPRECATED T qVariantValue(const QVariant &variant)
{ return qvariant_cast<T>(variant); }

template<typename T>
inline QT_DEPRECATED bool qVariantCanConvert(const QVariant &variant)
{ return variant.template canConvert<T>(); }
#endif

#endif

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QVariant &);
Q_CORE_EXPORT QDebug operator<<(QDebug, const QVariant::Type);
#endif

QT_END_NAMESPACE

#endif // QVARIANT_H
