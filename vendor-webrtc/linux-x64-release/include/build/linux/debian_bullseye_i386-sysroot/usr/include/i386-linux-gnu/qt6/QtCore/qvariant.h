// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QVARIANT_H
#define QVARIANT_H

#include <QtCore/qatomic.h>
#include <QtCore/qcontainerfwd.h>
#include <QtCore/qmetatype.h>
#ifndef QT_NO_DEBUG_STREAM
#include <QtCore/qdebug.h>
#endif
#include <memory>
#include <type_traits>
#include <variant>
#if !defined(QT_LEAN_HEADERS) || QT_LEAN_HEADERS < 1
#  include <QtCore/qlist.h>
#  include <QtCore/qstringlist.h>
#  include <QtCore/qbytearraylist.h>
#  include <QtCore/qhash.h>
#  include <QtCore/qmap.h>
#  include <QtCore/qobject.h>
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
class QTransform;
class QTime;
class QPoint;
class QPointF;
class QSize;
class QSizeF;
class QRect;
class QRectF;
#if QT_CONFIG(regularexpression)
class QRegularExpression;
#endif // QT_CONFIG(regularexpression)
class QTextFormat;
class QTextLength;
class QUrl;
class QVariant;

template<typename T>
inline T qvariant_cast(const QVariant &);

class Q_CORE_EXPORT QVariant
{
 public:
#if QT_DEPRECATED_SINCE(6, 0)
    enum QT_DEPRECATED_VERSION_X_6_0("Use QMetaType::Type instead.") Type
    {
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
#if QT_CONFIG(regularexpression)
        RegularExpression = QMetaType::QRegularExpression,
#endif
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
#if QT_CONFIG(shortcut)
        KeySequence = QMetaType::QKeySequence,
#endif
        Pen = QMetaType::QPen,
        TextLength = QMetaType::QTextLength,
        TextFormat = QMetaType::QTextFormat,
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
#endif
    QVariant() noexcept : d() {}
    ~QVariant();
    explicit QVariant(QMetaType type, const void *copy = nullptr);
    QVariant(const QVariant &other);

    QVariant(int i);
    QVariant(uint ui);
    QVariant(qlonglong ll);
    QVariant(qulonglong ull);
    QVariant(bool b);
    QVariant(double d);
    QVariant(float f);
#ifndef QT_NO_CAST_FROM_ASCII
    QT_ASCII_CAST_WARN QVariant(const char *str)
        : QVariant(QString::fromUtf8(str))
    {}
#endif

    QVariant(const QByteArray &bytearray);
    QVariant(const QBitArray &bitarray);
    QVariant(const QString &string);
    QVariant(QLatin1StringView string);
    QVariant(const QStringList &stringlist);
    QVariant(QChar qchar);
    QVariant(QDate date);
    QVariant(QTime time);
    QVariant(const QDateTime &datetime);
    QVariant(const QList<QVariant> &list);
    QVariant(const QMap<QString, QVariant> &map);
    QVariant(const QHash<QString, QVariant> &hash);
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
#if !defined(Q_CC_GHS)
    // GHS has an ICE with this code; use the simplified version below
    template <typename T,
              std::enable_if_t<std::disjunction_v<std::is_pointer<T>, std::is_member_pointer<T>>, bool> = false>
    QVariant(T) = delete;
#else
    QVariant(const volatile void *) = delete;
#endif

    QVariant& operator=(const QVariant &other);
    inline QVariant(QVariant &&other) noexcept : d(other.d)
    { other.d = Private(); }
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QVariant)

    inline void swap(QVariant &other) noexcept { std::swap(d, other.d); }

    int userType() const { return typeId(); }
    int typeId() const { return metaType().id(); }

    const char *typeName() const;
    QMetaType metaType() const;

    bool canConvert(QMetaType targetType) const
    { return QMetaType::canConvert(d.type(), targetType); }
    bool convert(QMetaType type);

