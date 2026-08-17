// Copyright (C) 2022 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QJNITYPES_H
#define QJNITYPES_H

#include <QtCore/qglobal.h>

#if defined(Q_QDOC) || defined(Q_OS_ANDROID)
#include <jni.h>

QT_BEGIN_NAMESPACE

namespace QtJniTypes
{

// a constexpr type for string literals of any character width, aware of the length
// of the string.
template<size_t N_WITH_NULL, typename BaseType = char>
struct String
{
    BaseType m_data[N_WITH_NULL] = {};

    constexpr String() noexcept {}
    // Can be instantiated (only) with a string literal
    constexpr explicit String(const BaseType (&data)[N_WITH_NULL]) noexcept
    {
        for (size_t i = 0; i < N_WITH_NULL - 1; ++i)
            m_data[i] = data[i];
    }

    constexpr BaseType at(size_t i) const { return m_data[i]; }
    constexpr BaseType operator[](size_t i) const { return at(i); }
    static constexpr size_t size() noexcept { return N_WITH_NULL; }
    constexpr operator const BaseType *() const noexcept { return m_data; }
    constexpr const BaseType *data() const noexcept { return m_data; }
    template<size_t N2_WITH_NULL>
    constexpr bool startsWith(const BaseType (&lit)[N2_WITH_NULL]) const noexcept
    {
        if constexpr (N2_WITH_NULL > N_WITH_NULL) {
            return false;
        } else {
            for (size_t i = 0; i < N2_WITH_NULL - 1; ++i) {
                if (m_data[i] != lit[i])
                    return false;
            }
        }
        return true;
    }
    constexpr bool startsWith(BaseType c) const noexcept
    {
        return N_WITH_NULL > 1 && m_data[0] == c;
    }
    template<size_t N2_WITH_NULL>
    constexpr bool endsWith(const BaseType (&lit)[N2_WITH_NULL]) const noexcept
    {
        if constexpr (N2_WITH_NULL > N_WITH_NULL) {
            return false;
        } else {
            for (size_t i = 0; i < N2_WITH_NULL; ++i) {
                if (m_data[N_WITH_NULL - i - 1] != lit[N2_WITH_NULL - i - 1])
                    return false;
            }
        }
        return true;
    }
    constexpr bool endsWith(BaseType c) const noexcept
    {
        return N_WITH_NULL > 1 && m_data[N_WITH_NULL - 2] == c;
    }

    template<size_t N2_WITH_NULL>
    friend inline constexpr bool operator==(const String<N_WITH_NULL> &lhs,
                                            const String<N2_WITH_NULL> &rhs) noexcept
    {
        if constexpr (N_WITH_NULL != N2_WITH_NULL) {
            return false;
        } else {
            for (size_t i = 0; i < N_WITH_NULL - 1; ++i) {
                if (lhs.at(i) != rhs.at(i))
                    return false;
            }
        }
        return true;
    }

    template<size_t N2_WITH_NULL>
    friend inline constexpr bool operator!=(const String<N_WITH_NULL> &lhs,
                                            const String<N2_WITH_NULL> &rhs) noexcept
    {
        return !operator==(lhs, rhs);
    }

    template<size_t N2_WITH_NULL>
    friend inline constexpr bool operator==(const String<N_WITH_NULL> &lhs,
                                            const BaseType (&rhs)[N2_WITH_NULL]) noexcept
    {
        return operator==(lhs, String<N2_WITH_NULL>(rhs));
    }
    template<size_t N2_WITH_NULL>
    friend inline constexpr bool operator==(const BaseType (&lhs)[N2_WITH_NULL],
                                            const String<N_WITH_NULL> &rhs) noexcept
    {
        return operator==(String<N2_WITH_NULL>(lhs), rhs);
    }

    template<size_t N2_WITH_NULL>
    friend inline constexpr bool operator!=(const String<N_WITH_NULL> &lhs,
                                            const BaseType (&rhs)[N2_WITH_NULL]) noexcept
    {
        return operator!=(lhs, String<N2_WITH_NULL>(rhs));
    }
    template<size_t N2_WITH_NULL>
    friend inline constexpr bool operator!=(const BaseType (&lhs)[N2_WITH_NULL],
                                            const String<N_WITH_NULL> &rhs) noexcept
    {
        return operator!=(String<N2_WITH_NULL>(lhs), rhs);
    }

