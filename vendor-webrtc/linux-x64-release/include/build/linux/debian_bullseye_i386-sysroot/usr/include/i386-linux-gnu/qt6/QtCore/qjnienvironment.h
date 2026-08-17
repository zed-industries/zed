// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QJNI_ENVIRONMENT_H
#define QJNI_ENVIRONMENT_H

#include <QtCore/QScopedPointer>

#if defined(Q_QDOC) || defined(Q_OS_ANDROID)
#include <jni.h>
#include <QtCore/qjnitypes.h>

QT_BEGIN_NAMESPACE

class QJniEnvironmentPrivate;

class Q_CORE_EXPORT QJniEnvironment
{
public:
    QJniEnvironment();
    ~QJniEnvironment();
    bool isValid() const;
    JNIEnv *operator->() const;
    JNIEnv &operator*() const;
    JNIEnv *jniEnv() const;
    jclass findClass(const char *className);
    template<typename Class>
    jclass findClass() { return findClass(QtJniTypes::className<Class>().data()); }
    jmethodID findMethod(jclass clazz, const char *methodName, const char *signature);
    template<typename ...Args>
    jmethodID findMethod(jclass clazz, const char *methodName) {
        constexpr auto signature = QtJniTypes::methodSignature<Args...>();
        return findMethod(clazz, methodName, signature.data());
    }
    jmethodID findStaticMethod(jclass clazz, const char *methodName, const char *signature);
    template<typename ...Args>
    jmethodID findStaticMethod(jclass clazz, const char *methodName) {
        constexpr auto signature = QtJniTypes::methodSignature<Args...>();
        return findStaticMethod(clazz, methodName, signature.data());
    }
    jfieldID findField(jclass clazz, const char *fieldName, const char *signature);
    template<typename T>
    jfieldID findField(jclass clazz, const char *fieldName) {
        constexpr auto signature = QtJniTypes::fieldSignature<T>();
        return findField(clazz, fieldName, signature.data());
    }
    jfieldID findStaticField(jclass clazz, const char *fieldName, const char *signature);
    template<typename T>
    jfieldID findStaticField(jclass clazz, const char *fieldName) {
        constexpr auto signature = QtJniTypes::fieldSignature<T>();
        return findStaticField(clazz, fieldName, signature.data());
    }
    static JavaVM *javaVM();
    bool registerNativeMethods(const char *className, const JNINativeMethod methods[], int size);
    bool registerNativeMethods(jclass clazz, const JNINativeMethod methods[], int size);

    bool registerNativeMethods(const char *className, std::initializer_list<JNINativeMethod> methods)
    {
        return registerNativeMethods(className, std::data(methods), methods.size());
    }

    bool registerNativeMethods(jclass clazz, std::initializer_list<JNINativeMethod> methods)
    {
        return registerNativeMethods(clazz, std::data(methods), methods.size());
    }

#if QT_DEPRECATED_SINCE(6, 2)
    // ### Qt 7: remove
    QT_DEPRECATED_VERSION_X_6_2("Use the overload with a const JNINativeMethod[] instead.")
    bool registerNativeMethods(const char *className, JNINativeMethod methods[], int size);
#endif

    enum class OutputMode {
        Silent,
        Verbose
    };

    bool checkAndClearExceptions(OutputMode outputMode = OutputMode::Verbose);
    static bool checkAndClearExceptions(JNIEnv *env, OutputMode outputMode = OutputMode::Verbose);

private:
    Q_DISABLE_COPY_MOVE(QJniEnvironment)
    QScopedPointer<QJniEnvironmentPrivate> d;
};

QT_END_NAMESPACE

#endif

#endif // QJNI_ENVIRONMENT_H
