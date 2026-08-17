// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCORE_QEXCEPTION_H
#define QTCORE_QEXCEPTION_H

#include <QtCore/qatomic.h>
#include <QtCore/qshareddata.h>

#ifndef QT_NO_EXCEPTIONS
#  include <exception>
#endif

QT_REQUIRE_CONFIG(future);

QT_BEGIN_NAMESPACE


#if !defined(QT_NO_EXCEPTIONS) || defined(Q_CLANG_QDOC)

class Q_CORE_EXPORT QException : public std::exception
{
public:
    ~QException() noexcept;
    virtual void raise() const;
    virtual QException *clone() const;
};

class QUnhandledExceptionPrivate;
class Q_CORE_EXPORT QUnhandledException final : public QException
{
public:
    QUnhandledException(std::exception_ptr exception = nullptr) noexcept;
    ~QUnhandledException() noexcept override;

    QUnhandledException(QUnhandledException &&other) noexcept;
    QUnhandledException(const QUnhandledException &other) noexcept;

    void swap(QUnhandledException &other) noexcept { d.swap(other.d); }

    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QUnhandledException)
    QUnhandledException &operator=(const QUnhandledException &other) noexcept;

    void raise() const override;
    QUnhandledException *clone() const override;

    std::exception_ptr exception() const;

private:
    QSharedDataPointer<QUnhandledExceptionPrivate> d;
};

namespace QtPrivate {

class Q_CORE_EXPORT ExceptionStore
{
public:
    void setException(const QException &e);
    void setException(std::exception_ptr e);
    bool hasException() const;
    std::exception_ptr exception() const;
    void throwPossibleException();
    Q_NORETURN void rethrowException() const;
    std::exception_ptr exceptionHolder;
};

} // namespace QtPrivate

#else // QT_NO_EXCEPTIONS

namespace QtPrivate {

class Q_CORE_EXPORT ExceptionStore
{
public:
    ExceptionStore() { }
    inline void throwPossibleException() {}
    inline void rethrowException() const { }
};

} // namespace QtPrivate

#endif // QT_NO_EXCEPTIONS

QT_END_NAMESPACE

#endif