    template<size_t N2_WITH_NULL>
    friend inline constexpr auto operator+(const String<N_WITH_NULL> &lhs,
                                           const String<N2_WITH_NULL> &rhs) noexcept
    {
        char data[N_WITH_NULL + N2_WITH_NULL - 1] = {};
        for (size_t i = 0; i < N_WITH_NULL - 1; ++i)
            data[i] = lhs[i];
        for (size_t i = 0; i < N2_WITH_NULL - 1; ++i)
            data[N_WITH_NULL - 1 + i] = rhs[i];
        return String<N_WITH_NULL + N2_WITH_NULL - 1>(data);
    }
};


// Helper types that allow us to disable variadic overloads that would conflict
// with overloads that take a const char*.
template<typename T, size_t N = 0> struct IsStringType : std::false_type {};
template<> struct IsStringType<const char*, 0> : std::true_type {};
template<size_t N> struct IsStringType<String<N>> : std::true_type {};
template<size_t N> struct IsStringType<const char[N]> : std::true_type {};

template<bool flag = false>
static void staticAssertTypeMismatch()
{
    static_assert(flag, "The used type is not supported by this template call. "
                        "Use a JNI based type instead.");
}

template<typename T>
constexpr auto typeSignature()
{
    if constexpr(std::is_same<T, jobject>::value)
        return String("Ljava/lang/Object;");
    else if constexpr(std::is_same<T, jclass>::value)
        return String("Ljava/lang/Class;");
    else if constexpr(std::is_same<T, jstring>::value)
        return String("Ljava/lang/String;");
    else if constexpr(std::is_same<T, jobjectArray>::value)
        return String("[Ljava/lang/Object;");
    else if constexpr(std::is_same<T, jthrowable>::value)
        return String("Ljava/lang/Throwable;");
    else if constexpr(std::is_same<T, jbooleanArray>::value)
        return String("[Z");
    else if constexpr(std::is_same<T, jbyteArray>::value)
        return String("[B");
    else if constexpr(std::is_same<T, jshortArray>::value)
        return String("[S");
    else if constexpr(std::is_same<T, jintArray>::value)
        return String("[I");
    else if constexpr(std::is_same<T, jlongArray>::value)
        return String("[J");
    else if constexpr(std::is_same<T, jfloatArray>::value)
        return String("[F");
    else if constexpr(std::is_same<T, jdoubleArray>::value)
        return String("[D");
    else if constexpr(std::is_same<T, jcharArray>::value)
        return String("[C");
    else if constexpr(std::is_same<T, jboolean>::value)
        return String("Z");
    else if constexpr(std::is_same<T, bool>::value)
        return String("Z");
    else if constexpr(std::is_same<T, jbyte>::value)
        return String("B");
    else if constexpr(std::is_same<T, jchar>::value)
        return String("C");
    else if constexpr(std::is_same<T, char>::value)
        return String("C");
    else if constexpr(std::is_same<T, jshort>::value)
        return String("S");
    else if constexpr(std::is_same<T, short>::value)
        return String("S");
    else if constexpr(std::is_same<T, jint>::value)
        return String("I");
    else if constexpr(std::is_same<T, int>::value)
        return String("I");
    else if constexpr(std::is_same<T, uint>::value)
        return String("I");
    else if constexpr(std::is_same<T, jlong>::value)
        return String("J");
    else if constexpr(std::is_same<T, long>::value)
        return String("J");
    else if constexpr(std::is_same<T, jfloat>::value)
        return String("F");
    else if constexpr(std::is_same<T, float>::value)
        return String("F");
    else if constexpr(std::is_same<T, jdouble>::value)
        return String("D");
    else if constexpr(std::is_same<T, double>::value)
        return String("D");
    else if constexpr(std::is_same<T, void>::value)
        return String("V");
    else if constexpr(IsStringType<T>::value)
        static_assert(!IsStringType<T>::value, "Don't use a literal type, call data!");
    else
        staticAssertTypeMismatch();
}

template<bool flag = false>
static void staticAssertClassNotRegistered()
{
    static_assert(flag, "Class not registered, use Q_DECLARE_JNI_CLASS");
}

template<typename T>
constexpr auto className()
{
    if constexpr(std::is_same<T, jstring>::value)
        return String("java/lang/String");
    else
        staticAssertClassNotRegistered();
}

template<typename T>
static constexpr bool isPrimitiveType()
{
    return typeSignature<T>().size() == 2;
}

template<typename T>
static constexpr bool isObjectType()
{
    if constexpr(std::is_convertible<T, jobject>::value) {
        return true;
    } else {
        constexpr auto signature = typeSignature<T>();
        return (signature.startsWith('L') || signature.startsWith('['))
             && signature.endsWith(';');
    }
}

template<typename T>
static constexpr void assertPrimitiveType()
{
    static_assert(isPrimitiveType<T>(), "Type needs to be a primitive JNI type!");
}

template<typename T>
static constexpr void assertObjectType()
{
    static_assert(isObjectType<T>(),
                  "Type needs to be a JNI object type (convertible to jobject, or with "
                  "an object type signature registered)!");
}

template<typename T>
static constexpr void assertType()
{
    static_assert(isPrimitiveType<T>() || isObjectType<T>(),
                  "Type needs to be a JNI type!");
}

template<typename R, typename ...Args>
static constexpr auto methodSignature()
{
    return (String("(") +
                ... + typeSignature<std::decay_t<Args>>())
            + String(")")
            + typeSignature<R>();
}

template<typename T>
static constexpr auto fieldSignature()
{
    return QtJniTypes::typeSignature<T>();
}

template<typename ...Args>
static constexpr auto constructorSignature()
{
    return methodSignature<void, Args...>();
}

template<typename Ret, typename ...Args>
static constexpr auto nativeMethodSignature(Ret (*)(JNIEnv *, jobject, Args...))
{
    return methodSignature<Ret, Args...>();
}

template<typename Ret, typename ...Args>
static constexpr auto nativeMethodSignature(Ret (*)(JNIEnv *, jclass, Args...))
{
    return methodSignature<Ret, Args...>();
}

// A generic thin wrapper around jobject, convertible to jobject.
// We need this as a baseclass so that QJniObject can be implicitly
// constructed from the various subclasses - we can't provide an
// operator QJniObject() here as the class is not declared.
struct Object
{
    jobject _object;
    constexpr operator jobject() const { return _object; }
};

} // namespace QtJniTypes

