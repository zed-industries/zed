// Copyright (C) 2022 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QJNIOBJECT_H
#define QJNIOBJECT_H

#include <QtCore/qsharedpointer.h>

#if defined(Q_QDOC) || defined(Q_OS_ANDROID)
#include <jni.h>
#include <QtCore/qjnienvironment.h>
#include <QtCore/qjnitypes.h>

QT_BEGIN_NAMESPACE

class QJniObjectPrivate;

class Q_CORE_EXPORT QJniObject
{
public:
    QJniObject();
    explicit QJniObject(const char *className);
    explicit QJniObject(const char *className, const char *signature, ...);
    template<typename ...Args
#ifndef Q_QDOC
        , std::enable_if_t<!std::disjunction_v<QtJniTypes::IsStringType<std::decay_t<Args>>...>>* = nullptr
#endif
        >
    explicit QJniObject(const char *className, Args &&...args)
        : QJniObject(className, QtJniTypes::constructorSignature<Args...>().data(),
                     std::forward<Args>(args)...)
    {}
    explicit QJniObject(jclass clazz);
    explicit QJniObject(jclass clazz, const char *signature, ...);
    template<typename ...Args
#ifndef Q_QDOC
        , std::enable_if_t<!std::disjunction_v<QtJniTypes::IsStringType<std::decay_t<Args>>...>>* = nullptr
#endif
        >
    explicit QJniObject(jclass clazz, Args &&...args)
        : QJniObject(clazz, QtJniTypes::constructorSignature<Args...>().data(),
                     std::forward<Args>(args)...)
    {}
    QJniObject(jobject globalRef);
    inline QJniObject(QtJniTypes::Object wrapper) noexcept : QJniObject(jobject(wrapper)) {}
    ~QJniObject();

    template<typename Class, typename ...Args>
    static inline QJniObject construct(Args &&...args)
    {
        return QJniObject(QtJniTypes::className<Class>().data(),
                          QtJniTypes::constructorSignature<Args...>().data(),
                          std::forward<Args>(args)...);
    }

    jobject object() const;
    template <typename T> T object() const
    {
        QtJniTypes::assertObjectType<T>();
        return static_cast<T>(javaObject());
    }

    jclass objectClass() const;
    QByteArray className() const;

    template <typename Ret, typename ...Args>
    auto callMethod(const char *methodName, const char *signature, Args &&...args) const
    {
        if constexpr (QtJniTypes::isObjectType<Ret>()) {
            return callObjectMethod(methodName, signature, std::forward<Args>(args)...);
        } else {
            QtJniTypes::assertPrimitiveType<Ret>();
            QJniEnvironment env;
            jmethodID id = getCachedMethodID(env.jniEnv(), methodName, signature);
            if (id) {
                if constexpr (std::is_same<Ret, void>::value) {
                    callVoidMethodV(env.jniEnv(), id, std::forward<Args>(args)...);
                    env.checkAndClearExceptions();
                } else {
                    Ret res{};
                    callMethodForType<Ret>(env.jniEnv(), res, object(), id, std::forward<Args>(args)...);
                    if (env.checkAndClearExceptions())
                        res = {};
                    return res;
                }
            }
            if constexpr (!std::is_same<Ret, void>::value)
                return Ret{};
        }
    }

    template <typename Ret, typename ...Args>
    auto callMethod(const char *methodName, Args &&...args) const
    {
        constexpr auto signature = QtJniTypes::methodSignature<Ret, Args...>();
        if constexpr (std::is_same<Ret, void>::value) {
            callMethod<void>(methodName, signature.data(), std::forward<Args>(args)...);
        } else {
            return callMethod<Ret>(methodName, signature.data(), std::forward<Args>(args)...);
        }
    }

    template <typename Ret, typename ...Args>
    QJniObject callObjectMethod(const char *methodName, Args &&...args) const
    {
        QtJniTypes::assertObjectType<Ret>();
        constexpr auto signature = QtJniTypes::methodSignature<Ret, Args...>();
        return callObjectMethod(methodName, signature.data(), std::forward<Args>(args)...);
    }

    QJniObject callObjectMethod(const char *methodName, const char *signature, ...) const;