    bool canView(QMetaType targetType) const
    { return QMetaType::canView(d.type(), targetType); }

#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_6_0
    bool canConvert(int targetTypeId) const
    { return QMetaType::canConvert(d.type(), QMetaType(targetTypeId)); }
    QT_DEPRECATED_VERSION_6_0
    bool convert(int targetTypeId)
    { return convert(QMetaType(targetTypeId)); }
#endif

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
#if QT_DEPRECATED_SINCE(6, 0)
    QT_WARNING_PUSH
    QT_WARNING_DISABLE_DEPRECATED
    QT_DEPRECATED_VERSION_X_6_0("Use the constructor taking a QMetaType instead.")
    explicit QVariant(Type type)
        : QVariant(QMetaType(int(type)))
    {}
    QT_DEPRECATED_VERSION_X_6_0("Use typeId() or metaType().")
    Type type() const
    {
        int type = d.typeId();
        return type >= QMetaType::User ? UserType : static_cast<Type>(type);
    }
    QT_DEPRECATED_VERSION_6_0
    static const char *typeToName(int typeId)
    { return QMetaType(typeId).name(); }
    QT_DEPRECATED_VERSION_6_0
    static Type nameToType(const char *name)
    {
        int metaType = QMetaType::fromName(name).id();
        return metaType <= int(UserType) ? QVariant::Type(metaType) : UserType;
    }
    QT_WARNING_POP
#endif

    void *data();
    const void *constData() const
    { return d.storage(); }
    inline const void *data() const { return constData(); }

    template<typename T, typename = std::enable_if_t<!std::is_same_v<std::decay_t<T>, QVariant>>>
    void setValue(T &&avalue)
    {
        using VT = std::decay_t<T>;
        QMetaType metaType = QMetaType::fromType<VT>();
        // If possible we reuse the current QVariant private.
        if (isDetached() && d.type() == metaType) {
            *reinterpret_cast<VT *>(const_cast<void *>(constData())) = std::forward<T>(avalue);
        } else {
            *this = QVariant::fromValue<VT>(std::forward<T>(avalue));
        }
    }

    void setValue(const QVariant &avalue)
    {
        *this = avalue;
    }

    void setValue(QVariant &&avalue)
    {
        *this = std::move(avalue);
    }

    template<typename T>
    inline T value() const
    { return qvariant_cast<T>(*this); }

    template<typename T>
    inline T view()
    {
        T t{};
        QMetaType::view(metaType(), data(), QMetaType::fromType<T>(), &t);
        return t;
    }

    template<typename T>
#ifndef Q_CLANG_QDOC
    static inline auto fromValue(const T &value) ->
    std::enable_if_t<std::is_copy_constructible_v<T>, QVariant>
#else
    static inline QVariant fromValue(const T &value)
#endif
    {
        return QVariant(QMetaType::fromType<T>(), std::addressof(value));
    }

    template<typename... Types>
    static inline QVariant fromStdVariant(const std::variant<Types...> &value)
    {
        if (value.valueless_by_exception())
            return QVariant();
        return std::visit([](const auto &arg) { return fromValue(arg); }, value);
    }

    template<typename T>
    bool canConvert() const
    { return canConvert(QMetaType::fromType<T>()); }

    template<typename T>
    bool canView() const
    { return canView(QMetaType::fromType<T>()); }

public:
    struct PrivateShared
    {
    private:
        inline PrivateShared() : ref(1) { }
    public:
        static PrivateShared *create(const QtPrivate::QMetaTypeInterface *type)
        {
            Q_ASSERT(type);
            size_t size = type->size;
            size_t align = type->alignment;

            size += sizeof(PrivateShared);
            if (align > sizeof(PrivateShared)) {
                // The alignment is larger than the alignment we can guarantee for the pointer
                // directly following PrivateShared, so we need to allocate some additional
                // memory to be able to fit the object into the available memory with suitable
                // alignment.
                size += align - sizeof(PrivateShared);
            }
            void *data = operator new(size);
            auto *ps = new (data) QVariant::PrivateShared();
            ps->offset = int(((quintptr(ps) + sizeof(PrivateShared) + align - 1) & ~(align - 1)) - quintptr(ps));
            return ps;
        }
        static void free(PrivateShared *p)
        {
            p->~PrivateShared();
            operator delete(p);
        }

