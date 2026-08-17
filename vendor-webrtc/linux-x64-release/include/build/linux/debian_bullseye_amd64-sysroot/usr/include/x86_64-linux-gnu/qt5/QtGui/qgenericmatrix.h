/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QGENERICMATRIX_H
#define QGENERICMATRIX_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qmetatype.h>
#include <QtCore/qdebug.h>
#include <QtCore/qdatastream.h>

QT_BEGIN_NAMESPACE


template <int N, int M, typename T>
class QGenericMatrix
{
public:
    QGenericMatrix();
    explicit QGenericMatrix(Qt::Initialization) {}
    explicit QGenericMatrix(const T *values);

    const T& operator()(int row, int column) const;
    T& operator()(int row, int column);

    bool isIdentity() const;
    void setToIdentity();

    void fill(T value);

    Q_REQUIRED_RESULT QGenericMatrix<M, N, T> transposed() const;

    QGenericMatrix<N, M, T>& operator+=(const QGenericMatrix<N, M, T>& other);
    QGenericMatrix<N, M, T>& operator-=(const QGenericMatrix<N, M, T>& other);
    QGenericMatrix<N, M, T>& operator*=(T factor);
    QGenericMatrix<N, M, T>& operator/=(T divisor);
    bool operator==(const QGenericMatrix<N, M, T>& other) const;
    bool operator!=(const QGenericMatrix<N, M, T>& other) const;

    void copyDataTo(T *values) const;

    T *data() { return *m; }
    const T *data() const { return *m; }
    const T *constData() const { return *m; }

#if !defined(Q_NO_TEMPLATE_FRIENDS)
    template<int NN, int MM, typename TT>
    friend QGenericMatrix<NN, MM, TT> operator+(const QGenericMatrix<NN, MM, TT>& m1, const QGenericMatrix<NN, MM, TT>& m2);
    template<int NN, int MM, typename TT>
    friend QGenericMatrix<NN, MM, TT> operator-(const QGenericMatrix<NN, MM, TT>& m1, const QGenericMatrix<NN, MM, TT>& m2);
    template<int NN, int M1, int M2, typename TT>
    friend QGenericMatrix<M1, M2, TT> operator*(const QGenericMatrix<NN, M2, TT>& m1, const QGenericMatrix<M1, NN, TT>& m2);
    template<int NN, int MM, typename TT>
    friend QGenericMatrix<NN, MM, TT> operator-(const QGenericMatrix<NN, MM, TT>& matrix);
    template<int NN, int MM, typename TT>
    friend QGenericMatrix<NN, MM, TT> operator*(TT factor, const QGenericMatrix<NN, MM, TT>& matrix);
    template<int NN, int MM, typename TT>
    friend QGenericMatrix<NN, MM, TT> operator*(const QGenericMatrix<NN, MM, TT>& matrix, TT factor);
    template<int NN, int MM, typename TT>
    friend QGenericMatrix<NN, MM, TT> operator/(const QGenericMatrix<NN, MM, TT>& matrix, TT divisor);

private:
#endif
    T m[N][M];    // Column-major order to match OpenGL.

#if !defined(Q_NO_TEMPLATE_FRIENDS)
    template <int NN, int MM, typename TT>
    friend class QGenericMatrix;
#endif
};
template <int N, int M, typename T>
class QTypeInfo<QGenericMatrix<N, M, T> >
    : public QTypeInfoMerger<QGenericMatrix<N, M, T>, T>
{
#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
public:
    enum {
        isStatic = true,
    }; // at least Q_RELOCATABLE_TYPE, for BC during Qt 5
#endif
};