    template <typename Ret, typename ...Args>
    static auto callStaticMethod(const char *className, const char *methodName, const char *signature, Args &&...args)
    {
        QJniEnvironment env;
        jclass clazz = QJniObject::loadClass(className, env.jniEnv());
        return callStaticMethod<Ret>(clazz, methodName, signature, std::forward<Args>(args)...);
    }

    template <typename Ret, typename ...Args>
    static auto callStaticMethod(jclass clazz, const char *methodName, const char *signature, Args &&...args)
    {
        QJniEnvironment env;
        jmethodID id = getMethodID(env.jniEnv(), clazz, methodName, signature, true);
        return callStaticMethod<Ret, Args...>(clazz, id, std::forward<Args>(args)...);
    }

    template <typename Ret, typename ...Args>
    static auto callStaticMethod(jclass clazz, jmethodID methodId, Args &&...args)
    {
        if constexpr (QtJniTypes::isObjectType<Ret>()) {
            return callStaticObjectMethod(clazz, methodId, std::forward<Args>(args)...);
        } else {
            QtJniTypes::assertPrimitiveType<Ret>();
            QJniEnvironment env;
            if (clazz && methodId) {
                if constexpr (std::is_same<Ret, void>::value) {
                    callStaticMethodForVoid(env.jniEnv(), clazz, methodId, std::forward<Args>(args)...);
                    env.checkAndClearExceptions();
                } else {
                    Ret res{};
                    callStaticMethodForType<Ret>(env.jniEnv(), res, clazz, methodId, std::forward<Args>(args)...);
                    if (env.checkAndClearExceptions())
                        res = {};
                    return res;
                }
            }
            if constexpr (!std::is_same<Ret, void>::value)
                return Ret{};
        }
    }

    template <typename Ret, typename ...Args>
    static auto callStaticMethod(const char *className, const char *methodName, Args &&...args)
    {
        QJniEnvironment env;
        jclass clazz = QJniObject::loadClass(className, env.jniEnv());
        return callStaticMethod<Ret, Args...>(clazz, methodName, std::forward<Args>(args)...);
    }

    template <typename Ret, typename ...Args>
    static auto callStaticMethod(jclass clazz, const char *methodName, Args &&...args)
    {
        constexpr auto signature = QtJniTypes::methodSignature<Ret, Args...>();
        return callStaticMethod<Ret>(clazz, methodName, signature.data(), std::forward<Args>(args)...);
    }

    static QJniObject callStaticObjectMethod(const char *className, const char *methodName,
                                             const char *signature, ...);

    static QJniObject callStaticObjectMethod(jclass clazz, const char *methodName,
                                             const char *signature, ...);

    static QJniObject callStaticObjectMethod(jclass clazz, jmethodID methodId, ...);


    template <typename Ret, typename ...Args>
    static QJniObject callStaticObjectMethod(const char *className, const char *methodName, Args &&...args)
    {
        QtJniTypes::assertObjectType<Ret>();
        constexpr auto signature = QtJniTypes::methodSignature<Ret, Args...>();
        return callStaticObjectMethod(className, methodName, signature.data(), std::forward<Args>(args)...);
    }

    template <typename Ret, typename ...Args>
    static QJniObject callStaticObjectMethod(jclass clazz, const char *methodName, Args &&...args)
    {
        QtJniTypes::assertObjectType<Ret>();
        constexpr auto signature = QtJniTypes::methodSignature<Ret, Args...>();
        return callStaticObjectMethod(clazz, methodName, signature.data(), std::forward<Args>(args)...);
    }

    template <typename T> auto getField(const char *fieldName) const
    {
        if constexpr (QtJniTypes::isObjectType<T>()) {
            return getObjectField<T>(fieldName);
        } else {
            QtJniTypes::assertPrimitiveType<T>();
            QJniEnvironment env;
            T res{};
            constexpr auto signature = QtJniTypes::fieldSignature<T>();
            jfieldID id = getCachedFieldID(env.jniEnv(), fieldName, signature);
            if (id) {
                getFieldForType<T>(env.jniEnv(), res, object(), id);
                if (env.checkAndClearExceptions())
                    res = {};
            }
            return res;
        }
    }

