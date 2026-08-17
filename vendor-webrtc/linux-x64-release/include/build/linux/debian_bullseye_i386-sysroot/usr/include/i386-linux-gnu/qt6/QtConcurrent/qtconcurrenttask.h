// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENTTASK_H
#define QTCONCURRENTTASK_H

#if !defined(QT_NO_CONCURRENT)

#include <QtConcurrent/qtaskbuilder.h>

QT_BEGIN_NAMESPACE

#ifdef Q_CLANG_QDOC

namespace QtConcurrent {

template <class Task>
[[nodiscard]]
QTaskBuilder<Task> task(Task &&task);

} // namespace QtConcurrent

#else

namespace QtConcurrent {

template <class Task>
[[nodiscard]]
constexpr auto task(Task &&t) { return QTaskBuilder(std::forward<Task>(t)); }

} // namespace QtConcurrent

#endif // Q_CLANG_QDOC

QT_END_NAMESPACE

#endif // !defined(QT_NO_CONCURRENT)

#endif // QTCONCURRENTTASK_H