#define Q_DECLARE_JNI_TYPE_HELPER(Type)                         \
namespace QtJniTypes {                                          \
struct Type : Object                                            \
{                                                               \
    constexpr Type(jobject o) noexcept : Object{o} {}           \
};                                                              \
}                                                               \


#define Q_DECLARE_JNI_TYPE(Type, Signature)                     \
Q_DECLARE_JNI_TYPE_HELPER(Type)                                 \
template<>                                                      \
constexpr auto QtJniTypes::typeSignature<QtJniTypes::Type>()    \
{                                                               \
    static_assert((Signature[0] == 'L' || Signature[0] == '[')  \
                && Signature[sizeof(Signature) - 2] == ';',     \
                "Type signature needs to start with 'L' or '['" \
                " and end with ';'");                           \
    return QtJniTypes::String(Signature);                       \
}                                                               \

#define Q_DECLARE_JNI_CLASS(Type, Signature)                    \
Q_DECLARE_JNI_TYPE_HELPER(Type)                                 \
template<>                                                      \
constexpr auto QtJniTypes::className<QtJniTypes::Type>()        \
{                                                               \
    return QtJniTypes::String(Signature);                       \
}                                                               \
template<>                                                      \
constexpr auto QtJniTypes::typeSignature<QtJniTypes::Type>()    \
{                                                               \
    return QtJniTypes::String("L")                              \
         + QtJniTypes::String(Signature)                        \
         + QtJniTypes::String(";");                             \
}                                                               \

#define Q_DECLARE_JNI_NATIVE_METHOD(...)                           \
    QT_OVERLOADED_MACRO(QT_DECLARE_JNI_NATIVE_METHOD, __VA_ARGS__) \

#define QT_DECLARE_JNI_NATIVE_METHOD_2(Method, Name)            \
namespace QtJniMethods {                                        \
static constexpr auto Method##_signature =                      \
    QtJniTypes::nativeMethodSignature(Method);                  \
static const JNINativeMethod Method##_method = {                \
    #Name, Method##_signature.data(),                           \
    reinterpret_cast<void *>(Method)                            \
};                                                              \
}                                                               \

#define QT_DECLARE_JNI_NATIVE_METHOD_1(Method)                  \
    QT_DECLARE_JNI_NATIVE_METHOD_2(Method, Method)              \

#define Q_JNI_NATIVE_METHOD(Method) QtJniMethods::Method##_method

#define Q_DECLARE_JNI_NATIVE_METHOD_IN_CURRENT_SCOPE(...)                                        \
    QT_OVERLOADED_MACRO(QT_DECLARE_JNI_NATIVE_METHOD_IN_CURRENT_SCOPE, __VA_ARGS__)              \

#define QT_DECLARE_JNI_NATIVE_METHOD_IN_CURRENT_SCOPE_2(Method, Name)                            \
    static inline constexpr auto Method##_signature = QtJniTypes::nativeMethodSignature(Method); \
    static inline const JNINativeMethod Method##_method = {                                      \
        #Name, Method##_signature.data(), reinterpret_cast<void *>(Method)                       \
    };

#define QT_DECLARE_JNI_NATIVE_METHOD_IN_CURRENT_SCOPE_1(Method)                                  \
    QT_DECLARE_JNI_NATIVE_METHOD_IN_CURRENT_SCOPE_2(Method, Method)                              \

#define Q_JNI_NATIVE_SCOPED_METHOD(Method, Scope) Scope::Method##_method

QT_END_NAMESPACE

#endif

#endif // QJNITYPES_H