    template <typename T>
    static auto getStaticField(const char *className, const char *fieldName)
    {
        if constexpr (QtJniTypes::isObjectType<T>()) {
            return getStaticObjectField<T>(className, fieldName);
        } else {
            QtJniTypes::assertPrimitiveType<T>();
            QJniEnvironment env;
            jclass clazz = QJniObject::loadClass(className, env.jniEnv());
            T res{};
            if (!clazz)
                return res;

            constexpr auto signature = QtJniTypes::fieldSignature<T>();
            jfieldID id = getCachedFieldID(env.jniEnv(), clazz,
                                        QJniObject::toBinaryEncClassName(className),
                                        fieldName,
                                        signature, true);
            if (!id)
                return res;

            getStaticFieldForType<T>(env.jniEnv(), res, clazz, id);
            if (env.checkAndClearExceptions())
                res = {};
            return res;
        }
    }

    template <typename T>
    static auto getStaticField(jclass clazz, const char *fieldName)
    {
        if constexpr (QtJniTypes::isObjectType<T>()) {
            return getStaticObjectField<T>(clazz, fieldName);
        } else {
            QtJniTypes::assertPrimitiveType<T>();
            QJniEnvironment env;
            T res{};
            constexpr auto signature = QtJniTypes::fieldSignature<T>();
            jfieldID id = getFieldID(env.jniEnv(), clazz, fieldName, signature, true);
            if (id) {
                getStaticFieldForType<T>(env.jniEnv(), res, clazz, id);
                if (env.checkAndClearExceptions())
                    res = {};
            }
            return res;
        }
    }

    template <typename Klass, typename T>
    static auto getStaticField(const char *fieldName)
    {
        return getStaticField<T>(QtJniTypes::className<Klass>(), fieldName);
    }

    template <typename T>
    QJniObject getObjectField(const char *fieldName) const
    {
        QtJniTypes::assertObjectType<T>();
        constexpr auto signature = QtJniTypes::fieldSignature<T>();
        return getObjectField(fieldName, signature);
    }

    QJniObject getObjectField(const char *fieldName, const char *signature) const;

    template <typename T>
    static QJniObject getStaticObjectField(const char *className, const char *fieldName)
    {
        QtJniTypes::assertObjectType<T>();
        constexpr auto signature = QtJniTypes::fieldSignature<T>();
        return getStaticObjectField(className, fieldName, signature);
    }

    static QJniObject getStaticObjectField(const char *className,
                                           const char *fieldName,
                                           const char *signature);

    template <typename T>
    static QJniObject getStaticObjectField(jclass clazz, const char *fieldName)
    {
        QtJniTypes::assertObjectType<T>();
        constexpr auto signature = QtJniTypes::fieldSignature<T>();
        return getStaticObjectField(clazz, fieldName, signature);
    }

    static QJniObject getStaticObjectField(jclass clazz, const char *fieldName,
                                           const char *signature);

    template <typename T> void setField(const char *fieldName, T value)
    {
        QtJniTypes::assertType<T>();
        QJniEnvironment env;
        constexpr auto signature = QtJniTypes::fieldSignature<T>();
        jfieldID id = getCachedFieldID(env.jniEnv(), fieldName, signature);
        if (id) {
            setFieldForType<T>(env.jniEnv(), object(), id, value);
            env.checkAndClearExceptions();
        }
    }

    template <typename T>
    void setField(const char *fieldName, const char *signature, T value)
    {
        QtJniTypes::assertType<T>();
        QJniEnvironment env;
        jfieldID id = getCachedFieldID(env.jniEnv(), fieldName, signature);
        if (id) {
            setFieldForType<T>(env.jniEnv(), object(), id, value);
            env.checkAndClearExceptions();
        }
    }

    template <typename T>
    static void setStaticField(const char *className, const char *fieldName, T value)
    {
        QtJniTypes::assertType<T>();
        QJniEnvironment env;
        jclass clazz = QJniObject::loadClass(className, env.jniEnv());
        if (!clazz)
            return;

        constexpr auto signature = QtJniTypes::fieldSignature<T>();
        jfieldID id = getCachedFieldID(env.jniEnv(), clazz, className, fieldName,
                                       signature, true);
        if (!id)
            return;

        setStaticFieldForType<T>(env.jniEnv(), clazz, id, value);
        env.checkAndClearExceptions();
    }