        alignas(8) QAtomicInt ref;
        int offset;

        const void *data() const
        { return reinterpret_cast<const unsigned char *>(this) + offset; }
        void *data()
        { return reinterpret_cast<unsigned char *>(this) + offset; }
    };
    struct Private
    {
        static constexpr size_t MaxInternalSize = 3*sizeof(void *);
        template<typename T>
        static constexpr bool CanUseInternalSpace = (QTypeInfo<T>::isRelocatable && sizeof(T) <= MaxInternalSize && alignof(T) <= alignof(double));
        static constexpr bool canUseInternalSpace(const QtPrivate::QMetaTypeInterface *type)
        {
            Q_ASSERT(type);
            return QMetaType::TypeFlags(type->flags) & QMetaType::RelocatableType &&
                   size_t(type->size) <= MaxInternalSize && size_t(type->alignment) <= alignof(double);
        }

        union
        {
            uchar data[MaxInternalSize] = {};
            PrivateShared *shared;
            double _forAlignment; // we want an 8byte alignment on 32bit systems as well
        } data;
        quintptr is_shared : 1;
        quintptr is_null : 1;
        quintptr packedType : sizeof(QMetaType) * 8 - 2;

        Private() noexcept : is_shared(false), is_null(true), packedType(0) {}
        explicit Private(QMetaType type) noexcept : is_shared(false), is_null(false)
        {
            quintptr mt = quintptr(type.d_ptr);
            Q_ASSERT((mt & 0x3) == 0);
            packedType = mt >> 2;
        }
        explicit Private(int type) noexcept : Private(QMetaType(type)) {}

        const void *storage() const
        { return is_shared ? data.shared->data() : &data.data; }

        const void *internalStorage() const
        { Q_ASSERT(is_shared); return &data.data; }

        // determine internal storage at compile time
        template<typename T>
        const T &get() const
        { return *static_cast<const T *>(CanUseInternalSpace<T> ? &data.data : data.shared->data()); }
        template<typename T>
        void set(const T &t)
        { *static_cast<T *>(CanUseInternalSpace<T> ? &data.data : data.shared->data()) = t; }

        inline const QtPrivate::QMetaTypeInterface* typeInterface() const
        {
            return reinterpret_cast<const QtPrivate::QMetaTypeInterface *>(packedType << 2);
        }

        inline QMetaType type() const
        {
            return QMetaType(typeInterface());
        }

        inline int typeId() const
        {
            return type().id();
        }
    };
 public:
    static QPartialOrdering compare(const QVariant &lhs, const QVariant &rhs);

private:
    friend inline bool operator==(const QVariant &a, const QVariant &b)
    { return a.equals(b); }
    friend inline bool operator!=(const QVariant &a, const QVariant &b)
    { return !a.equals(b); }
#ifndef QT_NO_DEBUG_STREAM
    template <typename T>
    friend auto operator<<(const QDebug &debug, const T &variant) -> std::enable_if_t<std::is_same_v<T, QVariant>, QDebug> {
        return  variant.qdebugHelper(debug);
    }
    QDebug qdebugHelper(QDebug) const;
#endif
    template<typename T>
    friend inline T qvariant_cast(const QVariant &);
protected:
    Private d;
    void create(int type, const void *copy);
    void create(QMetaType type, const void *copy);
    bool equals(const QVariant &other) const;
    bool convert(int type, void *ptr) const;
    bool view(int type, void *ptr);

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

    // These constructors don't create QVariants of the type associated
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

template<>
inline QVariant QVariant::fromValue(const QVariant &value)
{
    return value;
}

template<>
inline QVariant QVariant::fromValue(const std::monostate &)
{
    return QVariant();
}

inline bool QVariant::isValid() const
{
    return d.type().isValid();
}

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &s, QVariant &p);
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &s, const QVariant &p);

