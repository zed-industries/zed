/****************************************************************************
**
** Copyright (C) 2016 Intel Corporation.
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

// qglobal.h includes this header, so keep it outside of our include guards
#include <QtCore/qglobal.h>

#if !defined(QVERSIONTAGGING_H)
#define QVERSIONTAGGING_H

QT_BEGIN_NAMESPACE

/*
 * Ugly hack warning and explanation:
 *
 * This file causes all ELF modules, be they libraries or applications, to use the
 * qt_version_tag symbol that is present in QtCore. Such symbol is versioned,
 * so the linker will automatically pull the current Qt version and add it to
 * the ELF header of the library/application. The assembly produces one section
 * called ".qtversion" containing two 32-bit values. The first is a
 * relocation to the qt_version_tag symbol (which is what causes the ELF
 * version to get used). The second value is the current Qt version at the time
 * of compilation.
 *
 * There will only be one copy of the section in the output library or application.
 */

#if defined(QT_BUILD_CORE_LIB) || defined(QT_BOOTSTRAPPED) || defined(QT_NO_VERSION_TAGGING) || defined(QT_STATIC)
// don't make tags in QtCore, bootstrapped systems or if the user asked not to
#elif defined(Q_CC_GNU) && !defined(Q_OS_ANDROID)
#  if defined(Q_PROCESSOR_X86) && (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD_KERNEL))
#    if defined(Q_PROCESSOR_X86_64) && QT_POINTER_SIZE == 8     // x86-64 64-bit
#      define QT_VERSION_TAG_RELOC(sym) ".quad " QT_STRINGIFY(QT_MANGLE_NAMESPACE(sym)) "@GOT\n"
#    else                                                       // x86 or x86-64 32-bit (x32)
#      define QT_VERSION_TAG_RELOC(sym) ".long " QT_STRINGIFY(QT_MANGLE_NAMESPACE(sym)) "@GOT\n"
#    endif
#    define QT_VERSION_TAG(sym) \
    asm (   \
    ".section .qtversion, \"aG\", @progbits, " QT_STRINGIFY(QT_MANGLE_NAMESPACE(sym)) ", comdat\n" \
    ".align 8\n" \
    QT_VERSION_TAG_RELOC(sym) \
    ".long " QT_STRINGIFY(QT_VERSION) "\n" \
    ".align 8\n" \
    ".previous" \
    )
#  endif
#endif

#if defined(QT_VERSION_TAG)
QT_VERSION_TAG(qt_version_tag);
#endif

QT_END_NAMESPACE

#endif // QVERSIONTAGGING_H