    template <typename T>
    static void setStaticField(const char *className, const char *fieldName,
                               const char *signature, T value)
    {
        QtJniTypes::assertType<T>();
        QJniEnvironment env;
        jclass clazz = QJniObject::loadClass(className, env.jniEnv());

        if (!clazz)
            return;

        jfieldID id = getCachedFieldID(env.jniEnv(), clazz, className, fieldName,
                                       signature, true);
        if (id) {
            setStaticFieldForType<T>(env.jniEnv(), clazz, id, value);
            env.checkAndClearExceptions();
        }
    }

    template <typename T>
    static void setStaticField(jclass clazz, const char *fieldName,
                               const char *signature, T value)
    {
        QtJniTypes::assertType<T>();
        QJniEnvironment env;
        jfieldID id = getFieldID(env.jniEnv(), clazz, fieldName, signature, true);

        if (id) {
            setStaticFieldForType<T>(env.jniEnv(), clazz, id, value);
            env.checkAndClearExceptions();
        }
    }

    template <typename T>
    static void setStaticField(jclass clazz, const char *fieldName, T value)
    {
        QtJniTypes::assertType<T>();
        QJniEnvironment env;
        constexpr auto signature = QtJniTypes::fieldSignature<T>();
        jfieldID id = getFieldID(env.jniEnv(), clazz, fieldName, signature, true);
        if (id) {
            setStaticFieldForType<T>(env.jniEnv(), clazz, id, value);
            env.checkAndClearExceptions();
        }
    }

    template <typename Klass, typename T>
    static void setStaticField(const char *fieldName, T value)
    {
        setStaticField(QtJniTypes::className<Klass>(), fieldName, value);
    }

    static QJniObject fromString(const QString &string);
    QString toString() const;

    static bool isClassAvailable(const char *className);
    bool isValid() const;

    // This function takes ownership of the jobject and releases the local ref. before returning.
    static QJniObject fromLocalRef(jobject lref);

    template <typename T> QJniObject &operator=(T obj)
    {
        QtJniTypes::assertType<T>();
        assign(static_cast<T>(obj));
        return *this;
    }

private:
    struct QVaListPrivate { operator va_list &() const { return m_args; } va_list &m_args; };
    QJniObject(const char *className, const char *signature, const QVaListPrivate &args);
    QJniObject(jclass clazz, const char *signature, const QVaListPrivate &args);

    static jclass loadClass(const QByteArray &className, JNIEnv *env, bool binEncoded = false);
    static QByteArray toBinaryEncClassName(const QByteArray &className);
    static QJniObject getCleanJniObject(jobject obj);

    static jfieldID getCachedFieldID(JNIEnv *env, jclass clazz, const QByteArray &className,
                                     const char *name, const char *signature,
                                     bool isStatic = false);
    jfieldID getCachedFieldID(JNIEnv *env, const char *name, const char *signature,
                              bool isStatic = false) const;
    static jmethodID getCachedMethodID(JNIEnv *env, jclass clazz, const QByteArray &className,
                                       const char *name, const char *signature,
                                       bool isStatic = false);
    jmethodID getCachedMethodID(JNIEnv *env, const char *name, const char *signature,
                                bool isStatic = false) const;

    static jfieldID getFieldID(JNIEnv *env, jclass clazz, const char *name,
                               const char *signature, bool isStatic = false);
    static jmethodID getMethodID(JNIEnv *env, jclass clazz, const char *name,
                                 const char *signature, bool isStatic = false);

    void callVoidMethodV(JNIEnv *env, jmethodID id, ...) const;
    QJniObject callObjectMethodV(const char *methodName, const char *signature,
                                 va_list args) const;

    static QJniObject callStaticObjectMethodV(const char *className, const char *methodName,
                                              const char *signature, va_list args);