#if QT_DEPRECATED_SINCE(6, 0)
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
QT_DEPRECATED_VERSION_6_0
inline QDataStream &operator>>(QDataStream &s, QVariant::Type &p)
{
    quint32 u;
    s >> u;
    p = static_cast<QVariant::Type>(u);
    return s;
}
QT_DEPRECATED_VERSION_6_0
inline QDataStream &operator<<(QDataStream &s, const QVariant::Type p)
{
    s << static_cast<quint32>(p);
    return s;
}
QT_WARNING_POP
#endif

#endif

inline bool QVariant::isDetached() const
{ return !d.is_shared || d.data.shared->ref.loadRelaxed() == 1; }

Q_DECLARE_SHARED(QVariant)

#ifndef QT_MOC

template<typename T> inline T qvariant_cast(const QVariant &v)
{
    QMetaType targetType = QMetaType::fromType<T>();
    if (v.d.type() == targetType)
        return v.d.get<T>();
    if constexpr (std::is_same_v<T,std::remove_const_t<std::remove_pointer_t<T>> const *>) {
        using nonConstT = std::remove_const_t<std::remove_pointer_t<T>> *;
        QMetaType nonConstTargetType = QMetaType::fromType<nonConstT>();
        if (v.d.type() == nonConstTargetType)
            return v.d.get<nonConstT>();
    }

    T t{};
    QMetaType::convert(v.metaType(), v.constData(), targetType, &t);
    return t;
}

template<> inline QVariant qvariant_cast<QVariant>(const QVariant &v)
{
    if (v.metaType().id() == QMetaType::QVariant)
        return *reinterpret_cast<const QVariant *>(v.constData());
    return v;
}

#endif

#ifndef QT_NO_DEBUG_STREAM
#if QT_DEPRECATED_SINCE(6, 0)
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
QT_DEPRECATED_VERSION_6_0
Q_CORE_EXPORT QDebug operator<<(QDebug, const QVariant::Type);
QT_WARNING_POP
#endif
#endif

namespace QtPrivate {
class Q_CORE_EXPORT QVariantTypeCoercer
{
public:
    // ### Qt7: Pass QMetaType as value rather than const ref.
    const void *convert(const QVariant &value, const QMetaType &type);
    const void *coerce(const QVariant &value, const QMetaType &type);

private:
    QVariant converted;
};
}

template<typename Pointer>
class QVariantRef
{
private:
    const Pointer *m_pointer = nullptr;

public:
    explicit QVariantRef(const Pointer *reference) : m_pointer(reference) {}
    QVariantRef(const QVariantRef &) = default;
    QVariantRef(QVariantRef &&) = default;
    ~QVariantRef() = default;

    operator QVariant() const;
    QVariantRef &operator=(const QVariant &value);
    QVariantRef &operator=(const QVariantRef &value) { return operator=(QVariant(value)); }
    QVariantRef &operator=(QVariantRef &&value) { return operator=(QVariant(value)); }

    friend void swap(QVariantRef a, QVariantRef b)
    {
        QVariant tmp = a;
        a = b;
        b = std::move(tmp);
    }
};

class Q_CORE_EXPORT QVariantConstPointer
{
private:
    QVariant m_variant;

public:
    explicit QVariantConstPointer(QVariant variant);

    QVariant operator*() const;
    const QVariant *operator->() const;
};

template<typename Pointer>
class QVariantPointer
{
private:
    const Pointer *m_pointer = nullptr;

public:
    explicit QVariantPointer(const Pointer *pointer) : m_pointer(pointer) {}
    QVariantRef<Pointer> operator*() const { return QVariantRef<Pointer>(m_pointer); }
    Pointer operator->() const { return *m_pointer; }
};

QT_END_NAMESPACE

#endif // QVARIANT_H