template <int N, int M, typename T>
Q_INLINE_TEMPLATE QGenericMatrix<N, M, T>::QGenericMatrix()
{
    setToIdentity();
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T>::QGenericMatrix(const T *values)
{
    for (int col = 0; col < N; ++col)
        for (int row = 0; row < M; ++row)
            m[col][row] = values[row * N + col];
}

template <int N, int M, typename T>
Q_INLINE_TEMPLATE const T& QGenericMatrix<N, M, T>::operator()(int row, int column) const
{
    Q_ASSERT(row >= 0 && row < M && column >= 0 && column < N);
    return m[column][row];
}

template <int N, int M, typename T>
Q_INLINE_TEMPLATE T& QGenericMatrix<N, M, T>::operator()(int row, int column)
{
    Q_ASSERT(row >= 0 && row < M && column >= 0 && column < N);
    return m[column][row];
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE bool QGenericMatrix<N, M, T>::isIdentity() const
{
    for (int col = 0; col < N; ++col) {
        for (int row = 0; row < M; ++row) {
            if (row == col) {
                if (m[col][row] != 1.0f)
                    return false;
            } else {
                if (m[col][row] != 0.0f)
                    return false;
            }
        }
    }
    return true;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE void QGenericMatrix<N, M, T>::setToIdentity()
{
    for (int col = 0; col < N; ++col) {
        for (int row = 0; row < M; ++row) {
            if (row == col)
                m[col][row] = 1.0f;
            else
                m[col][row] = 0.0f;
        }
    }
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE void QGenericMatrix<N, M, T>::fill(T value)
{
    for (int col = 0; col < N; ++col)
        for (int row = 0; row < M; ++row)
            m[col][row] = value;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<M, N, T> QGenericMatrix<N, M, T>::transposed() const
{
    QGenericMatrix<M, N, T> result(Qt::Uninitialized);
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            result.m[row][col] = m[col][row];
    return result;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T>& QGenericMatrix<N, M, T>::operator+=(const QGenericMatrix<N, M, T>& other)
{
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            m[col][row] += other.m[col][row];
    return *this;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T>& QGenericMatrix<N, M, T>::operator-=(const QGenericMatrix<N, M, T>& other)
{
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            m[col][row] -= other.m[col][row];
    return *this;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T>& QGenericMatrix<N, M, T>::operator*=(T factor)
{
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            m[col][row] *= factor;
    return *this;
}

QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wfloat-equal")
QT_WARNING_DISABLE_GCC("-Wfloat-equal")
QT_WARNING_DISABLE_INTEL(1572)

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE bool QGenericMatrix<N, M, T>::operator==(const QGenericMatrix<N, M, T>& other) const
{
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)  {
            if (m[col][row] != other.m[col][row])
                return false;
        }
    return true;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE bool QGenericMatrix<N, M, T>::operator!=(const QGenericMatrix<N, M, T>& other) const
{
    return !(*this == other);
}

QT_WARNING_POP

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T>& QGenericMatrix<N, M, T>::operator/=(T divisor)
{
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            m[col][row] /= divisor;
    return *this;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T> operator+(const QGenericMatrix<N, M, T>& m1, const QGenericMatrix<N, M, T>& m2)
{
    QGenericMatrix<N, M, T> result(Qt::Uninitialized);
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            result.m[col][row] = m1.m[col][row] + m2.m[col][row];
    return result;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T> operator-(const QGenericMatrix<N, M, T>& m1, const QGenericMatrix<N, M, T>& m2)
{
    QGenericMatrix<N, M, T> result(Qt::Uninitialized);
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            result.m[col][row] = m1.m[col][row] - m2.m[col][row];
    return result;
}

template <int N, int M1, int M2, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<M1, M2, T> operator*(const QGenericMatrix<N, M2, T>& m1, const QGenericMatrix<M1, N, T>& m2)
{
    QGenericMatrix<M1, M2, T> result(Qt::Uninitialized);
    for (int row = 0; row < M2; ++row) {
        for (int col = 0; col < M1; ++col) {
            T sum(0.0f);
            for (int j = 0; j < N; ++j)
                sum += m1.m[j][row] * m2.m[col][j];
            result.m[col][row] = sum;
        }
    }
    return result;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T> operator-(const QGenericMatrix<N, M, T>& matrix)
{
    QGenericMatrix<N, M, T> result(Qt::Uninitialized);
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            result.m[col][row] = -matrix.m[col][row];
    return result;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T> operator*(T factor, const QGenericMatrix<N, M, T>& matrix)
{
    QGenericMatrix<N, M, T> result(Qt::Uninitialized);
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            result.m[col][row] = matrix.m[col][row] * factor;
    return result;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T> operator*(const QGenericMatrix<N, M, T>& matrix, T factor)
{
    QGenericMatrix<N, M, T> result(Qt::Uninitialized);
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            result.m[col][row] = matrix.m[col][row] * factor;
    return result;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE QGenericMatrix<N, M, T> operator/(const QGenericMatrix<N, M, T>& matrix, T divisor)
{
    QGenericMatrix<N, M, T> result(Qt::Uninitialized);
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            result.m[col][row] = matrix.m[col][row] / divisor;
    return result;
}

template <int N, int M, typename T>
Q_OUTOFLINE_TEMPLATE void QGenericMatrix<N, M, T>::copyDataTo(T *values) const
{
    for (int col = 0; col < N; ++col)
        for (int row = 0; row < M; ++row)
            values[row * N + col] = T(m[col][row]);
}

// Define aliases for the useful variants of QGenericMatrix.
typedef QGenericMatrix<2, 2, float> QMatrix2x2;
typedef QGenericMatrix<2, 3, float> QMatrix2x3;
typedef QGenericMatrix<2, 4, float> QMatrix2x4;
typedef QGenericMatrix<3, 2, float> QMatrix3x2;
typedef QGenericMatrix<3, 3, float> QMatrix3x3;
typedef QGenericMatrix<3, 4, float> QMatrix3x4;
typedef QGenericMatrix<4, 2, float> QMatrix4x2;
typedef QGenericMatrix<4, 3, float> QMatrix4x3;

#ifndef QT_NO_DEBUG_STREAM

template <int N, int M, typename T>
QDebug operator<<(QDebug dbg, const QGenericMatrix<N, M, T> &m)
{
    QDebugStateSaver saver(dbg);
    dbg.nospace() << "QGenericMatrix<" << N << ", " << M
        << ", " << QTypeInfo<T>::name()
        << ">(" << Qt::endl << qSetFieldWidth(10);
    for (int row = 0; row < M; ++row) {
        for (int col = 0; col < N; ++col)
            dbg << m(row, col);
        dbg << Qt::endl;
    }
    dbg << qSetFieldWidth(0) << ')';
    return dbg;
}

#endif

#ifndef QT_NO_DATASTREAM

template <int N, int M, typename T>
QDataStream &operator<<(QDataStream &stream, const QGenericMatrix<N, M, T> &matrix)
{
    for (int row = 0; row < M; ++row)
        for (int col = 0; col < N; ++col)
            stream << double(matrix(row, col));
    return stream;
}

template <int N, int M, typename T>
QDataStream &operator>>(QDataStream &stream, QGenericMatrix<N, M, T> &matrix)
{
    double x;
    for (int row = 0; row < M; ++row) {
        for (int col = 0; col < N; ++col) {
            stream >> x;
            matrix(row, col) = T(x);
        }
    }
    return stream;
}

#endif

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QMatrix2x2)
Q_DECLARE_METATYPE(QMatrix2x3)
Q_DECLARE_METATYPE(QMatrix2x4)
Q_DECLARE_METATYPE(QMatrix3x2)
Q_DECLARE_METATYPE(QMatrix3x3)
Q_DECLARE_METATYPE(QMatrix3x4)
Q_DECLARE_METATYPE(QMatrix4x2)
Q_DECLARE_METATYPE(QMatrix4x3)

#endif