    static QJniObject callStaticObjectMethodV(jclass clazz, const char *methodName,
                                              const char *signature, va_list args);

    bool isSameObject(jobject obj) const;
    bool isSameObject(const QJniObject &other) const;
    void assign(jobject obj);
    jobject javaObject() const;

    friend bool operator==(const QJniObject &, const QJniObject &);
    friend bool operator!=(const QJniObject&, const QJniObject&);

    template<typename T>
    static constexpr void callMethodForType(JNIEnv *env, T &res, jobject obj,
                                            jmethodID id, ...)
    {
        va_list args = {};
        va_start(args, id);

        if constexpr(std::is_same<T, jboolean>::value)
            res = env->CallBooleanMethodV(obj, id, args);
        else if constexpr(std::is_same<T, jbyte>::value)
            res = env->CallByteMethodV(obj, id, args);
        else if constexpr(std::is_same<T, jchar>::value)
            res = env->CallCharMethodV(obj, id, args);
        else if constexpr(std::is_same<T, jshort>::value)
            res = env->CallShortMethodV(obj, id, args);
        else if constexpr(std::is_same<T, jint>::value)
            res = env->CallIntMethodV(obj, id, args);
        else if constexpr(std::is_same<T, jlong>::value)
            res = env->CallLongMethodV(obj, id, args);
        else if constexpr(std::is_same<T, jfloat>::value)
            res = env->CallFloatMethodV(obj, id, args);
        else if constexpr(std::is_same<T, jdouble>::value)
            res = env->CallDoubleMethodV(obj, id, args);
        else
            QtJniTypes::staticAssertTypeMismatch();
        va_end(args);
    }

    template<typename T>
    static constexpr void callStaticMethodForType(JNIEnv *env, T &res, jclass clazz,
                                                  jmethodID id, ...)
    {
        va_list args = {};
        va_start(args, id);
        if constexpr(std::is_same<T, jboolean>::value)
            res = env->CallStaticBooleanMethodV(clazz, id, args);
        else if constexpr(std::is_same<T, jbyte>::value)
            res = env->CallStaticByteMethodV(clazz, id, args);
        else if constexpr(std::is_same<T, jchar>::value)
            res = env->CallStaticCharMethodV(clazz, id, args);
        else if constexpr(std::is_same<T, jshort>::value)
            res = env->CallStaticShortMethodV(clazz, id, args);
        else if constexpr(std::is_same<T, jint>::value)
            res = env->CallStaticIntMethodV(clazz, id, args);
        else if constexpr(std::is_same<T, jlong>::value)
            res = env->CallStaticLongMethodV(clazz, id, args);
        else if constexpr(std::is_same<T, jfloat>::value)
            res = env->CallStaticFloatMethodV(clazz, id, args);
        else if constexpr(std::is_same<T, jdouble>::value)
            res = env->CallStaticDoubleMethodV(clazz, id, args);
        else
            QtJniTypes::staticAssertTypeMismatch();
        va_end(args);
    }

    static void callStaticMethodForVoid(JNIEnv *env, jclass clazz, jmethodID id, ...)
    {
        va_list args;
        va_start(args, id);
        env->CallStaticVoidMethodV(clazz, id, args);
        va_end(args);
    }


    template<typename T>
    static constexpr void getFieldForType(JNIEnv *env, T &res, jobject obj,
                                          jfieldID id)
    {
        if constexpr(std::is_same<T, jboolean>::value)
            res = env->GetBooleanField(obj, id);
        else if constexpr(std::is_same<T, jbyte>::value)
            res = env->GetByteField(obj, id);
        else if constexpr(std::is_same<T, jchar>::value)
            res = env->GetCharField(obj, id);
        else if constexpr(std::is_same<T, jshort>::value)
            res = env->GetShortField(obj, id);
        else if constexpr(std::is_same<T, jint>::value)
            res = env->GetIntField(obj, id);
        else if constexpr(std::is_same<T, jlong>::value)
            res = env->GetLongField(obj, id);
        else if constexpr(std::is_same<T, jfloat>::value)
            res = env->GetFloatField(obj, id);
        else if constexpr(std::is_same<T, jdouble>::value)
            res = env->GetDoubleField(obj, id);
        else
            QtJniTypes::staticAssertTypeMismatch();
    }

    template<typename T>
    static constexpr void getStaticFieldForType(JNIEnv *env, T &res, jclass clazz,
                                                jfieldID id)
    {
        if constexpr(std::is_same<T, jboolean>::value)
            res = env->GetStaticBooleanField(clazz, id);
        else if constexpr(std::is_same<T, jbyte>::value)
            res = env->GetStaticByteField(clazz, id);
        else if constexpr(std::is_same<T, jchar>::value)
            res = env->GetStaticCharField(clazz, id);
        else if constexpr(std::is_same<T, jshort>::value)
            res = env->GetStaticShortField(clazz, id);
        else if constexpr(std::is_same<T, jint>::value)
            res = env->GetStaticIntField(clazz, id);
        else if constexpr(std::is_same<T, jlong>::value)
            res = env->GetStaticLongField(clazz, id);
        else if constexpr(std::is_same<T, jfloat>::value)
            res = env->GetStaticFloatField(clazz, id);
        else if constexpr(std::is_same<T, jdouble>::value)
            res = env->GetStaticDoubleField(clazz, id);
        else
            QtJniTypes::staticAssertTypeMismatch();
    }

    template<typename T>
    static constexpr void setFieldForType(JNIEnv *env, jobject obj,
                                          jfieldID id, T value)
    {
        if constexpr(std::is_same<T, jboolean>::value)
            env->SetBooleanField(obj, id, value);
        else if constexpr(std::is_same<T, jbyte>::value)
            env->SetByteField(obj, id, value);
        else if constexpr(std::is_same<T, jchar>::value)
            env->SetCharField(obj, id, value);
        else if constexpr(std::is_same<T, jshort>::value)
            env->SetShortField(obj, id, value);
        else if constexpr(std::is_same<T, jint>::value)
            env->SetIntField(obj, id, value);
        else if constexpr(std::is_same<T, jlong>::value)
            env->SetLongField(obj, id, value);
        else if constexpr(std::is_same<T, jfloat>::value)
            env->SetFloatField(obj, id, value);
        else if constexpr(std::is_same<T, jdouble>::value)
            env->SetDoubleField(obj, id, value);
        else if constexpr(std::is_convertible<T, jobject>::value)
            env->SetObjectField(obj, id, value);
        else
            QtJniTypes::staticAssertTypeMismatch();
    }

    template<typename T>
    static constexpr void setStaticFieldForType(JNIEnv *env, jclass clazz,
                                          jfieldID id, T value)
    {
        if constexpr(std::is_same<T, jboolean>::value)
            env->SetStaticBooleanField(clazz, id, value);
        else if constexpr(std::is_same<T, jbyte>::value)
            env->SetStaticByteField(clazz, id, value);
        else if constexpr(std::is_same<T, jchar>::value)
            env->SetStaticCharField(clazz, id, value);
        else if constexpr(std::is_same<T, jshort>::value)
            env->SetStaticShortField(clazz, id, value);
        else if constexpr(std::is_same<T, jint>::value)
            env->SetStaticIntField(clazz, id, value);
        else if constexpr(std::is_same<T, jlong>::value)
            env->SetStaticLongField(clazz, id, value);
        else if constexpr(std::is_same<T, jfloat>::value)
            env->SetStaticFloatField(clazz, id, value);
        else if constexpr(std::is_same<T, jdouble>::value)
            env->SetStaticDoubleField(clazz, id, value);
        else if constexpr(std::is_convertible<T, jobject>::value)
            env->SetStaticObjectField(clazz, id, value);
        else
            QtJniTypes::staticAssertTypeMismatch();
    }

    friend QJniObjectPrivate;
    QSharedPointer<QJniObjectPrivate> d;
};

inline bool operator==(const QJniObject &obj1, const QJniObject &obj2)
{
    return obj1.isSameObject(obj2);
}

inline bool operator!=(const QJniObject &obj1, const QJniObject &obj2)
{
    return !obj1.isSameObject(obj2);
}

QT_END_NAMESPACE

#endif

#endif // QJNIOBJECT_H
